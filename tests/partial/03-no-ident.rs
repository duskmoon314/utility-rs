use utility_types::Partial;

#[derive(Partial)]
#[partial()]
pub struct A {
    a: usize,
    b: Option<usize>,
}

fn main() {}
