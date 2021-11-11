use derive_more::{Deref, DerefMut};
use frunk_core::hlist::HNil;
use ref_cast::RefCast;

mod de;
mod ser;
#[cfg(test)]
mod tests;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, RefCast, Deref, DerefMut)]
#[repr(transparent)]
pub struct HMap<T>(pub T);

impl Default for HMap<HNil> {
    fn default() -> Self {
        Self(HNil)
    }
}
