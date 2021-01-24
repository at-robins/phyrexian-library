//! The 'card' module provides structures for card classification.

extern crate uuid;

use uuid::Uuid;
use super::language::LocalisedString;
use super::rarity::Rarity;
use super::colour::{Colour, Mana, ManaCost};

#[derive(Builder, Debug, CopyGetters, Getters)]
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
    #[builder(setter(into, strip_option), default)]
    /// The flavor text of the card if any.
    flavor_text: Option<LocalisedString>,

    #[getset(get = "pub")]
    /// The mana cost of the card.
    mana_cost: ManaCost,

    #[getset(get = "pub")]
    /// The name of the card.
    name: LocalisedString,

    #[getset(get = "pub")]
    #[builder(setter(into, strip_option), default)]
    /// The power if the card is a creature.
    power: Option<String>,

    #[getset(get_copy = "pub")]
    /// The rarity of the card.
    rarity: Rarity,

    #[getset(get = "pub")]
    set: String,

    #[getset(get = "pub")]
    #[builder(default)]
    /// The UUIDs of other card faces.
    faces: Vec<Uuid>,

    #[getset(get = "pub")]
    #[builder(setter(into, strip_option), default)]
    /// The rules text on the card.
    text: Option<LocalisedString>,

    #[getset(get = "pub")]
    #[builder(setter(into, strip_option), default)]
    /// The toughness if the card is a creature.
    toughness: Option<String>,

    #[getset(get = "pub")]
    /// The full type of the card.
    card_type: LocalisedString,

    #[getset(get_copy = "pub")]
    /// The UUID of the card.
    uuid: Uuid,

    #[getset(get = "pub")]
    #[builder(default)]
    /// UUIDs of card variations.
    variations: Vec<Uuid>,
}

impl Card {

}
