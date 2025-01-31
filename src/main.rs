use std::{
    collections::{HashMap, HashSet},
    fs,
};

use serde::{Deserialize, Deserializer};

mod rarity;
pub use rarity::Rarity;

#[derive(Debug, Deserialize)]
pub struct Collection(pub HashMap<String, HashSet<usize>>);

#[derive(Debug, Deserialize)]
pub struct Expansion {
    pub name: String,
    pub packs: Vec<String>,
    #[serde(skip)]
    pub cards: HashMap<usize, Card>,
}

fn load_expansions_from_file() -> HashMap<String, Expansion> {
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
        Vec::default()
    } else {
        s.split(':').map(ToOwned::to_owned).collect()
    })
}

fn main() {
    let collection = {
        let collection_file_content =
            fs::read_to_string("collection.toml").expect("Failed to open collection file");

        toml::from_str::<Collection>(&collection_file_content)
            .expect("Failed to parse collection file")
    };

    let expansions = load_expansions_from_file();

    let mut pack_probabilities = Vec::new();

    for (expansion_id, expansion) in &expansions {
        let Some(expansion_collection) = collection.0.get(expansion_id) else {
            continue;
        };

        for pack in &expansion.packs {
            let pack_cards = expansion
                .cards
                .values()
                .filter(|card| card.packs.contains(pack))
                .collect::<Vec<_>>();

            let new_card_probability =
                calculate_probability_of_new_card(pack_cards, expansion_collection);

            pack_probabilities.push((pack, new_card_probability));
        }
    }

    pack_probabilities.sort_unstable_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap().reverse());

    for (pack, new_card_probability) in &pack_probabilities {
        println!("{pack}: {:.3}%", new_card_probability * 100.0);
    }
}

fn calculate_probability_of_new_card(
    pack_cards: Vec<&Card>,
    expansion_collection: &HashSet<usize>,
) -> f64 {
    #[derive(Debug, Default)]
    struct CardCount {
        collected: usize,
        total: usize,
    }

    let card_rarity_counts =
        pack_cards
            .iter()
            .fold(HashMap::<Rarity, CardCount>::new(), |mut counts, card| {
                let card_count = counts.entry(card.rarity).or_default();

                if expansion_collection.contains(&card.number) {
                    card_count.collected += 1;
                }
                card_count.total += 1;

                counts
            });

    let card_rarity_owned_percentages = card_rarity_counts
        .into_iter()
        .map(|(rarity, card_count)| {
            let owned_percentage = (card_count.collected as f64) / (card_count.total as f64);
            (rarity, owned_percentage)
        })
        .collect::<HashMap<_, _>>();

    let first_three_cards_probability = card_rarity_owned_percentages[&Rarity::OneDiamond];

    let fourth_card_probability = card_rarity_owned_percentages
        .iter()
        .map(|(rarity, owned_percentage)| rarity.fourth_card_offering_rate() * owned_percentage)
        .sum::<f64>();

    let fifth_card_probability = card_rarity_owned_percentages
        .iter()
        .map(|(rarity, owned_percentage)| rarity.fifth_card_offering_rate() * owned_percentage)
        .sum::<f64>();

    1.0 - (first_three_cards_probability.powi(3) * fourth_card_probability * fifth_card_probability)
}
