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

/// The 'Language' of a Magic: The Gathering product.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}
