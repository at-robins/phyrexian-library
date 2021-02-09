//! The 'legality' module provides structures for card legality classification.

extern crate serde;

use serde::{Serialize, Deserialize};
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt;

// The literal representation of all the supported legalities.
const LEGALITY_BANNED: &str = "Banned";
const LEGALITY_LEGAL: &str = "Legal";
const LEGALITY_NOT_LEGAL: &str = "Not Legal";
const LEGALITY_RESTRICTED: &str = "Restricted";

/// The 'Legality' of a Magic card.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Legality {
    Banned,
    Legal,
    NotLegal,
    Restricted,
}

impl Legality {
    /// Returns a number for ordering of legalities.
    fn ordering_number(&self) -> u8 {
        match self {
            Legality::Banned => 0,
            Legality::Legal => 1,
            Legality::NotLegal => 2,
            Legality::Restricted => 3,
        }
    }
}

impl Default for Legality {
    fn default() -> Self {
        Legality::Legal
    }
}

impl PartialOrd for Legality {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Legality {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ordering_number().cmp(&other.ordering_number())
    }
}

impl From<Legality> for &str {
    fn from(legality: Legality) -> Self {
        (&legality).into()
    }
}

impl From<&Legality> for &str {
    fn from(legality: &Legality) -> Self {
        match legality {
            Legality::Banned => LEGALITY_BANNED,
            Legality::Legal => LEGALITY_LEGAL,
            Legality::NotLegal => LEGALITY_NOT_LEGAL,
            Legality::Restricted => LEGALITY_RESTRICTED,
        }
    }
}

impl TryFrom<&str> for Legality {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            LEGALITY_BANNED => Ok(Legality::Banned),
            LEGALITY_LEGAL => Ok(Legality::Legal),
            LEGALITY_NOT_LEGAL => Ok(Legality::NotLegal),
            LEGALITY_RESTRICTED => Ok(Legality::Restricted),
            _ => Err(format!("{} is not a valid legality.", value)),
        }
    }
}

impl TryFrom<String> for Legality {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Legality::try_from(value.as_str())
    }
}

impl fmt::Display for Legality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}
