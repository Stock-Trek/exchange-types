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
    error::EncryptResult,
    http_method::HttpMethod,
    signer::Signer,
};

#[cfg(feature = "serde")]
use {
    crate::error::ETError,
    serde::{Deserialize, Serialize},
};

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
    Success(BinanceHttpResponseResult),
    Failure(BinanceError),
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
    pub fn into_signed(self, signer: &Signer) -> EncryptResult<BinanceHttpRequest> {
        macro_rules! sign_arm {
            ($params:expr, $variant:ident) => {{
                let mut params = $params;
                params.apiKey = signer.api_key();
                let param_bytes = params.query_params(true).into_bytes();
                let signature = signer.signature(&param_bytes)?;
                Ok(BinanceHttpRequest {
                    params: BinanceHttpUnsignedRequest::$variant(params),
                    signature: Some(signature),
                })
            }};
        }
        match self {
            BinanceHttpUnsignedRequest::AmendOrderRequest(params) => {
                sign_arm!(params, AmendOrderRequest)
            }
            BinanceHttpUnsignedRequest::AssetLimits(params) => {
                sign_arm!(params, AssetLimits)
            }
            BinanceHttpUnsignedRequest::CancelAllOrdersRequest(params) => {
                sign_arm!(params, CancelAllOrdersRequest)
            }
            BinanceHttpUnsignedRequest::CancelOrderRequest(params) => {
                sign_arm!(params, CancelOrderRequest)
            }
            BinanceHttpUnsignedRequest::SpotOrderRequest(params) => {
                sign_arm!(params, SpotOrderRequest)
            }
            params => Ok(BinanceHttpRequest {
                params,
                signature: None,
            }),
        }
    }
}

impl BinanceHttpRequest {
    pub fn http_method(&self) -> HttpMethod {
        match self.params {
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
        match self.params {
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
    pub fn serialize(&self) -> EncryptResult<String> {
        serde_json::to_string(self).map_err(ETError::SerializeRequest)
    }
}
