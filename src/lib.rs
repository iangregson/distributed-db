use std::collections::HashMap;

pub struct KvStore {
  map: HashMap<String, String>
}

impl KvStore {
  /// Creates a new `KvStore`
  pub fn new() -> KvStore {
    KvStore {
      map: HashMap::new(),
    }
  }
  /// Sets the value of a string key to string
  ///
  /// If the key exists, the previous value is overwritten
  pub fn set(&mut self, key: String, value: String) {
    panic!("unimplemented")
  }
  /// Gets the string value of the given string key
  ///
  /// If the key does not exist, None is returned
  pub fn get(&self, key: String) -> Option<String> {
    panic!("unimplemented")
  }
  /// Removes a given key from the store
  pub fn remove(&mut self, key: String) {
    panic!("unimplemented")
  }
}

