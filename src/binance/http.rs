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
        signature::BinanceSignature,
        spot::{BinanceSpotOrderParams, BinanceSpotOrderResult},
        time::{BinanceTimeParams, BinanceTimeResult},
    },
    error::ETResult,
    http_method::HttpMethod,
    signer::{IntoSigned, Signer},
};

#[cfg(feature = "serde")]
use {
    serde::{Deserialize, Serialize},
    serde_with::skip_serializing_none,
};

#[cfg_attr(feature = "serde", derive(Serialize))]
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

#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", skip_serializing_none)]
#[derive(Debug, Clone)]
pub struct BinanceHttpRequest {
    pub unsigned: BinanceHttpUnsignedRequest,
    pub signature: Option<BinanceSignature>,
}

#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[derive(Debug, Clone)]
pub enum BinanceHttpResponse {
    Success(BinanceHttpResponseResult),
    Failure(BinanceError),
}

#[cfg_attr(feature = "serde", derive(Deserialize))]
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
    fn query_params(&self) -> String {
        match &self {
            BinanceHttpUnsignedRequest::AmendOrderRequest(params) => params.query_params(true),
            BinanceHttpUnsignedRequest::AssetLimits(params) => params.query_params(true),
            BinanceHttpUnsignedRequest::CancelAllOrdersRequest(params) => params.query_params(true),
            BinanceHttpUnsignedRequest::CancelOrderRequest(params) => params.query_params(true),
            BinanceHttpUnsignedRequest::SpotOrderRequest(params) => params.query_params(true),
            BinanceHttpUnsignedRequest::ExchangeInfo(params) => params.query_params(),
            BinanceHttpUnsignedRequest::Time(params) => params.query_params(),
        }
    }
}

impl IntoSigned for BinanceHttpUnsignedRequest {
    type Signed = BinanceHttpRequest;

    fn into_signed(self, signer: &Signer) -> ETResult<BinanceHttpRequest> {
        let query_string = self.query_params();
        let signature = signer.signature(&query_string.into_bytes())?;
        Ok(BinanceHttpRequest {
            unsigned: self,
            signature: Some(BinanceSignature {
                apiKey: signer.api_key(),
                signature,
            }),
        })
    }
}

impl BinanceHttpRequest {
    pub fn endpoint(&self) -> &str {
        match self.unsigned {
            BinanceHttpUnsignedRequest::AmendOrderRequest(..) => "order/cancelReplace",
            BinanceHttpUnsignedRequest::AssetLimits(..) => "myFilters",
            BinanceHttpUnsignedRequest::CancelAllOrdersRequest(..) => "openOrders",
            BinanceHttpUnsignedRequest::CancelOrderRequest(..) => "order",
            BinanceHttpUnsignedRequest::ExchangeInfo(..) => "exchangeInfo",
            BinanceHttpUnsignedRequest::SpotOrderRequest(..) => "order",
            BinanceHttpUnsignedRequest::Time(..) => "time",
        }
    }
    pub fn http_method(&self) -> HttpMethod {
        match self.unsigned {
            BinanceHttpUnsignedRequest::AssetLimits(..)
            | BinanceHttpUnsignedRequest::ExchangeInfo(..)
            | BinanceHttpUnsignedRequest::Time(..) => HttpMethod::GET,
            BinanceHttpUnsignedRequest::CancelAllOrdersRequest(..)
            | BinanceHttpUnsignedRequest::CancelOrderRequest(..) => HttpMethod::DELETE,
            BinanceHttpUnsignedRequest::SpotOrderRequest(..) => HttpMethod::POST,
            BinanceHttpUnsignedRequest::AmendOrderRequest(..) => HttpMethod::PUT,
        }
    }
    pub fn headers(&self) -> Vec<(String, String)> {
        match &self.signature {
            Some(signature) => vec![("X-MBX-APIKEY".into(), signature.apiKey.clone())],
            None => vec![],
        }
    }
    pub fn query_params(&self) -> String {
        let query_params = self.unsigned.query_params();
        match &self.signature {
            Some(signature) => format!("{}&signature={}", query_params, signature.signature),
            None => query_params,
        }
    }
}
