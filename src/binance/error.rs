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
