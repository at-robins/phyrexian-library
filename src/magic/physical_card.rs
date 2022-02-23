//! The 'physical_card' module provides structures for card classification.

use crate::magic::language::Language;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Builder, Clone, Debug, CopyGetters, Getters, Serialize, Deserialize)]
/// An actual physical card.
pub struct PhysicalCard {
    #[getset(get = "pub", set = "pub")]
    #[builder(setter(into, strip_option), default)]
    /// An optional custom image path.
    image_path_front: Option<String>,

    #[getset(get = "pub", set = "pub")]
    #[builder(setter(into, strip_option), default)]
    /// An optional custom image path.
    image_path_back: Option<String>,

    #[getset(get_copy = "pub", set = "pub")]
    #[builder(default = "false")]
    // Is the card foiled.
    foil: bool,

    #[getset(get_copy = "pub", set = "pub")]
    #[builder(default = "Language::EnglishAmerican")]
    // The language of the card.
    language: Language,

    #[getset(get = "pub", set = "pub")]
    #[builder(setter(into, strip_option), default)]
    /// An optional comment on by whom the card was signed.
    signed: Option<String>,

    #[getset(get = "pub", set = "pub")]
    #[builder(setter(into, strip_option), default)]
    /// An optional comment on how the card was altered.
    altered: Option<String>,

    #[getset(get = "pub", set = "pub")]
    #[builder(setter(into, strip_option), default)]
    /// An optional comment.
    comment: Option<String>,

    #[getset(get_copy = "pub")]
    /// The card template this card is a physical copy of.
    template: Uuid,

    #[getset(get_copy = "pub")]
    /// The UUID of the card.
    uuid: Uuid,
}
