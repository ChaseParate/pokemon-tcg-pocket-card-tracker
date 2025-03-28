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
    OneShiny,
    TwoShinies,
    Crown,
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
            "✵" => Ok(Rarity::OneShiny),
            "✵✵" => Ok(Rarity::TwoShinies),
            "♕" => Ok(Rarity::Crown),
            _ => Err(E::unknown_variant(
                s,
                &["♢", "♢♢", "♢♢♢", "♢♢♢♢", "☆", "☆☆", "☆☆☆", "✵", "✵✵", "♕"],
            )),
        }
    }
}
