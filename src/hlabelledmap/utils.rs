use std::borrow::Cow;
use std::fmt::Formatter;

use serde::de::{MapAccess, Visitor};
use serde::{de, Deserialize, Deserializer};

pub fn deserialize_cow_str<'de, A: MapAccess<'de>>(
    map: &mut A,
) -> Result<Option<Cow<'de, str>>, A::Error> {
    struct CowStr<'de>(Cow<'de, str>);
    struct CowStrVisitor;
    impl<'de> Visitor<'de> for CowStrVisitor {
        type Value = CowStr<'de>;

        fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
            write!(formatter, "an owned or borrowed string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(CowStr(Cow::Owned(v.to_owned())))
        }

        fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(CowStr(Cow::Borrowed(v)))
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(CowStr(Cow::Owned(v)))
        }
    }

    impl<'de> Deserialize<'de> for CowStr<'de> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_str(CowStrVisitor)
        }
    }

    Ok(map.next_key::<CowStr>()?.map(|k| k.0))
}
