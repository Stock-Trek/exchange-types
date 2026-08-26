use crate::binance::{
    asset_limits::BinanceAssetLimitsParams,
    error::BinanceError,
    exchange_info::{BinanceExchangeInfoParams, BinanceExchangeInfoResult},
    filters::BinanceAssetFilter,
    signed::BinanceSignedParams,
    spot::{BinanceSpotOrderParams, BinanceSpotOrderResult},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[derive(Debug, Clone)]
pub enum BinanceHttpBody {
    Request(BinanceHttpRequest),
    Response(BinanceHttpResponse),
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[derive(Debug, Clone, Hash)]
pub enum BinanceHttpUnsignedRequest {
    AssetLimits(BinanceAssetLimitsParams),
    ExchangeInfo(BinanceExchangeInfoParams),
    SpotOrderRequest(Box<BinanceSpotOrderParams>),
}

pub type BinanceHttpRequest = BinanceSignedParams<BinanceHttpUnsignedRequest>;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[derive(Debug, Clone)]
pub enum BinanceHttpResponse {
    Result(BinanceHttpResponseResult),
    Error(BinanceError),
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[derive(Debug, Clone)]
pub enum BinanceHttpResponseResult {
    AssetLimits(Vec<BinanceAssetFilter>),
    ExchangeInfo(BinanceExchangeInfoResult),
    SpotOrder(BinanceSpotOrderResult),
}
