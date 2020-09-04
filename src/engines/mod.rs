//! This module provides various key value storage engines

use crate::Result;

/// Trait for a key value storage engine
pub trait KvsEngine {
    /// Sets the value of a string key to string.
    ///
    /// If the key exists, the prvious value is overwritten
    fn set(&self, key: String, value: String) -> Result<()>;

    /// Gets the string value of a string key.
    ///
    /// Returns `None` if the given key is not found.
    fn get(&self, key: String) -> Result<Option<String>>;

    /// Removes a given key.
    ///
    /// # Errors
    ///
    /// It returns `KvsError::KeyNotFound` if the given key is not found.
    fn remove(&self, key: String) -> Result<()>;
}

mod kvs;
mod sled;

pub use self::kvs::KvStore;
pub use self::sled::SledKvsEngine;
