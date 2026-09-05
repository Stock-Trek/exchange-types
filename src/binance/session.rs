use crate::{
    binance::{
        recv_window::BinanceRecvWindow, request::BinanceRequestFactory, response::BinanceResponse,
    },
    error::ETResult,
    rate_limited::RateLimitRestriction,
    request::{ETRequest, ETWebsocketRequest},
    signer::Signer,
    websocket_id::ETWebsocketId,
};
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

#[derive(Serialize, Debug, Clone, Default, Hash, QueryParams)]
pub struct BinanceSessionStatusRequest {}

#[derive(Serialize, Debug, Clone, Default, Hash, QueryParams)]
pub struct BinanceSessionLogoutRequest {}

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

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BinanceSessionCloseResponse {}

impl ETRequest for BinanceSessionLogonRequest {
    type Response = BinanceResponse<BinanceSessionAuthenticationResponse>;

    fn is_signed(&self) -> bool {
        true
    }
    fn rate_limit_usage(&self, restriction: RateLimitRestriction) -> u32 {
        match restriction {
            RateLimitRestriction::Weight => 2,
            _ => 0,
        }
    }
    fn set_api_key(&mut self, api_key: Option<String>) {
        self.apiKey = api_key;
    }
    fn query_params(&self, percent_encode: bool) -> String {
        self.query_params(true, percent_encode)
    }
}

impl ETWebsocketRequest for BinanceSessionLogonRequest {
    fn method(&self) -> &'static str {
        "session.logon"
    }
    fn try_into_websocket(self, signer: &Signer, id: ETWebsocketId) -> ETResult<String> {
        BinanceRequestFactory::try_into_websocket(self, signer, id)
    }
}

impl ETRequest for BinanceSessionStatusRequest {
    type Response = BinanceResponse<BinanceSessionAuthenticationResponse>;

    fn is_signed(&self) -> bool {
        true
    }
    fn rate_limit_usage(&self, restriction: RateLimitRestriction) -> u32 {
        match restriction {
            RateLimitRestriction::Weight => 2,
            _ => 0,
        }
    }
    fn set_api_key(&mut self, _api_key: Option<String>) {}
    fn query_params(&self, percent_encode: bool) -> String {
        self.query_params(true, percent_encode)
    }
}

impl ETWebsocketRequest for BinanceSessionStatusRequest {
    fn method(&self) -> &'static str {
        "session.status"
    }
    fn try_into_websocket(self, signer: &Signer, id: ETWebsocketId) -> ETResult<String> {
        BinanceRequestFactory::try_into_websocket(self, signer, id)
    }
}

impl ETRequest for BinanceSessionLogoutRequest {
    type Response = BinanceResponse<BinanceSessionAuthenticationResponse>;

    fn is_signed(&self) -> bool {
        true
    }
    fn rate_limit_usage(&self, restriction: RateLimitRestriction) -> u32 {
        match restriction {
            RateLimitRestriction::Weight => 2,
            _ => 0,
        }
    }
    fn set_api_key(&mut self, _api_key: Option<String>) {}
    fn query_params(&self, percent_encode: bool) -> String {
        self.query_params(true, percent_encode)
    }
}

impl ETWebsocketRequest for BinanceSessionLogoutRequest {
    fn method(&self) -> &'static str {
        "session.logout"
    }
    fn try_into_websocket(self, signer: &Signer, id: ETWebsocketId) -> ETResult<String> {
        BinanceRequestFactory::try_into_websocket(self, signer, id)
    }
}
