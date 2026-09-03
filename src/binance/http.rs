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
    http::{HttpMethod, HttpRequest},
    rate_limited::RateLimited,
    signer::{IntoSigned, Signer},
};

#[cfg(feature = "serde")]
use {
    crate::http::HttpResponse,
    serde::{Deserialize, Serialize},
    serde_json,
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

impl RateLimited for BinanceHttpUnsignedRequest {
    fn order_count(&self) -> u32 {
        match self {
            BinanceHttpUnsignedRequest::SpotOrderRequest(..) => 1,
            _ => 0,
        }
    }
    fn weight(&self) -> u32 {
        match self {
            BinanceHttpUnsignedRequest::AmendOrderRequest(..) => 4,
            BinanceHttpUnsignedRequest::AssetLimits(..) => 40,
            BinanceHttpUnsignedRequest::CancelAllOrdersRequest(..) => 1,
            BinanceHttpUnsignedRequest::CancelOrderRequest(..) => 1,
            BinanceHttpUnsignedRequest::ExchangeInfo(..) => 20,
            BinanceHttpUnsignedRequest::SpotOrderRequest(..) => 1,
            BinanceHttpUnsignedRequest::Time(..) => 1,
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

impl From<BinanceHttpRequest> for HttpRequest {
    fn from(value: BinanceHttpRequest) -> Self {
        let method = match value.unsigned {
            BinanceHttpUnsignedRequest::AssetLimits(..)
            | BinanceHttpUnsignedRequest::ExchangeInfo(..)
            | BinanceHttpUnsignedRequest::Time(..) => HttpMethod::GET,
            BinanceHttpUnsignedRequest::CancelAllOrdersRequest(..)
            | BinanceHttpUnsignedRequest::CancelOrderRequest(..) => HttpMethod::DELETE,
            BinanceHttpUnsignedRequest::SpotOrderRequest(..) => HttpMethod::POST,
            BinanceHttpUnsignedRequest::AmendOrderRequest(..) => HttpMethod::PUT,
        };
        let endpoint = match value.unsigned {
            BinanceHttpUnsignedRequest::AmendOrderRequest(..) => "order/cancelReplace",
            BinanceHttpUnsignedRequest::AssetLimits(..) => "myFilters",
            BinanceHttpUnsignedRequest::CancelAllOrdersRequest(..) => "openOrders",
            BinanceHttpUnsignedRequest::CancelOrderRequest(..) => "order",
            BinanceHttpUnsignedRequest::ExchangeInfo(..) => "exchangeInfo",
            BinanceHttpUnsignedRequest::SpotOrderRequest(..) => "order",
            BinanceHttpUnsignedRequest::Time(..) => "time",
        };
        let unsigned_query_params = value.unsigned.query_params();
        let query_params = match &value.signature {
            Some(signature) => format!(
                "{}&signature={}",
                unsigned_query_params, signature.signature
            ),
            None => unsigned_query_params,
        };
        let query = Some(format!("{}?{}", endpoint, query_params));
        let headers = match &value.signature {
            Some(signature) => vec![("X-MBX-APIKEY".into(), signature.apiKey.clone())],
            None => vec![],
        };
        let body = None;
        HttpRequest {
            method,
            query,
            headers,
            body,
        }
    }
}

#[cfg(feature = "serde")]
impl From<HttpResponse> for BinanceHttpResponse {
    fn from(value: HttpResponse) -> Self {
        serde_json::from_slice(&value.body).unwrap_or_else(|_| {
            // The body is not a Binance result or error JSON payload (e.g. an
            // HTML page from an intermediary proxy). Surface it as a failure
            // carrying the HTTP status and the raw body so nothing is lost.
            BinanceHttpResponse::Failure(BinanceError {
                code: i64::from(value.status),
                msg: String::from_utf8_lossy(&value.body).into_owned(),
            })
        })
    }
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    use super::*;

    fn response(status: u16, body: &[u8]) -> HttpResponse {
        HttpResponse {
            status,
            headers: vec![],
            body: body.to_vec(),
        }
    }

    #[test]
    fn http_response_with_result_body_becomes_success() {
        let response = response(200, br#"{"serverTime":1700000000000}"#);
        match BinanceHttpResponse::from(response) {
            BinanceHttpResponse::Success(BinanceHttpResponseResult::Time(result)) => {
                assert_eq!(result.serverTime, 1700000000000);
            }
            other => panic!("expected Time, got: {other:?}"),
        }
    }

    #[test]
    fn any_2xx_http_response_with_result_body_becomes_success() {
        let response = response(201, br#"[]"#);
        assert!(matches!(
            BinanceHttpResponse::from(response),
            BinanceHttpResponse::Success(BinanceHttpResponseResult::AssetLimits(ref filters))
                if filters.is_empty()
        ));
    }

    #[test]
    fn http_response_with_error_body_becomes_failure_regardless_of_status() {
        // Binance reports API errors with a 4xx status...
        let http_response = response(400, br#"{"code":-2014,"msg":"API-key format invalid."}"#);
        match BinanceHttpResponse::from(http_response) {
            BinanceHttpResponse::Failure(error) => {
                assert_eq!(error.code, -2014);
                assert_eq!(error.msg, "API-key format invalid.");
            }
            other => panic!("expected Failure, got: {other:?}"),
        }
        // ...and sometimes with a 2xx status, which must still be a failure.
        let http_response = response(200, br#"{"code":-2015,"msg":"Invalid API-key."}"#);
        match BinanceHttpResponse::from(http_response) {
            BinanceHttpResponse::Failure(error) => {
                assert_eq!(error.code, -2015);
                assert_eq!(error.msg, "Invalid API-key.");
            }
            other => panic!("expected Failure, got: {other:?}"),
        }
    }

    #[test]
    fn undecodable_body_becomes_failure_carrying_status_and_body() {
        let response = response(502, b"<html>Bad Gateway</html>");
        match BinanceHttpResponse::from(response) {
            BinanceHttpResponse::Failure(error) => {
                assert_eq!(error.code, 502);
                assert_eq!(error.msg, "<html>Bad Gateway</html>");
            }
            other => panic!("expected Failure, got: {other:?}"),
        }
    }
}
