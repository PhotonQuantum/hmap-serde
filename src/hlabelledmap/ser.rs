use frunk_core::hlist::{HCons, HList, HNil};
use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};

use crate::hlabelledmap::{HLabelledMapRef, Labelled};

pub trait LabelledMapSerializable {
    fn serialize_map<S: SerializeMap>(&self, serializer: &mut S) -> Result<(), S::Error>;
}

impl<'a, E, T> LabelledMapSerializable for HLabelledMapRef<'a, HCons<E, T>>
where
    E: Serialize + Labelled,
    HLabelledMapRef<'a, T>: LabelledMapSerializable,
{
    fn serialize_map<S: SerializeMap>(&self, serializer: &mut S) -> Result<(), S::Error> {
        let e = &self.0.head;
        serializer.serialize_entry(E::KEY, e)?;
        HLabelledMapRef(&self.0.tail).serialize_map(serializer)
    }
}

impl<'a, E, T> LabelledMapSerializable for HLabelledMapRef<'a, HCons<Option<E>, T>>
where
    E: Serialize + Labelled,
    HLabelledMapRef<'a, T>: LabelledMapSerializable,
{
    fn serialize_map<S: SerializeMap>(&self, serializer: &mut S) -> Result<(), S::Error> {
        if let Some(e) = &self.0.head {
            serializer.serialize_entry(E::KEY, e)?;
        }
        HLabelledMapRef(&self.0.tail).serialize_map(serializer)
    }
}

impl LabelledMapSerializable for HLabelledMapRef<'_, HNil> {
    fn serialize_map<S: SerializeMap>(&self, _serializer: &mut S) -> Result<(), S::Error> {
        Ok(())
    }
}

impl<'a, T> Serialize for HLabelledMapRef<'a, T>
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
