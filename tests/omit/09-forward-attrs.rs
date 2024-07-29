use serde::{Deserialize, Serialize};
use utility_types::Omit;

#[derive(Omit, Debug, PartialEq, Serialize, Deserialize)]
#[omit(forward_attrs(serde))]
#[omit(arg(ident = Meta, fields(title, author), derive(Debug, PartialEq, Serialize, Deserialize)))]
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
            "CONTENT": "This is an article.",
            "TAGS": ["hello", "world"]
        }"#,
    )
    .unwrap();

    assert_eq!(
        meta,
        Meta {
            content: "This is an article.".to_string(),
            tags: vec!["hello".to_string(), "world".to_string()],
        }
    );
}
