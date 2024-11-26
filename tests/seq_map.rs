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
fn iter() {
    let mut map = SeqMap::new();

    map.insert(10, 20).expect("should work");
    map.insert(42, -13).expect("should work");
    for (k, v) in map.iter() {
        println!("{}: {}", k, v);
    }
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
