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

/// The result of a `session.logon` (or `session.status`/`session.logout`)
/// request.
///
/// Real payloads include the authenticated `apiKey`, and unauthenticated
/// `session.status`/`session.logout` payloads report `apiKey` and
/// `authorizedSince` as JSON `null`, so both are optional.
#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct BinanceSessionAuthenticationResult {
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub apiKey: Option<String>,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub authorizedSince: Option<i64>,
    pub connectedSince: i64,
    pub returnRateLimits: bool,
    pub serverTime: i64,
    pub userDataStream: bool,
}
