use crate::{
    binance::{
        error::BinanceError,
        rate_limits::{BinanceRateLimit, BinanceRateLimitInterval, BinanceRateLimitType},
    },
    error::{ETError, ETResult},
    http::HttpResponse,
    response::{ETHttpResponse, ETWebsocketResponse},
    websocket_id::ETWebsocketId,
};
use serde::Deserialize;
use serde::de::DeserializeOwned;

#[derive(Debug, Clone)]
pub struct BinanceResponse<R> {
    pub metadata: BinanceMetadata,
    pub payload: BinanceResponsePayload<R>,
}

#[derive(Debug, Clone)]
pub enum BinanceResponsePayload<R> {
    Success(R),
    Failure(BinanceError),
}

#[derive(Debug, Clone)]
pub struct BinanceMetadata {
    pub usage: BinanceUsage,
    pub retry_after: Option<u64>,
    pub websocket_id: Option<ETWebsocketId>,
    pub status: u16,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BinanceUsage {
    pub used_weight_1m: Option<u32>,
    pub order_count_10s: Option<u32>,
    pub order_count_1m: Option<u32>,
    pub order_count_1h: Option<u32>,
    pub order_count_1d: Option<u32>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
struct BinanceWebsocketResponse {
    pub error: Option<BinanceError>,
    pub id: Option<ETWebsocketId>,
    #[serde(default)]
    pub rateLimits: Vec<BinanceRateLimit>,
    pub result: Option<serde_json::Value>,
    pub status: u16,
}

impl<R> ETHttpResponse for BinanceResponse<R>
where
    R: DeserializeOwned,
{
    fn try_from_http(response: HttpResponse) -> ETResult<Self> {
        let mut usage = BinanceUsage::default();
        let mut retry_after = None;
        for (name, value) in response.headers {
            let name = name.to_ascii_lowercase();
            let value = value.trim();
            match name.as_str() {
                "x-mbx-used-weight-1m" => usage.used_weight_1m = value.parse().ok(),
                "x-mbx-order-count-10s" => usage.order_count_10s = value.parse().ok(),
                "x-mbx-order-count-1m" => usage.order_count_1m = value.parse().ok(),
                "x-mbx-order-count-1h" => usage.order_count_1h = value.parse().ok(),
                "x-mbx-order-count-1d" => usage.order_count_1d = value.parse().ok(),
                "retry-after" => retry_after = value.parse().ok(),
                _ => {}
            }
        }
        let metadata = BinanceMetadata {
            usage,
            retry_after,
            websocket_id: None,
            status: response.status,
        };
        let payload = match serde_json::from_slice::<BinanceError>(&response.body) {
            Ok(error) => BinanceResponsePayload::Failure(error),
            Err(_) => match serde_json::from_slice::<R>(&response.body) {
                Ok(result) => BinanceResponsePayload::Success(result),
                Err(error) => {
                    if (200..300).contains(&response.status) {
                        // A 2xx body that is neither a Binance error payload nor a
                        // result of the expected type is not a valid response to
                        // the request this result type answers (e.g. an HTML page
                        // served by an intermediary proxy, or schema drift).
                        return Err(ETError::DeserializeResponse(error));
                    }
                    // Non-2xx responses can have an empty or non-JSON body
                    // (e.g. HTTP 429/418 rate limiting, 5xx gateway pages).
                    // Surface them as a failure carrying the HTTP status and
                    // the raw body so nothing is lost.
                    BinanceResponsePayload::Failure(BinanceError {
                        code: i64::from(response.status),
                        msg: String::from_utf8_lossy(&response.body).into_owned(),
                        data: None,
                    })
                }
            },
        };
        Ok(BinanceResponse { metadata, payload })
    }
}

impl<R> ETWebsocketResponse for BinanceResponse<R>
where
    R: DeserializeOwned,
{
    fn try_from_websocket(response: String) -> ETResult<Self> {
        let websocket_response: BinanceWebsocketResponse =
            serde_json::from_str(&response).map_err(ETError::DeserializeResponse)?;
        let mut metadata = BinanceMetadata {
            usage: BinanceUsage::default(),
            retry_after: None,
            websocket_id: websocket_response.id,
            status: websocket_response.status,
        };
        for rate_limit in websocket_response.rateLimits {
            let count_u32 = rate_limit.count.map(|c| c as u32);
            match (
                rate_limit.rateLimitType,
                rate_limit.interval,
                rate_limit.intervalNum,
            ) {
                (BinanceRateLimitType::REQUEST_WEIGHT, BinanceRateLimitInterval::MINUTE, 1) => {
                    metadata.usage.used_weight_1m = count_u32;
                }
                (BinanceRateLimitType::ORDERS, BinanceRateLimitInterval::SECOND, 10) => {
                    metadata.usage.order_count_10s = count_u32;
                }
                (BinanceRateLimitType::ORDERS, BinanceRateLimitInterval::MINUTE, 1) => {
                    metadata.usage.order_count_1m = count_u32;
                }
                (BinanceRateLimitType::ORDERS, BinanceRateLimitInterval::HOUR, 1) => {
                    metadata.usage.order_count_1h = count_u32;
                }
                (BinanceRateLimitType::ORDERS, BinanceRateLimitInterval::DAY, 1) => {
                    metadata.usage.order_count_1d = count_u32;
                }
                _ => {}
            }
        }
        let payload = if let Some(error) = websocket_response.error {
            BinanceResponsePayload::Failure(error)
        } else if let Some(result) = websocket_response.result {
            match serde_json::from_value::<R>(result) {
                Ok(result) => BinanceResponsePayload::Success(result),
                // The frame parsed but its success payload is not of the
                // expected result type. Degrade to a failure instead of
                // failing the entire frame parse.
                Err(error) => BinanceResponsePayload::Failure(BinanceError {
                    code: -1,
                    msg: format!("Could not deserialize websocket result: {error}"),
                    data: None,
                }),
            }
        } else {
            // No error and no result – treat as a generic failure
            BinanceResponsePayload::Failure(BinanceError {
                code: -1,
                msg: "Websocket response missing both error and result".to_string(),
                data: None,
            })
        };
        Ok(BinanceResponse { metadata, payload })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        binance::{
            account::BinanceAccountResult,
            amend::BinanceAmendOrderResult,
            asset_limits::BinanceAssetLimitsResult,
            cancel::BinanceCancelReport,
            exchange_info::BinanceExchangeInfoResult,
            query_order::BinanceOrderResult,
            session::BinanceSessionAuthenticationResult,
            spot::BinanceSpotOrderResult,
            time::{BinanceTimeParams, BinanceTimeResult},
        },
        response::ResponseFor,
    };

    fn assert_response_for<P: ResponseFor<Result = R>, R>() {}

    fn http_response(status: u16, body: &[u8]) -> HttpResponse {
        HttpResponse {
            status,
            headers: Vec::new(),
            body: body.to_vec(),
        }
    }

    #[test]
    fn pairs_each_request_params_struct_with_its_result_type() {
        assert_response_for::<BinanceTimeParams, BinanceTimeResult>();
        assert_response_for::<crate::binance::account::BinanceAccountParams, BinanceAccountResult>(
        );
        assert_response_for::<
            crate::binance::amend::BinanceAmendOrderParams,
            BinanceAmendOrderResult,
        >();
        assert_response_for::<
            crate::binance::asset_limits::BinanceAssetLimitsParams,
            BinanceAssetLimitsResult,
        >();
        assert_response_for::<
            crate::binance::cancel::BinanceCancelAllOrdersParams,
            Vec<BinanceCancelReport>,
        >();
        assert_response_for::<crate::binance::cancel::BinanceCancelOrderParams, BinanceCancelReport>(
        );
        assert_response_for::<
            crate::binance::exchange_info::BinanceExchangeInfoParams,
            BinanceExchangeInfoResult,
        >();
        assert_response_for::<
            crate::binance::open_orders::BinanceOpenOrdersParams,
            Vec<BinanceOrderResult>,
        >();
        assert_response_for::<
            crate::binance::query_order::BinanceQueryOrderParams,
            BinanceOrderResult,
        >();
        assert_response_for::<
            crate::binance::session::BinanceSessionLogonParams,
            BinanceSessionAuthenticationResult,
        >();
        assert_response_for::<
            crate::binance::session::BinanceSessionLogoutParams,
            BinanceSessionAuthenticationResult,
        >();
        assert_response_for::<crate::binance::spot::BinanceSpotOrderParams, BinanceSpotOrderResult>(
        );
    }

    #[test]
    fn http_success_parses_into_the_requested_result_type() {
        let response = http_response(200, br#"{"serverTime": 123}"#);
        let response = BinanceResponse::<BinanceTimeResult>::try_from_http(response).unwrap();
        match response.payload {
            BinanceResponsePayload::Success(result) => assert_eq!(result.serverTime, 123),
            BinanceResponsePayload::Failure(_) => panic!("expected a success payload"),
        }
    }

    #[test]
    fn http_error_payload_parses_as_failure() {
        let response = http_response(400, br#"{"code": -1121, "msg": "Invalid symbol."}"#);
        let response = BinanceResponse::<BinanceTimeResult>::try_from_http(response).unwrap();
        match response.payload {
            BinanceResponsePayload::Success(_) => panic!("expected a failure payload"),
            BinanceResponsePayload::Failure(error) => {
                assert_eq!(error.code, -1121);
                assert_eq!(error.msg, "Invalid symbol.");
            }
        }
    }

    #[test]
    fn http_schema_drift_errors_instead_of_silently_matching() {
        // An exchangeInfo-style payload with only serverTime (the fields an
        // untagged classifier could drop) must not parse as BinanceTimeResult
        // when it answers an exchange info request: the pairing is
        // BinanceExchangeInfoParams -> BinanceExchangeInfoResult, so this
        // payload is a deserialization error rather than a success.
        let response = http_response(
            200,
            br#"{"timezone": "UTC", "serverTime": 1, "symbols": [{"symbol": "BTCUSDT"}]}"#,
        );
        let result = BinanceResponse::<BinanceExchangeInfoResult>::try_from_http(response);
        assert!(matches!(result, Err(ETError::DeserializeResponse(_))));
    }

    #[test]
    fn http_non_2xx_without_json_body_becomes_failure() {
        let response = http_response(429, b"");
        let response = BinanceResponse::<BinanceTimeResult>::try_from_http(response).unwrap();
        match response.payload {
            BinanceResponsePayload::Success(_) => panic!("expected a failure payload"),
            BinanceResponsePayload::Failure(error) => {
                assert_eq!(error.code, 429);
                assert_eq!(error.msg, "");
            }
        }
    }

    #[test]
    fn websocket_success_parses_into_the_requested_result_type() {
        let response = r#"{"id": "1", "status": 200, "result": {"serverTime": 123}}"#.to_string();
        let response = BinanceResponse::<BinanceTimeResult>::try_from_websocket(response).unwrap();
        assert_eq!(
            response.metadata.websocket_id,
            Some(ETWebsocketId::Str("1".into()))
        );
        match response.payload {
            BinanceResponsePayload::Success(result) => assert_eq!(result.serverTime, 123),
            BinanceResponsePayload::Failure(_) => panic!("expected a success payload"),
        }
    }

    #[test]
    fn websocket_unrecognized_result_degrades_instead_of_failing_the_frame() {
        let response = r#"{"id": "1", "status": 200, "result": {}}"#.to_string();
        let response = BinanceResponse::<BinanceTimeResult>::try_from_websocket(response).unwrap();
        assert!(matches!(
            response.payload,
            BinanceResponsePayload::Failure(BinanceError { code: -1, .. })
        ));
    }
}
