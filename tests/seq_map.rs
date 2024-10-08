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
