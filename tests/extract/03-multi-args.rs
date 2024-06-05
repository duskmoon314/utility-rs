use utility_types::Extract;

#[derive(Debug, PartialEq, Extract)]
#[extract(arg(ident = Terrestrial, variants(Mercury, Venus, Earth, Mars), derive(Debug, PartialEq, Clone, Copy)))]
#[extract(arg(ident = Jovian, variants(Jupiter, Saturn, Uranus, Neptune), derive(Debug, PartialEq, Clone, Copy)))]
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
