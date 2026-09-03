#[cfg(feature = "serde")]
use serde::Serialize;

#[allow(non_snake_case)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, Clone)]
pub struct BinanceSignature {
    /// REST requests send the API key in the `X-MBX-APIKEY` header, so the
    /// HTTP `into_signed` stores it here (`Some`). WebSocket API requests
    /// carry `apiKey` on the params themselves (it is part of the signed
    /// payload), so the WS `into_signed` leaves this `None` to avoid
    /// serializing the key twice.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub apiKey: Option<String>,
    pub signature: String,
}
