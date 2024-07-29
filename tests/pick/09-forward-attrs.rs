use serde::{Deserialize, Serialize};
use utility_types::Pick;

#[derive(Pick, Debug, PartialEq, Serialize, Deserialize)]
#[pick(arg(ident = Meta, fields(title, author), derive(Debug, PartialEq, Serialize, Deserialize), forward_attrs(serde)))]
#[serde(rename_all = "UPPERCASE")]
struct Article {
    title: String,
    author: String,
    content: String,
    tags: Vec<String>,
}

fn main() {
    let meta: Meta = serde_json::from_str(
        r#"{
            "TITLE": "Hello, world!",
            "AUTHOR": "Alice"
        }"#,
    )
    .unwrap();

    assert_eq!(
        meta,
        Meta {
            title: "Hello, world!".to_string(),
            author: "Alice".to_string(),
        }
    );
}
