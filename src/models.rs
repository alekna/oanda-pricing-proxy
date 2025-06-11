use serde::{Deserialize, Deserializer, Serialize};
use chrono::{DateTime, Utc};

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
    #[serde(deserialize_with = "deserialize_oanda_time_to_nanoseconds")]
    pub time: i64,
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
    #[serde(deserialize_with = "deserialize_oanda_time_to_nanoseconds")]
    pub time: i64,
}

pub fn deserialize_oanda_time_to_nanoseconds<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let dt = s.parse::<DateTime<Utc>>()
        .map_err(serde::de::Error::custom)?;
    dt.timestamp_nanos_opt().ok_or_else(|| serde::de::Error::custom("timestamp out of range"))
}