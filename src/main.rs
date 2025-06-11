mod errors;
mod models;
mod oanda_api;

use crate::errors::AppError;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    oanda_api::connect_and_stream_pricing().await
}