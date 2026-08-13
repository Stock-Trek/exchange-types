use crate::binance::{
    exchange_info::BinanceExchangeInfoParams, logon::BinanceLogonParams,
    spot::BinanceSpotOrderParams,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "serde")]
use serde_with::skip_serializing_none;

#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", skip_serializing_none)]
#[derive(Debug, Clone)]
pub struct BinanceParams {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub params: BinanceUnsignedParams,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub signature: Option<BinanceSignature>,
}

#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct BinanceSignature {
    pub apiKey: String,
    pub signature: String,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[derive(Debug, Clone, Hash)]
pub enum BinanceUnsignedParams {
    ExchangeInfo(BinanceExchangeInfoParams),
    Logon(BinanceLogonParams),
    SpotOrderRequest(Box<BinanceSpotOrderParams>),
}
