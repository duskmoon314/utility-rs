use utility_types::Exclude;

#[derive(Debug, PartialEq, Exclude)]
#[exclude(arg(
    variants(Mercury, Venus, Earth, Mars),
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
