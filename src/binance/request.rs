use crate::{
    encode::ByteEncoder,
    error::{ETError, ETResult},
    http::HttpRequest,
    request::{ETHttpRequest, ETWebsocketRequest},
    signer::Signer,
    websocket_id::ETWebsocketId,
};
use serde::Serialize;
use serde_with::skip_serializing_none;

#[derive(Debug, Clone, Serialize)]
struct WebsocketRequest<R> {
    id: ETWebsocketId,
    method: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<BinanceWebsocketParams<R>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
struct BinanceWebsocketParams<R> {
    #[serde(flatten)]
    request: R,
    signature: Option<String>,
}

pub(crate) struct BinanceRequestFactory;

impl BinanceRequestFactory {
    pub fn try_into_http<R>(mut request: R, signer: &Signer) -> ETResult<HttpRequest>
    where
        R: ETHttpRequest,
    {
        let method = request.method();
        let endpoint = request.endpoint();
        let is_signed = request.is_signed();
        let query_params = request.query_params(true);
        let (query_params, headers) = if is_signed {
            request.set_api_key(None);
            let signature = signer.signature(query_params.as_bytes())?;
            (
                format!(
                    "{}&signature={}",
                    query_params,
                    ByteEncoder::Percent.encode(signature.as_bytes())
                ),
                vec![("X-MBX-APIKEY".into(), signer.api_key().clone())],
            )
        } else {
            (query_params, vec![])
        };
        let query = if query_params.is_empty() {
            Some(endpoint.to_string())
        } else {
            Some(format!("{}?{}", endpoint, query_params))
        };
        let body = None;
        Ok(HttpRequest {
            method,
            query,
            headers,
            body,
        })
    }
    pub fn try_into_websocket<R>(
        mut request: R,
        signer: &Signer,
        id: ETWebsocketId,
    ) -> ETResult<String>
    where
        R: ETWebsocketRequest,
    {
        let method = request.method();
        let query_params = request.query_params(false);
        let params = if query_params.is_empty() {
            None
        } else {
            let signature = if request.is_signed() {
                request.set_api_key(Some(signer.api_key()));
                Some(signer.signature(query_params.as_bytes())?)
            } else {
                None
            };
            Some(BinanceWebsocketParams { request, signature })
        };
        let websocket_request = WebsocketRequest { id, method, params };
        let message =
            serde_json::to_string(&websocket_request).map_err(ETError::SerializeRequest)?;
        Ok(message)
    }
}

// BinanceRequest::Account(..) => (HttpMethod::GET, "account", true),
// BinanceRequest::AmendOrderRequest(..) => (HttpMethod::PUT, "order/amend/keepPriority", true)
// BinanceRequest::AssetLimits(..) => (HttpMethod::GET, "myFilters", true),
// BinanceRequest::CancelAllOrdersRequest(..) => (HttpMethod::DELETE, "openOrders", true),
// BinanceRequest::CancelOrderRequest(..) => (HttpMethod::DELETE, "order", true),
// BinanceRequest::ExchangeInfo(..) => (HttpMethod::GET, "exchangeInfo", false),
// BinanceRequest::OpenOrders(..) => (HttpMethod::GET, "openOrders", true),
// BinanceRequest::QueryOrder(..) => (HttpMethod::GET, "order", true),
// BinanceRequest::SpotOrderRequest(..) => (HttpMethod::POST, "order", true),
// BinanceRequest::Time(..) => (HttpMethod::GET, "time", false),

// #[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq)]
// enum WebsocketMethod {
//     #[serde(rename = "order.amend.keepPriority")]
//     AmendOrder,
//     #[serde(rename = "myFilters")]
//     AssetLimits,
//     #[serde(rename = "account.status")]
//     Account,
//     #[serde(rename = "order.cancel")]
//     CancelOrder,
//     #[serde(rename = "openOrders.cancelAll")]
//     CancelAllOrders,
//     #[serde(rename = "exchangeInfo")]
//     ExchangeInfo,
//     #[serde(rename = "openOrders.status")]
//     OpenOrders,
//     #[serde(rename = "order.status")]
//     QueryOrder,
//     #[serde(rename = "session.logon")]
//     Logon,
//     #[serde(rename = "session.logout")]
//     Logout,
//     #[serde(rename = "order.place")]
//     PlaceOrder,
//     #[serde(rename = "time")]
//     Time,
// }

// impl RateLimited for BinanceRequest {
//     fn order_count(&self, _protocol: Protocol) -> u32 {
//         match self {
//             BinanceRequest::SpotOrderRequest(..) => 1,
//             _ => 0,
//         }
//     }
//     fn weight(&self, _protocol: Protocol) -> u32 {
//         match self {
//             BinanceRequest::Account(..) => 20,
//             BinanceRequest::AmendOrderRequest(..) => 4,
//             BinanceRequest::AssetLimits(..) => 40,
//             BinanceRequest::CancelAllOrdersRequest(..) => 1,
//             BinanceRequest::CancelOrderRequest(..) => 1,
//             BinanceRequest::ExchangeInfo(..) => 20,
//             BinanceRequest::OpenOrders(params) => {
//                 if params.symbol.is_some() {
//                     6
//                 } else {
//                     80
//                 }
//             }
//             BinanceRequest::QueryOrder(..) => 4,
//             BinanceRequest::SpotOrderRequest(..) => 1,
//             BinanceRequest::Time(..) => 1,
//             BinanceRequest::WebsocketSessionLogon(..) => 2,
//             BinanceRequest::WebsocketSessionLogout(..) => 2,
//         }
//     }
// }
