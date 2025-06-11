mod oanda_api;
mod errors;
mod models;

use crate::errors::AppError;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    oanda_api::connect_and_stream_pricing().await
}
