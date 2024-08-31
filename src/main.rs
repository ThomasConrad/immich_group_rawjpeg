use chrono::DateTime;
use clap::{Parser, ValueEnum};
use indicatif::ProgressBar;
use itertools::Itertools;
use openapi::{
    apis::{
        assets_api::{delete_assets, update_asset, update_assets},
        configuration::{ApiKey, Configuration},
        stacks_api::create_stack,
        timeline_api::{get_time_bucket, get_time_buckets},
        Error,
    },
    models::{
        AssetBulkDeleteDto, AssetBulkUpdateDto, AssetResponseDto, StackCreateDto,
        TimeBucketResponseDto, TimeBucketSize,
    },
};
use std::collections::{BTreeMap, HashMap};

use thiserror::Error;
use tracing::info;
use uuid::Uuid;

#[derive(ValueEnum, Debug, Clone)]
#[clap(rename_all = "kebab_case")]
enum Action {
    Duplicates,
    JpgArw,
    Bursts,
}

#[derive(Parser, Clone)]
struct Opt {
    #[arg(value_enum)]
    action: Action,
    #[arg(short, long)]
    dry_run: bool,
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
    #[error("error grouping assets: {0:?}")]
    InvalidGroup((Vec<Uuid>, Uuid)),
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
            Some(false),
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
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(false),
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

    async fn group_assets(&self, ids: Vec<Uuid>, parent: Uuid) -> Result<(), ClientError> {
        if !ids.contains(&parent) {
            return Err(ClientError::InvalidGroup((ids, parent)));
        }
        let res = create_stack(&self.configuration, StackCreateDto { asset_ids: ids }).await?;
        Ok(())
    }
}

async fn group_duplicates(
    client: Client,
    assets: &HashMap<Uuid, AssetResponseDto>,
    dry_run: bool,
) -> Result<(), ClientError> {
    let mut assets_map: HashMap<_, Vec<Uuid>> = HashMap::new();
    for (id, asset) in assets.iter() {
        let date = DateTime::parse_from_rfc3339(&asset.file_created_at)?;
        assets_map.entry(date).or_default().push(*id);
    }

    // check for assets with same timestamp
    for group in assets_map.into_iter() {
        if group.1.len() > 1 {
            // check that name is the same for all assets in the group
            for id in &group.1 {
                let asset = assets.get(id).unwrap();
                if asset.original_file_name != assets.get(&group.1[0]).unwrap().original_file_name {
                    continue;
                }
            }

            // group assets
            info!("Grouping assets: {:?}", group.1);
            if !dry_run {
                client.group_assets(group.1.clone(), group.1[0]).await?;
            }
        }
    }
    Ok(())
}

async fn group_jpeg_arw(
    client: Client,
    assets: &HashMap<Uuid, AssetResponseDto>,
    dry_run: bool,
) -> Result<(), ClientError> {
    let mut assets_map: HashMap<_, Vec<Uuid>> = HashMap::new();
    for (id, asset) in assets.iter() {
        let date = DateTime::parse_from_rfc3339(&asset.file_created_at)?;
        assets_map.entry(date).or_default().push(*id);
    }

    // check for assets with same timestamp
    for group in assets_map.into_iter() {
        if group.1.len() == 2 {
            let mut jpeg = None;
            let mut arw = None;

            for id in &group.1 {
                let asset = assets.get(id).unwrap();
                if asset.original_file_name.ends_with(".JPG") {
                    jpeg = Some((*id, asset.original_file_name.clone()));
                } else if asset.original_file_name.ends_with(".ARW") {
                    arw = Some((*id, asset.original_file_name.clone()));
                }
            }

            if let (Some(jpeg), Some(arw)) = (jpeg, arw) {
                info!("Grouping assets: {:?} {:?}", jpeg, arw);
                if !dry_run {
                    client.group_assets(vec![jpeg.0, arw.0], jpeg.0).await?;
                }
            }
        }
    }
    Ok(())
}

async fn group_bursts(
    client: Client,
    assets: &HashMap<Uuid, AssetResponseDto>,
    dry_run: bool,
) -> Result<(), ClientError> {
    // build binary tree
    let mut assets_tree: BTreeMap<_, Vec<Uuid>> = BTreeMap::new();
    for (id, asset) in assets.iter() {
        let date = DateTime::parse_from_rfc3339(&asset.file_created_at)?;
        assets_tree.entry(date).or_default().push(*id);
    }

    let mut groups = Vec::<Vec<Uuid>>::new();
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

    for group in groups {
        info!("Grouping assets: {:?}", group);

        if !dry_run {
            client.group_assets(group.to_vec(), group[0]).await?;
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), ClientError> {
    tracing_subscriber::fmt::init();

    let opt = Opt::parse();

    let configuration = Configuration::from(opt.clone());

    let client = Client::new(configuration.clone());
    let count = client.count_photos().await?;
    info!("Library contatins {} photos.", count);

    let mut assets = HashMap::with_capacity(count);
    let buckets = client.get_buckets().await?;
    let bar = ProgressBar::new(count as u64);

    for bucket in buckets {
        let result = client.get_bucket(bucket).await?;
        bar.inc(result.len() as u64);
        for asset in result {
            let id = Uuid::parse_str(&asset.id)?;
            assets.insert(id, asset);
        }
    }

    bar.finish();

    match opt.action {
        Action::Duplicates => group_duplicates(client, &assets, opt.dry_run).await?,
        Action::JpgArw => group_jpeg_arw(client, &assets, opt.dry_run).await?,
        Action::Bursts => group_bursts(client, &assets, opt.dry_run).await?,
    }

    Ok(())
}
