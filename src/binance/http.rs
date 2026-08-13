use crate::binance::{
    error::BinanceError,
    exchange_info::BinanceExchangeInfoResult,
    signed::{BinanceParams, BinanceUnsignedParams},
    spot::BinanceSpotOrderResult,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type BinanceHttpUnsignedRequest = BinanceUnsignedParams;
pub type BinanceHttpRequest = BinanceParams;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct BinanceHttpResponse {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub result: Option<BinanceHttpResponseResult>,
    pub error: Option<BinanceError>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[derive(Debug, Clone)]
pub enum BinanceHttpResponseResult {
    ExchangeInfo(BinanceExchangeInfoResult),
    SpotOrder(BinanceSpotOrderResult),
}
