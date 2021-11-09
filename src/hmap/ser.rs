use frunk_core::hlist::{HCons, HList, HNil};
use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};

use super::HMapRef;

// TODO Serialize for HMap
impl<'a, T> Serialize for HMapRef<'a, T>
where
    T: HList,
    Self: MapSerializable,
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

pub trait MapSerializable {
    fn serialize_map<S: SerializeMap>(&self, serializer: &mut S) -> Result<(), S::Error>;
}

impl<'a, K, V, T> MapSerializable for HMapRef<'a, HCons<(K, V), T>>
where
    K: Serialize,
    V: Serialize,
    HMapRef<'a, T>: MapSerializable,
{
    fn serialize_map<S: SerializeMap>(&self, serializer: &mut S) -> Result<(), S::Error> {
        let (k, v) = &self.0.head;
        serializer.serialize_entry(k, v)?;
        HMapRef(&self.0.tail).serialize_map(serializer)
    }
}

impl<'a, K, V, T> MapSerializable for HMapRef<'a, HCons<Option<(K, V)>, T>>
where
    K: Serialize,
    V: Serialize,
    HMapRef<'a, T>: MapSerializable,
{
    fn serialize_map<S: SerializeMap>(&self, serializer: &mut S) -> Result<(), S::Error> {
        if let Some((k, v)) = &self.0.head {
            serializer.serialize_entry(k, v)?;
        }
        HMapRef(&self.0.tail).serialize_map(serializer)
    }
}

impl MapSerializable for HMapRef<'_, HNil> {
    fn serialize_map<S: SerializeMap>(&self, _serializer: &mut S) -> Result<(), S::Error> {
        Ok(())
    }
}
