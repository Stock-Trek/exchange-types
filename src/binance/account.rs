use crate::{
    binance::{exchange_info::BinanceExchangeInfoPermission, recv_window::BinanceRecvWindow},
    response::ResponseFor,
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
    pub accountType: Option<String>,
    pub balances: Option<Vec<BinanceAccountBalance>>,
    pub brokered: Option<bool>,
    pub buyerCommission: Option<i64>,
    pub canDeposit: Option<bool>,
    pub canTrade: Option<bool>,
    pub canWithdraw: Option<bool>,
    pub commissionRates: Option<BinanceAccountCommissionRates>,
    pub makerCommission: Option<i64>,
    pub permissions: Option<Vec<BinanceExchangeInfoPermission>>,
    pub preventSor: Option<bool>,
    pub requireSelfTradePrevention: Option<bool>,
    pub sellerCommission: Option<i64>,
    pub takerCommission: Option<i64>,
    pub uid: Option<i64>,
    pub updateTime: Option<i64>,
}

impl ResponseFor for BinanceAccountRequest {
    type Response = BinanceAccountResponse;
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BinanceAccountBalance {
    pub asset: Option<String>,
    pub free: Option<Decimal>,
    pub locked: Option<Decimal>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BinanceAccountCommissionRates {
    pub buyer: Option<Decimal>,
    pub maker: Option<Decimal>,
    pub seller: Option<Decimal>,
    pub taker: Option<Decimal>,
}
