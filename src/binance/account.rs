use crate::{
    binance::{
        exchange_info::BinanceExchangeInfoPermission, recv_window::BinanceRecvWindow,
        request::BinanceRequestFactory, response::BinanceResponse,
    },
    error::ETResult,
    http::{HttpMethod, HttpRequest},
    rate_limited::RateLimitRestriction,
    request::{ETHttpRequest, ETRequest, ETWebsocketRequest},
    signer::Signer,
    websocket_id::ETWebsocketId,
};
use query_params::QueryParams;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Debug, Clone, Default, Hash, QueryParams)]
pub struct BinanceAccountRequest {
    pub apiKey: Option<String>,
    pub omitZeroBalances: Option<bool>,
    pub recvWindow: Option<BinanceRecvWindow>,
    pub timestamp: i64,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BinanceAccountResponse {
    pub accountType: String,
    pub balances: Vec<BinanceAccountBalance>,
    pub brokered: bool,
    pub buyerCommission: i64,
    pub canDeposit: bool,
    pub canTrade: bool,
    pub canWithdraw: bool,
    pub commissionRates: BinanceAccountCommissionRates,
    pub makerCommission: i64,
    pub permissions: Vec<BinanceExchangeInfoPermission>,
    pub preventSor: bool,
    pub requireSelfTradePrevention: bool,
    pub sellerCommission: i64,
    pub takerCommission: i64,
    pub uid: i64,
    pub updateTime: i64,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BinanceAccountBalance {
    pub asset: String,
    pub free: Decimal,
    pub locked: Decimal,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BinanceAccountCommissionRates {
    pub buyer: Decimal,
    pub maker: Decimal,
    pub seller: Decimal,
    pub taker: Decimal,
}

impl ETRequest for BinanceAccountRequest {
    type Response = BinanceResponse<BinanceAccountResponse>;

    fn is_signed(&self) -> bool {
        true
    }
    fn rate_limit_usage(&self, restriction: RateLimitRestriction) -> u32 {
        match restriction {
            RateLimitRestriction::Weight => 20,
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

impl ETHttpRequest for BinanceAccountRequest {
    fn endpoint(&self) -> &'static str {
        "account"
    }
    fn method(&self) -> HttpMethod {
        HttpMethod::GET
    }
    fn try_into_http(self, signer: &Signer) -> ETResult<HttpRequest> {
        BinanceRequestFactory::try_into_http(self, signer)
    }
}

impl ETWebsocketRequest for BinanceAccountRequest {
    fn method(&self) -> &'static str {
        "account.status"
    }
    fn try_into_websocket(self, signer: &Signer, id: ETWebsocketId) -> ETResult<String> {
        BinanceRequestFactory::try_into_websocket(self, signer, id)
    }
}
