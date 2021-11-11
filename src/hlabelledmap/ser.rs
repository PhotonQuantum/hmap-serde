use frunk_core::hlist::{HCons, HList, HNil};
use ref_cast::RefCast;
use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};

use crate::Labelled;

use super::HLabelledMap;

impl<T> Serialize for HLabelledMap<T>
where
    T: HList,
    Self: LabelledMapSerializable,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(T::LEN))?;
        self.serialize_map(&mut map)?;
        map.end()
    }
}

pub trait LabelledMapSerializable {
    fn serialize_map<S: SerializeMap>(&self, serializer: &mut S) -> Result<(), S::Error>;
}

impl<E, T> LabelledMapSerializable for HLabelledMap<HCons<E, T>>
where
    E: Serialize + Labelled,
    HLabelledMap<T>: LabelledMapSerializable,
{
    fn serialize_map<S: SerializeMap>(&self, serializer: &mut S) -> Result<(), S::Error> {
        let e = &self.0.head;
        serializer.serialize_entry(E::KEY, e)?;
        HLabelledMap::ref_cast(&self.0.tail).serialize_map(serializer)
    }
}

impl<E, T> LabelledMapSerializable for HLabelledMap<HCons<Option<E>, T>>
where
    E: Serialize + Labelled,
    HLabelledMap<T>: LabelledMapSerializable,
{
    fn serialize_map<S: SerializeMap>(&self, serializer: &mut S) -> Result<(), S::Error> {
        if let Some(e) = &self.0.head {
            serializer.serialize_entry(E::KEY, e)?;
        }
        HLabelledMap::ref_cast(&self.0.tail).serialize_map(serializer)
    }
}

impl LabelledMapSerializable for HLabelledMap<HNil> {
    fn serialize_map<S: SerializeMap>(&self, _serializer: &mut S) -> Result<(), S::Error> {
        Ok(())
    }
}
