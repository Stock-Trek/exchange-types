use crate::binance::{
    filters::{BinanceAssetFilter, BinanceExchangeFilter, BinanceSymbolFilter},
    recv_window::BinanceRecvWindow,
};
use query_params::QueryParams;

#[cfg(feature = "serde")]
use {
    serde::{Deserialize, Serialize},
    serde_with::skip_serializing_none,
};

#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", skip_serializing_none)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, Clone, Hash, QueryParams)]
pub struct BinanceAssetLimitsParams {
    pub recvWindow: Option<BinanceRecvWindow>,
    pub symbol: String,
    pub timestamp: i64,
}

#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[derive(Debug, Clone)]
pub struct BinanceAssetLimitsResult {
    pub assetFilters: Vec<BinanceAssetFilter>,
    pub exchangeFilters: Vec<BinanceExchangeFilter>,
    pub symbolFilters: Vec<BinanceSymbolFilter>,
}
