use utility_types::Pick;

#[derive(Pick, Debug, PartialEq)]
#[pick(arg(ident = Meta, fields(title, author), derive(Debug, PartialEq)))]
struct Article {
    title: String,
    author: String,
    content: String,
    tags: Vec<String>,
}

fn main() {
    let article = Article {
        title: "Hello, world!".to_string(),
        author: "Alice".to_string(),
        content: "This is an article.".to_string(),
        tags: vec!["hello".to_string(), "world".to_string()],
    };

    let meta: Meta = article.into();

    assert_eq!(
        meta,
        Meta {
            title: "Hello, world!".to_string(),
            author: "Alice".to_string(),
        }
    );
}
