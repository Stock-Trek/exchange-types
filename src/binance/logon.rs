use query_params::QueryParams;

#[cfg(feature = "serde")]
use {
    serde::{Deserialize, Serialize},
    serde_with::skip_serializing_none,
};

#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", skip_serializing_none)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, Clone, Hash, QueryParams)]
pub struct BinanceLogonParams {
    /// The API key. `into_signed` sets it from the signer when signing a
    /// WebSocket API request; it must be `None` for HTTP requests.
    pub apiKey: Option<String>,
    pub timestamp: i64,
}

#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[derive(Debug, Clone)]
pub struct BinanceSessionAuthenticationResult {
    pub apiKey: Option<String>,
    pub authorizedSince: Option<i64>,
    pub connectedSince: i64,
    pub returnRateLimits: bool,
    pub serverTime: i64,
    pub userDataStream: bool,
}
