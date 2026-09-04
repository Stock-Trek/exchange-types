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

impl ResponseFor for BinanceAccountRequest {
    type Response = BinanceAccountResponse;
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
