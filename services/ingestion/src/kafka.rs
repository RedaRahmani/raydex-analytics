use anyhow::Result;
use log::{error, info};
use tokio::time::Duration;
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};

use crate::models::RawSwapEvent;

pub fn build_producer(brokers: &str)-> Result<FutureProducer>{
    let producer : FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()?;
    Ok(producer)
}

pub async fn send_raw_swap_event (
    producer : &FutureProducer,
    topic: &str,
    event : &RawSwapEvent,  
) -> Result<()> {
    let payload = serde_json::to_string(event)?;
    let record = FutureRecord::to(topic)
        .payload(&payload)
        .key(&event.market);
    match producer.send(record, Duration::from_millis(0)).await {
        Ok(delivery) => {
            info!("sent RawSwapEvent to topic={} delivery={:?} ", topic, delivery);
        }
        Err((e, _msg)) => {
            error!("Failed to send RawSwapEvent: {}", e)
        }
    }
    Ok(())
}