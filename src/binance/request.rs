use crate::{
    binance::{
        account::BinanceAccountParams,
        amend::BinanceAmendOrderParams,
        asset_limits::BinanceAssetLimitsParams,
        cancel::{BinanceCancelAllOrdersParams, BinanceCancelOrderParams},
        exchange_info::BinanceExchangeInfoParams,
        open_orders::BinanceOpenOrdersParams,
        query_order::BinanceQueryOrderParams,
        session::{BinanceSessionLogonParams, BinanceSessionLogoutParams},
        spot::BinanceSpotOrderParams,
        time::BinanceTimeParams,
    },
    error::{ETError, ETResult},
    http::{HttpMethod, HttpRequest},
    rate_limited::RateLimited,
    request::{ETHttpRequest, ETWebsocketRequest},
    signer::Signer,
    urls::Protocol,
    websocket_id::ETWebsocketId,
};
use serde::Serialize;
use serde_with::skip_serializing_none;

#[derive(Serialize)]
#[serde(untagged)]
#[derive(Debug, Clone, Hash)]
pub enum BinanceRequest {
    Account(BinanceAccountParams),
    AmendOrderRequest(BinanceAmendOrderParams),
    AssetLimits(BinanceAssetLimitsParams),
    CancelAllOrdersRequest(BinanceCancelAllOrdersParams),
    CancelOrderRequest(BinanceCancelOrderParams),
    ExchangeInfo(BinanceExchangeInfoParams),
    OpenOrders(BinanceOpenOrdersParams),
    QueryOrder(BinanceQueryOrderParams),
    SpotOrderRequest(BinanceSpotOrderParams),
    Time(BinanceTimeParams),
    WebsocketSessionLogon(BinanceSessionLogonParams),
    WebsocketSessionLogout(BinanceSessionLogoutParams),
}

#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq)]
enum WebsocketMethod {
    #[serde(rename = "order.amend.keepPriority")]
    AmendOrder,
    #[serde(rename = "myFilters")]
    AssetLimits,
    #[serde(rename = "account.status")]
    Account,
    #[serde(rename = "order.cancel")]
    CancelOrder,
    #[serde(rename = "openOrders.cancelAll")]
    CancelAllOrders,
    #[serde(rename = "exchangeInfo")]
    ExchangeInfo,
    #[serde(rename = "openOrders.status")]
    OpenOrders,
    #[serde(rename = "order.status")]
    QueryOrder,
    #[serde(rename = "session.logon")]
    Logon,
    #[serde(rename = "session.logout")]
    Logout,
    #[serde(rename = "order.place")]
    PlaceOrder,
    #[serde(rename = "time")]
    Time,
}

#[derive(Debug, Clone, Serialize)]
struct WebsocketRequest {
    id: ETWebsocketId,
    method: WebsocketMethod,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<WebsocketParams>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
struct WebsocketParams {
    #[serde(flatten)]
    request: BinanceRequest,
    signature: Option<String>,
}

impl RateLimited for BinanceRequest {
    fn order_count(&self) -> u32 {
        match self {
            BinanceRequest::SpotOrderRequest(..) => 1,
            _ => 0,
        }
    }
    fn weight(&self) -> u32 {
        match self {
            BinanceRequest::Account(..) => 20,
            BinanceRequest::AmendOrderRequest(..) => 4,
            BinanceRequest::AssetLimits(..) => 40,
            BinanceRequest::CancelAllOrdersRequest(..) => 1,
            BinanceRequest::CancelOrderRequest(..) => 1,
            BinanceRequest::ExchangeInfo(..) => 20,
            BinanceRequest::OpenOrders(params) => {
                if params.symbol.is_some() {
                    6
                } else {
                    80
                }
            }
            BinanceRequest::QueryOrder(..) => 4,
            BinanceRequest::SpotOrderRequest(..) => 1,
            BinanceRequest::Time(..) => 1,
            BinanceRequest::WebsocketSessionLogon(..) => 2,
            BinanceRequest::WebsocketSessionLogout(..) => 2,
        }
    }
}

impl BinanceRequest {
    fn set_api_key(&mut self, api_key: Option<String>) {
        match self {
            BinanceRequest::Account(params) => params.apiKey = api_key,
            BinanceRequest::AmendOrderRequest(params) => params.apiKey = api_key,
            BinanceRequest::AssetLimits(params) => params.apiKey = api_key,
            BinanceRequest::CancelAllOrdersRequest(params) => params.apiKey = api_key,
            BinanceRequest::CancelOrderRequest(params) => params.apiKey = api_key,
            BinanceRequest::OpenOrders(params) => params.apiKey = api_key,
            BinanceRequest::QueryOrder(params) => params.apiKey = api_key,
            BinanceRequest::SpotOrderRequest(params) => params.apiKey = api_key,
            BinanceRequest::WebsocketSessionLogon(params) => params.apiKey = api_key,
            BinanceRequest::ExchangeInfo(..)
            | BinanceRequest::Time(..)
            | BinanceRequest::WebsocketSessionLogout(..) => {}
        }
    }
    fn query_params(&self, percent_encode: bool) -> String {
        match self {
            BinanceRequest::Account(params) => params.query_params(true, percent_encode),
            BinanceRequest::AmendOrderRequest(params) => params.query_params(true, percent_encode),
            BinanceRequest::AssetLimits(params) => params.query_params(true, percent_encode),
            BinanceRequest::CancelAllOrdersRequest(params) => {
                params.query_params(true, percent_encode)
            }
            BinanceRequest::CancelOrderRequest(params) => params.query_params(true, percent_encode),
            BinanceRequest::ExchangeInfo(params) => params.query_params(),
            BinanceRequest::OpenOrders(params) => params.query_params(true, percent_encode),
            BinanceRequest::QueryOrder(params) => params.query_params(true, percent_encode),
            BinanceRequest::SpotOrderRequest(params) => params.query_params(true, percent_encode),
            BinanceRequest::Time(params) => params.query_params(),
            BinanceRequest::WebsocketSessionLogon(params) => {
                params.query_params(true, percent_encode)
            }
            BinanceRequest::WebsocketSessionLogout(params) => {
                params.query_params(true, percent_encode)
            }
        }
    }
    fn websocket_method(&self) -> WebsocketMethod {
        match self {
            BinanceRequest::Account(..) => WebsocketMethod::Account,
            BinanceRequest::AmendOrderRequest(..) => WebsocketMethod::AmendOrder,
            BinanceRequest::AssetLimits(..) => WebsocketMethod::AssetLimits,
            BinanceRequest::CancelAllOrdersRequest(..) => WebsocketMethod::CancelAllOrders,
            BinanceRequest::CancelOrderRequest(..) => WebsocketMethod::CancelOrder,
            BinanceRequest::ExchangeInfo(..) => WebsocketMethod::ExchangeInfo,
            BinanceRequest::OpenOrders(..) => WebsocketMethod::OpenOrders,
            BinanceRequest::QueryOrder(..) => WebsocketMethod::QueryOrder,
            BinanceRequest::SpotOrderRequest(..) => WebsocketMethod::PlaceOrder,
            BinanceRequest::Time(..) => WebsocketMethod::Time,
            BinanceRequest::WebsocketSessionLogon(..) => WebsocketMethod::Logon,
            BinanceRequest::WebsocketSessionLogout(..) => WebsocketMethod::Logout,
        }
    }
}

impl ETHttpRequest for BinanceRequest {
    fn try_into_http(mut self, signer: &Signer) -> ETResult<HttpRequest> {
        self.set_api_key(None);
        let (method, endpoint, is_signed) = match self {
            BinanceRequest::Account(..) => (HttpMethod::GET, "account", true),
            BinanceRequest::AmendOrderRequest(..) => {
                (HttpMethod::PUT, "order/amend/keepPriority", true)
            }
            BinanceRequest::AssetLimits(..) => (HttpMethod::GET, "myFilters", true),
            BinanceRequest::CancelAllOrdersRequest(..) => (HttpMethod::DELETE, "openOrders", true),
            BinanceRequest::CancelOrderRequest(..) => (HttpMethod::DELETE, "order", true),
            BinanceRequest::ExchangeInfo(..) => (HttpMethod::GET, "exchangeInfo", false),
            BinanceRequest::OpenOrders(..) => (HttpMethod::GET, "openOrders", true),
            BinanceRequest::QueryOrder(..) => (HttpMethod::GET, "order", true),
            BinanceRequest::SpotOrderRequest(..) => (HttpMethod::POST, "order", true),
            BinanceRequest::Time(..) => (HttpMethod::GET, "time", false),
            BinanceRequest::WebsocketSessionLogon(..)
            | BinanceRequest::WebsocketSessionLogout(..) => {
                let websocket_method = self.websocket_method();
                let request_type =
                    serde_json::to_string(&websocket_method).map_err(ETError::SerializeRequest)?;
                return Err(ETError::BadProtocol {
                    request_type,
                    protocol: Protocol::Http,
                });
            }
        };
        let query_params = self.query_params(true);
        let (query_params, headers) = if is_signed {
            let signature = signer.signature(query_params.as_bytes())?;
            (
                format!("{}&signature={}", query_params, signature),
                vec![("X-MBX-APIKEY".into(), signer.api_key().clone())],
            )
        } else {
            (query_params, vec![])
        };
        let query = Some(format!("{}?{}", endpoint, query_params));
        let body = None;
        Ok(HttpRequest {
            method,
            query,
            headers,
            body,
        })
    }
}

impl ETWebsocketRequest for BinanceRequest {
    fn try_into_websocket(mut self, signer: &Signer, id: ETWebsocketId) -> ETResult<String> {
        self.set_api_key(Some(signer.api_key()));
        let method = self.websocket_method();
        let params = match self {
            BinanceRequest::Time(..) | BinanceRequest::WebsocketSessionLogout(..) => None,
            BinanceRequest::ExchangeInfo(..) => Some(WebsocketParams {
                request: self,
                signature: None,
            }),
            request => {
                let signature = Some(signer.signature(request.query_params(false).as_bytes())?);
                Some(WebsocketParams { request, signature })
            }
        };
        let websocket_request = WebsocketRequest { id, method, params };
        let message =
            serde_json::to_string(&websocket_request).map_err(ETError::SerializeRequest)?;
        Ok(message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        api_key_credential::ApiKeyCredentials,
        encode::ByteEncoder,
        encrypt::EncryptionAlgorithm,
        request::{ETHttpRequest, ETWebsocketRequest},
        signer::Signer,
    };
    use secrecy::SecretString;

    fn signer() -> Signer {
        let credentials = ApiKeyCredentials {
            api_key: "api-key".into(),
            secret: SecretString::from("secret".to_string()),
        };
        let encryptor = EncryptionAlgorithm::HmacSha256
            .encryptor(credentials)
            .expect("hmac encryptor");
        Signer::new("api-key".into(), encryptor, ByteEncoder::Base16)
    }

    fn account(omit_zero_balances: bool) -> BinanceRequest {
        BinanceRequest::Account(BinanceAccountParams {
            apiKey: None,
            omitZeroBalances: omit_zero_balances.then_some(true),
            recvWindow: None,
            timestamp: 1_660_801_720_951,
        })
    }

    fn open_orders(symbol: Option<&str>) -> BinanceRequest {
        BinanceRequest::OpenOrders(BinanceOpenOrdersParams {
            apiKey: None,
            recvWindow: None,
            symbol: symbol.map(String::from),
            timestamp: 1_660_801_720_951,
        })
    }

    fn query_order() -> BinanceRequest {
        BinanceRequest::QueryOrder(BinanceQueryOrderParams {
            apiKey: None,
            orderId: Some(1),
            origClientOrderId: None,
            recvWindow: None,
            symbol: "BTCUSDT".into(),
            timestamp: 1_660_801_720_951,
        })
    }

    #[test]
    fn http_account_serializes_signed_get_account() {
        let http = account(true).try_into_http(&signer()).unwrap();
        assert_eq!(http.method, HttpMethod::GET);
        assert_eq!(
            http.query.as_deref(),
            Some(
                "account?omitZeroBalances=true&timestamp=1660801720951&signature=EC27A58EE5A2CB9498773DB08DA9010B49991EE468B3C8F9772716D80B5FA198"
            )
        );
        assert_eq!(
            http.headers,
            vec![("X-MBX-APIKEY".into(), "api-key".into())]
        );
        assert!(http.body.is_none());
    }

    #[test]
    fn websocket_account_serializes_account_status() {
        let websocket = account(false)
            .try_into_websocket(&signer(), ETWebsocketId::Int(1))
            .unwrap();
        let json: serde_json::Value = serde_json::from_str(&websocket).unwrap();
        assert_eq!(json["id"], 1);
        assert_eq!(json["method"], "account.status");
        assert_eq!(json["params"]["apiKey"], "api-key");
        assert_eq!(json["params"]["timestamp"], 1_660_801_720_951_i64);
        assert_eq!(
            json["params"]["signature"],
            "A9BA89C55B377EB0CCB2ED4115F7FFC6A006531A0B350BFDD4F6DCE0E8DD82AF"
        );
        assert!(json["params"].get("omitZeroBalances").is_none());
        assert!(json["params"].get("recvWindow").is_none());
    }

    #[test]
    fn http_open_orders_serializes_signed_get_open_orders() {
        let http = open_orders(Some("BTCUSDT"))
            .try_into_http(&signer())
            .unwrap();
        assert_eq!(http.method, HttpMethod::GET);
        assert_eq!(
            http.query.as_deref(),
            Some(
                "openOrders?symbol=BTCUSDT&timestamp=1660801720951&signature=9C8A4C7C79966AABF88CEFDEEE9E7F55299011E5804AC5819FC1BF0F6C11B9FF"
            )
        );
        assert_eq!(
            http.headers,
            vec![("X-MBX-APIKEY".into(), "api-key".into())]
        );
    }

    #[test]
    fn websocket_open_orders_without_symbol_serializes_open_orders_status() {
        let websocket = open_orders(None)
            .try_into_websocket(&signer(), ETWebsocketId::Int(2))
            .unwrap();
        let json: serde_json::Value = serde_json::from_str(&websocket).unwrap();
        assert_eq!(json["id"], 2);
        assert_eq!(json["method"], "openOrders.status");
        assert_eq!(json["params"]["apiKey"], "api-key");
        assert_eq!(json["params"]["timestamp"], 1_660_801_720_951_i64);
        assert_eq!(
            json["params"]["signature"],
            "A9BA89C55B377EB0CCB2ED4115F7FFC6A006531A0B350BFDD4F6DCE0E8DD82AF"
        );
        assert!(json["params"].get("symbol").is_none());
    }

    #[test]
    fn http_query_order_serializes_signed_get_order() {
        let http = query_order().try_into_http(&signer()).unwrap();
        assert_eq!(http.method, HttpMethod::GET);
        assert_eq!(
            http.query.as_deref(),
            Some(
                "order?orderId=1&symbol=BTCUSDT&timestamp=1660801720951&signature=D4BC8844607409AA95EBD43A710DD3601C484C7555313BEE22FF9CE5EEC7D6CE"
            )
        );
        assert_eq!(
            http.headers,
            vec![("X-MBX-APIKEY".into(), "api-key".into())]
        );
    }

    #[test]
    fn websocket_query_order_serializes_order_status() {
        let websocket = query_order()
            .try_into_websocket(&signer(), ETWebsocketId::Int(3))
            .unwrap();
        let json: serde_json::Value = serde_json::from_str(&websocket).unwrap();
        assert_eq!(json["id"], 3);
        assert_eq!(json["method"], "order.status");
        assert_eq!(json["params"]["symbol"], "BTCUSDT");
        assert_eq!(json["params"]["orderId"], 1);
        assert_eq!(json["params"]["apiKey"], "api-key");
        assert_eq!(json["params"]["timestamp"], 1_660_801_720_951_i64);
        assert_eq!(
            json["params"]["signature"],
            "738B3C77F9E6135F4A17FE313306C6E9296EDF250E0D813F171E688508C4A2FD"
        );
        assert!(json["params"].get("recvWindow").is_none());
        assert!(json["params"].get("origClientOrderId").is_none());
    }

    #[test]
    fn open_orders_weight_depends_on_symbol() {
        assert_eq!(open_orders(Some("BTCUSDT")).weight(), 6);
        assert_eq!(open_orders(None).weight(), 80);
    }

    #[test]
    fn account_and_query_order_weights() {
        assert_eq!(account(false).weight(), 20);
        assert_eq!(query_order().weight(), 4);
    }
}
