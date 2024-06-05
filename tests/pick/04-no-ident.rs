use utility_types::Pick;

#[derive(Pick, Debug, Clone, PartialEq)]
#[pick(arg(fields(title, author), derive(Debug, PartialEq)))]
struct Article {
    title: String,
    author: String,
    content: String,
    tags: Vec<String>,
}

fn main() {}
