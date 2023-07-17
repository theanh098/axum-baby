use crate::database::prisma::{business, PrismaClient};
use anyhow::Result;
use std::{collections::HashMap, env};
use tokio_cron_scheduler::{Job, JobScheduler};

#[axum::async_trait]
pub trait CmcCrawling {
  async fn crawl_cmc(&self) -> Result<()>;
}

#[derive(serde::Serialize)]
struct CmcQuery {
  id: String,
}

#[derive(serde::Deserialize)]
struct CmcResponse {
  data: HashMap<String, serde_json::Value>,
}

#[axum::async_trait]
impl CmcCrawling for JobScheduler {
  async fn crawl_cmc(&self) -> Result<()> {
    self
      .add(Job::new("1/10 * * * * *", |_uuid, _l| {
        println!("I run every 10 seconds");
      })?)
      .await?;

    Ok(())
  }
}

async fn crawl_cryptocurrency_quotes(
  prisma_client: &PrismaClient,
  redis_client: &redis::Client,
) -> Result<(), surf::Error> {
  let mut i = 1;
  let chunk_size = 50;

  loop {
    let businesses = prisma_client
      .business()
      .find_many(vec![
        business::token::not(None),
        business::cmc_id::not(None),
      ])
      .order_by(business::id::order(prisma_client_rust::Direction::Asc))
      .take(chunk_size)
      .skip((i - 1) * chunk_size)
      .select(business::select!({ cmc_id }))
      .exec()
      .await?;

    if businesses.is_empty() {
      return Ok(());
    }

    let cmc_query = CmcQuery {
      id: businesses
        .iter()
        .map(|b| b.cmc_id.unwrap().to_string())
        .collect::<Vec<String>>()
        .join(","),
    };

    let cmc_data = surf::get("https://pro-api.coinmarketcap.com/v2/cryptocurrency/quotes/latest")
      .header(
        "X-CMC_PRO_API_KEY",
        env::var("CMC_KEY").expect("CMC_KEY must be set"),
      )
      .query(&cmc_query)
      .unwrap()
      .await?
      .body_json::<CmcResponse>()
      .await?;

    cmc_query.id.s
  }
}
