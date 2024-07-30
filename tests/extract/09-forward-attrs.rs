use serde::{Deserialize, Serialize};
use utility_types::Extract;

#[derive(Debug, PartialEq, Extract, Serialize, Deserialize)]
#[extract(forward_attrs(serde))]
#[extract(arg(ident = Terrestrial, variants(Mercury, Venus, Earth, Mars), derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)))]
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
