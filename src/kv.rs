#![deny(missing_docs)]

//! A simple key-value store
//!
//! This crate contains a `KvStore` struct that exposes an api for storing
//! string pairs in key/value format.

use crate::Result;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// Struct contains a single property `map` that holds
/// our key/value pairs in memory
pub struct KvStore {
    writer: BufWriter<File>,
    map: HashMap<String, String>,
}

impl KvStore {
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
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let cmd = Command::set(key, value);
        serde_json::to_writer(&mut self.writer, &cmd)?;
        self.writer.flush()?;
        Ok(())
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
    pub fn get(&self, key: String) -> Result<Option<String>> {
        unimplemented!();
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
    pub fn remove(&mut self, key: String) -> Result<()> {
        unimplemented!();
    }

    /// Opens a new store
    ///
    /// # Examples
    ///
    /// ```
    /// # use kvs::KvStore;
    /// let mut store = KvStore::new();

    /// assert_ok!(store.open("/".to_owned()));
    /// ```
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open("log.log")?;
        let mut buf_writer = BufWriter::new(file);
        Ok(KvStore {
            writer: buf_writer,
            map: HashMap::new(),
        })
    }
}

/// Struct representing a command that we can serialize
#[derive(Serialize, Deserialize, Debug)]
enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}

impl Command {
    fn set(key: String, value: String) -> Command {
        Command::Set { key, value }
    }

    fn remove(key: String) -> Command {
        Command::Remove { key }
    }
}
