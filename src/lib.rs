/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/seq-map
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
use std::{
    collections::HashMap,
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    hash::{Hash, Hasher},
    ops::Index,
};

/// A deterministic map that preserves insertion order.
///
/// Internally, it uses a [`HashMap`] for quick key lookups and a [`Vec`] to maintain the order
/// of inserted key-value pairs.
#[derive(Clone)]
pub struct SeqMap<K, V> {
    key_to_index: HashMap<K, usize>, // Maps keys to their index in `entries`
    entries: Vec<(K, V)>,            // Stores key-value pairs in insertion order
}

impl<K, V> Hash for SeqMap<K, V>
where
    K: Hash,
    V: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (key, value) in &self.entries {
            key.hash(state);
            value.hash(state);
        }
    }
}

impl<K, V> PartialEq for SeqMap<K, V>
where
    K: Eq,
    V: Eq,
{
    fn eq(&self, other: &Self) -> bool {
        if self.entries.len() != other.entries.len() {
            return false;
        }
        self.entries
            .iter()
            .zip(other.entries.iter())
            .all(|(a, b)| a == b)
    }
}

impl<K, V> Eq for SeqMap<K, V>
where
    K: Eq,
    V: Eq,
{
}

impl<K, V> Display for SeqMap<K, V>
where
    K: Eq + Hash + Display,
    V: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "SeqMap({})", self.entries.len())?;
        for (key, value) in &self.entries {
            write!(f, "\n{key}: {value}")?;
        }
        Ok(())
    }
}

impl<K, V> Debug for SeqMap<K, V>
where
    K: Eq + Hash + Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "SeqMap(")?;
        let mut first = true;
        for (key, value) in &self.entries {
            if !first {
                write!(f, ", ")?;
            }
            first = false;
            write!(f, "{key:?}: {value:?}")?;
        }
        write!(f, ")")
    }
}

/// Errors that can occur when manipulating a `SeqMap`.
#[derive(Debug)]
pub enum SeqMapError {
    /// Occurs when attempting to insert a key that already exists in the map.
    KeyAlreadyExists,
}

impl Display for SeqMapError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SeqMapError::KeyAlreadyExists => write!(f, "The key already exists in the SeqMap."),
        }
    }
}

impl Error for SeqMapError {}

impl<K, V> SeqMap<K, V>
where
    K: Eq + Hash + Clone, // Clone is because we add it to two containers
{
    /// Creates a new, empty `SeqMap`.
    ///
    /// # Examples
    ///
    /// ```
    /// use seq_map::SeqMap;
    /// let map: SeqMap<String, i32> = SeqMap::new();
    /// ```
    pub fn new() -> Self {
        Self {
            key_to_index: HashMap::new(),
            entries: Vec::new(),
        }
    }

    /// Inserts a key-value pair into the map.
    ///
    /// Returns an error if the key already exists.
    ///
    /// # Errors
    ///
    /// Returns `SeqMapError::KeyAlreadyExists` if the key is already present.
    ///
    /// # Examples
    ///
    /// ```
    /// use seq_map::SeqMap;
    /// let mut map = SeqMap::new();
    /// map.insert("key".to_string(), 42).unwrap();
    /// assert!(map.insert("key".to_string(), 43).is_err());
    /// ```
    pub fn insert(&mut self, key: K, value: V) -> Result<(), SeqMapError> {
        #[allow(clippy::map_entry)]
        if self.key_to_index.contains_key(&key) {
            Err(SeqMapError::KeyAlreadyExists)
        } else {
            self.entries.push((key.clone(), value));
            self.key_to_index.insert(key, self.entries.len() - 1);
            Ok(())
        }
    }

    /// Checks if the map contains a key.
    pub fn contains_key(&self, key: &K) -> bool {
        self.key_to_index.contains_key(key)
    }

    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// Returns `None` if the key does not exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use seq_map::SeqMap;
    /// let mut map = SeqMap::new();
    /// map.insert("key".to_string(), 42).unwrap();
    /// if let Some(value) = map.get_mut(&"key".to_string()) {
    ///     *value = 100;
    /// }
    /// assert_eq!(map[&"key".to_string()], 100);
    /// ```
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.key_to_index
            .get(key)
            .map(|&index| &mut self.entries[index].1)
    }

    /// Returns the number of key-value pairs in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// use seq_map::SeqMap;
    /// let mut map = SeqMap::new();
    /// assert_eq!(map.len(), 0);
    /// map.insert("key", 42).unwrap();
    /// assert_eq!(map.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns `true` if the map contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use seq_map::SeqMap;
    /// let map: SeqMap<String, i32> = SeqMap::new();
    /// assert!(map.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns an iterator over the keys of the map in insertion order.
    ///
    /// # Examples
    ///
    /// ```
    /// use seq_map::SeqMap;
    /// let mut map = SeqMap::new();
    /// map.insert("a".to_string(), 1).unwrap();
    /// map.insert("b".to_string(), 2).unwrap();
    /// let keys: Vec<_> = map.keys().cloned().collect();
    /// assert_eq!(keys, vec!["a", "b"]);
    /// ```
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.entries.iter().map(|(k, _)| k)
    }

    /// Returns an iterator over the values of the map in insertion order.
    ///
    /// # Examples
    ///
    /// ```
    /// use seq_map::SeqMap;
    /// let mut map = SeqMap::new();
    /// map.insert("a".to_string(), 1).unwrap();
    /// map.insert("b".to_string(), 2).unwrap();
    /// let values: Vec<_> = map.values().cloned().collect();
    /// assert_eq!(values, vec![1, 2]);
    /// ```
    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.entries.iter().map(|(_, v)| v)
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut V> {
        self.entries.iter_mut().map(|(_, v)| v)
    }

    pub fn get_index(&self, key: &K) -> Option<usize> {
        self.key_to_index.get(key).copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.entries.iter().map(|(k, v)| (k, v))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&K, &mut V)> {
        self.entries.iter_mut().map(|(k, v)| (&*k, v))
    }

    /// Retrieves a reference to the value corresponding to the key.
    ///
    /// This method performs a faster lookup using the internal `HashMap`.
    ///
    /// For a slower but simpler lookup, see [`slow_get`](Self::slow_get).
    ///
    /// # Examples
    ///
    /// ```
    /// use seq_map::SeqMap;
    /// let mut map = SeqMap::new();
    /// map.insert("key".to_string(), 42).unwrap();
    /// assert_eq!(map.get(&"key".to_string()), Some(&42));
    /// ```
    pub fn get(&self, key: &K) -> Option<&V> {
        self.key_to_index
            .get(key)
            .map(|&index| &self.entries[index].1)
    }

    /// Removes all elements from the map
    pub fn clear(&mut self) {
        self.key_to_index.clear();
        self.entries.clear();
    }

    /// Removes a key from the map, returning the value if it existed
    pub fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(&index) = self.key_to_index.get(key) {
            self.key_to_index.remove(key);
            // Update indices for all elements after the removed one
            for k in self.entries[index + 1..].iter().map(|(k, _)| k) {
                if let Some(idx) = self.key_to_index.get_mut(k) {
                    *idx -= 1;
                }
            }
            Some(self.entries.remove(index).1)
        } else {
            None
        }
    }

    /// Removes all elements from the map and returns them as an iterator
    pub fn drain(&mut self) -> impl Iterator<Item = (K, V)> + '_ {
        self.key_to_index.clear();
        self.entries.drain(..)
    }
}

impl<K, V> Index<&K> for SeqMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    type Output = V;

    /// Allows accessing values using the indexing syntax (`map[&key]`).
    ///
    /// # Panics
    ///
    /// Panics if the key is not present in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// use seq_map::SeqMap;
    /// let mut map = SeqMap::new();
    /// map.insert("key".to_string(), 42).unwrap();
    /// assert_eq!(map[&"key".to_string()], 42);
    /// ```
    fn index(&self, key: &K) -> &Self::Output {
        self.get(key).expect("Key not found in SeqMap")
    }
}

impl<K, V> From<&[(K, V)]> for SeqMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    /// Creates a `SeqMap` from a slice of key-value pairs.
    ///
    /// If duplicate keys are present in the slice, the first occurrence is kept, and subsequent
    /// duplicates are ignored.
    ///
    /// # Examples
    ///
    /// ```
    /// use seq_map::SeqMap;
    /// let pairs = vec![
    ///     ("a".to_string(), 1),
    ///     ("b".to_string(), 2),
    /// ];
    /// let map: SeqMap<_, _> = SeqMap::from(&pairs[..]);
    /// assert_eq!(map.len(), 2);
    /// ```
    fn from(slice: &[(K, V)]) -> Self {
        let mut map = SeqMap::new();
        for (key, value) in slice {
            // Ignore errors to keep the first occurrence
            let _ = map.insert(key.clone(), value.clone());
        }
        map
    }
}

/// Creates a `SeqMap` from an iterator of key-value pairs.
///
/// If duplicate keys are present in the iterator, the first occurrence is kept,
/// and subsequent duplicates are silently ignored.
impl<K: Hash, V> FromIterator<(K, V)> for SeqMap<K, V>
where
    K: Eq + Clone,
    V: Clone,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut map = SeqMap::new();
        for (k, v) in iter {
            let _ = map.insert(k, v); // Intentionally ignore errors for this trait
        }
        map
    }
}

impl<'a, K, V> IntoIterator for &'a SeqMap<K, V>
where
    K: Eq + Hash + Clone,
{
    type Item = (&'a K, &'a V);
    type IntoIter = std::iter::Map<std::slice::Iter<'a, (K, V)>, fn(&'a (K, V)) -> (&'a K, &'a V)>;

    /// Returns an iterator over references to the key-value pairs in insertion order.
    ///
    /// This implementation allows you to iterate over references to the keys and values
    /// without consuming the `SeqMap`. It's useful for scenarios where you want to inspect
    /// the contents of the map without modifying it.
    ///
    /// # Examples
    ///
    /// ## Iterating Over References
    ///
    /// ```rust
    /// use seq_map::SeqMap;
    ///
    /// let mut map = SeqMap::new();
    /// map.insert("a".to_string(), 1).unwrap();
    /// map.insert("b".to_string(), 2).unwrap();
    /// map.insert("c".to_string(), 3).unwrap();
    ///
    /// for (key, value) in &map {
    ///     println!("{}: {}", key, value);
    /// }
    /// ```
    ///
    /// ## Collecting into a Vector of References
    ///
    /// ```rust
    /// use seq_map::SeqMap;
    ///
    /// let mut map = SeqMap::new();
    /// map.insert("x".to_string(), 100).unwrap();
    /// map.insert("y".to_string(), 200).unwrap();
    ///
    /// let entries: Vec<_> = (&map).into_iter().collect();
    /// assert_eq!(
    ///     entries,
    ///     vec![
    ///         (&"x".to_string(), &100),
    ///         (&"y".to_string(), &200),
    ///     ]
    /// );
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.entries.iter().map(|(k, v)| (k, v))
    }
}

impl<K, V> Default for SeqMap<K, V> {
    /// Creates a new, empty `SeqMap`.
    ///
    /// # Examples
    ///
    /// ```
    /// use seq_map::SeqMap;
    /// let map: SeqMap<String, i32> = SeqMap::default();
    /// ```
    fn default() -> Self {
        Self {
            key_to_index: HashMap::default(),
            entries: Vec::default(),
        }
    }
}

// Mutable reference iterator
impl<'a, K, V> IntoIterator for &'a mut SeqMap<K, V>
where
    K: Eq + Hash + Clone,
{
    type Item = (&'a K, &'a mut V);
    type IntoIter =
        std::iter::Map<std::slice::IterMut<'a, (K, V)>, fn(&'a mut (K, V)) -> (&'a K, &'a mut V)>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.iter_mut().map(|(k, v)| (&*k, v))
    }
}

// Consuming iterator
impl<K, V> IntoIterator for SeqMap<K, V>
where
    K: Eq + Hash + Clone,
{
    type Item = (K, V);
    type IntoIter = std::vec::IntoIter<(K, V)>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

impl<K, V> SeqMap<K, V>
where
    K: Eq + Hash + Clone,
{
    pub fn into_keys(self) -> impl Iterator<Item = K> {
        self.entries.into_iter().map(|(k, _)| k)
    }

    pub fn into_values(self) -> impl Iterator<Item = V> {
        self.entries.into_iter().map(|(_, v)| v)
    }
}

impl<K, V> Extend<(K, V)> for SeqMap<K, V>
where
    K: Eq + Hash + Clone,
{
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for (k, v) in iter {
            let _ = self.insert(k, v);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::SeqMap;

    #[test]
    fn test_clear_and_drain() {
        let mut map = SeqMap::new();
        map.insert("a", 1).unwrap();
        map.insert("b", 2).unwrap();

        let mut other_map = SeqMap::new();
        for (k, v) in map.drain() {
            other_map.insert(k, v).unwrap();
        }
        assert!(map.is_empty());
        assert_eq!(other_map.len(), 2);

        other_map.clear();
        assert!(other_map.is_empty());
        assert_eq!(other_map.key_to_index.len(), 0);
        assert_eq!(other_map.entries.len(), 0);
    }
}
