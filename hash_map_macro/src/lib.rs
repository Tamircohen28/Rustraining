#![cfg_attr(debug_assertions, allow(unused_imports))]
use std::collections::HashMap;
use std::collections::hash_map::RandomState;

#[macro_export]
macro_rules! HashMap {
    // create empty hash map
    () => {
        HashMap::new()
    };

    // create hash map with capacity to hold without reallocating
    ($capacity:expr) => {
        HashMap::with_capacity($capacity);
    };   

    // create hash map with user hash builder
    ($hash_builder:tt;) => {
        HashMap::with_hasher($hash_builder);
    };

    ($capacity:expr, $hash_builder:tt) => {
        HashMap::with_capacity_and_hasher($capacity, $hash_builder)
    };
}

/// test for empty map
#[test]
fn empty_map() {
    let map: HashMap<String, String> = HashMap!();
    assert!(map.is_empty());
}

/// test for capacity check map
#[test]
fn capacity_map() {
    let capacity = 100;
    let map: HashMap<String, String> = HashMap!(capacity);
    assert!(map.is_empty());
    assert!(map.capacity() >= capacity);
}

/// test for hash builder
#[test]
fn hash_builder_map() {
    let hash_builder = RandomState::new();
    let map: HashMap<String, String> = HashMap!(hash_builder;);
    assert!(map.is_empty());
}

/// test for hash builder with capacity
#[test]
fn hash_builder_and_capacity_map() {
    let hash_builder = RandomState::new();
    let capacity = 100;
    let map: HashMap<String, String> = HashMap!(capacity, hash_builder);
    assert!(map.is_empty());
    assert!(map.capacity() >= capacity);
}


