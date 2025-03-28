use std::collections::HashMap;
use std::fs;

use serde::Deserialize;

use crate::Card;

#[derive(Debug, Deserialize)]
pub struct Expansion {
    pub name: String,
    #[serde(default)]
    pub packs: Vec<String>,
    #[serde(skip)]
    pub cards: HashMap<usize, Card>,
    pub offering_rate_table: String,
}

impl Expansion {
    #[must_use]
    pub fn load_from_file() -> HashMap<String, Expansion> {
        let expansions_file_content =
            fs::read_to_string("data/expansions.toml").expect("Failed to open expansions file");

        let mut expansions = toml::from_str::<HashMap<String, Expansion>>(&expansions_file_content)
            .expect("Failed to parse expansions file");

        for (expansion_id, expansion) in &mut expansions {
            let path = format!("data/cards/{expansion_id}.csv");

            let mut reader = match csv::Reader::from_path(&path) {
                Ok(reader) => reader,
                Err(error) => {
                    panic!(
                        "Failed to open \"{}\" cards file (located at \"{path}\"): {error}",
                        expansion.name
                    )
                }
            };

            let mut cards: HashMap<usize, Card> = HashMap::new();

            for result in reader.deserialize::<Card>() {
                let card = result.expect("Failed to deserialize card");
                cards.insert(card.number, card);
            }

            expansion.cards = cards;
        }

        expansions
    }
}
