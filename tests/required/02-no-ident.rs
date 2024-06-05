use utility_types::Required;

#[derive(Required)]
#[required]
pub struct A {
    a: usize,
    b: Option<usize>,
}

fn main() {}
