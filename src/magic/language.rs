//! The 'language' module provides structures for localisation.

extern crate serde;

use serde::{Serialize, Deserialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;

// The literal representation of all the supported languages.
const LANGUAGE_ANCIENT_GREEK: &str = "Ancient Greek";
const LANGUAGE_ARABIC: &str = "Arabic";
const LANGUAGE_CHINESE_SIMPLIFIED: &str = "Chinese Simplified";
const LANGUAGE_CHINESE_TRADITIONAL: &str = "Chinese Traditional";
const LANGUAGE_ENGLISH_AMERICAN: &str = "English American";
const LANGUAGE_FRENCH: &str = "French";
const LANGUAGE_GERMAN: &str = "German";
const LANGUAGE_HEBREW: &str = "Hebrew";
const LANGUAGE_ITALIAN: &str = "Italian";
const LANGUAGE_JAPANESE: &str = "Japanese";
const LANGUAGE_KOREAN: &str = "Korean";
const LANGUAGE_LATIN: &str = "Latin";
const LANGUAGE_PHYREXIAN: &str = "Phyrexian";
const LANGUAGE_PORTUGUESE_BRAZIL: &str = "Portuguese (Brazil)";
const LANGUAGE_RUSSIAN: &str = "Russian";
const LANGUAGE_SANSKRIT: &str = "Sanskrit";
const LANGUAGE_SPANISH: &str = "Spanish";

/// The 'Language' of a Magic card.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    /// Ancient greek.
    AncientGreek,
    /// Arabic.
    Arabic,
    /// Simplified chinese.
    ChineseSimplified,
    /// Traditional chinese.
    ChineseTraditional,
    /// American english.
    EnglishAmerican,
    /// French.
    French,
    /// German.
    German,
    /// Hebrew.
    Hebrew,
    /// Italian.
    Italian,
    /// Japanese.
    Japanese,
    /// Korean.
    Korean,
    /// Latin.
    Latin,
    /// Phyrexian.
    Phyrexian,
    /// Brasilian portuguese.
    PortugueseBrazil,
    /// Russian.
    Russian,
    /// Sanskrit.
    Sanskrit,
    /// Spanish.
    Spanish,
}

impl Language {
    /// Returns the language code.
    pub fn code(&self) -> &str {
        match self {
            Language::AncientGreek => "grc",
            Language::Arabic => "ar",
            Language::ChineseSimplified => "zhs",
            Language::ChineseTraditional => "zht",
            Language::EnglishAmerican => "en",
            Language::French => "fr",
            Language::German => "de",
            Language::Hebrew => "he",
            Language::Italian => "it",
            Language::Japanese => "ja",
            Language::Korean => "ko",
            Language::Latin => "la",
            Language::Phyrexian => "ph",
            Language::PortugueseBrazil => "pt",
            Language::Russian => "ru",
            Language::Sanskrit => "sa",
            Language::Spanish => "es",
        }
    }
}

impl Default for Language {
    fn default() -> Self {
        Language::EnglishAmerican
    }
}

impl From<Language> for &str {
    fn from(language: Language) -> Self {
        (&language).into()
    }
}

impl From<&Language> for &str {
    fn from(language: &Language) -> Self {
        match language {
            Language::AncientGreek => LANGUAGE_ANCIENT_GREEK,
            Language::Arabic => LANGUAGE_ARABIC,
            Language::ChineseSimplified => LANGUAGE_CHINESE_SIMPLIFIED,
            Language::ChineseTraditional => LANGUAGE_CHINESE_TRADITIONAL,
            Language::EnglishAmerican => LANGUAGE_ENGLISH_AMERICAN,
            Language::French => LANGUAGE_FRENCH,
            Language::German => LANGUAGE_GERMAN,
            Language::Hebrew => LANGUAGE_HEBREW,
            Language::Italian => LANGUAGE_ITALIAN,
            Language::Japanese => LANGUAGE_JAPANESE,
            Language::Korean => LANGUAGE_KOREAN,
            Language::Latin => LANGUAGE_LATIN,
            Language::Phyrexian => LANGUAGE_PHYREXIAN,
            Language::PortugueseBrazil => LANGUAGE_PORTUGUESE_BRAZIL,
            Language::Russian => LANGUAGE_RUSSIAN,
            Language::Sanskrit => LANGUAGE_SANSKRIT,
            Language::Spanish => LANGUAGE_SPANISH,
        }
    }
}

impl TryFrom<&str> for Language {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            LANGUAGE_ANCIENT_GREEK => Ok(Language::AncientGreek),
            LANGUAGE_ARABIC => Ok(Language::Arabic),
            LANGUAGE_CHINESE_SIMPLIFIED => Ok(Language::ChineseSimplified),
            LANGUAGE_CHINESE_TRADITIONAL => Ok(Language::ChineseTraditional),
            LANGUAGE_ENGLISH_AMERICAN => Ok(Language::EnglishAmerican),
            LANGUAGE_FRENCH => Ok(Language::French),
            LANGUAGE_GERMAN => Ok(Language::German),
            LANGUAGE_HEBREW => Ok(Language::Hebrew),
            LANGUAGE_ITALIAN => Ok(Language::Italian),
            LANGUAGE_JAPANESE => Ok(Language::Japanese),
            LANGUAGE_KOREAN => Ok(Language::Korean),
            LANGUAGE_LATIN => Ok(Language::Latin),
            LANGUAGE_PHYREXIAN => Ok(Language::Phyrexian),
            LANGUAGE_PORTUGUESE_BRAZIL => Ok(Language::PortugueseBrazil),
            LANGUAGE_RUSSIAN => Ok(Language::Russian),
            LANGUAGE_SANSKRIT => Ok(Language::Sanskrit),
            LANGUAGE_SPANISH => Ok(Language::Spanish),
            _ => Err(format!("{} is not a valid language.", value)),
        }
    }
}

impl TryFrom<String> for Language {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Language::try_from(value.as_str())
    }
}

impl PartialOrd for Language {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Language {
    fn cmp(&self, other: &Self) -> Ordering {
        Into::<&str>::into(self).cmp(&Into::<&str>::into(other))
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

/// A localised string.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalisedString {
    content: HashMap<Language, String>,
}

impl LocalisedString {
    /// Creates a new localised string.
    ///
    /// # Parameters
    ///
    /// * default - the string in the default ['Language'].
    ///
    /// # Examples
    ///
    /// ```
    /// use phyrexian_library::magic::language::LocalisedString;
    ///
    /// let default = "Default";
    /// let localised = LocalisedString::new(default.to_string());
    /// assert_eq!(default, localised.get_default());
    /// ```
    ///
    /// ['Language']: ./enum.Language.html
    pub fn new<T: Into<String>>(default: T) -> Self {
        let mut content = HashMap::new();
        content.insert(Language::default(), default.into());
        Self{content}
    }

    /// Sets the string in the specified ['Language'] and returns the prevoiusly set
    /// string if any.
    ///
    /// # Parameters
    ///
    /// * language - the ['Language'] to set the string in
    /// * value - value of the string
    ///
    /// # Examples
    ///
    /// ```
    /// use phyrexian_library::magic::language::{Language, LocalisedString};
    ///
    /// let default = "Default";
    /// let mut localised = LocalisedString::new(default.to_string());
    /// let new_default = "New";
    /// assert_eq!(Some(default.to_string()), localised.set(Language::default(), new_default.to_string()));
    /// assert_eq!(new_default, localised.get_default());
    /// ```
    /// ```
    /// use phyrexian_library::magic::language::{Language, LocalisedString};
    ///
    /// let default = "Default";
    /// let mut localised = LocalisedString::new(default.to_string());
    /// let german = "Irgendetwas";
    /// assert_eq!(None, localised.get_localised(Language::German));
    /// assert_eq!(None, localised.set(Language::German, german.to_string()));
    /// assert_eq!(Some(german), localised.get_localised(Language::German));
    /// ```
    ///
    /// ['Language']: ./enum.Language.html
    pub fn set<T: Into<String>>(&mut self, language: Language, value: T) -> Option<String> {
        self.content.insert(language, value.into())
    }

    /// Returns the string in the default ['Language'].
    ///
    /// ['Language']: ./enum.Language.html
    pub fn get_default(&self) -> &str {
        self.content
            .get(&Language::default())
            .expect("There must be a default value.")
    }

    /// Returns the string in the specified ['Language'] if set.
    ///
    /// # Parameters
    ///
    /// * language - the ['Language'] to get the string in
    ///
    /// ['Language']: ./enum.Language.html
    pub fn get_localised(&self, language: Language) -> Option<&str> {
        self.content
            .get(&language)
            .map(|value| value.as_str())
    }

    /// Returns the string in the specified ['Language'] if set,
    /// otherwise returns the default.
    ///
    /// # Parameters
    ///
    /// * language - the ['Language'] to get the string in
    ///
    /// ```
    /// use phyrexian_library::magic::language::{Language, LocalisedString};
    ///
    /// let default = "Default";
    /// let mut localised = LocalisedString::new(default.to_string());
    /// let german = "Irgendetwas";
    /// assert_eq!(default, localised.get_localised_or_default(Language::German));
    /// assert_eq!(None, localised.set(Language::German, german.to_string()));
    /// assert_eq!(german, localised.get_localised_or_default(Language::German));
    /// ```
    ///
    /// ['Language']: ./enum.Language.html
    pub fn get_localised_or_default(&self, language: Language) -> &str {
        self.content
            .get(&language)
            .map_or(self.get_default(), |value| value.as_str())
    }
}

impl PartialOrd for LocalisedString {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LocalisedString {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_default().cmp(&other.get_default())
    }
}

impl fmt::Display for LocalisedString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = self.content.iter()
            .map(|(language, value)| format!("{}: \"{}\"", language, value))
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "[{}]", string)
    }
}

impl From<String> for LocalisedString {
    fn from(value: String) -> Self {
        LocalisedString::new(value)
    }
}

impl From<&str> for LocalisedString {
    fn from(value: &str) -> Self {
        LocalisedString::new(value)
    }
}

#[cfg(test)]
mod test;
