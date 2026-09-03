use crate::binance::{
    exchange_info::BinanceOrderType,
    recv_window::BinanceRecvWindow,
    spot::{BinanceOrderStatus, BinanceSelfTradeProtection, BinanceSide, BinanceTimeInForce},
};
use query_params::QueryParams;
use rust_decimal::Decimal;
use strum::Display;

#[cfg(feature = "serde")]
use {
    serde::{Deserialize, Serialize},
    serde_with::skip_serializing_none,
};

#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", skip_serializing_none)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, QueryParams)]
pub struct BinanceCancelAllOrdersParams {
    pub recvWindow: Option<BinanceRecvWindow>,
    pub symbol: String,
    pub timestamp: i64,
}

#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", skip_serializing_none)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, QueryParams)]
pub struct BinanceCancelOrderParams {
    pub cancelRestrictions: Option<BinanceCancelRestrictions>,
    pub newClientOrderId: Option<String>,
    pub orderId: Option<i64>,
    pub origClientOrderId: Option<String>,
    pub recvWindow: Option<BinanceRecvWindow>,
    pub symbol: String,
    pub timestamp: i64,
}

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Hash, Display)]
pub enum BinanceCancelRestrictions {
    ONLY_NEW,
    ONLY_PARTIALLY_FILLED,
    #[cfg_attr(feature = "serde", serde(other))]
    Unknown,
}

#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct BinanceCancelOrderResult {
    pub clientOrderId: String,
    pub cummulativeQuoteQty: Decimal,
    pub executedQty: Decimal,
    pub orderId: i64,
    pub orderListId: i32,
    pub origClientOrderId: String,
    pub origQty: Decimal,
    pub origQuoteOrderQty: Decimal,
    pub price: Decimal,
    pub selfTradePreventionMode: BinanceSelfTradeProtection,
    pub side: BinanceSide,
    pub status: BinanceOrderStatus,
    pub symbol: String,
    pub timeInForce: BinanceTimeInForce,
    pub transactTime: i64,
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub r#type: BinanceOrderType,
    pub workingTime: i64,
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    use super::*;

    #[test]
    fn unknown_cancel_restrictions_deserialize_as_unknown() {
        let restrictions: BinanceCancelRestrictions =
            serde_json::from_str(r#""FUTURE_RESTRICTION""#).unwrap();
        assert!(matches!(restrictions, BinanceCancelRestrictions::Unknown));
    }
}
