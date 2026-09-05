use crate::{
    binance::{
        filters::{BinanceAssetFilter, BinanceExchangeFilter, BinanceSymbolFilter},
        recv_window::BinanceRecvWindow,
        request::BinanceRequestFactory,
        response::BinanceResponse,
    },
    error::ETResult,
    http::{HttpMethod, HttpRequest},
    rate_limited::RateLimitRestriction,
    request::{ETHttpRequest, ETRequest, ETWebsocketRequest},
    signer::Signer,
    websocket_id::ETWebsocketId,
};
use query_params::QueryParams;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Debug, Clone, Default, Hash, QueryParams)]
pub struct BinanceAssetLimitsRequest {
    pub apiKey: Option<String>,
    pub recvWindow: Option<BinanceRecvWindow>,
    pub symbol: String,
    pub timestamp: i64,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BinanceAssetLimitsResponse {
    pub assetFilters: Vec<BinanceAssetFilter>,
    pub exchangeFilters: Vec<BinanceExchangeFilter>,
    pub symbolFilters: Vec<BinanceSymbolFilter>,
}

impl ETRequest for BinanceAssetLimitsRequest {
    type Response = BinanceResponse<BinanceAssetLimitsResponse>;

    fn is_signed(&self) -> bool {
        true
    }
    fn rate_limit_usage(&self, restriction: RateLimitRestriction) -> u32 {
        match restriction {
            RateLimitRestriction::Weight => 40,
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

impl ETHttpRequest for BinanceAssetLimitsRequest {
    fn endpoint(&self) -> &'static str {
        "myFilters"
    }
    fn method(&self) -> HttpMethod {
        HttpMethod::GET
    }
    fn try_into_http(self, signer: &Signer) -> ETResult<HttpRequest> {
        BinanceRequestFactory::try_into_http(self, signer)
    }
}

impl ETWebsocketRequest for BinanceAssetLimitsRequest {
    fn method(&self) -> &'static str {
        "myFilters"
    }
    fn try_into_websocket(self, signer: &Signer, id: ETWebsocketId) -> ETResult<String> {
        BinanceRequestFactory::try_into_websocket(self, signer, id)
    }
}
