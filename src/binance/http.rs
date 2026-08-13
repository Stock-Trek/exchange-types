use crate::binance::{
    error::BinanceError,
    exchange_info::BinanceExchangeInfoResult,
    signed::{BinanceParams, BinanceUnsignedParams},
    spot::BinanceSpotOrderResult,
};
use serde::{Deserialize, Serialize};

pub type BinanceHttpUnsignedRequest = BinanceUnsignedParams;
pub type BinanceHttpRequest = BinanceParams;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinanceHttpResponse {
    #[serde(flatten)]
    pub result: Option<BinanceHttpResponseResult>,
    pub error: Option<BinanceError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BinanceHttpResponseResult {
    ExchangeInfo(BinanceExchangeInfoResult),
    SpotOrder(BinanceSpotOrderResult),
}
