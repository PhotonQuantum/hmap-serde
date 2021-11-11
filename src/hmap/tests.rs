use frunk_core::{hlist, HList};

use crate::HMap;

#[test]
fn must_serialize_to_json() {
    let l = hlist![
        (1, "2"),
        ("2", 3),
        ("3", String::from("4")),
        None::<(i64, i64)>,
        Some((4, 5))
    ];
    let m = HMap(l);
    let s = serde_json::to_string(&m).expect("unable to serialize to json");
    assert_eq!(s, "{\"1\":\"2\",\"2\":3,\"3\":\"4\",\"4\":5}");
}

// TODO fuzzer
#[test]
#[allow(clippy::extra_unused_lifetimes)] // false positive
fn must_ser_de_refl<'a>() {
    type List<'a> = HList![(i32, &'a str), (&'a str, i32), (i32, i32)];
    let l: List<'a> = hlist![(1, "2"), ("2", 3), (3, 4)];
    let m = HMap(l);
    let ser = serde_json::to_string(&m).expect("unable to serialize to json");
    let de: HMap<List> =
        serde_json::from_str(ser.as_str()).expect("unable to deserialize from json");
    assert_eq!(m, de, "ser_de_refl doesn't hold");
}
