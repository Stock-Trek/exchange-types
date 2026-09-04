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
use serde::{Deserialize, de::DeserializeOwned};
use std::collections::HashMap;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BinanceUsageInterval {
    pub interval: BinanceRateLimitInterval,
    pub interval_num: u32,
}

impl TryFrom<&str> for BinanceUsageInterval {
    type Error = ETError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let letter = match value.chars().next_back() {
            Some(letter) => letter,
            None => return Err(ETError::ParseError("Empty usage interval".into())),
        };
        if !letter.is_ascii_alphabetic() {
            return Err(ETError::ParseError(value.into()));
        }
        let interval = BinanceRateLimitInterval::try_from(letter)?;
        let interval_num = match value[..value.len() - letter.len_utf8()].parse() {
            Ok(interval_num) => interval_num,
            Err(_) => return Err(ETError::ParseError(value.into())),
        };
        if interval_num == 0 {
            return Err(ETError::ParseError(format!(
                "interval number must be positive: {}",
                interval_num
            )));
        }
        Ok(Self {
            interval,
            interval_num,
        })
    }
}

impl BinanceUsageInterval {
    fn try_from_rate_limit(interval: BinanceRateLimitInterval, interval_num: u32) -> Option<Self> {
        (interval_num > 0).then_some(Self {
            interval_num,
            interval,
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct BinanceUsage {
    pub used_weight: HashMap<BinanceUsageInterval, u32>,
    pub order_count: HashMap<BinanceUsageInterval, u32>,
}

impl BinanceUsage {
    fn parse_header(&mut self, name: &str, value: &str) {
        let (usage, suffix) = match name.strip_prefix("x-mbx-order-count-") {
            Some(suffix) => (&mut self.order_count, suffix),
            None => match name.strip_prefix("x-mbx-used-weight-") {
                Some(suffix) => (&mut self.used_weight, suffix),
                None => return,
            },
        };
        let Ok(interval) = BinanceUsageInterval::try_from(suffix) else {
            return;
        };
        let Ok(count) = value.parse::<u32>() else {
            return;
        };
        usage.insert(interval, count);
    }
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
            if name == "retry-after" {
                retry_after = value.parse().ok();
            } else {
                usage.parse_header(&name, value);
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
            let Some(interval) = BinanceUsageInterval::try_from_rate_limit(
                rate_limit.interval,
                rate_limit.intervalNum,
            ) else {
                continue;
            };
            let Some(count) = rate_limit.count.map(|count| count as u32) else {
                continue;
            };
            match rate_limit.rateLimitType {
                BinanceRateLimitType::REQUEST_WEIGHT => {
                    metadata.usage.used_weight.insert(interval, count);
                }
                BinanceRateLimitType::ORDERS => {
                    metadata.usage.order_count.insert(interval, count);
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
