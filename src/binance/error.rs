use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct BinanceError {
    pub code: String,
    pub msg: String,
}
