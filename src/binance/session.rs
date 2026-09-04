use crate::{binance::recv_window::BinanceRecvWindow, response::ResponseFor};
use query_params::QueryParams;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Debug, Clone, Default, Hash, QueryParams)]
pub struct BinanceSessionLogonRequest {
    pub apiKey: Option<String>,
    pub recvWindow: Option<BinanceRecvWindow>,
    pub timestamp: i64,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BinanceSessionAuthenticationResponse {
    pub apiKey: Option<String>,
    pub authorizedSince: Option<i64>,
    pub connectedSince: i64,
    pub returnRateLimits: bool,
    pub serverTime: i64,
    pub userDataStream: bool,
}

impl ResponseFor for BinanceSessionLogonRequest {
    type Response = BinanceSessionAuthenticationResponse;
}

#[derive(Serialize, Debug, Clone, Default, Hash, QueryParams)]
pub struct BinanceSessionLogoutRequest {}

impl ResponseFor for BinanceSessionLogoutRequest {
    type Response = BinanceSessionAuthenticationResponse;
}
