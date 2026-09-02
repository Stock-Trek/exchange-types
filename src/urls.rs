use strum::Display;

pub trait Urls {
    const NAME: &str;

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
