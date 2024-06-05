use utility_types::Omit;

#[derive(Omit, Debug, Clone, PartialEq)]
#[omit(arg(ident = Meta, fields(title, author), derive(Debug, PartialEq)))]
#[omit(arg(ident = Meta2, fields(author), derive(Debug, PartialEq)))]
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

    let meta: Meta = article.clone().into();
    let meta2: Meta2 = article.into();

    assert_eq!(
        meta,
        Meta {
            content: "This is an article.".to_string(),
            tags: vec!["hello".to_string(), "world".to_string()],
        }
    );

    assert_eq!(
        meta2,
        Meta2 {
            title: "Hello, world!".to_string(),
            content: "This is an article.".to_string(),
            tags: vec!["hello".to_string(), "world".to_string()],
        }
    );
}
