use std::fs;

use serde::{Deserialize, Deserializer};

mod rarity;
pub use rarity::Rarity;

#[derive(Debug, Deserialize)]
struct Expansions {
    expansions: Vec<Expansion>,
}

#[derive(Debug, Deserialize)]
pub struct Expansion {
    pub name: String,
    pub id: String,
    pub packs: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Card {
    pub name: String,
    pub number: usize,
    pub rarity: Rarity,
    #[serde(deserialize_with = "deserialize_csv_packs")]
    pub packs: Vec<String>,
}

fn deserialize_csv_packs<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    Ok(if s.is_empty() {
        Vec::new()
    } else {
        s.split(':').map(ToOwned::to_owned).collect()
    })
}

fn main() {
    let expansions_file_content =
        fs::read_to_string("data/expansions.toml").expect("Failed to open expansions file");
    let expansions = toml::from_str::<Expansions>(&expansions_file_content)
        .expect("Failed to parse expansions file")
        .expansions;

    let mut cards = Vec::new();

    for expansion in expansions {
        let path = format!("data/cards/{}.csv", expansion.id);

        let mut reader = match csv::Reader::from_path(&path) {
            Ok(reader) => reader,
            Err(error) => {
                panic!(
                    "Failed to open \"{}\" cards file (located at \"{path}\"): {error}",
                    expansion.name
                )
            }
        };

        for result in reader.deserialize::<Card>() {
            let card = result.expect("Failed to deserialize card");
            cards.push(card);
        }
    }

    dbg!(&cards);
}
