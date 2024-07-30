use serde::{Deserialize, Serialize};
use utility_types::Partial;

#[derive(Partial, Serialize, Deserialize)]
#[partial(ident = PartialA, derive(Debug, PartialEq, Serialize, Deserialize), forward_attrs(serde))]
#[serde(rename_all = "UPPERCASE")]
pub struct A {
    a: usize,
    b: Option<usize>,
}

fn main() {
    let a: PartialA = serde_json::from_str(r#"{"A":0,"B":1}"#).unwrap();

    assert_eq!(
        a,
        PartialA {
            a: Some(0),
            b: Some(Some(1))
        }
    );
}
