use frunk_core::hlist::{HCons, HList, HNil};
use serde::de;

use super::Labelled;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum MaybeUnfilled<T> {
    Filled(T),
    Unfilled,
}

impl<T> MaybeUnfilled<T> {
    pub(crate) fn fill(&mut self, item: T) {
        *self = Self::Filled(item);
    }
    fn try_unwrap<E: de::Error>(self, field: &'static str) -> Result<T, E> {
        Option::from(self).ok_or_else(|| de::Error::missing_field(field))
    }
}

impl<T> From<MaybeUnfilled<T>> for Option<T> {
    fn from(item: MaybeUnfilled<T>) -> Self {
        match item {
            MaybeUnfilled::Filled(inner) => Some(inner),
            MaybeUnfilled::Unfilled => None,
        }
    }
}

pub trait HListMaybeUnfilled: HList {}

impl<H, T: HListMaybeUnfilled> HListMaybeUnfilled for HCons<MaybeUnfilled<H>, T> {}

impl HListMaybeUnfilled for HNil {}

pub trait IntoHListMaybeUnfilled: HList {
    type Output: HListMaybeUnfilled;
    fn create() -> Self::Output;
}

impl<H, T> IntoHListMaybeUnfilled for HCons<H, T>
where
    T: IntoHListMaybeUnfilled,
{
    type Output = HCons<MaybeUnfilled<H>, T::Output>;

    fn create() -> Self::Output {
        T::create().prepend(MaybeUnfilled::Unfilled)
    }
}

impl IntoHListMaybeUnfilled for HNil {
    type Output = Self;

    fn create() -> Self::Output {
        Self
    }
}

pub trait IntoHListFilled<Output>: HListMaybeUnfilled {
    fn convert<E: de::Error>(self) -> Result<Output, E>;
}

impl<H, T, TOutput> IntoHListFilled<HCons<Option<H>, TOutput>>
    for HCons<MaybeUnfilled<Option<H>>, T>
where
    T: IntoHListFilled<TOutput>,
    TOutput: HList,
{
    fn convert<E: de::Error>(self) -> Result<HCons<Option<H>, TOutput>, E> {
        Ok(self
            .tail
            .convert()?
            .prepend(Option::from(self.head).and_then(|i| i)))
    }
}

impl<H, T, TOutput> IntoHListFilled<HCons<H, TOutput>> for HCons<MaybeUnfilled<H>, T>
where
    H: Labelled,
    T: IntoHListFilled<TOutput>,
    TOutput: HList,
{
    fn convert<E: de::Error>(self) -> Result<HCons<H, TOutput>, E> {
        Ok(self.tail.convert()?.prepend(self.head.try_unwrap(H::KEY)?))
    }
}

impl IntoHListFilled<Self> for HNil {
    fn convert<E: de::Error>(self) -> Result<Self, E> {
        Ok(Self)
    }
}
