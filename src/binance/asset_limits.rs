use crate::binance::filters::{BinanceAssetFilter, BinanceExchangeFilter, BinanceSymbolFilter};
use query_params::QueryParams;
use rust_decimal::Decimal;

#[cfg(feature = "serde")]
use {
    serde::{Deserialize, Serialize},
    serde_with::skip_serializing_none,
};

#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", skip_serializing_none)]
#[derive(Debug, Clone, Hash, QueryParams)]
pub struct BinanceAssetLimitsParams {
    pub recvWindow: Option<Decimal>,
    pub symbol: String,
    pub timestamp: i64,
}

/// The result of a `myFilters` request (`GET /api/v3/myFilters`): the
/// exchange-, symbol- and asset-level filters relevant to the account on a
/// symbol. This is the only response that can contain
/// [`BinanceAssetFilter`]s.
#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct BinanceAssetLimitsResult {
    pub exchangeFilters: Vec<BinanceExchangeFilter>,
    pub symbolFilters: Vec<BinanceSymbolFilter>,
    pub assetFilters: Vec<BinanceAssetFilter>,
}
