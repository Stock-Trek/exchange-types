use crate::{
    binance::{
        filters::{BinanceExchangeFilter, BinanceSymbolFilter},
        rate_limits::BinanceRateLimit,
        supporting_types::{BinanceOrderType, BinanceSelfTradeProtection},
    },
    encode::ByteEncoder,
    response::ResponseFor,
    ticker::Ticker,
};
use serde::{Deserialize, Serialize};
use strum::Display;

#[allow(non_snake_case)]
#[derive(Serialize, Debug, Clone, Hash)]
#[serde(untagged)]
pub enum BinanceExchangeInfoRequest {
    All {
        #[serde(skip_serializing_if = "Option::is_none")]
        symbolStatus: Option<BinanceExchangeInfoSymbolStatus>,
    },
    Symbol {
        symbol: String,
    },
    Symbols {
        symbols: Vec<String>,
    },
    Permissions {
        permissions: BinanceExchangeInfoPermissions,
        #[serde(skip_serializing_if = "Option::is_none")]
        symbolStatus: Option<BinanceExchangeInfoSymbolStatus>,
    },
}

impl Default for BinanceExchangeInfoRequest {
    fn default() -> Self {
        Self::All { symbolStatus: None }
    }
}

#[derive(Serialize, Debug, Clone, Hash)]
#[serde(untagged)]
pub enum BinanceExchangeInfoPermissions {
    Single(BinanceExchangeInfoPermission),
    List(Vec<BinanceExchangeInfoPermission>),
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
pub struct BinanceExchangeInfoResponse {
    pub exchangeFilters: Vec<BinanceExchangeFilter>,
    pub rateLimits: Vec<BinanceRateLimit>,
    pub serverTime: i64,
    pub sors: Option<Vec<BinanceExchangeInfoSors>>,
    pub symbols: Vec<BinanceExchangeInfoSymbol>,
    pub timezone: String,
}

impl ResponseFor for BinanceExchangeInfoRequest {
    type Response = BinanceExchangeInfoResponse;
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

impl BinanceExchangeInfoRequest {
    pub fn query_params(&self) -> String {
        let mut pairs = Vec::new();
        match self {
            Self::All { symbolStatus } => {
                if let Some(symbol_status) = symbolStatus {
                    pairs.push(format!("symbolStatus={symbol_status}"));
                }
            }
            Self::Symbol { symbol } => pairs.push(format!("symbol={symbol}")),
            Self::Symbols { symbols } => pairs.push(format!(
                "symbols={}",
                ByteEncoder::Percent.encode(Self::json_array(symbols).as_bytes())
            )),
            Self::Permissions {
                permissions,
                symbolStatus,
            } => {
                match permissions {
                    BinanceExchangeInfoPermissions::Single(permission) => {
                        pairs.push(format!("permissions={permission}"));
                    }
                    BinanceExchangeInfoPermissions::List(permissions) => pairs.push(format!(
                        "permissions={}",
                        ByteEncoder::Percent.encode(Self::json_array(permissions).as_bytes())
                    )),
                }
                if let Some(symbol_status) = symbolStatus {
                    pairs.push(format!("symbolStatus={symbol_status}"));
                }
            }
        }
        pairs.join("&")
    }
    fn json_array(values: &[impl std::fmt::Display]) -> String {
        let values = values
            .iter()
            .map(|value| format!("\"{value}\""))
            .collect::<Vec<_>>()
            .join(",");
        format!("[{values}]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binance::{filters::BinanceSymbolFilter, supporting_types::BinanceOrderType};

    // Captured live from `GET https://api.binance.us/api/v3/exchangeInfo?symbol=BTCUSD`
    // on 2026-09-04. Binance.US quotes against USD (BTCUSD) rather than USDT, and
    // returns the same schema as Binance's global API.
    const BINANCE_US_BTCUSD_EXCHANGE_INFO: &str = r#"{"timezone":"UTC","serverTime":1788540324317,"rateLimits":[{"rateLimitType":"REQUEST_WEIGHT","interval":"MINUTE","intervalNum":1,"limit":6000},{"rateLimitType":"ORDERS","interval":"SECOND","intervalNum":10,"limit":100},{"rateLimitType":"ORDERS","interval":"DAY","intervalNum":1,"limit":200000},{"rateLimitType":"RAW_REQUESTS","interval":"MINUTE","intervalNum":5,"limit":300000}],"exchangeFilters":[],"symbols":[{"symbol":"BTCUSD","status":"TRADING","baseAsset":"BTC","baseAssetPrecision":8,"quoteAsset":"USD","quotePrecision":8,"quoteAssetPrecision":8,"baseCommissionPrecision":8,"quoteCommissionPrecision":8,"orderTypes":["LIMIT","LIMIT_MAKER","MARKET","STOP_LOSS","STOP_LOSS_LIMIT","TAKE_PROFIT","TAKE_PROFIT_LIMIT"],"icebergAllowed":true,"ocoAllowed":true,"otoAllowed":true,"opoAllowed":true,"quoteOrderQtyMarketAllowed":true,"allowTrailingStop":true,"cancelReplaceAllowed":true,"amendAllowed":true,"pegInstructionsAllowed":true,"isSpotTradingAllowed":true,"isMarginTradingAllowed":false,"filters":[{"filterType":"PRICE_FILTER","minPrice":"0.01000000","maxPrice":"1000000.00000000","tickSize":"0.01000000"},{"filterType":"PERCENT_PRICE","multiplierUp":"5","multiplierDown":"0.2","avgPriceMins":5},{"filterType":"LOT_SIZE","minQty":"0.00001000","maxQty":"9000.00000000","stepSize":"0.00001000"},{"filterType":"MIN_NOTIONAL","minNotional":"1.00000000","applyToMarket":true,"avgPriceMins":5},{"filterType":"ICEBERG_PARTS","limit":10},{"filterType":"MARKET_LOT_SIZE","minQty":"0.00000000","maxQty":"10.64771804","stepSize":"0.00000000"},{"filterType":"TRAILING_DELTA","minTrailingAboveDelta":10,"maxTrailingAboveDelta":2000,"minTrailingBelowDelta":10,"maxTrailingBelowDelta":2000},{"filterType":"PERCENT_PRICE_BY_SIDE","bidMultiplierUp":"5","bidMultiplierDown":"0.2","askMultiplierUp":"5","askMultiplierDown":"0.2","avgPriceMins":5},{"filterType":"MAX_NUM_ORDERS","maxNumOrders":200},{"filterType":"MAX_NUM_ORDER_LISTS","maxNumOrderLists":20},{"filterType":"MAX_NUM_ALGO_ORDERS","maxNumAlgoOrders":5},{"filterType":"MAX_NUM_ORDER_AMENDS","maxNumOrderAmends":10}],"permissions":[],"permissionSets":[["SPOT"]],"defaultSelfTradePreventionMode":"EXPIRE_MAKER","allowedSelfTradePreventionModes":["EXPIRE_TAKER","EXPIRE_MAKER","EXPIRE_BOTH","DECREMENT","TRANSFER"]}]}"#;

    #[test]
    fn deserializes_real_binance_us_exchange_info() {
        let response: BinanceExchangeInfoResponse =
            serde_json::from_str(BINANCE_US_BTCUSD_EXCHANGE_INFO).unwrap();

        assert_eq!(response.timezone, "UTC");
        assert_eq!(response.serverTime, 1788540324317);
        assert_eq!(response.rateLimits.len(), 4);
        assert!(response.exchangeFilters.is_empty());

        assert_eq!(response.symbols.len(), 1);
        let symbol = &response.symbols[0];
        assert_eq!(symbol.symbol, "BTCUSD");
        assert_eq!(symbol.baseAsset.to_string(), "BTC");
        assert_eq!(symbol.quoteAsset.to_string(), "USD");
        assert_eq!(symbol.baseAssetPrecision, 8);
        assert!(symbol.isSpotTradingAllowed);
        assert!(!symbol.isMarginTradingAllowed);
        assert_eq!(symbol.orderTypes.len(), 7);
        assert!(symbol.orderTypes.contains(&BinanceOrderType::LIMIT));
        assert!(symbol.orderTypes.contains(&BinanceOrderType::MARKET));
        assert!(matches!(
            symbol.filters.first(),
            Some(BinanceSymbolFilter::PRICE_FILTER { .. })
        ));
        assert!(
            symbol
                .filters
                .iter()
                .any(|filter| matches!(filter, BinanceSymbolFilter::LOT_SIZE { .. }))
        );
        assert_eq!(symbol.permissions.len(), 0);
        assert_eq!(
            symbol.permissionSets,
            vec![vec![BinanceExchangeInfoPermission::SPOT]]
        );
        assert_eq!(
            symbol.defaultSelfTradePreventionMode,
            BinanceSelfTradeProtection::EXPIRE_MAKER
        );
        assert!(
            symbol
                .allowedSelfTradePreventionModes
                .contains(&BinanceSelfTradeProtection::DECREMENT)
        );
    }
}
