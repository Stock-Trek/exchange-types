use crate::{
    binance::{
        filters::{BinanceAssetFilter, BinanceExchangeFilter, BinanceSymbolFilter},
        recv_window::BinanceRecvWindow,
    },
    ticker::Ticker,
};
use query_params::QueryParams;

use {
    serde::{Deserialize, Serialize},
    serde_with::skip_serializing_none,
};

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Debug, Clone, Hash, QueryParams)]
pub struct BinanceAssetLimitsParams {
    pub apiKey: Option<String>,
    pub recvWindow: Option<BinanceRecvWindow>,
    pub symbol: Ticker,
    pub timestamp: i64,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BinanceAssetLimitsResult {
    pub assetFilters: Vec<BinanceAssetFilter>,
    pub exchangeFilters: Vec<BinanceExchangeFilter>,
    pub symbolFilters: Vec<BinanceSymbolFilter>,
}
