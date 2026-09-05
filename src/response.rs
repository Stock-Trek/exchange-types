use crate::{error::ETResult, http::HttpResponse, rate_limited::RateLimit};
use std::collections::HashMap;

pub trait ETResponse {
    fn rate_limit_usage(&self) -> Option<&HashMap<RateLimit, u32>>;
    fn retry_after(&self) -> Option<u64>;
}

pub trait ETHttpResponse: ETResponse
where
    Self: Sized,
{
    fn try_from_http(response: HttpResponse) -> ETResult<Self>;
}

pub trait ETWebsocketResponse: ETResponse
where
    Self: Sized,
{
    fn try_from_websocket(response: String) -> ETResult<Self>;
}
