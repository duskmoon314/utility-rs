use utility_types::Required;

#[derive(Required)]
#[required(ident)]
pub struct A {
    a: usize,
    b: Option<usize>,
}

fn main() {}
