use crate::{
    binance::{
        amend::BinanceAmendOrderParams,
        asset_limits::BinanceAssetLimitsParams,
        cancel::{BinanceCancelAllOrdersParams, BinanceCancelOrderParams},
        exchange_info::BinanceExchangeInfoParams,
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
    AmendOrderRequest(BinanceAmendOrderParams),
    AssetLimits(BinanceAssetLimitsParams),
    CancelAllOrdersRequest(BinanceCancelAllOrdersParams),
    CancelOrderRequest(BinanceCancelOrderParams),
    ExchangeInfo(BinanceExchangeInfoParams),
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
    #[serde(rename = "order.cancel")]
    CancelOrder,
    #[serde(rename = "openOrders.cancelAll")]
    CancelAllOrders,
    #[serde(rename = "exchangeInfo")]
    ExchangeInfo,
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
            BinanceRequest::AmendOrderRequest(..) => 4,
            BinanceRequest::AssetLimits(..) => 40,
            BinanceRequest::CancelAllOrdersRequest(..) => 1,
            BinanceRequest::CancelOrderRequest(..) => 1,
            BinanceRequest::ExchangeInfo(..) => 20,
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
            BinanceRequest::AmendOrderRequest(params) => params.apiKey = api_key,
            BinanceRequest::AssetLimits(params) => params.apiKey = api_key,
            BinanceRequest::CancelAllOrdersRequest(params) => params.apiKey = api_key,
            BinanceRequest::CancelOrderRequest(params) => params.apiKey = api_key,
            BinanceRequest::SpotOrderRequest(params) => params.apiKey = api_key,
            BinanceRequest::WebsocketSessionLogon(params) => params.apiKey = api_key,
            BinanceRequest::ExchangeInfo(..)
            | BinanceRequest::Time(..)
            | BinanceRequest::WebsocketSessionLogout(..) => {}
        }
    }
    fn query_params(&self, percent_encode: bool) -> String {
        match self {
            BinanceRequest::AmendOrderRequest(params) => params.query_params(true, percent_encode),
            BinanceRequest::AssetLimits(params) => params.query_params(true, percent_encode),
            BinanceRequest::CancelAllOrdersRequest(params) => {
                params.query_params(true, percent_encode)
            }
            BinanceRequest::CancelOrderRequest(params) => params.query_params(true, percent_encode),
            BinanceRequest::ExchangeInfo(params) => params.query_params(),
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
            BinanceRequest::AmendOrderRequest(..) => WebsocketMethod::AmendOrder,
            BinanceRequest::AssetLimits(..) => WebsocketMethod::AssetLimits,
            BinanceRequest::CancelAllOrdersRequest(..) => WebsocketMethod::CancelAllOrders,
            BinanceRequest::CancelOrderRequest(..) => WebsocketMethod::CancelOrder,
            BinanceRequest::ExchangeInfo(..) => WebsocketMethod::ExchangeInfo,
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
            BinanceRequest::AmendOrderRequest(..) => {
                (HttpMethod::PUT, "order/amend/keepPriority", true)
            }
            BinanceRequest::AssetLimits(..) => (HttpMethod::GET, "myFilters", true),
            BinanceRequest::CancelAllOrdersRequest(..) => (HttpMethod::DELETE, "openOrders", true),
            BinanceRequest::CancelOrderRequest(..) => (HttpMethod::DELETE, "order", true),
            BinanceRequest::ExchangeInfo(..) => (HttpMethod::GET, "exchangeInfo", false),
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
        binance::exchange_info::{
            BinanceExchangeInfoParams, BinanceExchangeInfoPermission,
            BinanceExchangeInfoPermissions, BinanceExchangeInfoSymbolStatus,
        },
        encode::ByteEncoder,
        encrypt::EncryptionAlgorithm,
    };
    use secrecy::SecretString;

    fn signer() -> Signer {
        let credentials = ApiKeyCredentials {
            api_key: "api-key".into(),
            secret: SecretString::from("secret"),
        };
        let encryptor = EncryptionAlgorithm::HmacSha256
            .encryptor(credentials)
            .unwrap();
        Signer::new("api-key".into(), encryptor, ByteEncoder::Base16)
    }

    #[test]
    fn exchange_info_http_query_params_match_the_docs() {
        let request = BinanceRequest::ExchangeInfo(BinanceExchangeInfoParams::Permissions {
            permissions: BinanceExchangeInfoPermissions::List(vec![
                BinanceExchangeInfoPermission::SPOT,
                BinanceExchangeInfoPermission::MARGIN,
            ]),
            symbolStatus: Some(BinanceExchangeInfoSymbolStatus::HALT),
        })
        .try_into_http(&signer())
        .unwrap();
        assert_eq!(
            request.query.as_deref(),
            Some("exchangeInfo?permissions=%5B%22SPOT%22%2C%22MARGIN%22%5D&symbolStatus=HALT")
        );
    }

    #[test]
    fn exchange_info_websocket_params_match_the_docs() {
        let all = BinanceRequest::ExchangeInfo(BinanceExchangeInfoParams::default())
            .try_into_websocket(&signer(), 1.into())
            .unwrap();
        assert_eq!(all, r#"{"id":1,"method":"exchangeInfo","params":{}}"#);
        let single_permission =
            BinanceRequest::ExchangeInfo(BinanceExchangeInfoParams::Permissions {
                permissions: BinanceExchangeInfoPermissions::Single(
                    BinanceExchangeInfoPermission::SPOT,
                ),
                symbolStatus: Some(BinanceExchangeInfoSymbolStatus::TRADING),
            })
            .try_into_websocket(&signer(), 1.into())
            .unwrap();
        assert_eq!(
            single_permission,
            r#"{"id":1,"method":"exchangeInfo","params":{"permissions":"SPOT","symbolStatus":"TRADING"}}"#
        );
    }
}
