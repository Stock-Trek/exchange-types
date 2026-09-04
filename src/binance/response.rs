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
};

#[cfg(feature = "serde")]
use {crate::error::ETError, serde::Deserialize, serde_json};

#[derive(Debug, Clone)]
pub struct BinanceResponse {
    pub metadata: BinanceMetadata,
    pub payload: BinanceResponsePayload,
}

#[allow(clippy::large_enum_variant)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[derive(Debug, Clone)]
pub enum BinanceResponsePayload {
    Success(BinanceResult),
    Failure(BinanceError),
}

#[derive(Debug, Clone, Default)]
pub struct BinanceMetadata {
    pub usage: BinanceUsage,
    pub retry_after: Option<u64>,
    pub websocket_id: Option<String>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BinanceUsage {
    pub used_weight_1m: Option<u32>,
    pub order_count_10s: Option<u32>,
    pub order_count_1m: Option<u32>,
    pub order_count_1h: Option<u32>,
    pub order_count_1d: Option<u32>,
}

#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[derive(Debug, Clone)]
struct BinanceWebsocketResponse {
    pub error: Option<BinanceError>,
    pub id: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rateLimits: Vec<BinanceRateLimit>,
    pub result: Option<BinanceWebsocketResponseResult>,
}

#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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
        let mut metadata = BinanceMetadata::default();
        for (name, value) in response.headers {
            let name = name.to_ascii_lowercase();
            let value = value.trim();
            match name.as_str() {
                "x-mbx-used-weight-1m" => metadata.usage.used_weight_1m = value.parse().ok(),
                "x-mbx-order-count-10s" => metadata.usage.order_count_10s = value.parse().ok(),
                "x-mbx-order-count-1m" => metadata.usage.order_count_1m = value.parse().ok(),
                "x-mbx-order-count-1h" => metadata.usage.order_count_1h = value.parse().ok(),
                "x-mbx-order-count-1d" => metadata.usage.order_count_1d = value.parse().ok(),
                "retry-after" => metadata.retry_after = value.parse().ok(),
                _ => {}
            }
        }
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
        let mut metadata = BinanceMetadata::default();
        metadata.websocket_id = Some(websocket_response.id);
        for rate_limit in websocket_response.rateLimits {
            let count_u32 = rate_limit.count.map(|c| c as u32);
            match (rate_limit.rateLimitType, rate_limit.interval) {
                (BinanceRateLimitType::REQUEST_WEIGHT, BinanceRateLimitInterval::MINUTE) => {
                    metadata.usage.used_weight_1m = count_u32;
                }
                (BinanceRateLimitType::ORDERS, BinanceRateLimitInterval::SECONDS_TEN) => {
                    metadata.usage.order_count_10s = count_u32;
                }
                (BinanceRateLimitType::ORDERS, BinanceRateLimitInterval::MINUTE) => {
                    metadata.usage.order_count_1m = count_u32;
                }
                (BinanceRateLimitType::ORDERS, BinanceRateLimitInterval::HOUR) => {
                    metadata.usage.order_count_1h = count_u32;
                }
                (BinanceRateLimitType::ORDERS, BinanceRateLimitInterval::DAY) => {
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
            })
        };
        Ok(BinanceResponse { metadata, payload })
    }
}
