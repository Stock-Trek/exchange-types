use crate::{
    binance::{
        recv_window::BinanceRecvWindow, request::BinanceRequestFactory, response::BinanceResponse,
        supporting_types::BinanceOrderResponse,
    },
    error::ETResult,
    http::{HttpMethod, HttpRequest},
    rate_limited::RateLimitRestriction,
    request::{ETHttpRequest, ETRequest, ETWebsocketRequest},
    signer::Signer,
    websocket_id::ETWebsocketId,
};
use query_params::QueryParams;
use serde::Serialize;
use serde_with::skip_serializing_none;

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Debug, Clone, Default, Hash, QueryParams)]
pub struct BinanceOpenOrdersRequest {
    pub apiKey: Option<String>,
    pub recvWindow: Option<BinanceRecvWindow>,
    pub symbol: Option<String>,
    pub timestamp: i64,
}

impl ETRequest for BinanceOpenOrdersRequest {
    type Response = BinanceResponse<Vec<BinanceOrderResponse>>;

    fn is_signed(&self) -> bool {
        true
    }
    fn rate_limit_usage(&self, restriction: RateLimitRestriction) -> u32 {
        match restriction {
            RateLimitRestriction::Weight => {
                if self.symbol.is_some() {
                    6
                } else {
                    80
                }
            }
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

impl ETHttpRequest for BinanceOpenOrdersRequest {
    fn endpoint(&self) -> &'static str {
        "openOrders"
    }
    fn method(&self) -> HttpMethod {
        HttpMethod::GET
    }
    fn try_into_http(self, signer: &Signer) -> ETResult<HttpRequest> {
        BinanceRequestFactory::try_into_http(self, signer)
    }
}

impl ETWebsocketRequest for BinanceOpenOrdersRequest {
    fn method(&self) -> &'static str {
        "openOrders.status"
    }
    fn try_into_websocket(self, signer: &Signer, id: ETWebsocketId) -> ETResult<String> {
        BinanceRequestFactory::try_into_websocket(self, signer, id)
    }
}
