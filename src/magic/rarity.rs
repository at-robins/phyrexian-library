//! The 'rarity' module provides structures for card rarity classification.

use std::convert::TryFrom;
use std::fmt;

// The literal representation of all the supported rarities.
const RARITY_COMMON: &str = "common";
const RARITY_UNCOMMON: &str = "uncommon";
const RARITY_RARE: &str = "rare";
const RARITY_MYTHIC: &str = "mythic";

/// The 'Rarity' of a Magic: The Gathering product.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Mythic,
}

impl Default for Rarity {
    fn default() -> Self {
        Rarity::Common
    }
}

impl From<Rarity> for &str {
    fn from(rarity: Rarity) -> Self {
        (&rarity).into()
    }
}

impl From<&Rarity> for &str {
    fn from(rarity: &Rarity) -> Self {
        match rarity {
            Rarity::Common => RARITY_COMMON,
            Rarity::Uncommon => RARITY_UNCOMMON,
            Rarity::Rare => RARITY_RARE,
            Rarity::Mythic => RARITY_MYTHIC,
        }
    }
}

impl TryFrom<&str> for Rarity {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            RARITY_COMMON => Ok(Rarity::Common),
            RARITY_UNCOMMON => Ok(Rarity::Uncommon),
            RARITY_RARE => Ok(Rarity::Rare),
            RARITY_MYTHIC => Ok(Rarity::Mythic),
            _ => Err(format!("{} is not a valid rarity.", value)),
        }
    }
}

impl TryFrom<String> for Rarity {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Rarity::try_from(value.as_str())
    }
}

impl fmt::Display for Rarity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}
