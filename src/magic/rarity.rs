//! The 'rarity' module provides structures for card rarity classification.

extern crate serde;

use serde::{Serialize, Deserialize};
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt;

// The literal representation of all the supported rarities.
const RARITY_COMMON: &str = "common";
const RARITY_UNCOMMON: &str = "uncommon";
const RARITY_RARE: &str = "rare";
const RARITY_MYTHIC: &str = "mythic";
const RARITY_SPECIAL: &str = "special";
const RARITY_BONUS: &str = "bonus";

/// The 'Rarity' of a Magic card.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Mythic,
    Special,
    Bonus,
}

impl Rarity {
    /// Returns a number for ordering of rarities.
    fn ordering_number(&self) -> u8 {
        match self {
            Rarity::Common => 0,
            Rarity::Uncommon => 1,
            Rarity::Rare => 2,
            Rarity::Mythic => 3,
            Rarity::Special => 4,
            Rarity::Bonus => 5,
        }
    }
}

impl Default for Rarity {
    fn default() -> Self {
        Rarity::Common
    }
}

impl PartialOrd for Rarity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Rarity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ordering_number().cmp(&other.ordering_number())
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
            Rarity::Special => RARITY_SPECIAL,
            Rarity::Bonus => RARITY_BONUS,
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
            RARITY_SPECIAL => Ok(Rarity::Special),
            RARITY_BONUS => Ok(Rarity::Bonus),
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

#[cfg(test)]
mod test;
