use crate::error::EGResult;
use std::sync::Arc;

pub(crate) type ArcCombineValues<TValue0, TValue1, TCombined> =
    Arc<dyn Fn(TValue0, TValue1) -> TCombined + Send + Sync>;

pub type ArcTryConvertRef<TFrom, TTo> = Arc<dyn Fn(&TFrom) -> EGResult<TTo> + Send + Sync>;

pub(crate) type TryConvertValue<From, To> = fn(From) -> EGResult<To>;
