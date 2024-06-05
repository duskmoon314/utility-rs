use utility_types::Required;

#[derive(Required)]
#[required(ident = RequiredA, derive())]
pub struct A {
    a: usize,
    b: Option<usize>,
}

fn main() {}
