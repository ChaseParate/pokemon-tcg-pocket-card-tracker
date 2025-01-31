use serde::{de::Visitor, Deserialize, Deserializer};

#[derive(Debug)]
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
