use reqwest::Client;
use futures_util::StreamExt;
use std::env;

use crate::errors::AppError;
use crate::models::StreamMessage;

pub async fn connect_and_stream_pricing() -> Result<(), AppError> {
    let auth_token = env::var("OANDA_AUTH_TOKEN")
        .map_err(|e| AppError::EnvVar("OANDA_AUTH_TOKEN".to_string(), e))?;
    let account_id = env::var("OANDA_ACCOUNT_ID")
        .map_err(|e| AppError::EnvVar("OANDA_ACCOUNT_ID".to_string(), e))?;

    let oanda_env_type_var_name = "OANDA_ENV_TYPE";
    let oanda_env_type_str = match env::var(oanda_env_type_var_name) {
        Ok(val) => {
            if val.is_empty() {
                eprintln!("Warning: Environment variable '{}' is empty. Using default: fxpractice", oanda_env_type_var_name);
                "fxpractice".to_string()
            } else {
                val.to_lowercase()
            }
        },
        Err(env::VarError::NotPresent) => {
            eprintln!("Warning: Environment variable '{}' not set. Using default: fxpractice", oanda_env_type_var_name);
            "fxpractice".to_string()
        },
        Err(e) => return Err(AppError::EnvVar(oanda_env_type_var_name.to_string(), e)),
    };

    let base_endpoint = match oanda_env_type_str.as_str() {
        "fxtrade" => "https://stream-fxtrade.oanda.com".to_string(),
        "fxpractice" => "https://stream-fxpractice.oanda.com".to_string(),
        _ => return Err(AppError::Custom(format!(
            "Invalid OANDA_ENV_TYPE value '{}'. Must be 'fxtrade' or 'fxpractice'.",
            oanda_env_type_str
        ))),
    };

    let instruments_var_name = "OANDA_INSTRUMENTS";
    let instruments = match env::var(instruments_var_name) {
        Ok(val) => {
            if val.is_empty() {
                eprintln!("Warning: Environment variable '{}' is empty. Using default: EUR_USD", instruments_var_name);
                "EUR_USD".to_string()
            } else {
                val
            }
        },
        Err(env::VarError::NotPresent) => {
            eprintln!("Warning: Environment variable '{}' not set. Using default: EUR_USD", instruments_var_name);
            "EUR_USD".to_string()
        },
        Err(e) => return Err(AppError::EnvVar(instruments_var_name.to_string(), e)),
    };

    let base_url_without_params = format!(
        "{}/v3/accounts/{}/pricing/stream",
        base_endpoint, account_id
    );

    let client = Client::new();
    let response = client
        .get(&base_url_without_params)
        .query(&[("instruments", instruments.clone())])
        .header("Authorization", format!("Bearer {}", auth_token))
        .send()
        .await?;

    let status = response.status();

    if !status.is_success() {
        eprintln!("Error: Received non-success status: {}", status);
        let body = response.text().await?;
        eprintln!("Response body: {}", body);
        return Err(AppError::Custom(format!(
            "Failed to connect to Oanda stream: HTTP status {}. Body: {}",
            status,
            body
        )));
    }

    println!("Connected to Oanda pricing stream from: {}", base_url_without_params);
    println!("Streaming instruments: {}", instruments);

    let mut stream = response.bytes_stream();
    let mut buffer = Vec::new();

    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result?;

        buffer.extend_from_slice(&chunk);

        while let Some(newline_pos) = buffer.iter().position(|&b| b == b'\n') {
            let line_bytes = buffer.drain(..newline_pos + 1).collect::<Vec<u8>>();

            let line = String::from_utf8(line_bytes)?;

            let trimmed_line = line.trim();
            if trimmed_line.is_empty() {
                continue;
            }

            match serde_json::from_str::<StreamMessage>(trimmed_line) {
                Ok(message) => {
                    match message {
                        StreamMessage::Pricing(pricing) => {
                            println!("Pricing Update: {:?}", pricing);
                        }
                        StreamMessage::Heartbeat(heartbeat) => {
                            println!("Heartbeat: {:?}", heartbeat);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to parse JSON: {} for line: {}", e, trimmed_line);
                }
            }
        }
    }

    println!("Oanda pricing stream ended gracefully.");
    Ok(())
}