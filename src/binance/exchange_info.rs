use crate::{
    binance::{
        filters::{BinanceExchangeFilter, BinanceSymbolFilter},
        rate_limits::BinanceRateLimit,
    },
    ticker::Ticker,
};
use strum::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash)]
pub struct BinanceExchangeInfoParams {
    pub permissions: Vec<BinanceExchangeInfoPermission>,
    pub symbolStatus: BinanceExchangeInfoSymbolStatus,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum BinanceExchangeInfoPermission {
    SPOT,
    MARGIN,
    LEVERAGED,
    TRD_GRP_002,
    TRD_GRP_003,
    TRD_GRP_004,
    TRD_GRP_005,
    TRD_GRP_006,
    TRD_GRP_007,
    TRD_GRP_008,
    TRD_GRP_009,
    TRD_GRP_010,
    TRD_GRP_011,
    TRD_GRP_012,
    TRD_GRP_013,
    TRD_GRP_014,
    TRD_GRP_015,
    TRD_GRP_016,
    TRD_GRP_017,
    TRD_GRP_018,
    TRD_GRP_019,
    TRD_GRP_020,
    TRD_GRP_021,
    TRD_GRP_022,
    TRD_GRP_023,
    TRD_GRP_024,
    TRD_GRP_025,
    #[cfg_attr(feature = "serde", serde(other))]
    Unknown,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinanceExchangeInfoSymbolStatus {
    TRADING,
    HALT,
    BREAK,
    #[cfg_attr(feature = "serde", serde(other))]
    Unknown,
}

#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct BinanceExchangeInfoResult {
    pub exchangeFilters: Vec<BinanceExchangeFilter>,
    pub rateLimits: Vec<BinanceRateLimit>,
    pub serverTime: i64,
    pub symbols: Vec<BinanceExchangeInfoSymbol>,
    pub timezone: String,
}

#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct BinanceExchangeInfoSymbol {
    pub baseAsset: Ticker,
    pub baseAssetPrecision: u8,
    pub baseCommissionPrecision: u8,
    pub filters: Vec<BinanceSymbolFilter>,
    pub isSpotTradingAllowed: bool,
    pub orderTypes: Vec<BinanceOrderType>,
    pub quoteAsset: Ticker,
    pub quoteAssetPrecision: u8,
    pub quoteCommissionPrecision: u8,
    pub quoteOrderQtyMarketAllowed: bool,
    pub quotePrecision: u8,
    pub status: String,
    pub symbol: String,
}

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Display, Clone, Copy, Hash)]
pub enum BinanceOrderType {
    LIMIT,
    LIMIT_MAKER,
    MARKET,
    STOP_LOSS,
    STOP_LOSS_LIMIT,
    TAKE_PROFIT,
    TAKE_PROFIT_LIMIT,
    #[cfg_attr(feature = "serde", serde(other))]
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

#[cfg(all(test, feature = "serde"))]
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
            BinanceExchangeInfoPermission::TRD_GRP_002,
            BinanceExchangeInfoPermission::TRD_GRP_025,
        ] {
            let json = serde_json::to_string(&permission).unwrap();
            assert_eq!(
                serde_json::from_str::<BinanceExchangeInfoPermission>(&json).unwrap(),
                permission
            );
        }
        for status in [
            BinanceExchangeInfoSymbolStatus::TRADING,
            BinanceExchangeInfoSymbolStatus::HALT,
            BinanceExchangeInfoSymbolStatus::BREAK,
        ] {
            let json = serde_json::to_string(&status).unwrap();
            assert_eq!(
                serde_json::from_str::<BinanceExchangeInfoSymbolStatus>(&json).unwrap(),
                status
            );
        }
    }

    #[test]
    fn unknown_enum_values_deserialize_as_unknown() {
        let permission: BinanceExchangeInfoPermission =
            serde_json::from_str(r#""FUTURE_PERMISSION""#).unwrap();
        assert!(matches!(permission, BinanceExchangeInfoPermission::Unknown));
        let status: BinanceExchangeInfoSymbolStatus =
            serde_json::from_str(r#""PRE_TRADING""#).unwrap();
        assert!(matches!(status, BinanceExchangeInfoSymbolStatus::Unknown));
        let order_type: BinanceOrderType = serde_json::from_str(r#""FUTURE_ORDER_TYPE""#).unwrap();
        assert!(matches!(order_type, BinanceOrderType::Unknown));
    }
}
