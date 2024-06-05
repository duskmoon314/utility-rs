use utility_types::Exclude;

#[derive(Debug, PartialEq, Exclude)]
#[exclude(arg(ident = Terrestrial, variants(Jupiter, Saturn, Uranus, Neptune), derive(Debug, PartialEq, Clone, Copy)))]
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

fn main() {
    let terrestrial = Terrestrial::Earth;

    let planet: Planet = terrestrial.into();

    assert_eq!(planet, Planet::Earth);
}
