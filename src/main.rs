use actix_web::{
    get,
    http::StatusCode,
    web::{self, Data},
    App, HttpServer, Responder,
};
use chrono::DateTime;
use clap::Parser;
use futures::stream::{FuturesUnordered, StreamExt};
use indicatif::ProgressBar;
use itertools::Itertools;
use leptos::*;
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
use serde::Serialize;
use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};
use thiserror::Error;
use tokio::sync::RwLock;
use tracing::info;
use utoipa::{OpenApi, ToSchema};
use utoipa_redoc::{Redoc, Servable};
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
    assets: Option<HashMap<Uuid, AssetResponseDto>>,
    count: Option<usize>,
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
        Self {
            configuration,
            assets: None,
            count: None,
        }
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

    async fn get_count(&mut self) -> Result<usize, ClientError> {
        if self.count.is_none() {
            self.count = Some(self.count_photos().await?);
        }
        Ok(self.count.unwrap())
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

    async fn set_assets(&mut self) -> Result<(), ClientError> {
        if self.assets.is_none() {
            info!("Fetching assets");
            let buckets = self.get_buckets().await?;
            let bar = ProgressBar::new(self.get_count().await? as u64);
            let mut assets = HashMap::with_capacity(self.get_count().await?);
            let mut tasks = FuturesUnordered::new();

            for bucket in buckets {
                let task = self.get_bucket(bucket);
                tasks.push(task);
            }

            while let Some(result) = tasks.next().await.transpose().unwrap_or(None) {
                bar.inc(result.len() as u64);
                for asset in result {
                    let id = Uuid::parse_str(&asset.id)?;
                    assets.insert(id, asset);
                }
            }

            drop(tasks);

            bar.finish();
            self.assets = Some(assets);
        } else {
            info!("Assets already fetched");
        }
        info!(
            "Assets fetched, count: {}",
            self.assets.as_ref().unwrap().len()
        );
        Ok(())
    }

    async fn generate_groups(&mut self) -> Result<Vec<Vec<Uuid>>, ClientError> {
        self.set_assets().await?;

        let mut assets_tree: BTreeMap<_, Vec<&Uuid>> = BTreeMap::new();
        for (id, asset) in self.assets.as_ref().unwrap().iter() {
            let date = DateTime::parse_from_rfc3339(&asset.file_created_at)?;
            assets_tree.entry(date).or_default().push(id);
        }

        let mut groups = Vec::<Vec<Uuid>>::new();
        // Check for assets with the same timestamp
        let mut group = Vec::new();
        let mut ongoing = false;
        for (this, next) in assets_tree.iter().tuple_windows() {
            if next.0.timestamp_millis() - this.0.timestamp_millis() <= 1000 {
                if !ongoing {
                    group.extend(this.1.iter().cloned());
                    ongoing = true;
                }
                group.extend(next.1.iter().cloned());
            } else if ongoing {
                group.extend(this.1.iter().cloned());
                if group.len() > 1 {
                    groups.push(group)
                };
                group = Vec::new();
                ongoing = false;
            }
        }
        if !group.is_empty() && group.len() > 1 {
            groups.push(group);
        }

        Ok(groups)
    }
}

impl actix_web::error::ResponseError for ClientError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

#[derive(Serialize, ToSchema)]
struct GroupResponse {
    groups: Vec<Vec<Uuid>>,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Groups of assets", body = GroupResponse),
    )
)]
#[get("/groups")]
async fn get_groups(client: web::Data<RwLock<Client>>) -> Result<impl Responder, ClientError> {
    // Generate groups
    let groups = client.write().await.generate_groups().await?;

    // Return groups as JSON
    Ok(web::Json(GroupResponse { groups }))
}

#[derive(OpenApi)]
#[openapi(paths(get_groups), components(schemas(GroupResponse)))]
struct ApiDoc;

fn main()  {
    tracing_subscriber::fmt::init();
    mount_to_body(|| view! { <p>"Hello, world!"</p> })
    // let openapi = ApiDoc::openapi();
    // let client = Data::new(RwLock::new(Client::new(Opt::parse().into())));
    // HttpServer::new(move || {
    //     App::new()
    //         .app_data(client.clone())
    //         .service(get_groups)
    //         .service(Redoc::with_url("/redoc", openapi.clone()))
    // })
    // .bind("127.0.0.1:8080")?
    // .run()
    // .await;

}
