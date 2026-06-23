use crate::binance::{
    error::BinanceError,
    exchange_info::BinanceExchangeInfoResult,
    signed::{BinanceParams, BinanceUnsignedParams},
    spot::BinanceSpotOrderResult,
};
use serde::Deserialize;

pub type BinanceHttpUnsignedMessage = BinanceUnsignedParams;
pub type BinanceHttpSignedMessage = BinanceParams;

#[derive(Debug, Deserialize)]
pub struct BinanceHttpResponse {
    #[serde(flatten)]
    pub result: BinanceHttpResponseResult,
    pub error: Option<BinanceError>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum BinanceHttpResponseResult {
    ExchangeInfo(BinanceExchangeInfoResult),
    SpotOrderResponse(BinanceSpotOrderResult),
}
