//! The 'card' module provides structures for card classification.

use crate::application::error::PhyrexianError;

use super::super::application::config::Configuration;
use super::colour::{ColourSet, ManaCost};
use super::language::LocalisedString;
use super::legality::Legality;
use super::rarity::Rarity;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fs::File;
use uuid::Uuid;

#[derive(Builder, Clone, Debug, CopyGetters, Getters, Serialize, Deserialize)]
/// An archetype of a card.
pub struct Card {
    #[getset(get = "pub")]
    #[builder(setter(into, strip_option), default)]
    /// The artist of a cards artwork.
    artist: Option<String>,

    #[getset(get = "pub")]
    #[builder(default)]
    /// The availability of the card.
    availability: Vec<String>,

    #[getset(get = "pub")]
    /// The border colour of the card.
    border_colour: String,

    #[getset(get = "pub")]
    /// The colour of the card.
    colour: ColourSet,

    #[getset(get = "pub")]
    /// The colour identity of the card.
    colour_identity: ColourSet,

    #[getset(get = "pub")]
    #[builder(default)]
    /// The UUIDs of other card faces.
    faces: Vec<Uuid>,

    #[getset(get = "pub")]
    #[builder(setter(into, strip_option), default)]
    /// The flavor text of the card if any.
    flavor_text: Option<LocalisedString>,

    #[getset(get = "pub")]
    #[builder(setter(into, strip_option), default)]
    /// The loyalty if the card is a planeswalker.
    loyalty: Option<String>,

    #[builder(default)]
    /// The legality of the card in different formats..
    legality: HashMap<String, Legality>,

    #[getset(get = "pub")]
    #[builder(setter(into, strip_option), default)]
    /// The mana cost of the card.
    mana_cost: Option<ManaCost>,

    #[getset(get = "pub")]
    /// The name of the card.
    name: LocalisedString,

    #[getset(get = "pub")]
    /// The name of the card.
    number: String,

    #[getset(get = "pub")]
    #[builder(setter(into, strip_option), default)]
    /// The power if the card is a creature.
    power: Option<String>,

    #[getset(get_copy = "pub")]
    /// The rarity of the card.
    rarity: Rarity,

    #[getset(get = "pub")]
    /// The code of the set the card belongs to.
    set_code: String,

    #[getset(get = "pub")]
    #[builder(setter(into, strip_option), default)]
    /// The rules text on the card.
    text: Option<LocalisedString>,

    #[getset(get = "pub")]
    #[builder(setter(into, strip_option), default)]
    /// The toughness if the card is a creature.
    toughness: Option<String>,

    #[getset(get_copy = "pub")]
    /// The UUID of the card.
    uuid: Uuid,

    // TODO: Reimplement missing members.
    // #[getset(get = "pub")]
    // /// The full type of the card.
    // card_type: LocalisedString,
    // #[getset(get = "pub")]
    // #[builder(default)]
    // /// UUIDs of card variations.
    // variations: Vec<Uuid>,
}

impl Card {
    /// Returns the legality of the card in the specified format. If the legality in the
    /// specified format is unknown, it is returned as not legal.
    ///
    /// # Parameters
    ///
    /// * `format` - the format to check the legality
    pub fn legality<T: Borrow<String>>(&self, format: T) -> Legality {
        self.legality
            .get(format.borrow())
            .map(|legality| *legality)
            .unwrap_or(Legality::NotLegal)
    }
}

#[derive(Builder, Clone, Debug, CopyGetters, Getters, Serialize, Deserialize)]
/// A set of [`Card`](Card)s.
pub struct CardSet {
    #[getset(get = "pub")]
    #[builder(setter(into, strip_option), default)]
    // The name of the block the set belongs to if any.
    block: Option<LocalisedString>,

    #[builder(default)]
    // The cards that are part of the set.
    cards: HashMap<Uuid, Card>,

    #[getset(get = "pub")]
    // The unique identifier of the set.
    code: String,

    #[getset(get = "pub")]
    // The name of the set.
    name: LocalisedString,

    #[getset(get = "pub")]
    // The keyrune image code.
    keyrune: String,

    #[getset(get = "pub")]
    // The release date of the set.
    release_date: NaiveDate,
}

impl CardSet {
    /// Adds a [`Card`](Card) to the `CardSet`. If a [`Card`](Card) with the same
    /// [`UUID`](uuid::Uuid) is present in the set it is removed and returned.
    pub fn insert(&mut self, card: Card) -> Option<Card> {
        self.cards.insert(card.uuid(), card)
    }

    /// Returns all [`Card`]s in this set.
    pub fn cards(&self) -> Vec<&Card> {
        self.cards.values().collect()
    }

    /// Writes this `Set` to a file.
    pub fn save(&self) -> Result<(), PhyrexianError> {
        let path = Configuration::set_file_path(self);
        if let Some(parent_path) = path.parent() {
            std::fs::create_dir_all(parent_path)?;
        }
        let file = File::create(path)?;
        bincode::serialize_into(file, &self)?;
        Ok(())
    }
}
