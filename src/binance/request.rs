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

use {serde::Serialize, serde_with::skip_serializing_none};

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
    fn query_params(&self) -> String {
        match self {
            BinanceRequest::AmendOrderRequest(params) => params.query_params(true),
            BinanceRequest::AssetLimits(params) => params.query_params(true),
            BinanceRequest::CancelAllOrdersRequest(params) => params.query_params(true),
            BinanceRequest::CancelOrderRequest(params) => params.query_params(true),
            BinanceRequest::ExchangeInfo(params) => params.query_params(),
            BinanceRequest::SpotOrderRequest(params) => params.query_params(true),
            BinanceRequest::Time(params) => params.query_params(),
            BinanceRequest::WebsocketSessionLogon(params) => params.query_params(true),
            BinanceRequest::WebsocketSessionLogout(params) => params.query_params(true),
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

/// Builds the payload Binance's WebSocket API signs over: the request
/// parameters sorted alphabetically by name and formatted as `key=value`
/// pairs joined by `&`, using the raw UTF-8 values with no percent-encoding
/// (REST instead signs the percent-encoded query string, which is exactly
/// what gets sent). Deriving the payload from the serialized parameters keeps
/// the signed values identical to the raw JSON values transmitted in the
/// message.
fn websocket_signing_payload(request: &BinanceRequest) -> ETResult<String> {
    let params = serde_json::to_value(request).map_err(ETError::SerializeRequest)?;
    let object = params.as_object().ok_or_else(|| {
        ETError::SerializeRequest(serde_json::Error::io(std::io::Error::other(
            "websocket request params did not serialize to an object",
        )))
    })?;
    let mut pairs: Vec<(String, String)> = Vec::with_capacity(object.len());
    for (key, value) in object {
        let value = match value {
            serde_json::Value::String(value) => value.clone(),
            serde_json::Value::Number(value) => value.to_string(),
            serde_json::Value::Bool(value) => value.to_string(),
            // Params are scalar fields serialized with `skip_serializing_none`,
            // so nested values and nulls never occur.
            serde_json::Value::Null => continue,
            serde_json::Value::Array(..) | serde_json::Value::Object(..) => continue,
        };
        pairs.push((key.clone(), value));
    }
    pairs.sort_by(|left, right| left.0.cmp(&right.0));
    Ok(pairs
        .into_iter()
        .map(|(key, value)| format!("{key}={value}"))
        .collect::<Vec<_>>()
        .join("&"))
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
        let query_params = self.query_params();
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
                let payload = websocket_signing_payload(&request)?;
                let signature = Some(signer.signature(payload.as_bytes())?);
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
    };
    use rust_decimal::Decimal;
    use secrecy::SecretString;

    // API key and secret from Binance's documented `session.logon` example.
    const API_KEY: &str =
        "vmPUZE6mv9SD5VNHk4HlWFsOr6aKE2zvsw0MuIgwCIPy6utIco14y7Ju91duEh8A";
    const SECRET_KEY: &str =
        "NhqPtmdSJYdKjVHjA7PZj4Mge3R5YNiP1e3UZjInClVN65XAbvqqM6A7H5fATj0j";
    const TIMESTAMP: i64 = 1_649_729_878_532;

    fn signer(api_key: &str) -> Signer {
        let encryptor = EncryptionAlgorithm::HmacSha256
            .encryptor(ApiKeyCredentials {
                api_key: api_key.into(),
                secret: SecretString::from(SECRET_KEY.to_string()),
            })
            .expect("hmac encryptor should build");
        Signer::new(api_key.into(), encryptor, ByteEncoder::HexLower)
    }

    fn signature_from(message: &str) -> String {
        serde_json::from_str::<serde_json::Value>(message)
            .expect("websocket message should be json")["params"]["signature"]
            .as_str()
            .expect("message should carry a signature")
            .to_string()
    }

    #[test]
    fn websocket_logon_signs_the_raw_payload() {
        let request = BinanceRequest::WebsocketSessionLogon(BinanceSessionLogonParams {
            apiKey: None,
            timestamp: TIMESTAMP,
        });
        let message = request
            .try_into_websocket(&signer(API_KEY), ETWebsocketId::Int(1))
            .expect("websocket message should build");
        // Golden vector from Binance's `session.logon` example: the signature
        // is the hex HMAC-SHA256 of the raw, alphabetically-sorted payload
        // `apiKey=...&timestamp=...` with no percent-encoding.
        assert_eq!(
            signature_from(&message),
            "1cf54395b336b0a9727ef27d5d98987962bc47aca6e13fe978612d0adee066ed"
        );
    }

    #[test]
    fn websocket_signature_is_not_over_the_percent_encoded_payload() {
        // A base64-style API key contains '+', '/' and '=' padding; signing
        // the percent-encoded form of these values is what caused Binance to
        // reject requests with -1022 Invalid signature.
        let api_key = "abc+def/ghi=jkl=";
        let request = BinanceRequest::WebsocketSessionLogon(BinanceSessionLogonParams {
            apiKey: None,
            timestamp: TIMESTAMP,
        });
        let signer = signer(api_key);
        let message = request
            .try_into_websocket(&signer, ETWebsocketId::Int(2))
            .expect("websocket message should build");
        let raw_payload = format!("apiKey={api_key}&timestamp={TIMESTAMP}");
        let encoded_payload = format!(
            "apiKey=abc%2Bdef%2Fghi%3Djkl%3D&timestamp={TIMESTAMP}"
        );
        assert_eq!(
            signature_from(&message),
            signer
                .signature(raw_payload.as_bytes())
                .expect("raw payload should sign")
        );
        assert_ne!(
            signature_from(&message),
            signer
                .signature(encoded_payload.as_bytes())
                .expect("encoded payload should sign")
        );
    }

    #[test]
    fn websocket_signature_keeps_non_ascii_values_raw() {
        // Binance's docs sign over raw UTF-8 (e.g. the full-width symbol in
        // their example payload) rather than the percent-encoded bytes.
        let full_width_symbol = "１２３４５６";
        let request = BinanceRequest::AmendOrderRequest(BinanceAmendOrderParams {
            apiKey: None,
            newClientOrderId: Some(full_width_symbol.into()),
            newQty: Decimal::new(1, 0),
            orderId: None,
            origClientOrderId: None,
            recvWindow: None,
            symbol: "BTCUSDT".into(),
            timestamp: TIMESTAMP,
        });
        let signer = signer(API_KEY);
        let message = request
            .try_into_websocket(&signer, ETWebsocketId::Int(3))
            .expect("websocket message should build");
        let raw_payload = format!(
            "apiKey={API_KEY}&newClientOrderId={full_width_symbol}&newQty=1&symbol=BTCUSDT&timestamp={TIMESTAMP}"
        );
        assert_eq!(
            signature_from(&message),
            signer
                .signature(raw_payload.as_bytes())
                .expect("raw payload should sign")
        );
        // The transmitted message must carry the raw value, not an encoded one.
        let message: serde_json::Value = serde_json::from_str(&message).unwrap();
        assert_eq!(message["params"]["newClientOrderId"], full_width_symbol);
    }
}
