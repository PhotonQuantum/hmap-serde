use frunk_core::hlist::HNil;

mod convert;
mod de;
mod ser;
#[cfg(test)]
mod tests;

pub trait Labelled {
    const KEY: &'static str;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct HLabelledMap<T>(pub T);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct HLabelledMapRef<'a, T>(pub &'a T);

impl<T> HLabelledMap<T> {
    pub const fn as_ref(&self) -> HLabelledMapRef<T> {
        HLabelledMapRef(&self.0)
    }
}

impl Default for HLabelledMap<HNil> {
    fn default() -> Self {
        Self(HNil)
    }
}
