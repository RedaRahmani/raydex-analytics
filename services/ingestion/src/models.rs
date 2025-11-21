use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RawSwapEvent {
    pub market : String,
    pub tx_signature : String,
    pub slot : u64,
    pub timestamp_ms : i64,
    pub in_mint : String,
    pub out_mint : String,
    pub in_amount: u64,
    pub out_amount: u64,
}