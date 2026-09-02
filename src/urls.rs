use strum::Display;

pub trait Urls {
    fn name(&self) -> &'static str;
    fn url(&self, protocol: Protocol, trading_mode: TradingMode) -> &str;
}

#[derive(Debug, Display, Clone, Copy)]
pub enum Protocol {
    Http,
    Websocket,
}

#[derive(Debug, Display, Clone, Copy)]
pub enum TradingMode {
    Real,
    Paper,
}
