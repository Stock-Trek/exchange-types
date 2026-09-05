use crate::{
    binance::request::WebsocketParams,
    error::ETResult,
    http::{HttpMethod, HttpRequest},
    signer::Signer,
    websocket_id::ETWebsocketId,
};

pub trait ETHttpRequest {
    fn try_into_http(self, signer: &Signer) -> ETResult<HttpRequest>;
    fn http_method(&self) -> HttpMethod;
    fn endpoint(&self) -> &str;
    fn is_signed(&self) -> bool;
}

pub trait ETWebsocketRequest {
    fn try_into_websocket(self, signer: &Signer, id: ETWebsocketId) -> ETResult<String>;
    fn into_websocket_params(self) -> Option<WebsocketParams>;
}
