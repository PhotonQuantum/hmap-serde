use std::fmt::Formatter;
use std::marker::PhantomData;

use frunk_core::hlist::{HCons, HList, HNil};
use frunk_core::traits::IntoReverse;
use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Deserializer};

use super::HMap;

struct HMapVisitor<L>(PhantomData<L>);

impl<L> Default for HMapVisitor<L> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<'de, L> Visitor<'de> for HMapVisitor<L>
where
    L: IntoReverse,
    L::Output: MapDeserializable<'de> + IntoReverse<Output = L>,
{
    type Value = HMap<L>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a heterogeneous map")
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let (reversed, _) = <L::Output as MapDeserializable<'de>>::visit_map(map)?;
        Ok(HMap(reversed.into_reverse()))
    }
}

pub trait MapDeserializable<'de>: HList {
    fn visit_map<A: MapAccess<'de>>(map: A) -> Result<(Self, A), A::Error>;
}

impl<'de, K, V, T> MapDeserializable<'de> for HCons<(K, V), T>
where
    K: Deserialize<'de>,
    V: Deserialize<'de>,
    T: MapDeserializable<'de>,
{
    fn visit_map<A: MapAccess<'de>>(map: A) -> Result<(Self, A), A::Error> {
        let (append, mut map) = T::visit_map(map)?;
        let (k, v) = map.next_entry()?.expect("unexpected eof");
        Ok((append.prepend((k, v)), map))
    }
}

impl<'de> MapDeserializable<'de> for HNil {
    fn visit_map<A: MapAccess<'de>>(map: A) -> Result<(Self, A), A::Error> {
        Ok((HNil, map))
    }
}

impl<'de, L> Deserialize<'de> for HMap<L>
where
    L: IntoReverse,
    L::Output: MapDeserializable<'de> + IntoReverse<Output = L>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(HMapVisitor::default())
    }
}
