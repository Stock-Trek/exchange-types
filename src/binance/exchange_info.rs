use crate::{
    binance::{
        filters::{BinanceExchangeFilter, BinanceSymbolFilter},
        rate_limits::BinanceRateLimit,
        supporting_types::{BinanceOrderType, BinanceSelfTradeProtection},
    },
    ticker::Ticker,
};
use strum::Display;

use {
    serde::{Deserialize, Serialize},
    serde_with::skip_serializing_none,
};

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Debug, Clone, Hash)]
pub struct BinanceExchangeInfoParams {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub permissions: Vec<BinanceExchangeInfoPermission>,
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub symbols: Vec<String>,
    pub symbolStatus: Option<BinanceExchangeInfoSymbolStatus>,
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
    pub symbol: String,
}

impl BinanceExchangeInfoParams {
    pub fn query_params(&self) -> String {
        let mut pairs = Vec::new();
        match self.permissions.as_slice() {
            [] => {}
            [permission] => pairs.push(format!("permissions={permission}")),
            many => {
                let permissions = many
                    .iter()
                    .map(|permission| format!("\"{permission}\""))
                    .collect::<Vec<_>>()
                    .join(",");
                pairs.push(format!(
                    "permissions={}",
                    percent_encode(&format!("[{permissions}]"))
                ));
            }
        }
        if let Some(symbol) = &self.symbol {
            pairs.push(format!("symbol={symbol}"));
        }
        if !self.symbols.is_empty() {
            let symbols = self
                .symbols
                .iter()
                .map(|symbol| format!("\"{symbol}\""))
                .collect::<Vec<_>>()
                .join(",");
            pairs.push(format!(
                "symbols={}",
                percent_encode(&format!("[{symbols}]"))
            ));
        }
        if let Some(symbol_status) = &self.symbolStatus {
            pairs.push(format!("symbolStatus={symbol_status}"));
        }
        pairs.join("&")
    }
}

fn percent_encode(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' | b'-' | b'.' | b'_' | b'~' => {
                encoded.push(char::from(byte))
            }
            byte => encoded.push_str(&format!("%{byte:02X}")),
        }
    }
    encoded
}

#[cfg(test)]
mod tests {
    use super::*;

    fn params(
        permissions: Vec<BinanceExchangeInfoPermission>,
        symbol: Option<&str>,
        symbols: Vec<&str>,
        symbol_status: Option<BinanceExchangeInfoSymbolStatus>,
    ) -> BinanceExchangeInfoParams {
        BinanceExchangeInfoParams {
            permissions,
            symbol: symbol.map(String::from),
            symbols: symbols.into_iter().map(String::from).collect(),
            symbolStatus: symbol_status,
        }
    }

    #[test]
    fn serializes_a_single_permission_as_a_bare_value() {
        let params = params(
            vec![BinanceExchangeInfoPermission::SPOT],
            None,
            vec![],
            None,
        );
        assert_eq!(params.query_params(), "permissions=SPOT");
    }

    #[test]
    fn serializes_multiple_permissions_as_a_url_encoded_json_array() {
        let params = params(
            vec![
                BinanceExchangeInfoPermission::SPOT,
                BinanceExchangeInfoPermission::MARGIN,
                BinanceExchangeInfoPermission::LEVERAGED,
            ],
            None,
            vec![],
            Some(BinanceExchangeInfoSymbolStatus::HALT),
        );
        assert_eq!(
            params.query_params(),
            "permissions=%5B%22SPOT%22%2C%22MARGIN%22%2C%22LEVERAGED%22%5D&symbolStatus=HALT"
        );
    }

    #[test]
    fn no_params_queries_all_symbols() {
        let params = params(vec![], None, vec![], None);
        assert_eq!(params.query_params(), "");
    }

    #[test]
    fn serializes_a_symbol_query() {
        let params = params(vec![], Some("BNBBTC"), vec![], None);
        assert_eq!(params.query_params(), "symbol=BNBBTC");
    }

    #[test]
    fn serializes_symbols_as_a_url_encoded_json_array() {
        let params = params(vec![], None, vec!["BNBBTC", "BTCUSDT"], None);
        assert_eq!(
            params.query_params(),
            "symbols=%5B%22BNBBTC%22%2C%22BTCUSDT%22%5D"
        );
    }

    #[test]
    fn websocket_json_omits_unset_filters() {
        let params = params(
            vec![BinanceExchangeInfoPermission::SPOT],
            None,
            vec![],
            Some(BinanceExchangeInfoSymbolStatus::TRADING),
        );
        let json = serde_json::to_value(params).unwrap();
        assert_eq!(json["permissions"], serde_json::json!(["SPOT"]));
        assert_eq!(json["symbolStatus"], "TRADING");
        assert!(json.get("symbol").is_none());
        assert!(json.get("symbols").is_none());
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
