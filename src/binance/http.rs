use crate::binance::{
    error::BinanceError,
    exchange_info::BinanceExchangeInfoResult,
    signed::{BinanceParams, BinanceUnsignedParams},
    spot::BinanceSpotOrderResult,
};
use serde::Deserialize;

pub type BinanceHttpUnsignedRequest = BinanceUnsignedParams;
pub type BinanceHttpRequest = BinanceParams;

#[derive(Debug, Clone, Deserialize)]
pub struct BinanceHttpResponse {
    #[serde(flatten)]
    pub result: Option<BinanceHttpResponseResult>,
    pub error: Option<BinanceError>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum BinanceHttpResponseResult {
    ExchangeInfo(BinanceExchangeInfoResult),
    SpotOrderResponse(BinanceSpotOrderResult),
}
