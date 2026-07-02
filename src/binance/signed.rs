use crate::binance::{
    exchange_info::BinanceExchangeInfoParams, logon::BinanceLogonParams,
    spot::BinanceSpotOrderParams,
};
use serde::Serialize;
use serde_with::skip_serializing_none;

#[derive(Debug, Clone, Serialize)]
#[skip_serializing_none]
pub struct BinanceParams {
    #[serde(flatten)]
    pub params: BinanceUnsignedParams,
    #[serde(flatten)]
    pub signature: Option<BinanceSignature>,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize)]
pub struct BinanceSignature {
    pub apiKey: String,
    pub signature: String,
}

#[derive(Debug, Clone, Hash, Serialize)]
#[serde(untagged)]
pub enum BinanceUnsignedParams {
    ExchangeInfo(BinanceExchangeInfoParams),
    Logon(BinanceLogonParams),
    SpotOrderRequest(Box<BinanceSpotOrderParams>),
}
