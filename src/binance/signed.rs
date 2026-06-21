use crate::binance::{logon::BinanceLogonParams, spot::BinanceSpotOrderParams};
use serde::Serialize;
use serde_with::skip_serializing_none;

#[derive(Debug, Serialize)]
#[skip_serializing_none]
pub struct BinanceSignedParams {
    #[serde(flatten)]
    pub signature: Option<BinanceSignature>,
    #[serde(flatten)]
    pub params: BinanceUnsignedParams,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct BinanceSignature {
    pub apiKey: String,
    pub signature: String,
}

#[derive(Debug, Hash, Serialize)]
#[serde(untagged)]
pub enum BinanceUnsignedParams {
    LogonParams(BinanceLogonParams),
    SpotOrderRequest(Box<BinanceSpotOrderParams>),
}
