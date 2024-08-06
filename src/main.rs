use chrono::DateTime;
use clap::Parser;
use futures::stream::{FuturesUnordered, StreamExt};
use indicatif::ProgressBar;
use openapi::{
    apis::{
        assets_api::{delete_assets, update_assets},
        configuration::{ApiKey, Configuration},
        timeline_api::{get_time_bucket, get_time_buckets},
        Error,
    },
    models::{
        AssetBulkDeleteDto, AssetBulkUpdateDto, AssetResponseDto, TimeBucketResponseDto,
        TimeBucketSize,
    },
};
// use serde::{Deserialize, Serialize};
use itertools::Itertools;
use std::collections::{BTreeMap, HashMap};
use std::fs::{self, File};
use std::io::{self, BufReader, BufWriter};
use std::time::Duration;
use thiserror::Error;
use tracing::info;
use uuid::Uuid;

#[derive(Parser, Clone)]
struct Opt {
    #[arg(short, long)]
    group: bool,
    #[arg(short, long)]
    endpoint: String,
    #[arg(short, long)]
    api_key: String,
}

impl From<Opt> for Configuration {
    fn from(opt: Opt) -> Self {
        Self {
            base_path: opt.endpoint,
            api_key: Some(ApiKey {
                key: opt.api_key.clone(),
                prefix: None,
            }),
            client: reqwest::Client::new(),
            user_agent: Some("OpenAPI-Generator/1.111.0/rust".to_owned()),
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: None,
        }
    }
}

struct Client {
    configuration: Configuration,
}

#[derive(Debug, Error)]
enum ClientError {
    #[error("api error")]
    Api(Box<dyn std::error::Error>),
    #[error("error parsing uuid: {0}")]
    UuidParse(#[from] uuid::Error),
    #[error("error parsing date time: {0}")]
    DateTimeParse(#[from] chrono::ParseError),
    #[error("error joining task: {0}")]
    Join(#[from] tokio::task::JoinError),
    #[error("error io: {0}")]
    Io(#[from] io::Error),
}

impl<T> From<Error<T>> for ClientError
where
    T: std::fmt::Debug + 'static,
{
    fn from(e: Error<T>) -> Self {
        Self::Api(e.into())
    }
}

impl Client {
    fn new(configuration: Configuration) -> Self {
        Self { configuration }
    }

    async fn get_buckets(&self) -> Result<Vec<TimeBucketResponseDto>, ClientError> {
        Ok(get_time_buckets(
            &self.configuration,
            TimeBucketSize::Month,
            None,
            Some(false),
            None,
            Some(false),
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await?)
    }

    async fn count_photos(&self) -> Result<usize, ClientError> {
        Ok(self
            .get_buckets()
            .await?
            .into_iter()
            .map(|b| b.count as usize)
            .sum())
    }

    async fn get_bucket(
        &self,
        bucket: TimeBucketResponseDto,
    ) -> Result<Vec<AssetResponseDto>, ClientError> {
        Ok(get_time_bucket(
            &self.configuration,
            TimeBucketSize::Month,
            &bucket.time_bucket,
            None,
            Some(false),
            None,
            Some(false),
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await?)
    }

    async fn delete_asset(&self, id: Uuid) -> Result<(), ClientError> {
        Ok(delete_assets(
            &self.configuration,
            AssetBulkDeleteDto {
                force: None,
                ids: vec![id],
            },
        )
        .await?)
    }

    async fn group_assets(&self, ids: Vec<Uuid>) -> Result<(), ClientError> {
        Ok(update_assets(
            &self.configuration,
            AssetBulkUpdateDto {
                ids: ids.clone(),
                stack_parent_id: Some(ids[0]),
                ..Default::default()
            },
        )
        .await?)
    }
}

const CACHE_FILE: &str = "assets_cache.json";
const CACHE_DURATION: Duration = Duration::from_secs(600); // 10 minutes

async fn load_assets_from_cache() -> io::Result<HashMap<Uuid, AssetResponseDto>> {
    let file = File::open(CACHE_FILE)?;
    let reader = BufReader::new(file);
    let assets: HashMap<Uuid, AssetResponseDto> = serde_json::from_reader(reader)?;
    Ok(assets)
}

async fn save_assets_to_cache(assets: &HashMap<Uuid, AssetResponseDto>) -> io::Result<()> {
    let file = File::create(CACHE_FILE)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, assets)?;
    Ok(())
}

fn is_cache_valid() -> bool {
    if let Ok(metadata) = fs::metadata(CACHE_FILE) {
        if let Ok(modified) = metadata.modified() {
            if let Ok(elapsed) = modified.elapsed() {
                return elapsed < CACHE_DURATION;
            }
        }
    }
    false
}

#[tokio::main]
async fn main() -> Result<(), ClientError> {
    tracing_subscriber::fmt::init();

    let opt = Opt::parse();

    let configuration = Configuration::from(opt.clone());

    let client = Client::new(configuration.clone());
    let count = client.count_photos().await?;
    info!("Library contatins {} photos.", count);

    let assets = if is_cache_valid() {
        load_assets_from_cache().await?
    } else {
        let mut assets = HashMap::with_capacity(count);
        let buckets = client.get_buckets().await?;
        let bar = ProgressBar::new(count as u64);
        let mut tasks = FuturesUnordered::new();

        for bucket in buckets {
            let task = client.get_bucket(bucket);
            tasks.push(task);
        }

        while let Some(result) = tasks.next().await.transpose().unwrap_or(None) {
            bar.inc(result.len() as u64);
            for asset in result {
                let id = Uuid::parse_str(&asset.id)?;
                assets.insert(id, asset);
            }
        }

        save_assets_to_cache(&assets).await?;
        bar.finish();
        assets
    };

    // build binary tree
    let mut assets_tree: BTreeMap<_, Vec<&Uuid>> = BTreeMap::new();
    for (id, asset) in assets.iter() {
        let date = DateTime::parse_from_rfc3339(&asset.file_created_at)?;
        assets_tree.entry(date).or_default().push(id);
    }

    let mut groups = Vec::<Vec<&Uuid>>::new();
    // check for assets with same timestamp
    let mut group = Vec::new();
    let mut ongoing = false;
    for (this, next) in assets_tree.iter().tuple_windows() {
        if next.0.timestamp_millis() - this.0.timestamp_millis() <= 1000 {
            if !ongoing {
                group.extend(this.1);
                ongoing = true;
            }
            group.extend(next.1);
        } else if ongoing {
            group.extend(this.1);
            if group.len() > 1 {
                groups.push(group)
            };
            group = Vec::new();
            ongoing = false;
        }
    }


    Ok(())
}
