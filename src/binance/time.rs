use crate::{
    binance::{request::BinanceRequestFactory, response::BinanceResponse},
    error::ETResult,
    http::{HttpMethod, HttpRequest},
    rate_limited::RateLimitRestriction,
    request::{ETHttpRequest, ETRequest, ETWebsocketRequest},
    signer::Signer,
    websocket_id::ETWebsocketId,
};
use query_params::QueryParams;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone, Default, Hash, QueryParams)]
pub struct BinanceTimeRequest {}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BinanceTimeResponse {
    pub serverTime: i64,
}

impl ETRequest for BinanceTimeRequest {
    type Response = BinanceResponse<BinanceTimeResponse>;

    fn is_signed(&self) -> bool {
        true
    }
    fn rate_limit_usage(&self, restriction: RateLimitRestriction) -> u32 {
        match restriction {
            RateLimitRestriction::Weight => 1,
            _ => 0,
        }
    }
    fn set_api_key(&mut self, _api_key: Option<String>) {}
    fn query_params(&self, percent_encode: bool) -> String {
        self.query_params(true, percent_encode)
    }
}

impl ETHttpRequest for BinanceTimeRequest {
    fn endpoint(&self) -> &'static str {
        "time"
    }
    fn http_method(&self) -> HttpMethod {
        HttpMethod::GET
    }
    fn try_into_http(self, signer: &Signer) -> ETResult<HttpRequest> {
        BinanceRequestFactory::try_into_http(self, signer)
    }
}

impl ETWebsocketRequest for BinanceTimeRequest {
    fn method_name(&self) -> &'static str {
        "time"
    }
    fn try_into_websocket(self, signer: &Signer, id: ETWebsocketId) -> ETResult<String> {
        BinanceRequestFactory::try_into_websocket(self, signer, id)
    }
}
