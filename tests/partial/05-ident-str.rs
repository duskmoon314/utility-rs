use utility_types::Partial;

#[derive(Partial)]
#[partial(ident = "PartialA", derive(Debug, PartialEq))]
pub struct A {
    a: usize,
    b: Option<usize>,
}

fn main() {
    let a = A { a: 0, b: Some(1) };

    let pa: PartialA = a.into();

    assert_eq!(
        pa,
        PartialA {
            a: Some(0),
            b: Some(Some(1))
        }
    );
}
