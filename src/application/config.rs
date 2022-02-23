/// The folder in which all resources are stored.
const DEFAULT_FOLDER_RESOURCE: &str = "resources";
/// The folder in which all databases are stored.
const DEFAULT_FOLDER_RESOURCE_DATABASE: &str = "databases";
/// The folder in which all sets are stored.
const DEFAULT_FOLDER_RESOURCE_SET: &str = "sets";
/// The name of the default database.
const DEFAULT_DATABASE_NAME: &str = "AllPrintings";
/// The file extension for a set.
pub const EXTENSION_SET: &str = "mtgset";
/// The file extension of a JSON.
const EXTENSION_JSON: &str = "json";

use std::{borrow::Borrow, path::PathBuf};
use super::super::magic::card::CardSet;

pub struct Configuration {}

impl Configuration {

    /// The path to the resource folder.
    pub fn resource_path() -> PathBuf {
        PathBuf::from(DEFAULT_FOLDER_RESOURCE)
    }

    /// The path to the database folder.
    pub fn database_path() -> PathBuf {
        let mut path = Configuration::resource_path();
        path.push(DEFAULT_FOLDER_RESOURCE_DATABASE);
        path
    }

    /// The path to the database for importing existing information
    /// and conversion into an internal format.
    pub fn database_import_path() -> PathBuf {
        let mut path = Configuration::database_path();
        path.push(DEFAULT_DATABASE_NAME);
        path.set_extension(EXTENSION_JSON);
        path
    }

    /// The path to the folder containing all `Set`s.
    pub fn set_path() -> PathBuf {
        let mut path = Configuration::resource_path();
        path.push(DEFAULT_FOLDER_RESOURCE_SET);
        path
    }

    /// The path to the specified `Set`.
    ///
    /// # Parameters
    ///
    /// * set - the set to get the path to
    pub fn set_file_path<T: Borrow<CardSet>>(set: T) -> PathBuf {
        let mut path = Configuration::set_path();
        path.push(set.borrow().code());
        path.set_extension(EXTENSION_SET);
        path
    }
    
}

#[cfg(test)]
mod test;