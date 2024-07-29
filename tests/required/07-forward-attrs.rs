use serde::{Deserialize, Serialize};
use utility_types::Required;

#[derive(Required, Debug, PartialEq, Serialize, Deserialize)]
#[required(ident = RequiredA, derive(Debug, PartialEq, Serialize, Deserialize), forward_attrs(serde))]
#[serde(rename_all = "UPPERCASE")]
pub struct A {
    a: usize,
    b: Option<usize>,
}

fn main() {
    let a: RequiredA = serde_json::from_str(r#"{"A":0,"B":1}"#).unwrap();

    assert_eq!(a, RequiredA { a: 0, b: 1 });
}
