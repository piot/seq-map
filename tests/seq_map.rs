/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/seq-map
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */

use seq_map::SeqMap;

pub struct TestStruct {
    pub a: i32,
    pub b: i32,
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct TestKey {
    pub a: i32,
}

#[test]
fn display() {
    let mut map = SeqMap::new();

    map.insert(10, 20).expect("should work");

    assert_eq!(map.to_string(), "SeqMap(1)\n10: 20")
}

#[test]
fn debug() {
    let mut map = SeqMap::new();

    map.insert(10, 20).expect("should work");
    map.insert(42, -13).expect("should work");

    assert_eq!(format!("{:?}", map), "SeqMap(10: 20, 42: -13)")
}

#[test]
fn index() {
    let mut map = SeqMap::new();

    map.insert(10, 20).expect("should work");
    map.insert(42, -13).expect("should work");

    assert_eq!(map.get_index(&42), Some(1));
    assert_eq!(map.get_index(&10), Some(0));
    assert_eq!(map.get_index(&100), None);
}

#[test]
fn index_struct() {
    let mut map = SeqMap::new();

    map.insert(TestKey { a: 20 }, 20).expect("should work");
    map.insert(TestKey { a: 42 }, -13).expect("should work");

    assert_eq!(map.get_index(&TestKey { a: 42 }), Some(1));
}

#[test]
fn test_hash() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut map1 = SeqMap::new();
    let mut map2 = SeqMap::new();

    map1.insert(1, "a").unwrap();
    map1.insert(2, "b").unwrap();

    map2.insert(1, "a").unwrap();
    map2.insert(2, "b").unwrap();

    let mut hasher1 = DefaultHasher::new();
    let mut hasher2 = DefaultHasher::new();

    map1.hash(&mut hasher1);
    map2.hash(&mut hasher2);

    assert_eq!(hasher1.finish(), hasher2.finish());
}

#[test]
fn iter_struct() {
    let mut map = SeqMap::new();

    map.insert(TestKey { a: 10 }, TestStruct { a: 10, b: 20 })
        .expect("should work");
    map.insert(TestKey { a: -10 }, TestStruct { a: -10, b: 42 })
        .expect("should work");

    for (k, v) in &map {
        assert_eq!(k.a, v.a)
    }
}

#[test]
fn test_iterators() {
    let mut map = SeqMap::new();
    map.insert("a", 1).unwrap();
    map.insert("b", 2).unwrap();

    let pairs: Vec<_> = map.iter().map(|(k, v)| (*k, *v)).collect();
    assert_eq!(pairs, vec![("a", 1), ("b", 2)]);

    for (_, v) in map.iter_mut() {
        *v *= 2;
    }

    assert_eq!(map[&"a"], 2);
    assert_eq!(map[&"b"], 4);

    let pairs: Vec<_> = map.into_iter().collect();
    assert_eq!(pairs, vec![("a", 2), ("b", 4)]);
}

#[test]
fn test_remove_and_reindex() {
    let mut map = SeqMap::new();
    map.insert("a", 1).unwrap();
    map.insert("b", 2).unwrap();
    map.insert("c", 3).unwrap();

    assert_eq!(map.remove(&"b"), Some(2));

    // Check order is preserved
    let keys: Vec<_> = map.keys().copied().collect();
    assert_eq!(keys, vec!["a", "c"]);

    // Verify indices were updated
    assert_eq!(map.get_index(&"a"), Some(0));
    assert_eq!(map.get_index(&"c"), Some(1));

    // Can still insert after remove
    map.insert("d", 4).unwrap();
    assert_eq!(map.get_index(&"d"), Some(2));
}

#[test]
fn test_extend() {
    let mut map = SeqMap::new();
    map.insert("a", 1).unwrap();

    let more = vec![("b", 2), ("c", 3)];
    map.extend(more);

    assert_eq!(map.len(), 3);
    let values: Vec<_> = map.values().copied().collect();
    assert_eq!(values, vec![1, 2, 3]);

    // Test that extend preserves insertion order
    let keys: Vec<_> = map.keys().copied().collect();
    assert_eq!(keys, vec!["a", "b", "c"]);
}

// Test `FromIterator` implementation

#[test]
fn from_iterator() {
    let pairs = vec![("a", 1), ("b", 2), ("c", 3)];

    let map: SeqMap<_, _> = pairs.into_iter().collect();

    assert_eq!(map.len(), 3);
    assert_eq!(map.get(&"a"), Some(&1));
    assert_eq!(map.get(&"b"), Some(&2));
    assert_eq!(map.get(&"c"), Some(&3));

    // Check order is preserved
    let keys: Vec<_> = map.keys().collect();
    assert_eq!(keys, vec![&"a", &"b", &"c"]);
}

#[test]
fn from_iterator_with_duplicates() {
    let pairs = vec![
        ("a", 1),
        ("b", 2),
        ("a", 3), // Duplicate key
        ("c", 4),
        ("b", 5), // Duplicate key
    ];

    let map: SeqMap<_, _> = pairs.into_iter().collect();

    // Check only first occurrences are kept
    assert_eq!(map.len(), 3);
    assert_eq!(map.get(&"a"), Some(&1)); // First value kept
    assert_eq!(map.get(&"b"), Some(&2)); // First value kept
    assert_eq!(map.get(&"c"), Some(&4));

    // Check order matches first occurrences
    let keys: Vec<_> = map.keys().collect();
    assert_eq!(keys, vec![&"a", &"b", &"c"]);

    let values: Vec<_> = map.values().collect();
    assert_eq!(values, vec![&1, &2, &4]);
}

#[test]
fn collect_into_seq_map() {
    // From filter
    let pairs = vec![("a", 1), ("b", 2), ("c", 3)];
    let map: SeqMap<_, _> = pairs.into_iter().filter(|(_, v)| *v > 1).collect();

    assert_eq!(map.len(), 2);
    assert_eq!(map.get(&"b"), Some(&2));
    assert_eq!(map.get(&"c"), Some(&3));

    // From map
    let strings = vec!["hello", "world"];
    let map: SeqMap<_, _> = strings.into_iter().map(|s| (s, s.len())).collect();

    assert_eq!(map.len(), 2);
    assert_eq!(map.get(&"hello"), Some(&5));
    assert_eq!(map.get(&"world"), Some(&5));
}

#[test]
fn chain_with_duplicates() {
    let first = vec![("a", 1), ("b", 2)];
    let second = vec![("b", 3), ("c", 4)]; // b is duplicate

    let map: SeqMap<_, _> = first.into_iter().chain(second).collect();

    assert_eq!(map.len(), 3);
    assert_eq!(map.get(&"a"), Some(&1));
    assert_eq!(map.get(&"b"), Some(&2)); // First occurrence kept
    assert_eq!(map.get(&"c"), Some(&4));

    // Check order
    let entries: Vec<_> = map.iter().collect();
    assert_eq!(entries, vec![(&"a", &1), (&"b", &2), (&"c", &4),]);
}

#[test]
fn empty_iterator() {
    let empty_vec: Vec<(&str, i32)> = Vec::new();
    let map: SeqMap<&str, i32> = empty_vec.into_iter().collect();

    assert!(map.is_empty());
    assert_eq!(map.len(), 0);
}
