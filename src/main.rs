use std::{
    collections::{HashMap, HashSet},
    fs,
    path::PathBuf,
};

use clap::Parser;
use serde::Deserialize;

mod card;
pub use card::{Card, Rarity};

mod expansion;
pub use expansion::Expansion;

mod offering_rates;
pub use offering_rates::OfferingRates;

#[derive(Debug, Deserialize)]
pub struct Collection(pub HashMap<String, HashSet<usize>>);

/// Pokemon TCG Pocket Card Tracker
#[derive(Debug, Parser)]
#[command(about, long_about = None)]
struct Cli {
    /// Collection file path
    #[arg(default_value = "collection.toml")]
    collection_file: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let collection = {
        let collection_file_content =
            fs::read_to_string(cli.collection_file).expect("Failed to open collection file");

        toml::from_str::<Collection>(&collection_file_content)
            .expect("Failed to parse collection file")
    };

    let expansions = Expansion::load_from_file();
    let offering_rates_tables = OfferingRates::load_from_file();

    let mut pack_probabilities = Vec::new();

    for (expansion_id, expansion) in &expansions {
        let Some(uncollected_expansion_card_numbers) = collection.0.get(expansion_id) else {
            continue;
        };

        let expansion_cards = expansion.cards.values().collect::<Vec<_>>();
        let offering_rates = offering_rates_tables
            .get(&expansion.offering_rate_table)
            .expect("Failed to get offering rate table");

        if expansion.packs.is_empty() {
            let new_card_probability = calculate_probability_of_new_card(
                &expansion_cards,
                offering_rates,
                uncollected_expansion_card_numbers,
            );

            pack_probabilities.push((expansion.name.clone(), new_card_probability));
        } else {
            for pack in &expansion.packs {
                let pack_cards = expansion_cards
                    .iter()
                    .filter(|card| card.packs.contains(pack))
                    .copied()
                    .collect::<Vec<_>>();

                let new_card_probability = calculate_probability_of_new_card(
                    &pack_cards,
                    offering_rates,
                    uncollected_expansion_card_numbers,
                );

                pack_probabilities
                    .push((format!("{} ({pack})", expansion.name), new_card_probability));
            }
        }
    }

    pack_probabilities.sort_unstable_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap().reverse());

    let longest_pack_title_length = pack_probabilities
        .iter()
        .map(|(pack_title, _)| pack_title.len())
        .max()
        .unwrap_or(0);

    for (pack_title, new_card_probability) in &pack_probabilities {
        println!(
            "{pack_title:<longest_pack_title_length$} | {:.3}%",
            new_card_probability * 100.0
        );
    }
}

fn calculate_probability_of_new_card(
    pack_cards: &[&Card],
    offering_rates: &OfferingRates,
    uncollected_expansion_card_numbers: &HashSet<usize>,
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

                if !uncollected_expansion_card_numbers.contains(&card.number) {
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
        .map(|(rarity, owned_percentage)| {
            offering_rates.fourth_card_offering_rate(rarity) * owned_percentage
        })
        .sum::<f64>();

    let fifth_card_probability = card_rarity_owned_percentages
        .iter()
        .map(|(rarity, owned_percentage)| {
            offering_rates.fifth_card_offering_rate(rarity) * owned_percentage
        })
        .sum::<f64>();

    1.0 - (first_three_cards_probability.powi(3) * fourth_card_probability * fifth_card_probability)
}
