use crate::{
    binance::{
        error::BinanceError,
        rate_limits::{BinanceRateLimit, BinanceRateLimitInterval, BinanceRateLimitType},
    },
    error::{ETError, ETResult},
    http::HttpResponse,
    rate_limited::{RateLimit, RateLimitRestriction},
    response::{ETHttpResponse, ETResponse, ETWebsocketResponse},
    time::Nanoseconds,
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
    pub retry_after: Option<u64>,
    pub status: u16,
    pub usage: HashMap<RateLimit, u32>,
    pub websocket_id: Option<ETWebsocketId>,
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

impl<R> BinanceResponse<R>
where
    R: DeserializeOwned,
{
    fn parse_usage_http(headers: &Vec<(String, String)>) -> HashMap<RateLimit, u32> {
        let mut usage = HashMap::new();
        for (name, value) in headers {
            let restriction = if name.starts_with("X-MBX-ORDER-COUNT-") {
                Some(RateLimitRestriction::OrderCount)
            } else if name.starts_with("X-MBX-USED_WEIGHT-") {
                Some(RateLimitRestriction::Weight)
            } else {
                None
            };
            if let Some(restriction) = restriction {
                match name.rsplit_once('-') {
                    None => {}
                    Some(last_part) => {
                        let last_part = last_part.1;
                        let (num_str, letter) = last_part.split_at(last_part.len() - 1);
                        let num = num_str.parse::<u64>().ok();
                        let interval_nanos = letter.chars().next().and_then(|c| {
                            BinanceRateLimitInterval::try_from(c)
                                .ok()
                                .and_then(|i| Nanoseconds::try_from(i).ok())
                        });
                        if let Some(num) = num
                            && let Some(interval_nanos) = interval_nanos
                        {
                            let interval_nanos = num * interval_nanos.0;
                            match value.parse::<u32>() {
                                Err(_) => {}
                                Ok(used) => {
                                    usage.insert(
                                        RateLimit {
                                            restriction,
                                            interval_nanos,
                                        },
                                        used,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
        usage
    }
    fn parse_usage_websocket(rate_limits: &Vec<BinanceRateLimit>) -> HashMap<RateLimit, u32> {
        let mut usage = HashMap::new();
        for rate_limit in rate_limits {
            let nanoseconds: Nanoseconds = match rate_limit.interval.try_into() {
                Err(_) => continue,
                Ok(nanos) => nanos,
            };
            let interval_nanos: u64 = nanoseconds.0 * rate_limit.intervalNum as u64;
            let restriction = match rate_limit.rateLimitType {
                BinanceRateLimitType::CONNECTIONS => RateLimitRestriction::Connection,
                BinanceRateLimitType::ORDERS => RateLimitRestriction::OrderCount,
                BinanceRateLimitType::RAW_REQUESTS => RateLimitRestriction::RawRequests,
                BinanceRateLimitType::REQUEST_WEIGHT => RateLimitRestriction::Weight,
                BinanceRateLimitType::Unknown => continue,
            };
            let count = match rate_limit.count {
                None => continue,
                Some(count) => count as u32,
            };
            usage.insert(
                RateLimit {
                    restriction,
                    interval_nanos,
                },
                count,
            );
        }
        usage
    }
}

impl<R> ETResponse for BinanceResponse<R> {
    fn retry_after(&self) -> Option<u64> {
        self.metadata.retry_after
    }
    fn rate_limit_usage(&self) -> Option<&HashMap<RateLimit, u32>> {
        Some(&self.metadata.usage)
    }
}

impl<R> ETHttpResponse for BinanceResponse<R>
where
    R: DeserializeOwned,
{
    fn try_from_http(response: HttpResponse) -> ETResult<Self> {
        let usage = Self::parse_usage_http(&response.headers);
        let retry_after: Option<u64> = match response
            .headers
            .iter()
            .find(|tuple| tuple.0.to_ascii_uppercase() == "RETRY-AFTER")
        {
            None => None,
            Some(tuple) => match tuple.1.parse::<u64>() {
                Err(_) => return Err(ETError::ParseError("Retry-After".into())),
                Ok(parsed) => Some(parsed),
            },
        };
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
                        // result of the expected type is usually schema drift
                        // (Binance deprecates or renames response fields) or an
                        // intermediary page. Degrade to a failure carrying the
                        // deserialization error instead of failing the whole
                        // response, so that the response metadata (usage,
                        // status) survives. This mirrors the websocket path,
                        // which already degrades unexpected success payloads.
                        BinanceResponsePayload::Failure(BinanceError {
                            code: -1,
                            msg: format!(
                                "Could not deserialize response body as the expected type: {error}"
                            ),
                            data: None,
                        })
                    } else {
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
        let usage = Self::parse_usage_websocket(&websocket_response.rateLimits);
        let retry_after = websocket_response.error.as_ref().and_then(|e| {
            e.data
                .as_ref()
                .and_then(|d| d.retryAfter.map(|value| value as u64))
        });
        let metadata = BinanceMetadata {
            usage,
            retry_after,
            websocket_id: websocket_response.id,
            status: websocket_response.status,
        };
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
