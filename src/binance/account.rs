use crate::binance::{
    exchange_info::BinanceExchangeInfoPermission, recv_window::BinanceRecvWindow,
};
use query_params::QueryParams;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Get current account information (`GET /api/v3/account`, WebSocket
/// `account.status`).
#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Debug, Clone, Hash, QueryParams)]
pub struct BinanceAccountParams {
    pub apiKey: Option<String>,
    pub omitZeroBalances: Option<bool>,
    pub recvWindow: Option<BinanceRecvWindow>,
    pub timestamp: i64,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BinanceAccountResult {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn params(omit_zero_balances: bool) -> BinanceAccountParams {
        BinanceAccountParams {
            apiKey: None,
            omitZeroBalances: omit_zero_balances.then_some(true),
            recvWindow: None,
            timestamp: 1_660_801_720_951,
        }
    }

    #[test]
    fn query_params_include_omit_zero_balances_only_when_set() {
        assert_eq!(
            params(false).query_params(true, true),
            "timestamp=1660801720951"
        );
        assert_eq!(
            params(true).query_params(true, true),
            "omitZeroBalances=true&timestamp=1660801720951"
        );
    }

    #[test]
    fn serialization_skips_unset_omit_zero_balances() {
        let json = serde_json::to_value(params(false)).unwrap();
        assert!(json.get("omitZeroBalances").is_none());
        assert!(json.get("apiKey").is_none());
        let json = serde_json::to_value(params(true)).unwrap();
        assert_eq!(json["omitZeroBalances"], true);
    }
}
