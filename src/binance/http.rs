use crate::{
    binance::{
        amend::{BinanceAmendOrderParams, BinanceAmendOrderResult},
        asset_limits::{BinanceAssetLimitsParams, BinanceAssetLimitsResult},
        cancel::{
            BinanceCancelAllOrdersParams, BinanceCancelOrderListResult, BinanceCancelOrderParams,
            BinanceCancelOrderResult, BinanceCancelReport,
        },
        error::BinanceError,
        exchange_info::{BinanceExchangeInfoParams, BinanceExchangeInfoResult},
        signature::BinanceSignature,
        spot::{BinanceSpotOrderParams, BinanceSpotOrderResult},
        time::{BinanceTimeParams, BinanceTimeResult},
    },
    error::ETResult,
    http::{HttpMethod, HttpRequest},
    rate_limited::RateLimited,
    signer::{IntoSigned, Signer},
};

#[cfg(feature = "serde")]
use {
    crate::{error::ETError, http::HttpResponse},
    serde::{Deserialize, Serialize},
    serde_json,
    serde_with::skip_serializing_none,
};

#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[derive(Debug, Clone, Hash)]
pub enum BinanceHttpUnsignedRequest {
    AmendOrderRequest(BinanceAmendOrderParams),
    AssetLimits(BinanceAssetLimitsParams),
    CancelAllOrdersRequest(BinanceCancelAllOrdersParams),
    CancelOrderRequest(BinanceCancelOrderParams),
    ExchangeInfo(BinanceExchangeInfoParams),
    SpotOrderRequest(Box<BinanceSpotOrderParams>),
    Time(BinanceTimeParams),
}

#[cfg_attr(feature = "serde", skip_serializing_none)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct BinanceHttpRequest {
    pub unsigned: BinanceHttpUnsignedRequest,
    pub signature: Option<BinanceSignature>,
}

#[allow(clippy::large_enum_variant)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[derive(Debug, Clone)]
pub enum BinanceHttpResponsePayload {
    Success(BinanceHttpResponseResult),
    Failure(BinanceError),
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BinanceHttpResponseHeaders {
    pub used_weight_1m: Option<u32>,
    pub order_count_10s: Option<u32>,
    pub order_count_1m: Option<u32>,
    pub order_count_1h: Option<u32>,
    pub order_count_1d: Option<u32>,
    pub retry_after: Option<u64>,
}

impl BinanceHttpResponseHeaders {
    #[cfg(feature = "serde")]
    fn parse(headers: &[(String, String)]) -> Self {
        let mut parsed = Self::default();
        for (name, value) in headers {
            let name = name.to_ascii_lowercase();
            let value = value.trim();
            match name.as_str() {
                "x-mbx-used-weight-1m" => parsed.used_weight_1m = value.parse().ok(),
                "x-mbx-order-count-10s" => parsed.order_count_10s = value.parse().ok(),
                "x-mbx-order-count-1m" => parsed.order_count_1m = value.parse().ok(),
                "x-mbx-order-count-1h" => parsed.order_count_1h = value.parse().ok(),
                "x-mbx-order-count-1d" => parsed.order_count_1d = value.parse().ok(),
                "retry-after" => parsed.retry_after = value.parse().ok(),
                _ => {}
            }
        }
        parsed
    }
}

#[derive(Debug, Clone)]
pub struct BinanceHttpResponse {
    pub status: u16,
    pub headers: BinanceHttpResponseHeaders,
    pub payload: BinanceHttpResponsePayload,
}

#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[derive(Debug, Clone)]
pub enum BinanceHttpResponseResult {
    AmendOrder(BinanceAmendOrderResult),
    AssetLimits(BinanceAssetLimitsResult),
    CancelAllOrders(Vec<BinanceCancelReport>),
    CancelOrder(BinanceCancelOrderResult),
    CancelOrderList(BinanceCancelOrderListResult),
    ExchangeInfo(BinanceExchangeInfoResult),
    SpotOrder(BinanceSpotOrderResult),
    Time(BinanceTimeResult),
}

impl BinanceHttpUnsignedRequest {
    fn query_params(&self) -> String {
        match &self {
            BinanceHttpUnsignedRequest::AmendOrderRequest(params) => params.query_params(true),
            BinanceHttpUnsignedRequest::AssetLimits(params) => params.query_params(true),
            BinanceHttpUnsignedRequest::CancelAllOrdersRequest(params) => params.query_params(true),
            BinanceHttpUnsignedRequest::CancelOrderRequest(params) => params.query_params(true),
            BinanceHttpUnsignedRequest::SpotOrderRequest(params) => params.query_params(true),
            BinanceHttpUnsignedRequest::ExchangeInfo(params) => params.query_params(),
            BinanceHttpUnsignedRequest::Time(params) => params.query_params(),
        }
    }
}

impl RateLimited for BinanceHttpUnsignedRequest {
    fn order_count(&self) -> u32 {
        match self {
            BinanceHttpUnsignedRequest::SpotOrderRequest(..) => 1,
            _ => 0,
        }
    }
    fn weight(&self) -> u32 {
        match self {
            BinanceHttpUnsignedRequest::AmendOrderRequest(..) => 4,
            BinanceHttpUnsignedRequest::AssetLimits(..) => 40,
            BinanceHttpUnsignedRequest::CancelAllOrdersRequest(..) => 1,
            BinanceHttpUnsignedRequest::CancelOrderRequest(..) => 1,
            BinanceHttpUnsignedRequest::ExchangeInfo(..) => 20,
            BinanceHttpUnsignedRequest::SpotOrderRequest(..) => 1,
            BinanceHttpUnsignedRequest::Time(..) => 1,
        }
    }
}

impl IntoSigned for BinanceHttpUnsignedRequest {
    type Signed = BinanceHttpRequest;

    fn into_signed(mut self, signer: &Signer) -> ETResult<BinanceHttpRequest> {
        match &mut self {
            BinanceHttpUnsignedRequest::AmendOrderRequest(params) => params.apiKey = None,
            BinanceHttpUnsignedRequest::AssetLimits(params) => params.apiKey = None,
            BinanceHttpUnsignedRequest::CancelAllOrdersRequest(params) => params.apiKey = None,
            BinanceHttpUnsignedRequest::CancelOrderRequest(params) => params.apiKey = None,
            BinanceHttpUnsignedRequest::SpotOrderRequest(params) => params.apiKey = None,
            BinanceHttpUnsignedRequest::ExchangeInfo(..) | BinanceHttpUnsignedRequest::Time(..) => {
            }
        }
        let query_string = self.query_params();
        let signature = signer.signature(&query_string.into_bytes())?;
        Ok(BinanceHttpRequest {
            unsigned: self,
            signature: Some(BinanceSignature {
                apiKey: signer.api_key(),
                signature,
            }),
        })
    }
}

impl From<BinanceHttpRequest> for HttpRequest {
    fn from(value: BinanceHttpRequest) -> Self {
        let method = match value.unsigned {
            BinanceHttpUnsignedRequest::AssetLimits(..)
            | BinanceHttpUnsignedRequest::ExchangeInfo(..)
            | BinanceHttpUnsignedRequest::Time(..) => HttpMethod::GET,
            BinanceHttpUnsignedRequest::CancelAllOrdersRequest(..)
            | BinanceHttpUnsignedRequest::CancelOrderRequest(..) => HttpMethod::DELETE,
            BinanceHttpUnsignedRequest::SpotOrderRequest(..) => HttpMethod::POST,
            BinanceHttpUnsignedRequest::AmendOrderRequest(..) => HttpMethod::PUT,
        };
        let endpoint = match value.unsigned {
            BinanceHttpUnsignedRequest::AmendOrderRequest(..) => "order/amend/keepPriority",
            BinanceHttpUnsignedRequest::AssetLimits(..) => "myFilters",
            BinanceHttpUnsignedRequest::CancelAllOrdersRequest(..) => "openOrders",
            BinanceHttpUnsignedRequest::CancelOrderRequest(..) => "order",
            BinanceHttpUnsignedRequest::ExchangeInfo(..) => "exchangeInfo",
            BinanceHttpUnsignedRequest::SpotOrderRequest(..) => "order",
            BinanceHttpUnsignedRequest::Time(..) => "time",
        };
        let unsigned_query_params = value.unsigned.query_params();
        let query_params = match &value.signature {
            Some(signature) => format!(
                "{}&signature={}",
                unsigned_query_params, signature.signature
            ),
            None => unsigned_query_params,
        };
        let query = Some(format!("{}?{}", endpoint, query_params));
        let headers = match &value.signature {
            Some(signature) => vec![("X-MBX-APIKEY".into(), signature.apiKey.clone())],
            None => vec![],
        };
        let body = None;
        HttpRequest {
            method,
            query,
            headers,
            body,
        }
    }
}

#[cfg(feature = "serde")]
impl TryFrom<HttpResponse> for BinanceHttpResponse {
    type Error = ETError;

    fn try_from(value: HttpResponse) -> Result<Self, Self::Error> {
        let headers = BinanceHttpResponseHeaders::parse(&value.headers);
        match serde_json::from_slice::<BinanceHttpResponsePayload>(&value.body) {
            Ok(payload) => Ok(BinanceHttpResponse {
                status: value.status,
                headers,
                payload,
            }),
            Err(error) => {
                if (200..300).contains(&value.status) {
                    Err(ETError::DeserializeResponse(error))
                } else {
                    Ok(BinanceHttpResponse {
                        status: value.status,
                        headers,
                        payload: BinanceHttpResponsePayload::Failure(BinanceError {
                            code: i64::from(value.status),
                            msg: String::from_utf8_lossy(&value.body).into_owned(),
                        }),
                    })
                }
            }
        }
    }
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    use super::*;
    use crate::binance::spot::BinanceOrderStatus;
    use serde_json::json;

    fn response(status: u16, headers: &[(&str, &str)], body: &[u8]) -> HttpResponse {
        HttpResponse {
            status,
            headers: headers
                .iter()
                .map(|(name, value)| ((*name).into(), (*value).into()))
                .collect(),
            body: body.to_vec(),
        }
    }

    #[test]
    fn http_response_with_result_body_becomes_success() {
        let response = response(200, &[], br#"{"serverTime":1700000000000}"#);
        let response = BinanceHttpResponse::try_from(response).unwrap();
        assert_eq!(response.status, 200);
        match response.payload {
            BinanceHttpResponsePayload::Success(BinanceHttpResponseResult::Time(result)) => {
                assert_eq!(result.serverTime, 1700000000000_i64);
            }
            other => panic!("expected Time, got: {other:?}"),
        }
    }

    #[test]
    fn any_2xx_http_response_with_result_body_becomes_success() {
        let body = serde_json::to_vec(&json!({
            "exchangeFilters": [],
            "symbolFilters": [],
            "assetFilters": [
                {
                    "filterType": "MAX_ASSET",
                    "asset": "JPY",
                    "limit": "1000000.00000000",
                }
            ],
        }))
        .unwrap();
        let response = response(201, &[], &body);
        let response = BinanceHttpResponse::try_from(response).unwrap();
        match response.payload {
            BinanceHttpResponsePayload::Success(BinanceHttpResponseResult::AssetLimits(result)) => {
                assert!(result.exchangeFilters.is_empty());
                assert!(result.symbolFilters.is_empty());
                assert_eq!(result.assetFilters.len(), 1);
            }
            other => panic!("expected AssetLimits, got: {other:?}"),
        }
    }

    #[test]
    fn parses_exchange_info_with_a_real_symbol() {
        let body = serde_json::to_vec(&json!({
            "timezone": "UTC",
            "serverTime": 1700000000000_i64,
            "rateLimits": [],
            "exchangeFilters": [],
            "sors": [
                {
                    "baseAsset": "BTC",
                    "symbols": ["BTCUSDT", "BTCUSDC"],
                }
            ],
            "symbols": [
                {
                    "symbol": "BTCUSDT",
                    "status": "TRADING",
                    "baseAsset": "BTC",
                    "baseAssetPrecision": 8,
                    "quoteAsset": "USDT",
                    "quotePrecision": 8,
                    "quoteAssetPrecision": 8,
                    "baseCommissionPrecision": 8,
                    "quoteCommissionPrecision": 8,
                    "orderTypes": ["LIMIT", "MARKET", "STOP_LOSS"],
                    "icebergAllowed": true,
                    "ocoAllowed": true,
                    "otoAllowed": true,
                    "opoAllowed": true,
                    "quoteOrderQtyMarketAllowed": true,
                    "allowTrailingStop": true,
                    "cancelReplaceAllowed": true,
                    "amendAllowed": true,
                    "pegInstructionsAllowed": true,
                    "isSpotTradingAllowed": true,
                    "isMarginTradingAllowed": true,
                    "permissions": [],
                    "permissionSets": [
                        ["SPOT", "MARGIN", "TRD_GRP_004", "TRD_GRP_005"],
                        ["LEVERAGED", "TRD_GRP_049"],
                    ],
                    "defaultSelfTradePreventionMode": "NONE",
                    "allowedSelfTradePreventionModes": [
                        "EXPIRE_TAKER",
                        "EXPIRE_MAKER",
                        "EXPIRE_BOTH",
                        "NONE",
                    ],
                    "filters": [
                        {
                            "filterType": "PRICE_FILTER",
                            "minPrice": "0.01000000",
                            "maxPrice": "1000000.00000000",
                            "tickSize": "0.01000000",
                        },
                        {
                            "filterType": "LOT_SIZE",
                            "minQty": "0.00001000",
                            "maxQty": "9000.00000000",
                            "stepSize": "0.00001000",
                        },
                    ],
                }
            ],
        }))
        .unwrap();
        let response = response(200, &[], &body);
        let response = BinanceHttpResponse::try_from(response).unwrap();
        match response.payload {
            BinanceHttpResponsePayload::Success(BinanceHttpResponseResult::ExchangeInfo(
                result,
            )) => {
                assert_eq!(result.symbols.len(), 1);
                let symbol = &result.symbols[0];
                assert!(symbol.icebergAllowed);
                assert!(symbol.isMarginTradingAllowed);
                assert_eq!(symbol.permissions.len(), 0);
                assert_eq!(symbol.permissionSets.len(), 2);
                assert_eq!(result.sors.as_ref().unwrap().len(), 1);
            }
            other => panic!("expected ExchangeInfo, got: {other:?}"),
        }
    }

    #[test]
    fn parses_order_place_ack_result_and_full_payloads() {
        let ack = serde_json::to_vec(&json!({
            "symbol": "BTCUSDT",
            "orderId": 28,
            "orderListId": -1,
            "clientOrderId": "6gCrw2kRUAF9CvJDGP16IP",
            "transactTime": 1507725176595_i64,
        }))
        .unwrap();
        let parsed = BinanceHttpResponse::try_from(response(200, &[], &ack)).unwrap();
        match parsed.payload {
            BinanceHttpResponsePayload::Success(BinanceHttpResponseResult::SpotOrder(result)) => {
                assert_eq!(result.orderId, 28);
                assert_eq!(result.orderListId, -1);
                assert_eq!(result.price, None);
                assert_eq!(result.status, None);
            }
            other => panic!("expected SpotOrder, got: {other:?}"),
        }

        let result = serde_json::to_vec(&json!({
            "symbol": "BTCUSDT",
            "orderId": 28,
            "orderListId": -1,
            "clientOrderId": "6gCrw2kRUAF9CvJDGP16IP",
            "transactTime": 1507725176595_i64,
            "price": "0.00000000",
            "origQty": "10.00000000",
            "executedQty": "10.00000000",
            "origQuoteOrderQty": "0.000000",
            "cummulativeQuoteQty": "10.00000000",
            "status": "FILLED",
            "timeInForce": "GTC",
            "type": "MARKET",
            "side": "SELL",
            "workingTime": 1507725176595_i64,
            "selfTradePreventionMode": "NONE",
            "stopPrice": "60000.00000000",
            "trailingDelta": 5000,
            "trailingTime": 1507725176000_i64,
            "icebergQty": "0.00000000",
            "strategyId": 37463720,
            "strategyType": 1000000,
        }))
        .unwrap();
        let parsed = BinanceHttpResponse::try_from(response(200, &[], &result)).unwrap();
        match parsed.payload {
            BinanceHttpResponsePayload::Success(BinanceHttpResponseResult::SpotOrder(result)) => {
                assert_eq!(result.status, Some(BinanceOrderStatus::FILLED));
                assert_eq!(result.strategyId, Some(37463720));
                assert_eq!(result.strategyType, Some(1000000));
                assert!(result.fills.is_none());
            }
            other => panic!("expected SpotOrder, got: {other:?}"),
        }

        let full = serde_json::to_vec(&json!({
            "symbol": "BTCUSDT",
            "orderId": 28,
            "orderListId": -1,
            "clientOrderId": "6gCrw2kRUAF9CvJDGP16IP",
            "transactTime": 1507725176595_i64,
            "price": "0.00000000",
            "origQty": "10.00000000",
            "executedQty": "10.00000000",
            "origQuoteOrderQty": "0.000000",
            "cummulativeQuoteQty": "10.00000000",
            "status": "FILLED",
            "timeInForce": "GTC",
            "type": "MARKET",
            "side": "SELL",
            "workingTime": 1507725176595_i64,
            "selfTradePreventionMode": "NONE",
            "fills": [
                {
                    "price": "4000.00000000",
                    "qty": "1.00000000",
                    "commission": "4.00000000",
                    "commissionAsset": "USDT",
                    "tradeId": 56,
                }
            ],
        }))
        .unwrap();
        let parsed = BinanceHttpResponse::try_from(response(200, &[], &full)).unwrap();
        match parsed.payload {
            BinanceHttpResponsePayload::Success(BinanceHttpResponseResult::SpotOrder(result)) => {
                assert_eq!(result.fills.as_ref().unwrap().len(), 1);
                assert_eq!(result.fills.as_ref().unwrap()[0].tradeId, 56);
            }
            other => panic!("expected SpotOrder, got: {other:?}"),
        }
    }

    #[test]
    fn parses_cancel_order_without_working_time() {
        let body = serde_json::to_vec(&json!({
            "symbol": "LTCBTC",
            "origClientOrderId": "myOrder1",
            "orderId": 4,
            "orderListId": -1,
            "clientOrderId": "cancelMyOrder1",
            "transactTime": 1684804350068_i64,
            "price": "2.00000000",
            "origQty": "1.00000000",
            "executedQty": "0.00000000",
            "origQuoteOrderQty": "0.000000",
            "cummulativeQuoteQty": "0.00000000",
            "status": "CANCELED",
            "timeInForce": "GTC",
            "type": "LIMIT",
            "side": "BUY",
            "selfTradePreventionMode": "NONE",
            "strategyId": 37463720,
            "strategyType": 1000000,
        }))
        .unwrap();
        let response = response(200, &[], &body);
        let response = BinanceHttpResponse::try_from(response).unwrap();
        match response.payload {
            BinanceHttpResponsePayload::Success(BinanceHttpResponseResult::CancelOrder(result)) => {
                assert_eq!(result.workingTime, None);
                assert_eq!(result.transactTime, Some(1684804350068_i64));
                assert_eq!(result.status, BinanceOrderStatus::CANCELED);
                assert_eq!(result.strategyId, Some(37463720));
            }
            other => panic!("expected CancelOrder, got: {other:?}"),
        }
    }

    #[test]
    fn parses_cancel_order_list_shaped_result() {
        let body = serde_json::to_vec(&json!({
            "orderListId": 0,
            "contingencyType": "OCO",
            "listStatusType": "ALL_DONE",
            "listOrderStatus": "ALL_DONE",
            "listClientOrderId": "C3wyj4WVEktd7u9aVBRXcN",
            "transactionTime": 1574040868128_i64,
            "symbol": "LTCBTC",
            "orders": [
                {
                    "symbol": "LTCBTC",
                    "orderId": 2,
                    "clientOrderId": "pO9ufTiFGg3nw2fOdgeOXa",
                }
            ],
            "orderReports": [
                {
                    "symbol": "LTCBTC",
                    "origClientOrderId": "pO9ufTiFGg3nw2fOdgeOXa",
                    "orderId": 2,
                    "orderListId": 0,
                    "clientOrderId": "unfWT8ig8i0uj6lPuYLez6",
                    "transactTime": 1688005070874_i64,
                    "price": "1.00000000",
                    "origQty": "10.00000000",
                    "executedQty": "0.00000000",
                    "origQuoteOrderQty": "0.000000",
                    "cummulativeQuoteQty": "0.00000000",
                    "status": "CANCELED",
                    "timeInForce": "GTC",
                    "type": "STOP_LOSS_LIMIT",
                    "side": "SELL",
                    "stopPrice": "1.00000000",
                    "selfTradePreventionMode": "NONE",
                }
            ],
        }))
        .unwrap();
        let response = response(200, &[], &body);
        let response = BinanceHttpResponse::try_from(response).unwrap();
        match response.payload {
            BinanceHttpResponsePayload::Success(BinanceHttpResponseResult::CancelOrderList(
                result,
            )) => {
                assert_eq!(result.contingencyType, "OCO");
                assert_eq!(result.orders.len(), 1);
                assert_eq!(result.orderReports.len(), 1);
                assert_eq!(result.orderReports[0].symbol, "LTCBTC");
            }
            other => panic!("expected CancelOrderList, got: {other:?}"),
        }
    }

    #[test]
    fn parses_cancel_all_mixed_order_and_order_list_reports() {
        let body = serde_json::to_vec(&json!([
            {
                "symbol": "BTCUSDT",
                "origClientOrderId": "4d96324ff9d44481926157",
                "orderId": 12569099453_i64,
                "orderListId": -1,
                "clientOrderId": "91fe37ce9e69c90d6358c0",
                "transactTime": 1684804350068_i64,
                "price": "23416.10000000",
                "origQty": "0.00847000",
                "executedQty": "0.00001000",
                "origQuoteOrderQty": "0.000000",
                "cummulativeQuoteQty": "0.23416100",
                "status": "CANCELED",
                "timeInForce": "GTC",
                "type": "LIMIT",
                "side": "SELL",
                "stopPrice": "0.00000000",
                "trailingDelta": 0,
                "trailingTime": -1,
                "icebergQty": "0.00000000",
                "strategyId": 37463720,
                "strategyType": 1000000,
                "selfTradePreventionMode": "NONE",
            },
            {
                "orderListId": 19431,
                "contingencyType": "OCO",
                "listStatusType": "ALL_DONE",
                "listOrderStatus": "ALL_DONE",
                "listClientOrderId": "iuVNVJYYrByz6C4yGOPPK0",
                "transactionTime": 1660803702431_i64,
                "symbol": "BTCUSDT",
                "orders": [
                    {
                        "symbol": "BTCUSDT",
                        "orderId": 12569099453_i64,
                        "clientOrderId": "bX5wROblo6YeDwa9iTLeyY",
                    }
                ],
                "orderReports": [
                    {
                        "symbol": "BTCUSDT",
                        "origClientOrderId": "bX5wROblo6YeDwa9iTLeyY",
                        "orderId": 12569099453_i64,
                        "orderListId": 19431,
                        "clientOrderId": "OFFXQtxVFZ6Nbcg4PgE2DA",
                        "transactTime": 1684804350068_i64,
                        "price": "23450.50000000",
                        "origQty": "0.00850000",
                        "executedQty": "0.00000000",
                        "origQuoteOrderQty": "0.000000",
                        "cummulativeQuoteQty": "0.00000000",
                        "status": "CANCELED",
                        "timeInForce": "GTC",
                        "type": "STOP_LOSS_LIMIT",
                        "side": "BUY",
                        "stopPrice": "23430.00000000",
                        "selfTradePreventionMode": "NONE",
                    }
                ],
            },
        ]))
        .unwrap();
        let response = response(200, &[], &body);
        let response = BinanceHttpResponse::try_from(response).unwrap();
        match response.payload {
            BinanceHttpResponsePayload::Success(BinanceHttpResponseResult::CancelAllOrders(
                reports,
            )) => {
                assert_eq!(reports.len(), 2);
                match &reports[0] {
                    BinanceCancelReport::Order(order) => {
                        assert_eq!(order.orderId, 12569099453_i64);
                    }
                    other => panic!("expected Order report, got: {other:?}"),
                }
                match &reports[1] {
                    BinanceCancelReport::OrderList(list) => {
                        assert_eq!(list.orderListId, 19431);
                        assert_eq!(list.orderReports.len(), 1);
                    }
                    other => panic!("expected OrderList report, got: {other:?}"),
                }
            }
            other => panic!("expected CancelAllOrders, got: {other:?}"),
        }
    }

    #[test]
    fn parses_my_filters_asset_limits_result() {
        let body = serde_json::to_vec(&json!({
            "exchangeFilters": [
                {
                    "filterType": "EXCHANGE_MAX_NUM_ORDERS",
                    "maxNumOrders": 1000,
                }
            ],
            "symbolFilters": [
                {
                    "filterType": "MAX_NUM_ORDER_LISTS",
                    "maxNumOrderLists": 20,
                }
            ],
            "assetFilters": [
                {
                    "filterType": "MAX_ASSET",
                    "asset": "JPY",
                    "limit": "1000000.00000000",
                }
            ],
        }))
        .unwrap();
        let response = response(200, &[], &body);
        let response = BinanceHttpResponse::try_from(response).unwrap();
        match response.payload {
            BinanceHttpResponsePayload::Success(BinanceHttpResponseResult::AssetLimits(result)) => {
                assert_eq!(result.exchangeFilters.len(), 1);
                assert_eq!(result.symbolFilters.len(), 1);
                assert_eq!(result.assetFilters.len(), 1);
            }
            other => panic!("expected AssetLimits, got: {other:?}"),
        }
    }

    #[test]
    fn parses_error_payload_with_extra_data_field() {
        let body = serde_json::to_vec(&json!({
            "code": -2022,
            "msg": "Order cancel-replace failed.",
            "data": {
                "cancelResult": "FAILURE",
                "newOrderResult": "NOT_ATTEMPTED",
                "cancelResponse": { "code": -2011, "msg": "Unknown order sent." },
                "newOrderResponse": null,
            },
        }))
        .unwrap();
        let response = response(400, &[], &body);
        let response = BinanceHttpResponse::try_from(response).unwrap();
        match response.payload {
            BinanceHttpResponsePayload::Failure(error) => {
                assert_eq!(error.code, -2022);
                assert_eq!(error.msg, "Order cancel-replace failed.");
            }
            other => panic!("expected Failure, got: {other:?}"),
        }
    }

    #[test]
    fn success_result_tolerates_unknown_fields() {
        let response = response(
            200,
            &[],
            br#"{"serverTime":1700000000000,"futureField":true}"#,
        );
        let response = BinanceHttpResponse::try_from(response).unwrap();
        match response.payload {
            BinanceHttpResponsePayload::Success(BinanceHttpResponseResult::Time(result)) => {
                assert_eq!(result.serverTime, 1700000000000);
            }
            other => panic!("expected Time, got: {other:?}"),
        }
    }

    #[test]
    fn error_payload_tolerates_unknown_fields() {
        let response = response(
            400,
            &[],
            br#"{"code":-2014,"msg":"API-key format invalid.","extra":true}"#,
        );
        let response = BinanceHttpResponse::try_from(response).unwrap();
        match response.payload {
            BinanceHttpResponsePayload::Failure(error) => {
                assert_eq!(error.code, -2014);
                assert_eq!(error.msg, "API-key format invalid.");
            }
            other => panic!("expected Failure, got: {other:?}"),
        }
    }

    #[test]
    fn http_response_with_error_body_becomes_failure_regardless_of_status() {
        let http_response = response(
            400,
            &[],
            br#"{"code":-2014,"msg":"API-key format invalid."}"#,
        );
        let parsed = BinanceHttpResponse::try_from(http_response).unwrap();
        match parsed.payload {
            BinanceHttpResponsePayload::Failure(error) => {
                assert_eq!(error.code, -2014);
                assert_eq!(error.msg, "API-key format invalid.");
            }
            other => panic!("expected Failure, got: {other:?}"),
        }
        let http_response = response(200, &[], br#"{"code":-2015,"msg":"Invalid API-key."}"#);
        let parsed = BinanceHttpResponse::try_from(http_response).unwrap();
        match parsed.payload {
            BinanceHttpResponsePayload::Failure(error) => {
                assert_eq!(error.code, -2015);
                assert_eq!(error.msg, "Invalid API-key.");
            }
            other => panic!("expected Failure, got: {other:?}"),
        }
    }

    #[test]
    fn undecodable_2xx_body_is_a_conversion_error() {
        let response = response(200, &[], b"<html>Bad Gateway</html>");
        assert!(matches!(
            BinanceHttpResponse::try_from(response),
            Err(ETError::DeserializeResponse(_))
        ));
    }

    #[test]
    fn undecodable_non_2xx_body_becomes_failure_carrying_status_and_body() {
        let response = response(502, &[], b"<html>Bad Gateway</html>");
        let response = BinanceHttpResponse::try_from(response).unwrap();
        match response.payload {
            BinanceHttpResponsePayload::Failure(error) => {
                assert_eq!(error.code, 502);
                assert_eq!(error.msg, "<html>Bad Gateway</html>");
            }
            other => panic!("expected Failure, got: {other:?}"),
        }
    }

    #[test]
    fn parses_binance_rate_limit_usage_headers() {
        let response = response(
            200,
            &[
                ("X-MBX-USED-WEIGHT-1M", "34"),
                ("X-MBX-ORDER-COUNT-10S", "1"),
                ("X-MBX-ORDER-COUNT-1M", "2"),
                ("X-MBX-ORDER-COUNT-1H", "5"),
                ("X-MBX-ORDER-COUNT-1D", "12"),
            ],
            br#"{"serverTime":1700000000000}"#,
        );
        let response = BinanceHttpResponse::try_from(response).unwrap();
        assert_eq!(response.headers.used_weight_1m, Some(34));
        assert_eq!(response.headers.order_count_10s, Some(1));
        assert_eq!(response.headers.order_count_1m, Some(2));
        assert_eq!(response.headers.order_count_1h, Some(5));
        assert_eq!(response.headers.order_count_1d, Some(12));
        assert_eq!(response.headers.retry_after, None);
    }

    #[test]
    fn parses_retry_after_on_rate_limited_response() {
        let response = response(429, &[("Retry-After", "30")], b"");
        let response = BinanceHttpResponse::try_from(response).unwrap();
        assert_eq!(response.headers.retry_after, Some(30));
        match response.payload {
            BinanceHttpResponsePayload::Failure(error) => assert_eq!(error.code, 429),
            other => panic!("expected Failure, got: {other:?}"),
        }
    }

    #[test]
    fn header_names_are_case_insensitive() {
        let response = response(
            200,
            &[("x-mbx-used-weight-1m", "7"), ("retry-after", "2")],
            br#"{"serverTime":1700000000000}"#,
        );
        let response = BinanceHttpResponse::try_from(response).unwrap();
        assert_eq!(response.headers.used_weight_1m, Some(7));
        assert_eq!(response.headers.retry_after, Some(2));
    }

    #[test]
    fn missing_or_malformed_usage_headers_are_none() {
        let response = response(
            200,
            &[("X-MBX-USED-WEIGHT-1M", "not-a-number")],
            br#"{"serverTime":1700000000000}"#,
        );
        let response = BinanceHttpResponse::try_from(response).unwrap();
        assert_eq!(response.headers.used_weight_1m, None);
        assert_eq!(response.headers.order_count_10s, None);
        assert_eq!(response.headers.retry_after, None);
    }

    #[test]
    fn error_response_headers_are_still_parsed() {
        let response = response(
            400,
            &[("X-MBX-USED-WEIGHT-1M", "56")],
            br#"{"code":-2014,"msg":"API-key format invalid."}"#,
        );
        let response = BinanceHttpResponse::try_from(response).unwrap();
        assert_eq!(response.headers.used_weight_1m, Some(56));
        assert!(matches!(
            response.payload,
            BinanceHttpResponsePayload::Failure(..)
        ));
    }

    #[test]
    fn http_into_signed_signs_without_api_key_in_payload() {
        use crate::{
            binance::{cancel::BinanceCancelOrderParams, recv_window::BinanceRecvWindow},
            encode::ByteEncoder,
            encrypt::Encryptor,
            signer::Signer,
        };
        use secrecy::SecretSlice;

        let signer = Signer::new(
            "api-key".into(),
            Encryptor::HmacSha256(SecretSlice::from(b"secret".to_vec())),
            ByteEncoder::HexLower,
        );
        let request = BinanceHttpUnsignedRequest::CancelOrderRequest(BinanceCancelOrderParams {
            apiKey: None,
            cancelRestrictions: None,
            newClientOrderId: Some("client order/1".into()),
            orderId: Some(123),
            origClientOrderId: None,
            recvWindow: BinanceRecvWindow::try_new(5000),
            symbol: "BTCUSDT".into(),
            timestamp: 1700000000000,
        })
        .into_signed(&signer)
        .unwrap();
        assert_eq!(
            request.signature.unwrap().signature,
            "28a956d64d671ba79627a129ff26ff157a0675054e2772a6228c1c9cc19fe0de"
        );
    }

    #[test]
    fn http_into_signed_overwrites_params_that_set_api_key() {
        use crate::{
            binance::cancel::BinanceCancelOrderParams, encode::ByteEncoder, encrypt::Encryptor,
            signer::Signer,
        };
        use secrecy::SecretSlice;

        let signer = Signer::new(
            "api-key".into(),
            Encryptor::HmacSha256(SecretSlice::from(b"secret".to_vec())),
            ByteEncoder::HexLower,
        );
        let with_api_key =
            BinanceHttpUnsignedRequest::CancelOrderRequest(BinanceCancelOrderParams {
                apiKey: Some("sneaky-api-key".into()),
                cancelRestrictions: None,
                newClientOrderId: None,
                orderId: Some(123),
                origClientOrderId: None,
                recvWindow: None,
                symbol: "BTCUSDT".into(),
                timestamp: 1700000000000,
            })
            .into_signed(&signer)
            .unwrap();
        let without_api_key =
            BinanceHttpUnsignedRequest::CancelOrderRequest(BinanceCancelOrderParams {
                apiKey: None,
                cancelRestrictions: None,
                newClientOrderId: None,
                orderId: Some(123),
                origClientOrderId: None,
                recvWindow: None,
                symbol: "BTCUSDT".into(),
                timestamp: 1700000000000,
            })
            .into_signed(&signer)
            .unwrap();
        assert_eq!(
            with_api_key.signature.unwrap().signature,
            without_api_key.signature.unwrap().signature
        );
    }
}
