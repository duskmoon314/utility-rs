use utility_types::Pick;

#[derive(Pick, Debug, PartialEq)]
struct Article {
    title: String,
    author: String,
    content: String,
    tags: Vec<String>,
}

fn main() {}
