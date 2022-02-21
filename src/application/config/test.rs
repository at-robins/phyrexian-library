use super::*;
use super::super::super::magic::card::CardSetBuilder;
use super::super::super::magic::language::LocalisedString;
use chrono::NaiveDate;

#[test]
/// Tests if the `resource_path` function returns the correct path.
fn test_resource_path() {
    let p = Configuration::resource_path();
    assert_eq!(p.to_str().unwrap(), "resources");
    assert!(p.is_relative());
}

#[test]
/// Tests if the `database_path` function returns the correct path.
fn test_database_path() {
    let p = Configuration::database_path();
    assert_eq!(p.to_str().unwrap(), "resources/databases");
    assert!(p.is_relative());
}

#[test]
/// Tests if the `import_database_path` function returns the correct path.
fn test_database_import_path() {
    let p = Configuration::database_import_path();
    assert_eq!(p.to_str().unwrap(), "resources/databases/AllPrintings.json");
    assert!(p.is_relative());
}

#[test]
/// Tests if the `set_path` function returns the correct path.
fn test_set_path() {
    let p = Configuration::set_path();
    assert_eq!(p.to_str().unwrap(), "resources/sets");
    assert!(p.is_relative());
}

#[test]
/// Tests if the `set_file_path` function returns the correct path.
fn test_set_file_path() {
    let s = set_with_code("TEST");
    let p = Configuration::set_file_path(s);
    assert_eq!(p.to_str().unwrap(), "resources/sets/TEST.mtgset");
    assert!(p.is_relative());
}

fn set_with_code(code: &str) -> CardSet {
    let mut set_builder = CardSetBuilder::default();
    set_builder
        .code(code.to_string())
        .keyrune("".to_string())
        .name(LocalisedString::new("Test"))
        .release_date(NaiveDate::from_ymd(2000, 1, 1));
    set_builder.build().unwrap()
}