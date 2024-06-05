use utility_types::Exclude;

#[derive(Debug, PartialEq, Exclude)]
#[exclude(arg(
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
