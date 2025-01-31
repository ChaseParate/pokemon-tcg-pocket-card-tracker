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

fn main() {
    let mut cards = Vec::new();

    let mut reader = csv::Reader::from_path("data/cards/genetic_apex.csv")
        .expect("Failed to open Genetic Apex cards file");
    for result in reader.deserialize() {
        let card: Card = result.expect("Failed to deserialize card");
        cards.push(card);
    }

    dbg!(&cards);
}
