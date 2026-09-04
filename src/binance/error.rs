use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct BinanceError {
    pub code: i64,
    pub msg: String,
    /// Optional error payload (e.g. rate-limit details) that Binance nests
    /// under `error.data` in WebSocket API responses. Modeling it means the
    /// retry-after information is no longer dropped during deserialization.
    pub data: Option<BinanceErrorData>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BinanceErrorData {
    /// Server time (epoch ms) at which the error was produced, when Binance
    /// includes it. Paired with `retryAfter` it lets a client compute how
    /// long to back off without needing a local clock.
    pub serverTime: Option<i64>,
    /// When a retry is allowed again: Binance's `error.data.retryAfter`.
    /// For 429/418 rate-limit errors this is an epoch-ms timestamp.
    pub retryAfter: Option<i64>,
}

impl std::fmt::Display for BinanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code, self.msg)
    }
}

impl std::error::Error for BinanceError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rate_limit_error_keeps_retry_after_data() {
        let error: BinanceError = serde_json::from_str(
            r#"{"code":-1003,"msg":"Way too much request weight used; IP banned until 1659146400000. Please use WebSocket Streams for live updates to avoid bans.","data":{"serverTime":1659142907531,"retryAfter":1659146400000}}"#,
        )
        .unwrap();
        assert_eq!(error.code, -1003);
        let data = error.data.expect("error data should be retained");
        assert_eq!(data.serverTime, Some(1659142907531));
        assert_eq!(data.retryAfter, Some(1659146400000));
    }

    #[test]
    fn error_without_data_parses_with_none() {
        let error: BinanceError =
            serde_json::from_str(r#"{"code":-2014,"msg":"API-key format invalid."}"#).unwrap();
        assert_eq!(error.code, -2014);
        assert_eq!(error.msg, "API-key format invalid.");
        assert!(error.data.is_none());
    }
}
