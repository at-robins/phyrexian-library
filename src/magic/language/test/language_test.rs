use super::super::*;
use std::convert::TryInto;

#[test]
/// Tests if the ordering of `Language` works as expected.
fn test_ordering() {
    let mut unordered = vec!(
        Language::Japanese,
        Language::Arabic,
        Language::Russian,
        Language::German,
        Language::Latin,
    );
    let ordered = vec!(
        Language::Arabic,
        Language::German,
        Language::Japanese,
        Language::Latin,
        Language::Russian,
    );
    assert_ne!(unordered, ordered);
    unordered.sort();
    assert_eq!(unordered, ordered);
}

#[test]
/// Tests if the conversion from `Language` to string works as expected.
fn test_conversion_to_string() {
    assert_eq!(Into::<&str>::into(Language::AncientGreek), LANGUAGE_ANCIENT_GREEK);
    assert_eq!(Into::<&str>::into(Language::Arabic), LANGUAGE_ARABIC);
    assert_eq!(Into::<&str>::into(Language::ChineseSimplified), LANGUAGE_CHINESE_SIMPLIFIED);
    assert_eq!(Into::<&str>::into(Language::ChineseTraditional), LANGUAGE_CHINESE_TRADITIONAL);
    assert_eq!(Into::<&str>::into(Language::EnglishAmerican), LANGUAGE_ENGLISH_AMERICAN);
    assert_eq!(Into::<&str>::into(Language::French), LANGUAGE_FRENCH);
    assert_eq!(Into::<&str>::into(Language::German), LANGUAGE_GERMAN);
    assert_eq!(Into::<&str>::into(Language::Hebrew), LANGUAGE_HEBREW);
    assert_eq!(Into::<&str>::into(Language::Italian), LANGUAGE_ITALIAN);
    assert_eq!(Into::<&str>::into(Language::Japanese), LANGUAGE_JAPANESE);
    assert_eq!(Into::<&str>::into(Language::Korean), LANGUAGE_KOREAN);
    assert_eq!(Into::<&str>::into(Language::Latin), LANGUAGE_LATIN);
    assert_eq!(Into::<&str>::into(Language::Phyrexian), LANGUAGE_PHYREXIAN);
    assert_eq!(Into::<&str>::into(Language::PortugueseBrazil), LANGUAGE_PORTUGUESE_BRAZIL);
    assert_eq!(Into::<&str>::into(Language::Russian), LANGUAGE_RUSSIAN);
    assert_eq!(Into::<&str>::into(Language::Sanskrit), LANGUAGE_SANSKRIT);
    assert_eq!(Into::<&str>::into(Language::Spanish), LANGUAGE_SPANISH);
}

#[test]
/// Tests if the conversion from string to `Language` works as expected.
fn test_conversion_from_string() {
    // Test reference.
    assert_eq!(TryInto::<Language>::try_into(LANGUAGE_ANCIENT_GREEK), Ok(Language::AncientGreek));
    assert_eq!(TryInto::<Language>::try_into(LANGUAGE_ARABIC), Ok(Language::Arabic));
    assert_eq!(TryInto::<Language>::try_into(LANGUAGE_CHINESE_SIMPLIFIED), Ok(Language::ChineseSimplified));
    assert_eq!(TryInto::<Language>::try_into(LANGUAGE_CHINESE_TRADITIONAL), Ok(Language::ChineseTraditional));
    assert_eq!(TryInto::<Language>::try_into(LANGUAGE_ENGLISH_AMERICAN), Ok(Language::EnglishAmerican));
    assert_eq!(TryInto::<Language>::try_into(LANGUAGE_FRENCH), Ok(Language::French));
    assert_eq!(TryInto::<Language>::try_into(LANGUAGE_GERMAN), Ok(Language::German));
    assert_eq!(TryInto::<Language>::try_into(LANGUAGE_HEBREW), Ok(Language::Hebrew));
    assert_eq!(TryInto::<Language>::try_into(LANGUAGE_ITALIAN), Ok(Language::Italian));
    assert_eq!(TryInto::<Language>::try_into(LANGUAGE_JAPANESE), Ok(Language::Japanese));
    assert_eq!(TryInto::<Language>::try_into(LANGUAGE_KOREAN), Ok(Language::Korean));
    assert_eq!(TryInto::<Language>::try_into(LANGUAGE_LATIN), Ok(Language::Latin));
    assert_eq!(TryInto::<Language>::try_into(LANGUAGE_PHYREXIAN), Ok(Language::Phyrexian));
    assert_eq!(TryInto::<Language>::try_into(LANGUAGE_PORTUGUESE_BRAZIL), Ok(Language::PortugueseBrazil));
    assert_eq!(TryInto::<Language>::try_into(LANGUAGE_RUSSIAN), Ok(Language::Russian));
    assert_eq!(TryInto::<Language>::try_into(LANGUAGE_SANSKRIT), Ok(Language::Sanskrit));
    assert_eq!(TryInto::<Language>::try_into(LANGUAGE_SPANISH), Ok(Language::Spanish));
    // Test owned.
    assert_eq!(TryInto::<Language>::try_into(LANGUAGE_ANCIENT_GREEK.to_string()), Ok(Language::AncientGreek));
    assert_eq!(TryInto::<Language>::try_into(LANGUAGE_ARABIC.to_string()), Ok(Language::Arabic));
    assert_eq!(TryInto::<Language>::try_into(LANGUAGE_CHINESE_SIMPLIFIED.to_string()), Ok(Language::ChineseSimplified));
    assert_eq!(TryInto::<Language>::try_into(LANGUAGE_CHINESE_TRADITIONAL.to_string()), Ok(Language::ChineseTraditional));
    assert_eq!(TryInto::<Language>::try_into(LANGUAGE_ENGLISH_AMERICAN.to_string()), Ok(Language::EnglishAmerican));
    assert_eq!(TryInto::<Language>::try_into(LANGUAGE_FRENCH.to_string()), Ok(Language::French));
    assert_eq!(TryInto::<Language>::try_into(LANGUAGE_GERMAN.to_string()), Ok(Language::German));
    assert_eq!(TryInto::<Language>::try_into(LANGUAGE_HEBREW.to_string()), Ok(Language::Hebrew));
    assert_eq!(TryInto::<Language>::try_into(LANGUAGE_ITALIAN.to_string()), Ok(Language::Italian));
    assert_eq!(TryInto::<Language>::try_into(LANGUAGE_JAPANESE.to_string()), Ok(Language::Japanese));
    assert_eq!(TryInto::<Language>::try_into(LANGUAGE_KOREAN.to_string()), Ok(Language::Korean));
    assert_eq!(TryInto::<Language>::try_into(LANGUAGE_LATIN.to_string()), Ok(Language::Latin));
    assert_eq!(TryInto::<Language>::try_into(LANGUAGE_PHYREXIAN.to_string()), Ok(Language::Phyrexian));
    assert_eq!(TryInto::<Language>::try_into(LANGUAGE_PORTUGUESE_BRAZIL.to_string()), Ok(Language::PortugueseBrazil));
    assert_eq!(TryInto::<Language>::try_into(LANGUAGE_RUSSIAN.to_string()), Ok(Language::Russian));
    assert_eq!(TryInto::<Language>::try_into(LANGUAGE_SANSKRIT.to_string()), Ok(Language::Sanskrit));
    assert_eq!(TryInto::<Language>::try_into(LANGUAGE_SPANISH.to_string()), Ok(Language::Spanish));
}
