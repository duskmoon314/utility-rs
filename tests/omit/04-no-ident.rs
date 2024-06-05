use utility_types::Omit;

#[derive(Omit, Debug, Clone, PartialEq)]
#[omit(arg(fields(title, author), derive(Debug, PartialEq)))]
struct Article {
    title: String,
    author: String,
    content: String,
    tags: Vec<String>,
}

fn main() {}
