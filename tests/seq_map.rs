/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/seq-map
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */

use seq_map::SeqMap;

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
