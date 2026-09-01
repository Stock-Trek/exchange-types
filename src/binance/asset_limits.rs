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
