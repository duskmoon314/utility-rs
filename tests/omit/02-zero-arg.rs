use utility_types::Omit;

#[derive(Omit, Debug, PartialEq)]
struct Article {
    title: String,
    author: String,
    content: String,
    tags: Vec<String>,
}

fn main() {}
