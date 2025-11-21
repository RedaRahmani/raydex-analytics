mod config;
mod kafka;
mod models;

use anyhow::Result;
use log::info;
use tokio::time::{sleep, Duration};

use crate::config::IngestionConfig;
use crate::kafka::{build_producer, send_raw_swap_event};
use crate::models::RawSwapEvent;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let cfg = IngestionConfig::from_env();
    info!("Starting Ingestion with config: {:?}", cfg);

    let produce = build_producer(&cfg.kafka_brokers)?;
    let mut counter: u64 = 0;

    loop{
        let event = RawSwapEvent {
            market: "RAY_USDC".to_string(),
            tx_signature: format!("dummy sig_{}", counter),
            slot : 123_456 + counter,
            timestamp_ms: chrono::Utc::now().timestamp_millis(),
            in_mint: "So11111111111111111111111111111111111111112".to_string(),
            out_mint: "USDC11111111111111111111111111111111111111".to_string(),
            in_amount: 100_000_000,
            out_amount: 10_000_000,
        };
        send_raw_swap_event(&produce, &cfg.swap_raw_topic, &event).await?;
        counter += 1;
        sleep(Duration::from_secs(5)).await;
    }
}