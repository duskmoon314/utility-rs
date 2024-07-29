use serde::{Deserialize, Serialize};
use utility_types::Exclude;

#[derive(Debug, PartialEq, Exclude, Serialize, Deserialize)]
#[exclude(forward_attrs(serde))]
#[exclude(arg(ident = Terrestrial, variants(Jupiter, Saturn, Uranus, Neptune), derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)))]
#[serde(rename_all = "snake_case")]
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
    let terrestrial: Terrestrial = serde_json::from_str(r#""mercury""#).unwrap();

    assert_eq!(terrestrial, Terrestrial::Mercury);
}
