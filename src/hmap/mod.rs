use frunk_core::hlist::HNil;
use std::ops::{Deref, DerefMut};

mod de;
mod ser;
#[cfg(test)]
mod tests;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct HMap<T>(pub T);

impl<T> HMap<T> {
    #[allow(clippy::missing_const_for_fn)] // const deref ptr is unstable
    fn ref_cast(from: &T) -> &Self {
        // SAFETY HMap is repr(transparent)
        unsafe { &*(from as *const T).cast() }
    }
}

impl<T> Deref for HMap<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for HMap<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for HMap<HNil> {
    fn default() -> Self {
        Self(HNil)
    }
}
