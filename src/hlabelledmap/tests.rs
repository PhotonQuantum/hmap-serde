use frunk_core::{hlist, HList};
use serde::{Deserialize, Serialize};

use crate::{HLabelledMap, Labelled};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
struct A {
    a: usize,
}

impl Labelled for A {
    const KEY: &'static str = "a";
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
struct B {
    b1: String,
    b2: bool,
}

impl Labelled for B {
    const KEY: &'static str = "b";
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
struct C {
    c: usize,
}

impl Labelled for C {
    const KEY: &'static str = "c";
}

#[test]
fn must_serialize_to_json() {
    let l = hlist![
        Some(A { a: 1 }),
        B {
            b1: String::from("test"),
            b2: false
        },
        C { c: 2 }
    ];
    let s = serde_json::to_string(&HLabelledMap(l).as_ref()).expect("unable to serialize to json");
    assert_eq!(
        s,
        "{\"a\":{\"a\":1},\"b\":{\"b1\":\"test\",\"b2\":false},\"c\":{\"c\":2}}"
    );
}

// TODO fuzzer
#[test]
fn must_ser_de_refl_without_option() {
    type List = HList![B, A];
    let l: List = hlist![
        B {
            b1: String::from("test"),
            b2: false
        },
        A { a: 1 }
    ];
    let m = HLabelledMap(l);
    let ser = serde_json::to_string(&m.as_ref()).expect("unable to serialize to json");
    let de: HLabelledMap<List> =
        serde_json::from_str(ser.as_str()).expect("unable to deserialize from json");
    assert_eq!(m, de, "ser_de_refl doesn't hold");
}

// TODO fuzzer
#[test]
fn must_ser_de_refl_with_option() {
    type List = HList![Option<B>, A];
    let l: List = hlist![
        Some(B {
            b1: String::from("test"),
            b2: false
        }),
        A { a: 1 }
    ];
    let m = HLabelledMap(l);
    let ser = serde_json::to_string(&m.as_ref()).expect("unable to serialize to json");
    let de: HLabelledMap<List> =
        serde_json::from_str(ser.as_str()).expect("unable to deserialize from json");
    assert_eq!(m, de, "ser_de_refl doesn't hold");
}

#[test]
fn must_serialize_skip_none() {
    let l = hlist![None::<A>, C { c: 2 }];
    let s = serde_json::to_string(&HLabelledMap(l).as_ref()).expect("unable to serialize to json");
    assert_eq!(s, "{\"c\":{\"c\":2}}");
}

#[test]
fn must_deserialize_skip_none() {
    let l = hlist![A { a: 1 }, C { c: 2 }];
    let m = HLabelledMap(l);
    let ser = serde_json::to_string(&m.as_ref()).expect("unable to serialize to json");
    let de: HLabelledMap<HList![Option<B>, C]> =
        serde_json::from_str(ser.as_str()).expect("unable to deserialize from json");
    assert_eq!(de, HLabelledMap(hlist![None, C { c: 2 }]));
}

#[test]
fn must_deserialize_unordered() {
    let l = hlist![A { a: 1 }, C { c: 2 }];
    let m = HLabelledMap(l);
    let ser = serde_json::to_string(&m.as_ref()).expect("unable to serialize to json");
    let de: HLabelledMap<HList![C, A]> =
        serde_json::from_str(ser.as_str()).expect("unable to deserialize from json");
    assert_eq!(de, HLabelledMap(hlist![C { c: 2 }, A { a: 1 }]));
}
