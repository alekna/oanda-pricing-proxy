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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use chrono::{TimeZone, Utc, Timelike};
    use serde::de::IntoDeserializer;

    #[test]
    fn test_pricing_message_deserialization() {
        let json_data = r#"
            {
                "asks": [
                    {"price": "1.09005", "liquidity": 1000000}
                ],
                "bids": [
                    {"price": "1.09000", "liquidity": 1000000}
                ],
                "closeoutAsk": "1.09015",
                "closeoutBid": "1.08990",
                "instrument": "EUR_USD",
                "status": "active",
                "time": "2024-06-11T14:30:00.123456789Z"
            }
        "#;
        let pricing: PricingMessage = serde_json::from_str(json_data).unwrap();

        assert_eq!(pricing.instrument, "EUR_USD");
        assert_eq!(pricing.asks[0].price, "1.09005");
        assert_eq!(pricing.asks[0].liquidity, 1000000);
        assert_eq!(pricing.time, Utc.with_ymd_and_hms(2024, 6, 11, 14, 30, 0).unwrap().with_nanosecond(123456789).unwrap().timestamp_nanos_opt().unwrap());
    }

    #[test]
    fn test_heartbeat_message_deserialization() {
        let json_data = r#"
            {
                "type": "HEARTBEAT",
                "time": "2024-06-11T14:31:05.987654321Z"
            }
        "#;
        let heartbeat: HeartbeatMessage = serde_json::from_str(json_data).unwrap();

        assert_eq!(heartbeat.message_type, "HEARTBEAT");
        assert_eq!(heartbeat.time, Utc.with_ymd_and_hms(2024, 6, 11, 14, 31, 5).unwrap().with_nanosecond(987654321).unwrap().timestamp_nanos_opt().unwrap());
    }

    #[test]
    fn test_stream_message_pricing_variant() {
        let json_data = r#"
            {
                "asks": [],
                "bids": [],
                "closeoutAsk": "1.0",
                "closeoutBid": "1.0",
                "instrument": "AUD_CAD",
                "status": "active",
                "time": "2024-06-11T14:32:00.000000000Z"
            }
        "#;
        let stream_msg: StreamMessage = serde_json::from_str(json_data).unwrap();

        match stream_msg {
            StreamMessage::Pricing(p) => {
                assert_eq!(p.instrument, "AUD_CAD");
            },
            _ => panic!("Expected PricingMessage variant"),
        }
    }

    #[test]
    fn test_stream_message_heartbeat_variant() {
        let json_data = r#"
            {
                "type": "HEARTBEAT",
                "time": "2024-06-11T14:33:00.000000000Z"
            }
        "#;
        let stream_msg: StreamMessage = serde_json::from_str(json_data).unwrap();

        match stream_msg {
            StreamMessage::Heartbeat(h) => {
                assert_eq!(h.message_type, "HEARTBEAT");
            },
            _ => panic!("Expected HeartbeatMessage variant"),
        }
    }

    #[test]
    fn test_deserialize_oanda_time_valid() {
        let json_string = serde_json::json!("2024-06-11T14:34:00.123456789Z");
        let timestamp = deserialize_oanda_time_to_nanoseconds(json_string.into_deserializer()).unwrap();
        assert_eq!(timestamp, Utc.with_ymd_and_hms(2024, 6, 11, 14, 34, 0).unwrap().with_nanosecond(123456789).unwrap().timestamp_nanos_opt().unwrap());
    }

    #[test]
    fn test_deserialize_oanda_time_invalid() {
        let json_string = serde_json::json!("not a valid time string");
        let result = deserialize_oanda_time_to_nanoseconds(json_string.into_deserializer());
        assert!(result.is_err());
    }
}
