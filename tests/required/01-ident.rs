use utility_types::Required;

#[derive(Required, Debug, PartialEq)]
#[required(ident = RequiredA, derive(Debug, PartialEq))]
pub struct A {
    a: usize,
    b: Option<usize>,
}

fn main() {
    let _a = A { a: 0, b: Some(1) };

    let _ra: RequiredA = RequiredA { a: 0, b: 1 };
}
