use crate::{
    binance::{
        amend::{BinanceAmendOrderParams, BinanceAmendOrderResult},
        asset_limits::BinanceAssetLimitsParams,
        cancel::{
            BinanceCancelAllOrdersParams, BinanceCancelOrderParams, BinanceCancelOrderResult,
        },
        error::BinanceError,
        exchange_info::{BinanceExchangeInfoParams, BinanceExchangeInfoResult},
        filters::BinanceAssetFilter,
        signed::BinanceSignedParams,
        spot::{BinanceSpotOrderParams, BinanceSpotOrderResult},
        time::{BinanceTimeParams, BinanceTimeResult},
    },
    http_method::HttpMethod,
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
    AmendOrderRequest(BinanceAmendOrderParams),
    AssetLimits(BinanceAssetLimitsParams),
    CancelAllOrdersRequest(BinanceCancelAllOrdersParams),
    CancelOrderRequest(BinanceCancelOrderParams),
    ExchangeInfo(BinanceExchangeInfoParams),
    SpotOrderRequest(Box<BinanceSpotOrderParams>),
    Time(BinanceTimeParams),
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
    AmendOrder(BinanceAmendOrderResult),
    AssetLimits(Vec<BinanceAssetFilter>),
    CancelAllOrders(Vec<BinanceCancelOrderResult>),
    CancelOrder(BinanceCancelOrderResult),
    ExchangeInfo(BinanceExchangeInfoResult),
    SpotOrder(BinanceSpotOrderResult),
    Time(BinanceTimeResult),
}

impl BinanceHttpUnsignedRequest {
    pub fn http_method(&self) -> HttpMethod {
        match self {
            BinanceHttpUnsignedRequest::AmendOrderRequest(..) => HttpMethod::PUT,
            BinanceHttpUnsignedRequest::AssetLimits(..) => HttpMethod::GET,
            BinanceHttpUnsignedRequest::CancelAllOrdersRequest(..) => HttpMethod::DELETE,
            BinanceHttpUnsignedRequest::CancelOrderRequest(..) => HttpMethod::DELETE,
            BinanceHttpUnsignedRequest::ExchangeInfo(..) => HttpMethod::GET,
            BinanceHttpUnsignedRequest::SpotOrderRequest(..) => HttpMethod::POST,
            BinanceHttpUnsignedRequest::Time(..) => HttpMethod::GET,
        }
    }
    pub fn endpoint(&self) -> &str {
        match self {
            BinanceHttpUnsignedRequest::AmendOrderRequest(..) => "order/cancelReplace",
            BinanceHttpUnsignedRequest::AssetLimits(..) => "myFilters",
            BinanceHttpUnsignedRequest::CancelAllOrdersRequest(..) => "openOrders",
            BinanceHttpUnsignedRequest::CancelOrderRequest(..) => "order",
            BinanceHttpUnsignedRequest::ExchangeInfo(..) => "exchangeInfo",
            BinanceHttpUnsignedRequest::SpotOrderRequest(..) => "order",
            BinanceHttpUnsignedRequest::Time(..) => "time",
        }
    }
}
