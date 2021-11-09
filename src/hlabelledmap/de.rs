use std::fmt::Formatter;

use frunk_core::hlist::{HCons, HNil};
use serde::de::{IgnoredAny, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};

use super::convert::{IntoHListFilled, IntoHListMaybeUnfilled, MaybeUnfilled};
use super::{HLabelledMap, Labelled};

struct HLabelledMapVisitor<L: IntoHListMaybeUnfilled> {
    maybe_unfilled: L::Output,
}

impl<L: IntoHListMaybeUnfilled> Default for HLabelledMapVisitor<L> {
    fn default() -> Self {
        Self {
            maybe_unfilled: L::create(),
        }
    }
}

pub trait FillByLabel<'de> {
    fn fill_by_name<A: MapAccess<'de>>(&mut self, name: &str, map: &mut A) -> Result<(), A::Error>;
}

impl<'de, H, T> FillByLabel<'de> for HCons<MaybeUnfilled<H>, T>
where
    H: Labelled + Deserialize<'de>,
    T: FillByLabel<'de>,
{
    fn fill_by_name<A: MapAccess<'de>>(&mut self, name: &str, map: &mut A) -> Result<(), A::Error> {
        if H::KEY == name {
            self.head.fill(map.next_value()?);
            Ok(())
        } else {
            self.tail.fill_by_name(name, map)
        }
    }
}

impl<'de, H, T> FillByLabel<'de> for HCons<MaybeUnfilled<Option<H>>, T>
where
    H: Labelled + Deserialize<'de>,
    T: FillByLabel<'de>,
{
    fn fill_by_name<A: MapAccess<'de>>(&mut self, name: &str, map: &mut A) -> Result<(), A::Error> {
        if H::KEY == name {
            self.head.fill(Some(map.next_value()?));
            Ok(())
        } else {
            self.tail.fill_by_name(name, map)
        }
    }
}

impl<'de> FillByLabel<'de> for HNil {
    fn fill_by_name<A: MapAccess<'de>>(
        &mut self,
        _name: &str,
        map: &mut A,
    ) -> Result<(), A::Error> {
        let _: IgnoredAny = map.next_value()?;
        Ok(())
    }
}

impl<'de, L> Visitor<'de> for HLabelledMapVisitor<L>
where
    L: IntoHListMaybeUnfilled,
    L::Output: IntoHListFilled<L> + FillByLabel<'de>,
{
    type Value = HLabelledMap<L>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a labelled heterogeneous map")
    }

    fn visit_map<A>(mut self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        while let Some(key) = map.next_key()? {
            self.maybe_unfilled.fill_by_name(key, &mut map)?;
        }
        Ok(HLabelledMap(self.maybe_unfilled.convert()?))
    }
}

impl<'de, L> Deserialize<'de> for HLabelledMap<L>
where
    L: IntoHListMaybeUnfilled,
    L::Output: IntoHListFilled<L> + FillByLabel<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(HLabelledMapVisitor::default())
    }
}
