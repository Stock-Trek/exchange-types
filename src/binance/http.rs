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
    error::{ETError, ETResult},
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

/// The deserialized body of a Binance REST response: either a successful
/// result payload or a Binance API error payload (`{"code":…,"msg":…}`).
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[derive(Debug, Clone)]
pub enum BinanceHttpResponsePayload {
    Success(BinanceHttpResponseResult),
    Failure(BinanceError),
}

/// The rate-limit usage and retry information Binance returns in REST
/// response headers.
///
/// Per the "LIMITS" and "HTTP Return Codes" sections of the Binance spot
/// REST API docs:
/// - every response includes an `X-MBX-USED-WEIGHT-(intervalNum)(intervalLetter)`
///   header with the current request weight used by the IP;
/// - every successful order response includes an
///   `X-MBX-ORDER-COUNT-(intervalNum)(intervalLetter)` header with the number
///   of orders placed in that interval;
/// - HTTP 429/418 responses include a `Retry-After` header with the number of
///   seconds to wait before retrying.
///
/// A header that is absent (or present with a value that is not a valid
/// number) parses as `None`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BinanceHttpResponseHeaders {
    /// `X-MBX-USED-WEIGHT-1M`: current request weight used by the IP.
    pub used_weight_1m: Option<u32>,
    /// `X-MBX-ORDER-COUNT-10S`: orders placed in the last 10 seconds.
    pub order_count_10s: Option<u32>,
    /// `X-MBX-ORDER-COUNT-1M`: orders placed in the last minute.
    pub order_count_1m: Option<u32>,
    /// `X-MBX-ORDER-COUNT-1H`: orders placed in the last hour.
    pub order_count_1h: Option<u32>,
    /// `X-MBX-ORDER-COUNT-1D`: orders placed in the last day.
    pub order_count_1d: Option<u32>,
    /// `Retry-After`: seconds to wait before retrying (sent with HTTP 429/418).
    pub retry_after: Option<u64>,
}

impl BinanceHttpResponseHeaders {
    /// Parses the Binance rate-limit usage headers out of an HTTP response's
    /// headers. Header names are matched case-insensitively.
    fn parse(headers: &[(String, String)]) -> Self {
        let mut parsed = Self::default();
        for (name, value) in headers {
            let name = name.to_ascii_lowercase();
            let value = value.trim();
            match name.as_str() {
                "x-mbx-used-weight-1m" => parsed.used_weight_1m = value.parse().ok(),
                "x-mbx-order-count-10s" => parsed.order_count_10s = value.parse().ok(),
                "x-mbx-order-count-1m" => parsed.order_count_1m = value.parse().ok(),
                "x-mbx-order-count-1h" => parsed.order_count_1h = value.parse().ok(),
                "x-mbx-order-count-1d" => parsed.order_count_1d = value.parse().ok(),
                "retry-after" => parsed.retry_after = value.parse().ok(),
                _ => {}
            }
        }
        parsed
    }
}

/// A Binance REST response parsed from an [`HttpResponse`]: the HTTP status,
/// the rate-limit usage headers Binance returned, and the deserialized body.
#[derive(Debug, Clone)]
pub struct BinanceHttpResponse {
    pub status: u16,
    pub headers: BinanceHttpResponseHeaders,
    pub payload: BinanceHttpResponsePayload,
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
impl TryFrom<HttpResponse> for BinanceHttpResponse {
    type Error = ETError;

    fn try_from(value: HttpResponse) -> Result<Self, Self::Error> {
        let headers = BinanceHttpResponseHeaders::parse(&value.headers);
        match serde_json::from_slice::<BinanceHttpResponsePayload>(&value.body) {
            Ok(payload) => Ok(BinanceHttpResponse {
                status: value.status,
                headers,
                payload,
            }),
            Err(error) => {
                if (200..300).contains(&value.status) {
                    // A 2xx body that is neither a Binance result nor a
                    // Binance error payload is not a Binance response (e.g.
                    // an HTML page served by an intermediary proxy).
                    Err(ETError::DeserializeResponse(error))
                } else {
                    // Non-2xx responses can have an empty or non-JSON body
                    // (e.g. HTTP 429/418 rate limiting, 5xx gateway pages).
                    // Surface them as a failure carrying the HTTP status and
                    // the raw body so nothing is lost.
                    Ok(BinanceHttpResponse {
                        status: value.status,
                        headers,
                        payload: BinanceHttpResponsePayload::Failure(BinanceError {
                            code: i64::from(value.status),
                            msg: String::from_utf8_lossy(&value.body).into_owned(),
                        }),
                    })
                }
            }
        }
    }
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    use super::*;

    fn response(status: u16, headers: &[(&str, &str)], body: &[u8]) -> HttpResponse {
        HttpResponse {
            status,
            headers: headers
                .iter()
                .map(|(name, value)| ((*name).into(), (*value).into()))
                .collect(),
            body: body.to_vec(),
        }
    }

    #[test]
    fn http_response_with_result_body_becomes_success() {
        let response = response(200, &[], br#"{"serverTime":1700000000000}"#);
        let response = BinanceHttpResponse::try_from(response).unwrap();
        assert_eq!(response.status, 200);
        match response.payload {
            BinanceHttpResponsePayload::Success(BinanceHttpResponseResult::Time(result)) => {
                assert_eq!(result.serverTime, 1700000000000);
            }
            other => panic!("expected Time, got: {other:?}"),
        }
    }

    #[test]
    fn any_2xx_http_response_with_result_body_becomes_success() {
        let response = response(201, &[], br#"[]"#);
        let response = BinanceHttpResponse::try_from(response).unwrap();
        assert!(matches!(
            response.payload,
            BinanceHttpResponsePayload::Success(BinanceHttpResponseResult::AssetLimits(
                ref filters
            )) if filters.is_empty()
        ));
    }

    #[test]
    fn http_response_with_error_body_becomes_failure_regardless_of_status() {
        // Binance reports API errors with a 4xx status...
        let http_response = response(
            400,
            &[],
            br#"{"code":-2014,"msg":"API-key format invalid."}"#,
        );
        let parsed = BinanceHttpResponse::try_from(http_response).unwrap();
        match parsed.payload {
            BinanceHttpResponsePayload::Failure(error) => {
                assert_eq!(error.code, -2014);
                assert_eq!(error.msg, "API-key format invalid.");
            }
            other => panic!("expected Failure, got: {other:?}"),
        }
        // ...and sometimes with a 2xx status, which must still be a failure.
        let http_response = response(200, &[], br#"{"code":-2015,"msg":"Invalid API-key."}"#);
        let parsed = BinanceHttpResponse::try_from(http_response).unwrap();
        match parsed.payload {
            BinanceHttpResponsePayload::Failure(error) => {
                assert_eq!(error.code, -2015);
                assert_eq!(error.msg, "Invalid API-key.");
            }
            other => panic!("expected Failure, got: {other:?}"),
        }
    }

    #[test]
    fn undecodable_2xx_body_is_a_conversion_error() {
        let response = response(200, &[], b"<html>Bad Gateway</html>");
        assert!(matches!(
            BinanceHttpResponse::try_from(response),
            Err(ETError::DeserializeResponse(_))
        ));
    }

    #[test]
    fn undecodable_non_2xx_body_becomes_failure_carrying_status_and_body() {
        let response = response(502, &[], b"<html>Bad Gateway</html>");
        let response = BinanceHttpResponse::try_from(response).unwrap();
        match response.payload {
            BinanceHttpResponsePayload::Failure(error) => {
                assert_eq!(error.code, 502);
                assert_eq!(error.msg, "<html>Bad Gateway</html>");
            }
            other => panic!("expected Failure, got: {other:?}"),
        }
    }

    #[test]
    fn parses_binance_rate_limit_usage_headers() {
        let response = response(
            200,
            &[
                ("X-MBX-USED-WEIGHT-1M", "34"),
                ("X-MBX-ORDER-COUNT-10S", "1"),
                ("X-MBX-ORDER-COUNT-1M", "2"),
                ("X-MBX-ORDER-COUNT-1H", "5"),
                ("X-MBX-ORDER-COUNT-1D", "12"),
            ],
            br#"{"serverTime":1700000000000}"#,
        );
        let response = BinanceHttpResponse::try_from(response).unwrap();
        assert_eq!(response.headers.used_weight_1m, Some(34));
        assert_eq!(response.headers.order_count_10s, Some(1));
        assert_eq!(response.headers.order_count_1m, Some(2));
        assert_eq!(response.headers.order_count_1h, Some(5));
        assert_eq!(response.headers.order_count_1d, Some(12));
        assert_eq!(response.headers.retry_after, None);
    }

    #[test]
    fn parses_retry_after_on_rate_limited_response() {
        // Binance rate limiting responds with HTTP 429/418 and no JSON body.
        let response = response(429, &[("Retry-After", "30")], b"");
        let response = BinanceHttpResponse::try_from(response).unwrap();
        assert_eq!(response.headers.retry_after, Some(30));
        match response.payload {
            BinanceHttpResponsePayload::Failure(error) => assert_eq!(error.code, 429),
            other => panic!("expected Failure, got: {other:?}"),
        }
    }

    #[test]
    fn header_names_are_case_insensitive() {
        let response = response(
            200,
            &[("x-mbx-used-weight-1m", "7"), ("retry-after", "2")],
            br#"{"serverTime":1700000000000}"#,
        );
        let response = BinanceHttpResponse::try_from(response).unwrap();
        assert_eq!(response.headers.used_weight_1m, Some(7));
        assert_eq!(response.headers.retry_after, Some(2));
    }

    #[test]
    fn missing_or_malformed_usage_headers_are_none() {
        let response = response(
            200,
            &[("X-MBX-USED-WEIGHT-1M", "not-a-number")],
            br#"{"serverTime":1700000000000}"#,
        );
        let response = BinanceHttpResponse::try_from(response).unwrap();
        assert_eq!(response.headers.used_weight_1m, None);
        assert_eq!(response.headers.order_count_10s, None);
        assert_eq!(response.headers.retry_after, None);
    }

    #[test]
    fn error_response_headers_are_still_parsed() {
        let response = response(
            400,
            &[("X-MBX-USED-WEIGHT-1M", "56")],
            br#"{"code":-2014,"msg":"API-key format invalid."}"#,
        );
        let response = BinanceHttpResponse::try_from(response).unwrap();
        assert_eq!(response.headers.used_weight_1m, Some(56));
        assert!(matches!(
            response.payload,
            BinanceHttpResponsePayload::Failure(..)
        ));
    }
}
