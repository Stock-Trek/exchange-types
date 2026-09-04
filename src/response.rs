use crate::{error::ETResult, http::HttpResponse};

pub trait ETHttpResponse
where
    Self: Sized,
{
    fn try_from_http(response: HttpResponse) -> ETResult<Self>;
}

pub trait ETWebsocketResponse
where
    Self: Sized,
{
    fn try_from_websocket(response: String) -> ETResult<Self>;
}
