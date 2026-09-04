use crate::urls::{Protocol, TradingMode, Urls};

pub struct BinanceUrls;

impl Urls for BinanceUrls {
    fn name(&self) -> &'static str {
        "BINANCE"
    }
    fn url(&self, protocol: Protocol, trading_mode: TradingMode) -> &str {
        match protocol {
            Protocol::Http => match trading_mode {
                TradingMode::Paper => "https://testnet.binance.vision/api/v3",
                TradingMode::Real => "https://api.binance.com/api/v3",
            },
            Protocol::Websocket => match trading_mode {
                TradingMode::Paper => "wss://ws-api.testnet.binance.vision:443/ws-api/v3",
                TradingMode::Real => "wss://ws-api.binance.com:443/ws-api/v3",
            },
        }
    }
}

/// Endpoints for Binance.US, the United States-regulated affiliate of Binance.
///
/// Binance.US is a separate legal entity from Binance and serves its API from
/// its own hosts (`api.binance.us` for REST and `ws-api.binance.us` for the
/// WebSocket API) rather than `api.binance.com`, which geo-blocks requests
/// originating from the United States.
///
/// Binance.US does not offer a paper/testnet environment, so both
/// [`TradingMode`]s resolve to the production endpoints.
pub struct BinanceUsUrls;

impl Urls for BinanceUsUrls {
    fn name(&self) -> &'static str {
        "BINANCE_US"
    }
    fn url(&self, protocol: Protocol, _trading_mode: TradingMode) -> &str {
        match protocol {
            Protocol::Http => "https://api.binance.us/api/v3",
            Protocol::Websocket => "wss://ws-api.binance.us:443/ws-api/v3",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binance_urls_are_stable() {
        let urls = BinanceUrls;
        assert_eq!(urls.name(), "BINANCE");
        assert_eq!(
            urls.url(Protocol::Http, TradingMode::Real),
            "https://api.binance.com/api/v3"
        );
        assert_eq!(
            urls.url(Protocol::Http, TradingMode::Paper),
            "https://testnet.binance.vision/api/v3"
        );
        assert_eq!(
            urls.url(Protocol::Websocket, TradingMode::Real),
            "wss://ws-api.binance.com:443/ws-api/v3"
        );
        assert_eq!(
            urls.url(Protocol::Websocket, TradingMode::Paper),
            "wss://ws-api.testnet.binance.vision:443/ws-api/v3"
        );
    }

    #[test]
    fn binance_us_urls_are_stable() {
        let urls = BinanceUsUrls;
        assert_eq!(urls.name(), "BINANCE_US");
        // api.binance.us and ws-api.binance.us were verified live on
        // 2026-09-04: both respond normally while api.binance.com and
        // testnet.binance.vision return HTTP 451 from a US location.
        assert_eq!(
            urls.url(Protocol::Http, TradingMode::Real),
            "https://api.binance.us/api/v3"
        );
        assert_eq!(
            urls.url(Protocol::Http, TradingMode::Paper),
            "https://api.binance.us/api/v3"
        );
        assert_eq!(
            urls.url(Protocol::Websocket, TradingMode::Real),
            "wss://ws-api.binance.us:443/ws-api/v3"
        );
        assert_eq!(
            urls.url(Protocol::Websocket, TradingMode::Paper),
            "wss://ws-api.binance.us:443/ws-api/v3"
        );
    }
}
