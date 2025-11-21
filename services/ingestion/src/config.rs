use std::env;

#[derive(Debug, Clone)]
pub struct IngestionConfig{
    pub kafka_brokers: String,
    pub swap_raw_topic: String,
}

impl IngestionConfig {
    pub fn from_env() -> Self {
        let kafka_brokers = env::var("KAFKA_BROKERS").unwrap_or_else(|_| "localhost:9092".to_string());
        let swap_raw_topic = env::var("SWAPS_RAW_TOPIC").unwrap_or_else(|_| "swaps_raw".to_string());
        Self{
            kafka_brokers,
            swap_raw_topic,
        }
    }
}