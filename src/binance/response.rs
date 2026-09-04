use crate::{
    binance::{
        amend::BinanceAmendOrderResult,
        asset_limits::BinanceAssetLimitsResult,
        cancel::{BinanceCancelOrderListResult, BinanceCancelOrderResult, BinanceCancelReport},
        error::BinanceError,
        exchange_info::BinanceExchangeInfoResult,
        rate_limits::{BinanceRateLimit, BinanceRateLimitInterval, BinanceRateLimitType},
        session::BinanceSessionAuthenticationResult,
        spot::BinanceSpotOrderResult,
        time::BinanceTimeResult,
    },
    error::ETResult,
    http::HttpResponse,
    response::{ETHttpResponse, ETWebsocketResponse},
    websocket_id::ETWebsocketId,
};

use {crate::error::ETError, serde::Deserialize, serde_json};

#[derive(Debug, Clone)]
pub struct BinanceResponse {
    pub metadata: BinanceMetadata,
    pub payload: BinanceResponsePayload,
}

#[allow(clippy::large_enum_variant)]
#[derive(Deserialize)]
#[serde(untagged)]
#[derive(Debug, Clone)]
pub enum BinanceResponsePayload {
    Success(BinanceResult),
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

#[derive(Deserialize)]
#[serde(untagged)]
#[derive(Debug, Clone)]
pub enum BinanceResult {
    AmendOrder(BinanceAmendOrderResult),
    AssetLimits(BinanceAssetLimitsResult),
    CancelAllOrders(Vec<BinanceCancelReport>),
    CancelOrder(BinanceCancelOrderResult),
    CancelOrderList(BinanceCancelOrderListResult),
    ExchangeInfo(BinanceExchangeInfoResult),
    SpotOrder(BinanceSpotOrderResult),
    Time(BinanceTimeResult),
    WebsocketSessionAuthentication(BinanceSessionAuthenticationResult),
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
struct BinanceWebsocketResponse {
    pub error: Option<BinanceError>,
    pub id: Option<ETWebsocketId>,
    #[serde(default)]
    pub rateLimits: Vec<BinanceRateLimit>,
    pub result: Option<BinanceWebsocketResponseResult>,
    pub status: u16,
}

#[derive(Deserialize)]
#[serde(untagged)]
#[derive(Debug, Clone)]
pub enum BinanceWebsocketResponseResult {
    AmendOrder(BinanceAmendOrderResult),
    AssetLimits(BinanceAssetLimitsResult),
    CancelAllOrders(Vec<BinanceCancelReport>),
    CancelOrder(BinanceCancelOrderResult),
    CancelOrderList(BinanceCancelOrderListResult),
    ExchangeInfo(BinanceExchangeInfoResult),
    SessionAuthentication(BinanceSessionAuthenticationResult),
    SpotOrder(BinanceSpotOrderResult),
    Time(BinanceTimeResult),
}

impl ETHttpResponse for BinanceResponse {
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
        match serde_json::from_slice::<BinanceResponsePayload>(&response.body) {
            Ok(payload) => Ok(BinanceResponse { metadata, payload }),
            Err(error) => {
                if (200..300).contains(&response.status) {
                    // A 2xx body that is neither a Binance result nor a
                    // Binance error payload is not a Binance response (e.g.
                    // an HTML page served by an intermediary proxy).
                    Err(ETError::DeserializeResponse(error))
                } else {
                    // Non-2xx responses can have an empty or non-JSON body
                    // (e.g. HTTP 429/418 rate limiting, 5xx gateway pages).
                    // Surface them as a failure carrying the HTTP status and
                    // the raw body so nothing is lost.
                    Ok(BinanceResponse {
                        metadata,
                        payload: BinanceResponsePayload::Failure(BinanceError {
                            code: i64::from(response.status),
                            msg: String::from_utf8_lossy(&response.body).into_owned(),
                            data: None,
                        }),
                    })
                }
            }
        }
    }
}

impl ETWebsocketResponse for BinanceResponse {
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
            let binance_result = match result {
                BinanceWebsocketResponseResult::AmendOrder(r) => BinanceResult::AmendOrder(r),
                BinanceWebsocketResponseResult::AssetLimits(r) => BinanceResult::AssetLimits(r),
                BinanceWebsocketResponseResult::CancelAllOrders(r) => {
                    BinanceResult::CancelAllOrders(r)
                }
                BinanceWebsocketResponseResult::CancelOrder(r) => BinanceResult::CancelOrder(r),
                BinanceWebsocketResponseResult::CancelOrderList(r) => {
                    BinanceResult::CancelOrderList(r)
                }
                BinanceWebsocketResponseResult::ExchangeInfo(r) => BinanceResult::ExchangeInfo(r),
                BinanceWebsocketResponseResult::SessionAuthentication(r) => {
                    BinanceResult::WebsocketSessionAuthentication(r)
                }
                BinanceWebsocketResponseResult::SpotOrder(r) => BinanceResult::SpotOrder(r),
                BinanceWebsocketResponseResult::Time(r) => BinanceResult::Time(r),
            };
            BinanceResponsePayload::Success(binance_result)
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
    use crate::{error::ETResult, response::ETWebsocketResponse, websocket_id::ETWebsocketId};

    fn parse_websocket(json: &str) -> ETResult<BinanceResponse> {
        BinanceResponse::try_from_websocket(json.to_string())
    }

    // Binance never sends the interval string "SECONDS_TEN"; a 10-second
    // order-count limit arrives as ORDERS + "SECOND" + intervalNum 10 (e.g. in
    // cancelReplace and 429 error frames). Assert those frames populate the
    // usage metadata.
    #[test]
    fn rate_limit_usage_is_parsed_from_websocket_error_frame() {
        let response = parse_websocket(
            r#"{
                "id": 1,
                "status": 429,
                "error": {
                    "code": -1003,
                    "msg": "Too many requests",
                    "data": { "retryAfter": 45 }
                },
                "rateLimits": [
                    { "rateLimitType": "REQUEST_WEIGHT", "interval": "MINUTE", "intervalNum": 1, "limit": 6000, "count": 21 },
                    { "rateLimitType": "ORDERS", "interval": "SECOND", "intervalNum": 10, "limit": 50, "count": 2 },
                    { "rateLimitType": "ORDERS", "interval": "DAY", "intervalNum": 1, "limit": 160000, "count": 9 }
                ]
            }"#,
        )
        .unwrap();
        assert_eq!(response.metadata.status, 429);
        assert_eq!(response.metadata.websocket_id, Some(ETWebsocketId::Int(1)));
        assert_eq!(response.metadata.usage.used_weight_1m, Some(21));
        assert_eq!(response.metadata.usage.order_count_10s, Some(2));
        assert_eq!(response.metadata.usage.order_count_1m, None);
        assert_eq!(response.metadata.usage.order_count_1h, None);
        assert_eq!(response.metadata.usage.order_count_1d, Some(9));
        match response.payload {
            BinanceResponsePayload::Failure(error) => {
                assert_eq!(error.code, -1003);
                assert_eq!(error.msg, "Too many requests");
                assert_eq!(error.data.unwrap().retryAfter, Some(45));
            }
            _ => panic!("expected a failure payload"),
        }
    }

    // intervalNum must be honoured: only ORDERS + SECOND + intervalNum 10 is the
    // 10-second order count, and only intervalNum 1 maps to the 1m/1h/1d counts.
    #[test]
    fn order_count_mappings_require_the_expected_interval_num() {
        let response = parse_websocket(
            r#"{
                "id": "2",
                "status": 429,
                "error": { "code": -1003, "msg": "Way too many requests" },
                "rateLimits": [
                    { "rateLimitType": "ORDERS", "interval": "SECOND", "intervalNum": 10, "limit": 50, "count": 3 },
                    { "rateLimitType": "ORDERS", "interval": "SECOND", "intervalNum": 1, "limit": 50, "count": 99 },
                    { "rateLimitType": "ORDERS", "interval": "SECONDS_TEN", "intervalNum": 10, "limit": 50, "count": 98 },
                    { "rateLimitType": "ORDERS", "interval": "MINUTE", "intervalNum": 5, "limit": 50, "count": 97 },
                    { "rateLimitType": "ORDERS", "interval": "MINUTE", "intervalNum": 1, "limit": 50, "count": 7 },
                    { "rateLimitType": "ORDERS", "interval": "HOUR", "intervalNum": 1, "limit": 50, "count": 2 },
                    { "rateLimitType": "ORDERS", "interval": "DAY", "intervalNum": 1, "limit": 160000, "count": 11 }
                ]
            }"#,
        )
        .unwrap();
        assert_eq!(response.metadata.usage.order_count_10s, Some(3));
        assert_eq!(response.metadata.usage.order_count_1m, Some(7));
        assert_eq!(response.metadata.usage.order_count_1h, Some(2));
        assert_eq!(response.metadata.usage.order_count_1d, Some(11));
    }
}
