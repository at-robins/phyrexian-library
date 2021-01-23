use super::super::*;

#[test]
/// Tests if the ordering of `LocalisedString` works as expected.
fn test_ordering() {
    let mut non_default = LocalisedString::new("B");
    non_default.set(Language::Italian, "Y");
    let mut unordered = vec!(
        LocalisedString::new("T"),
        LocalisedString::new("Ac"),
        non_default.clone(),
        LocalisedString::new("V"),
        LocalisedString::new("A"),
        LocalisedString::new("Ab"),
    );
    let ordered = vec!(
        LocalisedString::new("A"),
        LocalisedString::new("Ab"),
        LocalisedString::new("Ac"),
        non_default,
        LocalisedString::new("T"),
        LocalisedString::new("V"),
    );
    assert_ne!(unordered, ordered);
    unordered.sort();
    assert_eq!(unordered, ordered);
}

#[test]
/// Tests if the conversion from string to `LocalisedString` works as expected.
fn test_conversion_from_string() {
    let test_string = "Test default";
    assert_eq!(test_string, Into::<LocalisedString>::into(test_string).get_default());
    assert_eq!(LocalisedString::new(test_string), Into::<LocalisedString>::into(test_string));
}

#[test]
/// Tests if the `get_default` method of `LocalisedString` works as expected.
fn test_get_default() {
    let test_default = "Test default";
    let test_italian = "Test italian";
    let mut test_localised_string = LocalisedString::new(test_default);
    assert_eq!(test_default, test_localised_string.get_default());
    test_localised_string.set(Language::Italian, test_italian);
    assert_eq!(test_default, test_localised_string.get_default());
    test_localised_string.set(Language::default(), test_italian);
    assert_eq!(test_italian, test_localised_string.get_default());
}

#[test]
/// Tests if the `set` method of `LocalisedString` works as expected.
fn test_set() {
    let test_default = "Test default";
    let test_italian = "Test italian";
    let mut test_localised_string = LocalisedString::new(test_default);
    assert_eq!(test_default, test_localised_string.get_default());
    assert_eq!(None, test_localised_string.set(Language::Italian, test_italian));
    assert_eq!(test_default, test_localised_string.get_default());
    assert_eq!(Some(test_italian), test_localised_string.get_localised(Language::Italian));
    assert_eq!(
        Some(test_default.to_string()),
        test_localised_string.set(Language::default(), test_italian)
    );
    assert_eq!(test_italian, test_localised_string.get_default());
}

#[test]
/// Tests if the `get_localised` method of `LocalisedString` works as expected.
fn test_get_localised() {
    let test_default = "Test default";
    let test_russian = "Test russian";
    let mut test_localised_string = LocalisedString::new(test_default);
    assert_eq!(Some(test_default), test_localised_string.get_localised(Language::default()));
    assert_eq!(None, test_localised_string.get_localised(Language::Russian));
    test_localised_string.set(Language::Russian, test_russian);
    assert_eq!(Some(test_default), test_localised_string.get_localised(Language::default()));
    assert_eq!(Some(test_russian), test_localised_string.get_localised(Language::Russian));
}

#[test]
/// Tests if the `get_localised_or_default` method of `LocalisedString` works as expected.
fn test_get_localised_or_default() {
    let test_default = "Test default";
    let test_russian = "Test russian";
    let mut test_localised_string = LocalisedString::new(test_default);
    assert_eq!(test_default, test_localised_string.get_localised_or_default(Language::default()));
    assert_eq!(test_default, test_localised_string.get_localised_or_default(Language::Russian));
    test_localised_string.set(Language::Russian, test_russian);
    assert_eq!(test_default, test_localised_string.get_localised_or_default(Language::default()));
    assert_eq!(test_russian, test_localised_string.get_localised_or_default(Language::Russian));
}
