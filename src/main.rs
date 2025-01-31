use serde::Deserialize;

mod rarity;
pub use rarity::Rarity;

mod packs;
pub use packs::Packs;

#[derive(Debug, Deserialize)]
pub struct Card {
    pub name: String,
    pub number: usize,
    pub rarity: Rarity,
    pub packs: Packs,
}

const EXPANSIONS: [&str; 3] = ["genetic_apex", "mythical_island", "space_time_smackdown"];

fn main() {
    let mut cards = Vec::new();

    for expansion in EXPANSIONS {
        let path = format!("data/cards/{expansion}.csv");

        let mut reader = match csv::Reader::from_path(&path) {
            Ok(reader) => reader,
            Err(error) => {
                panic!("Failed to open \"{expansion}\" cards file (located at \"{path}\"): {error}")
            }
        };

        for result in reader.deserialize() {
            let card: Card = result.expect("Failed to deserialize card");
            cards.push(card);
        }
    }

    dbg!(&cards);
}
