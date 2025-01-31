use std::collections::HashSet;

use serde::{de::Visitor, Deserialize, Deserializer};

#[derive(Debug)]
pub struct Packs(pub HashSet<String>);

impl<'de> Deserialize<'de> for Packs {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(PacksVisitor)
    }
}

struct PacksVisitor;

impl Visitor<'_> for PacksVisitor {
    type Value = Packs;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a colon separated list of packs")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Packs(if s.is_empty() {
            HashSet::new()
        } else {
            s.split(':').map(ToOwned::to_owned).collect()
        }))
    }
}
