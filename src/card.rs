use serde::{de::Visitor, Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub struct Card {
    pub name: String,
    pub number: usize,
    pub rarity: Rarity,
    #[serde(default, deserialize_with = "deserialize_card_packs")]
    pub packs: Vec<String>,
}

fn deserialize_card_packs<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let packs_string = String::deserialize(deserializer)?;

    // This check *shouldn't* be necessary, but Mew (from Genetic Apex) isn't part of any packs.
    let packs = if packs_string.is_empty() {
        Vec::default()
    } else {
        packs_string.split('|').map(ToOwned::to_owned).collect()
    };

    Ok(packs)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rarity {
    OneDiamond,
    TwoDiamonds,
    ThreeDiamonds,
    FourDiamonds,
    OneStar,
    TwoStars,
    ThreeStars,
    Crown,
}

impl Rarity {
    #[must_use]
    pub const fn fourth_card_offering_rate(&self) -> f64 {
        match self {
            Rarity::OneDiamond => 0.0,
            Rarity::TwoDiamonds => 0.90000,
            Rarity::ThreeDiamonds => 0.05000,
            Rarity::FourDiamonds => 0.01666,
            Rarity::OneStar => 0.02572,
            Rarity::TwoStars => 0.00500,
            Rarity::ThreeStars => 0.00222,
            Rarity::Crown => 0.0004,
        }
    }

    #[must_use]
    pub const fn fifth_card_offering_rate(&self) -> f64 {
        match self {
            Rarity::OneDiamond => 0.0,
            Rarity::TwoDiamonds => 0.60000,
            Rarity::ThreeDiamonds => 0.20000,
            Rarity::FourDiamonds => 0.06664,
            Rarity::OneStar => 0.10288,
            Rarity::TwoStars => 0.02000,
            Rarity::ThreeStars => 0.00888,
            Rarity::Crown => 0.00160,
        }
    }
}

impl<'de> Deserialize<'de> for Rarity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(RarityVisitor)
    }
}

struct RarityVisitor;

impl Visitor<'_> for RarityVisitor {
    type Value = Rarity;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a card rarity")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match s {
            "♢" => Ok(Rarity::OneDiamond),
            "♢♢" => Ok(Rarity::TwoDiamonds),
            "♢♢♢" => Ok(Rarity::ThreeDiamonds),
            "♢♢♢♢" => Ok(Rarity::FourDiamonds),
            "☆" => Ok(Rarity::OneStar),
            "☆☆" => Ok(Rarity::TwoStars),
            "☆☆☆" => Ok(Rarity::ThreeStars),
            "♕" => Ok(Rarity::Crown),
            _ => Err(E::unknown_variant(
                s,
                &["♢", "♢♢", "♢♢♢", "♢♢♢♢", "☆", "☆☆", "☆☆☆", "♕"],
            )),
        }
    }
}
