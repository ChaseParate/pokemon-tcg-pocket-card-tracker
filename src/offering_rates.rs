use std::collections::HashMap;
use std::fs;

use serde::Deserialize;

use crate::Rarity;

#[derive(Debug, Deserialize)]
pub struct OfferingRates {
    fourth_card: HashMap<Rarity, f64>,
    fifth_card: HashMap<Rarity, f64>,
}

impl OfferingRates {
    #[must_use]
    pub fn load_from_file() -> HashMap<String, OfferingRates> {
        let offering_rates_file_content = fs::read_to_string("data/offering_rates.toml")
            .expect("Failed to open offering rates file");

        toml::from_str::<HashMap<String, OfferingRates>>(&offering_rates_file_content)
            .expect("Failed to parse offering rates file")
    }

    #[must_use]
    pub fn fourth_card_offering_rate(&self, rarity: &Rarity) -> f64 {
        self.fourth_card.get(rarity).copied().unwrap_or(0.0)
    }

    #[must_use]
    pub fn fifth_card_offering_rate(&self, rarity: &Rarity) -> f64 {
        self.fifth_card.get(rarity).copied().unwrap_or(0.0)
    }
}
