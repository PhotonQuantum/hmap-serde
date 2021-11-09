use frunk_core::hlist::HNil;

mod de;
mod ser;
#[cfg(test)]
mod tests;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct HMap<T>(pub T);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct HMapRef<'a, T>(pub &'a T);

impl<T> HMap<T> {
    pub const fn as_ref(&self) -> HMapRef<T> {
        HMapRef(&self.0)
    }
}

impl Default for HMap<HNil> {
    fn default() -> Self {
        Self(HNil)
    }
}
