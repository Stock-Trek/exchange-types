use crate::{
    error::ETResult, http::HttpMethod, http::HttpRequest, rate_limited::RateLimitRestriction,
    signer::Signer, websocket_id::ETWebsocketId,
};
use serde::Serialize;

pub trait ETRequest: Serialize {
    type Response;

    fn rate_limit_usage(&self, restriction: RateLimitRestriction) -> u32;
    fn is_signed(&self) -> bool;
    fn set_api_key(&mut self, api_key: Option<String>);
    fn query_params(&self, percent_encode: bool) -> String;
}

pub trait ETHttpRequest: ETRequest {
    fn method(&self) -> HttpMethod;
    fn endpoint(&self) -> &'static str;
    fn try_into_http(self, signer: &Signer) -> ETResult<HttpRequest>
    where
        Self: Sized;
}

pub trait ETWebsocketRequest: ETRequest {
    fn method(&self) -> &'static str;
    fn try_into_websocket(self, signer: &Signer, id: ETWebsocketId) -> ETResult<String>
    where
        Self: Sized;
}
