use crate::binance::{exchange_info::BinanceOrderType, recv_window::BinanceRecvWindow};
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
pub struct BinanceSpotOrderParams {
    pub icebergQty: Option<Decimal>,
    pub newClientOrderId: String,
    pub newOrderRespType: Option<BinanceNewOrderResponseType>,
    pub pegPriceType: Option<BinancePegPriceType>,
    pub pegOffsetValue: Option<i32>,
    pub pegOffsetType: Option<BinancePegOffsetType>,
    pub price: Option<Decimal>,
    pub quantity: Option<Decimal>,
    pub quoteOrderQty: Option<Decimal>,
    pub recvWindow: Option<BinanceRecvWindow>,
    pub selfTradePreventionMode: BinanceSelfTradeProtection,
    pub side: BinanceSide,
    pub stopPrice: Option<Decimal>,
    pub strategyId: Option<i64>,
    pub strategyType: Option<i32>,
    pub symbol: String,
    pub timeInForce: Option<BinanceTimeInForce>,
    pub timestamp: i64,
    pub trailingDelta: Option<u32>,
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub r#type: BinanceOrderType,
}

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinanceNewOrderResponseType {
    ACK,
    RESULT,
    FULL,
}

impl Default for BinanceNewOrderResponseType {
    /// The `newOrderRespType` Binance applies when the parameter is omitted:
    /// `FULL` for `MARKET`/`LIMIT` orders.
    fn default() -> Self {
        Self::FULL
    }
}

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Display, Clone, Copy, Hash)]
pub enum BinancePegPriceType {
    PRIMARY_PEG,
    MARKET_PEG,
}

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Display, Clone, Copy, Hash)]
pub enum BinancePegOffsetType {
    PRICE_LEVEL,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Display, Clone, Copy, Hash)]
pub enum BinanceSide {
    BUY,
    SELL,
}

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Display, Clone, Copy, Hash)]
pub enum BinanceSelfTradeProtection {
    EXPIRE_BOTH,
    EXPIRE_MAKER,
    EXPIRE_TAKER,
    DECREMENT,
    NONE,
    TRANSFER,
    #[cfg_attr(feature = "serde", serde(other))]
    UNKNOWN,
}

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Display, Clone, Copy, Hash)]
pub enum BinanceTimeInForce {
    FOK,
    GTC,
    IOC,
}

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Display, Clone, Copy)]
pub enum BinanceOrderStatus {
    CANCELED,
    EXPIRED,
    EXPIRED_IN_MATCH,
    FILLED,
    NEW,
    PARTIALLY_FILLED,
    PENDING_CANCEL,
    REJECTED,
    #[cfg_attr(feature = "serde", serde(other))]
    UNKNOWN,
}

#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct BinanceSpotOrderResult {
    pub clientOrderId: String,
    pub cummulativeQuoteQty: Decimal,
    pub executedQty: Decimal,
    pub orderId: i64,
    pub orderListId: i32,
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

    fn params(new_order_resp_type: Option<BinanceNewOrderResponseType>) -> BinanceSpotOrderParams {
        BinanceSpotOrderParams {
            icebergQty: None,
            newClientOrderId: "new-client-order-id".into(),
            newOrderRespType: new_order_resp_type,
            pegPriceType: None,
            pegOffsetValue: None,
            pegOffsetType: None,
            price: Some(Decimal::new(10_000, 2)),
            quantity: Some(Decimal::new(1, 0)),
            quoteOrderQty: None,
            recvWindow: None,
            selfTradePreventionMode: BinanceSelfTradeProtection::NONE,
            side: BinanceSide::BUY,
            stopPrice: None,
            strategyId: None,
            strategyType: None,
            symbol: "BTCUSDT".into(),
            timeInForce: Some(BinanceTimeInForce::GTC),
            timestamp: 1_700_000_000_000,
            trailingDelta: None,
            r#type: BinanceOrderType::LIMIT,
        }
    }

    #[test]
    fn omits_optional_new_order_resp_type_when_unset() {
        // newOrderRespType is optional and Binance applies its default (FULL)
        // when it is omitted.
        let json = serde_json::to_value(params(None)).unwrap();
        assert!(json.get("newOrderRespType").is_none());
        assert!(json.get("recvWindow").is_none());
        assert!(
            !params(None)
                .query_params(true)
                .contains("newOrderRespType=")
        );
    }

    #[test]
    fn serializes_new_order_resp_type_when_set() {
        let json = serde_json::to_value(params(Some(BinanceNewOrderResponseType::FULL))).unwrap();
        assert_eq!(json["newOrderRespType"], "FULL");
        assert!(
            params(Some(BinanceNewOrderResponseType::RESULT))
                .query_params(true)
                .contains("newOrderRespType=RESULT")
        );
    }

    #[test]
    fn defaults_to_the_documented_full_response_type() {
        assert_eq!(
            BinanceNewOrderResponseType::default(),
            BinanceNewOrderResponseType::FULL
        );
    }

    #[test]
    fn serializes_recv_window_as_an_integer() {
        let mut params = params(None);
        params.recvWindow = BinanceRecvWindow::try_new(60_000);
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["recvWindow"], 60_000);
        assert!(params.query_params(true).contains("recvWindow=60000"));
    }
}
