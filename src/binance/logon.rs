use query_params::QueryParams;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[derive(Debug, Clone, Hash, QueryParams)]
pub struct BinanceLogonParams {
    pub timestamp: i64,
}

#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[derive(Debug, Clone)]
pub struct BinanceSessionAuthenticationResult {
    pub authorizedSince: i64,
    pub connectedSince: i64,
    pub returnRateLimits: bool,
    pub serverTime: i64,
    pub userDataStream: bool,
}
