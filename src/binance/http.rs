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
    crate::error::ETError,
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

impl IntoSigned for BinanceHttpUnsignedRequest {
    type Signed = BinanceHttpRequest;

    fn into_signed(self, signer: &Signer) -> ETResult<BinanceHttpRequest> {
        let query_string = match &self {
            BinanceHttpUnsignedRequest::AmendOrderRequest(params) => {
                Some(params.query_params(true))
            }
            BinanceHttpUnsignedRequest::AssetLimits(params) => Some(params.query_params(true)),
            BinanceHttpUnsignedRequest::CancelAllOrdersRequest(params) => {
                Some(params.query_params(true))
            }
            BinanceHttpUnsignedRequest::CancelOrderRequest(params) => {
                Some(params.query_params(true))
            }
            BinanceHttpUnsignedRequest::SpotOrderRequest(params) => Some(params.query_params(true)),
            _ => None,
        };
        let signature = match query_string {
            Some(query_string) => Some(signer.signature(&query_string.into_bytes())?),
            None => None,
        };
        Ok(BinanceHttpRequest {
            unsigned: self,
            signature: match signature {
                Some(signature) => Some(BinanceSignature {
                    apiKey: signer.api_key(),
                    signature,
                }),
                None => None,
            },
        })
    }
}

impl BinanceHttpRequest {
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
    #[cfg(feature = "serde")]
    pub fn serialize(&self) -> ETResult<String> {
        serde_json::to_string(self).map_err(ETError::SerializeRequest)
    }
}
