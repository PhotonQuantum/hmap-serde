use std::ops::{Deref, DerefMut};

use frunk_core::hlist::HNil;

mod convert;
mod de;
mod ser;
#[cfg(test)]
mod tests;
mod utils;

pub trait Labelled {
    const KEY: &'static str;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct HLabelledMap<T>(pub T);

impl<T> HLabelledMap<T> {
    #[allow(clippy::missing_const_for_fn)] // const deref ptr is unstable
    fn ref_cast(from: &T) -> &Self {
        // SAFETY HLabelledMap is repr(transparent)
        unsafe { &*(from as *const T).cast() }
    }
}

impl<T> Deref for HLabelledMap<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for HLabelledMap<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for HLabelledMap<HNil> {
    fn default() -> Self {
        Self(HNil)
    }
}
