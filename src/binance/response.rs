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

/// The `{intervalLetter}` component of a Binance usage interval. Binance
/// reports usage in windows denoted by a letter and a number, e.g. `10S` or
/// `1M`, where the letter is one of `S` (second), `M` (minute), `H` (hour)
/// or `D` (day).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinanceUsageIntervalLetter {
    Second,
    Minute,
    Hour,
    Day,
}

impl BinanceUsageIntervalLetter {
    fn parse(letter: char) -> Option<Self> {
        match letter.to_ascii_lowercase() {
            's' => Some(Self::Second),
            'm' => Some(Self::Minute),
            'h' => Some(Self::Hour),
            'd' => Some(Self::Day),
            _ => None,
        }
    }
}

/// A usage interval, parsed from the `{intervalNum}{intervalLetter}` suffix
/// of an `X-MBX-USED-WEIGHT-*`/`X-MBX-ORDER-COUNT-*` header (e.g.
/// `X-MBX-ORDER-COUNT-10S`) or derived from the `interval`/`intervalNum`
/// fields of a websocket rate limit. Per Binance, `intervalNum` is the
/// number that multiplies the `intervalLetter`, so `10S` is a ten-second
/// window and `1M` a one-minute window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BinanceUsageInterval {
    pub interval_num: u32,
    pub interval_letter: BinanceUsageIntervalLetter,
}

impl BinanceUsageInterval {
    /// Parses the `{intervalNum}{intervalLetter}` header suffix, e.g. `10s`
    /// or `1m` (the letter case is not significant).
    fn parse(suffix: &str) -> Option<Self> {
        let letter = suffix.chars().next_back()?;
        if !letter.is_ascii_alphabetic() {
            return None;
        }
        let interval_num = suffix[..suffix.len() - letter.len_utf8()].parse().ok()?;
        let interval_letter = BinanceUsageIntervalLetter::parse(letter)?;
        (interval_num > 0).then_some(Self {
            interval_num,
            interval_letter,
        })
    }

    /// Derives the interval from a websocket rate limit's `interval` and
    /// `intervalNum` fields.
    fn from_rate_limit(interval: BinanceRateLimitInterval, interval_num: i32) -> Option<Self> {
        let interval_letter = match interval {
            BinanceRateLimitInterval::DAY => BinanceUsageIntervalLetter::Day,
            BinanceRateLimitInterval::HOUR => BinanceUsageIntervalLetter::Hour,
            BinanceRateLimitInterval::MINUTE => BinanceUsageIntervalLetter::Minute,
            BinanceRateLimitInterval::SECOND | BinanceRateLimitInterval::SECONDS_TEN => {
                BinanceUsageIntervalLetter::Second
            }
            BinanceRateLimitInterval::Unknown => return None,
        };
        let interval_num = u32::try_from(interval_num).ok()?;
        (interval_num > 0).then_some(Self {
            interval_num,
            interval_letter,
        })
    }
}

/// Usage reported by Binance in response metadata: the current value of each
/// order count and used weight rate limit window. Windows are keyed by their
/// `intervalNum`/`intervalLetter` instead of being hard-coded, so any
/// interval Binance reports is retained.
#[derive(Debug, Clone, Default)]
pub struct BinanceUsage {
    /// Current used weight per window, e.g. from `X-MBX-USED-WEIGHT-1M`.
    pub used_weight: HashMap<BinanceUsageInterval, u32>,
    /// Current order count per window, e.g. from `X-MBX-ORDER-COUNT-10S`.
    pub order_count: HashMap<BinanceUsageInterval, u32>,
}

impl BinanceUsage {
    /// Parses a single response header. Recognizes the dynamic
    /// `x-mbx-order-count-{intervalNum}{intervalLetter}` and
    /// `x-mbx-used-weight-{intervalNum}{intervalLetter}` headers; `name` is
    /// expected to be lower-cased.
    fn parse_header(&mut self, name: &str, value: &str) {
        let (usage, suffix) = match name.strip_prefix("x-mbx-order-count-") {
            Some(suffix) => (&mut self.order_count, suffix),
            None => match name.strip_prefix("x-mbx-used-weight-") {
                Some(suffix) => (&mut self.used_weight, suffix),
                None => return,
            },
        };
        let Some(interval) = BinanceUsageInterval::parse(suffix) else {
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
            let Some(interval) =
                BinanceUsageInterval::from_rate_limit(rate_limit.interval, rate_limit.intervalNum)
            else {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn interval(
        interval_num: u32,
        interval_letter: BinanceUsageIntervalLetter,
    ) -> BinanceUsageInterval {
        BinanceUsageInterval {
            interval_num,
            interval_letter,
        }
    }

    fn interval_letter(letter: char) -> BinanceUsageIntervalLetter {
        BinanceUsageIntervalLetter::parse(letter).unwrap()
    }

    #[test]
    fn parses_usage_headers_dynamically() {
        let response = HttpResponse {
            status: 200,
            headers: vec![
                ("X-MBX-USED-WEIGHT-1M".into(), "10".into()),
                ("X-MBX-ORDER-COUNT-10S".into(), "2".into()),
                ("X-MBX-ORDER-COUNT-1H".into(), "3".into()),
                ("X-MBX-ORDER-COUNT-1D".into(), "4".into()),
                // Non-standard intervals are not dropped.
                ("X-MBX-ORDER-COUNT-5M".into(), "5".into()),
                ("Retry-After".into(), "30".into()),
            ],
            body: br#"{}"#.into(),
        };
        let response = BinanceResponse::<serde_json::Value>::try_from_http(response).unwrap();
        let usage = &response.metadata.usage;
        assert_eq!(
            usage.used_weight.get(&interval(1, interval_letter('m'))),
            Some(&10)
        );
        assert_eq!(
            usage.order_count.get(&interval(10, interval_letter('s'))),
            Some(&2)
        );
        assert_eq!(
            usage.order_count.get(&interval(1, interval_letter('h'))),
            Some(&3)
        );
        assert_eq!(
            usage.order_count.get(&interval(1, interval_letter('d'))),
            Some(&4)
        );
        assert_eq!(
            usage.order_count.get(&interval(5, interval_letter('m'))),
            Some(&5)
        );
        assert_eq!(response.metadata.retry_after, Some(30));
    }

    #[test]
    fn ignores_unparseable_usage_headers() {
        let response = HttpResponse {
            status: 200,
            headers: vec![
                // Unsupported interval letter.
                ("X-MBX-USED-WEIGHT-1W".into(), "10".into()),
                // Missing interval number.
                ("X-MBX-ORDER-COUNT-S".into(), "2".into()),
                // Not a usage header.
                ("Content-Type".into(), "application/json".into()),
            ],
            body: br#"{}"#.into(),
        };
        let response = BinanceResponse::<serde_json::Value>::try_from_http(response).unwrap();
        assert!(response.metadata.usage.used_weight.is_empty());
        assert!(response.metadata.usage.order_count.is_empty());
    }

    #[test]
    fn parses_websocket_rate_limits_dynamically() {
        let frame = r#"{
            "id": 1,
            "status": 0,
            "rateLimits": [
                {"rateLimitType":"REQUEST_WEIGHT","interval":"MINUTE","intervalNum":1,"limit":6000,"count":8},
                {"rateLimitType":"ORDERS","interval":"SECOND","intervalNum":10,"limit":100,"count":2},
                {"rateLimitType":"ORDERS","interval":"HOUR","intervalNum":2,"limit":1000,"count":3}
            ],
            "result": {}
        }"#;
        let response =
            BinanceResponse::<serde_json::Value>::try_from_websocket(frame.into()).unwrap();
        let usage = &response.metadata.usage;
        assert_eq!(
            usage.used_weight.get(&interval(1, interval_letter('m'))),
            Some(&8)
        );
        assert_eq!(
            usage.order_count.get(&interval(10, interval_letter('s'))),
            Some(&2)
        );
        assert_eq!(
            usage.order_count.get(&interval(2, interval_letter('h'))),
            Some(&3)
        );
    }
}
