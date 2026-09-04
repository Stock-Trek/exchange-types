use crate::{error::ETResult, http::HttpRequest, signer::Signer};

pub trait ETHttpRequest {
    fn try_into_http(self, signer: &Signer) -> ETResult<HttpRequest>;
}

pub trait ETWebsocketRequest {
    fn try_into_websocket(self, signer: &Signer, id: String) -> ETResult<String>;
}
