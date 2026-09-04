use serde::{Deserialize, Serialize};

use crate::{error::ETResult, http::HttpRequest, signer::Signer};

/// The `id` of a websocket envelope.
///
/// Binance's WebSocket API documents the envelope `id` as INT or STRING and
/// its own examples use integer ids (e.g. `"id": 1`). Unsolicited messages
/// (e.g. a session revocation) arrive with `"id": null`, which is
/// represented as [`Option`] on the response side.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum ETWebsocketId {
    /// An integer id (e.g. `1`).
    Int(i64),
    /// A string id (e.g. `"abc"`).
    Str(String),
}

impl From<i64> for ETWebsocketId {
    fn from(id: i64) -> Self {
        Self::Int(id)
    }
}

impl From<String> for ETWebsocketId {
    fn from(id: String) -> Self {
        Self::Str(id)
    }
}

impl From<&str> for ETWebsocketId {
    fn from(id: &str) -> Self {
        Self::Str(id.into())
    }
}

pub trait ETHttpRequest {
    fn try_into_http(self, signer: &Signer) -> ETResult<HttpRequest>;
}

pub trait ETWebsocketRequest {
    fn try_into_websocket(self, signer: &Signer, id: ETWebsocketId) -> ETResult<String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_ids_untagged() {
        assert_eq!(serde_json::to_string(&ETWebsocketId::Int(1)).unwrap(), "1");
        assert_eq!(
            serde_json::to_string(&ETWebsocketId::Str("abc".into())).unwrap(),
            r#""abc""#
        );
    }

    #[test]
    fn deserializes_integer_and_string_ids() {
        assert_eq!(
            serde_json::from_str::<ETWebsocketId>("1").unwrap(),
            ETWebsocketId::Int(1)
        );
        assert_eq!(
            serde_json::from_str::<ETWebsocketId>(r#""abc""#).unwrap(),
            ETWebsocketId::Str("abc".into())
        );
    }
}
