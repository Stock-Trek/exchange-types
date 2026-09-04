use crate::{
    binance::{exchange_info::BinanceOrderType, recv_window::BinanceRecvWindow},
    ticker::Ticker,
};
use query_params::QueryParams;
use rust_decimal::Decimal;
use strum::Display;

use {
    serde::{Deserialize, Serialize},
    serde_with::skip_serializing_none,
};

#[allow(non_snake_case)]
#[skip_serializing_none]
#[derive(Serialize, Debug, Clone, Hash, QueryParams)]
pub struct BinanceSpotOrderParams {
    pub apiKey: Option<String>,
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
    #[serde(rename = "type")]
    pub r#type: BinanceOrderType,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinanceNewOrderResponseType {
    ACK,
    RESULT,
    FULL,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Debug, Display, Clone, Copy, Hash)]
pub enum BinancePegPriceType {
    PRIMARY_PEG,
    MARKET_PEG,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Debug, Display, Clone, Copy, Hash)]
pub enum BinancePegOffsetType {
    PRICE_LEVEL,
}

#[derive(Serialize, Deserialize, Debug, Display, Clone, Copy, Hash)]
pub enum BinanceSide {
    BUY,
    SELL,
    #[serde(other)]
    Unknown,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Display, Clone, Copy, Hash)]
pub enum BinanceSelfTradeProtection {
    EXPIRE_BOTH,
    EXPIRE_MAKER,
    EXPIRE_TAKER,
    DECREMENT,
    NONE,
    TRANSFER,
    #[serde(other)]
    Unknown,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Display, Clone, Copy, Hash)]
pub enum BinanceTimeInForce {
    FOK,
    GTC,
    IOC,
    #[serde(other)]
    Unknown,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize, Debug, Display, Clone, Copy, PartialEq, Eq)]
pub enum BinanceOrderStatus {
    CANCELED,
    EXPIRED,
    EXPIRED_IN_MATCH,
    FILLED,
    NEW,
    PARTIALLY_FILLED,
    PENDING_CANCEL,
    REJECTED,
    #[serde(other)]
    Unknown,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
#[skip_serializing_none]
#[derive(Debug, Clone)]
pub struct BinanceSpotOrderResult {
    pub clientOrderId: String,
    pub cummulativeQuoteQty: Option<Decimal>,
    pub executedQty: Option<Decimal>,
    pub fills: Option<Vec<BinanceFill>>,
    pub icebergQty: Option<Decimal>,
    pub orderId: i64,
    pub orderListId: i32,
    pub origQty: Option<Decimal>,
    pub origQuoteOrderQty: Option<Decimal>,
    pub preventedMatchId: Option<i64>,
    pub preventedQuantity: Option<Decimal>,
    pub price: Option<Decimal>,
    pub selfTradePreventionMode: Option<BinanceSelfTradeProtection>,
    pub side: Option<BinanceSide>,
    pub status: Option<BinanceOrderStatus>,
    pub stopPrice: Option<Decimal>,
    pub strategyId: Option<i64>,
    pub strategyType: Option<i32>,
    pub symbol: String,
    pub timeInForce: Option<BinanceTimeInForce>,
    pub trailingDelta: Option<i64>,
    pub trailingTime: Option<i64>,
    pub transactTime: i64,
    #[serde(rename = "type")]
    pub r#type: Option<BinanceOrderType>,
    pub workingTime: Option<i64>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BinanceFill {
    pub commission: Decimal,
    pub commissionAsset: Ticker,
    pub price: Decimal,
    pub qty: Decimal,
    pub tradeId: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn params(new_order_resp_type: Option<BinanceNewOrderResponseType>) -> BinanceSpotOrderParams {
        BinanceSpotOrderParams {
            apiKey: None,
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
    fn serializes_recv_window_as_an_integer() {
        let mut params = params(None);
        params.recvWindow = BinanceRecvWindow::try_new(60_000);
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["recvWindow"], 60_000);
        assert!(params.query_params(true).contains("recvWindow=60000"));
    }

    #[test]
    fn unknown_enum_values_deserialize_as_unknown() {
        let side: BinanceSide = serde_json::from_str(r#""FUTURE_SIDE""#).unwrap();
        assert!(matches!(side, BinanceSide::Unknown));
        let time_in_force: BinanceTimeInForce = serde_json::from_str(r#""FUTURE_TIF""#).unwrap();
        assert!(matches!(time_in_force, BinanceTimeInForce::Unknown));
    }
}
