use crate::{
    binance::{
        filters::{BinanceExchangeFilter, BinanceSymbolFilter},
        rate_limits::BinanceRateLimit,
        spot::BinanceSelfTradeProtection,
    },
    ticker::Ticker,
};
use strum::Display;

use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Debug, Clone, Hash)]
pub struct BinanceExchangeInfoParams {
    pub permissions: Vec<BinanceExchangeInfoPermission>,
    pub symbolStatus: BinanceExchangeInfoSymbolStatus,
}

#[derive(Serialize, Deserialize, Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum BinanceExchangeInfoPermission {
    LEVERAGED,
    MARGIN,
    SPOT,
    #[serde(other)]
    Unknown,
}

#[derive(Serialize, Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinanceExchangeInfoSymbolStatus {
    TRADING,
    HALT,
    BREAK,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BinanceExchangeInfoResult {
    pub exchangeFilters: Vec<BinanceExchangeFilter>,
    pub rateLimits: Vec<BinanceRateLimit>,
    pub serverTime: i64,
    pub sors: Option<Vec<BinanceExchangeInfoSors>>,
    pub symbols: Vec<BinanceExchangeInfoSymbol>,
    pub timezone: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BinanceExchangeInfoSors {
    pub baseAsset: Ticker,
    pub symbols: Vec<Ticker>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BinanceExchangeInfoSymbol {
    pub allowTrailingStop: bool,
    pub allowedSelfTradePreventionModes: Vec<BinanceSelfTradeProtection>,
    pub amendAllowed: bool,
    pub baseAsset: Ticker,
    pub baseAssetPrecision: u8,
    pub baseCommissionPrecision: u8,
    pub cancelReplaceAllowed: bool,
    pub defaultSelfTradePreventionMode: BinanceSelfTradeProtection,
    pub filters: Vec<BinanceSymbolFilter>,
    pub icebergAllowed: bool,
    pub isMarginTradingAllowed: bool,
    pub isSpotTradingAllowed: bool,
    pub ocoAllowed: bool,
    pub opoAllowed: bool,
    pub orderTypes: Vec<BinanceOrderType>,
    pub otoAllowed: bool,
    pub pegInstructionsAllowed: bool,
    pub permissionSets: Vec<Vec<BinanceExchangeInfoPermission>>,
    pub permissions: Vec<BinanceExchangeInfoPermission>,
    pub quoteAsset: Ticker,
    pub quoteAssetPrecision: u8,
    pub quoteCommissionPrecision: u8,
    pub quoteOrderQtyMarketAllowed: bool,
    pub quotePrecision: u8,
    pub status: String,
    pub symbol: Ticker,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Display, Clone, Copy, Hash)]
pub enum BinanceOrderType {
    LIMIT,
    LIMIT_MAKER,
    MARKET,
    STOP_LOSS,
    STOP_LOSS_LIMIT,
    TAKE_PROFIT,
    TAKE_PROFIT_LIMIT,
    #[serde(other)]
    Unknown,
}

impl BinanceExchangeInfoParams {
    pub fn query_params(&self) -> String {
        let mut pairs = Vec::new();
        if !self.permissions.is_empty() {
            let permissions_string = self
                .permissions
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(",");
            pairs.push(format!("permissions={}", permissions_string));
        }
        pairs.push(format!("symbolStatus={}", self.symbolStatus));
        pairs.join("&")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_the_permissions_and_symbol_status_filter() {
        let params = BinanceExchangeInfoParams {
            permissions: vec![
                BinanceExchangeInfoPermission::SPOT,
                BinanceExchangeInfoPermission::MARGIN,
                BinanceExchangeInfoPermission::LEVERAGED,
            ],
            symbolStatus: BinanceExchangeInfoSymbolStatus::HALT,
        };
        assert_eq!(
            params.query_params(),
            "permissions=SPOT,MARGIN,LEVERAGED&symbolStatus=HALT"
        );
    }

    #[test]
    fn recognizes_documented_permissions_and_symbol_statuses() {
        for permission in [
            BinanceExchangeInfoPermission::SPOT,
            BinanceExchangeInfoPermission::MARGIN,
            BinanceExchangeInfoPermission::LEVERAGED,
        ] {
            let json = serde_json::to_string(&permission).unwrap();
            assert_eq!(
                serde_json::from_str::<BinanceExchangeInfoPermission>(&json).unwrap(),
                permission
            );
        }
    }

    #[test]
    fn unknown_enum_values_deserialize_as_unknown() {
        let permission: BinanceExchangeInfoPermission =
            serde_json::from_str(r#""FUTURE_PERMISSION""#).unwrap();
        assert!(matches!(permission, BinanceExchangeInfoPermission::Unknown));
        let order_type: BinanceOrderType = serde_json::from_str(r#""FUTURE_ORDER_TYPE""#).unwrap();
        assert!(matches!(order_type, BinanceOrderType::Unknown));
    }
}
