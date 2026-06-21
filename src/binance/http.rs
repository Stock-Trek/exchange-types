use crate::binance::{
    error::BinanceError,
    exchange_info::BinanceExchangeInfoResult,
    signed::{BinanceSignedParams, BinanceUnsignedParams},
    spot::BinanceSpotOrderResult,
};
use serde::Deserialize;

pub type BinanceHttpUnsignedMessage = BinanceUnsignedParams;
pub type BinanceHttpSignedMessage = BinanceSignedParams;

#[derive(Debug, Deserialize)]
pub struct BinanceHttpResponse {
    pub error: Option<BinanceError>,
    #[serde(flatten)]
    pub result: BinanceHttpResponseResult,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum BinanceHttpResponseResult {
    ExchangeInfo(BinanceExchangeInfoResult),
    SpotOrderResponse(BinanceSpotOrderResult),
}
