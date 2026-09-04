use serde::{Deserialize, Serialize};
use strum::Display;

#[allow(non_camel_case_types)]
#[derive(Deserialize, Debug, Display, Clone, Copy, PartialEq, Eq)]
pub enum BinanceExpiryReason {
    EXCHANGE_CANCELED,
    EXECUTION_RULE_PRICE_RANGE_EXCEEDED,
    INSUFFICIENT_LIQUIDITY,
    NONE,
    OCO_TRIGGER,
    OTO_PHASE_ONE_EXPIRED,
    REJECTED,
    UNFILLED_FOK_ORDER_EXPIRED,
    UNFILLED_IOC_QUANTITY_EXPIRED,
    #[serde(other)]
    Unknown,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BinanceOrderListOrder {
    pub clientOrderId: String,
    pub orderId: i64,
    pub symbol: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct BinanceOrderListStatus {
    pub contingencyType: String,
    pub listClientOrderId: String,
    pub listOrderStatus: String,
    pub orderListId: i64,
    #[serde(default)]
    pub orders: Vec<BinanceOrderListOrder>,
    pub symbol: String,
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
    PENDING_NEW,
    REJECTED,
    #[serde(other)]
    Unknown,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinanceOrderType {
    LIMIT,
    LIMIT_MAKER,
    MARKET,
    STOP_LOSS,
    STOP_LOSS_LIMIT,
    TAKE_PROFIT,
    TAKE_PROFIT_LIMIT,
    #[serde(other)]
    Unknown,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinancePegOffsetType {
    PRICE_LEVEL,
    #[serde(other)]
    Unknown,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinancePegPriceType {
    PRIMARY_PEG,
    MARKET_PEG,
    #[serde(other)]
    Unknown,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Serialize, Deserialize, Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinanceSide {
    BUY,
    SELL,
    #[serde(other)]
    Unknown,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinanceTimeInForce {
    FOK,
    GTC,
    IOC,
    #[serde(other)]
    Unknown,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize, Debug, Display, Clone, Copy, PartialEq, Eq)]
pub enum BinanceWorkingFloor {
    EXCHANGE,
    SOR,
    #[serde(other)]
    Unknown,
}
