# SeqMap

**SeqMap** is a deterministic and ordered map implementation in Rust that preserves the insertion order of key-value pairs. It combines the efficiency of a `HashMap` for quick key lookups with the ordered iteration provided by a `Vec`. This makes `SeqMap` ideal for scenarios where the order of elements is important and predictable.

## Features

- **Deterministic Ordering**: Maintains the order of key-value pairs based on their insertion sequence.
- **Efficient Lookups**: Utilizes a `HashMap` internally for `O(1)` average-case key lookups.
- **Comprehensive API**: Provides methods for insertion, retrieval, mutation, iteration, and more.

## Installation

Add `seq-map` to your `Cargo.toml` dependencies:

```toml
[dependencies]
seq-map = "0.0.1"
```
