use utility_types::Extract;

#[derive(Debug, PartialEq, Extract)]
#[extract(arg(
    ident = Terrestrial,
    variants(Foo, Bar),
    derive(Debug, PartialEq, Clone, Copy)
))]
pub enum Planet {
    Mercury,
    Venus,
    Earth,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
}

fn main() {}
