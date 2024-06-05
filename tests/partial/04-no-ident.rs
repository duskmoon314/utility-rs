use utility_types::Partial;

#[derive(Partial)]
#[partial(ident)]
pub struct A {
    a: usize,
    b: Option<usize>,
}

fn main() {}
