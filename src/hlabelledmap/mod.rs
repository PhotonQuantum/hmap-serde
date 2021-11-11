use derive_more::{Deref, DerefMut};
use frunk_core::hlist::HNil;
use ref_cast::RefCast;

mod convert;
mod de;
mod ser;
#[cfg(test)]
mod tests;

pub trait Labelled {
    const KEY: &'static str;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, RefCast, Deref, DerefMut)]
#[repr(transparent)]
pub struct HLabelledMap<T>(pub T);

impl Default for HLabelledMap<HNil> {
    fn default() -> Self {
        Self(HNil)
    }
}
