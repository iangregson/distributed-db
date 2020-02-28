#![deny(missing_docs)]

//! A simple key-value store
//!
//! This crate contains a `KvStore` struct that exposes an api for storing
//! string pairs in key/value format.

use std::collections::HashMap;

#[derive(Default)]
/// Struct contains a single property `map` that holds
/// our key/value pairs in memory
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    /// Creates a new `KvStore`
    ///
    /// # Examples
    /// 
    /// ```
    /// # use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// ```
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }

    /// Sets the value of a string key to string
    ///
    /// If the key exists, the previous value is overwritten
    ///
    /// # Examples
    /// 
    /// ```
    /// # use kvs::KvStore;
    /// let mut store = KvStore::new();

    /// store.set("key1".to_owned(), "value1".to_owned());
    /// assert_eq!(store.get("key1".to_owned()), Some("value1".to_owned()));
    /// ```
    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    /// Gets the string value of the given string key
    ///
    /// If the key does not exist, None is returned
    /// 
    /// # Examples
    ///
    /// ```
    /// # use kvs::KvStore;
    /// let mut store = KvStore::new();

    /// store.set("key1".to_owned(), "value1".to_owned());
    /// store.set("key2".to_owned(), "value2".to_owned());

    /// assert_eq!(store.get("key1".to_owned()), Some("value1".to_owned()));
    /// assert_eq!(store.get("key2".to_owned()), Some("value2".to_owned()));
    /// ```
    pub fn get(&self, key: String) -> Option<String> {
        self.map.get(&key).cloned()
    }

    /// Removes a given key from the store
    ///
    /// # Examples
    ///
    /// ```
    /// # use kvs::KvStore;
    /// let mut store = KvStore::new();

    /// store.set("key1".to_owned(), "value1".to_owned());
    /// store.remove("key1".to_owned());
    /// assert_eq!(store.get("key1".to_owned()), None);
    /// ```
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}

