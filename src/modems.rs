use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StreamMessage {
    Pricing(PricingMessage),
    Heartbeat(HeartbeatMessage),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PricingMessage {
    pub asks: Vec<LiquidityPrice>,
    pub bids: Vec<LiquidityPrice>,
    pub closeout_ask: String,
    pub closeout_bid: String,
    pub instrument: String,
    pub status: String,
    pub time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LiquidityPrice {
    pub liquidity: u64,
    pub price: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeartbeatMessage {
    #[serde(rename = "type")]
    pub message_type: String,
    pub time: String,
}