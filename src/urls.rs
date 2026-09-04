use strum::Display;

pub trait Urls {
    fn name(&self) -> &'static str;
    fn url(&self, protocol: Protocol, trading_mode: TradingMode) -> &str;
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    Http,
    Websocket,
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq)]
pub enum TradingMode {
    Real,
    Paper,
}
