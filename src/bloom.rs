/// A Bloom filter library
/// Authors: Sam Gehman, Josh Keating, Phillip Quinn
///
/// https://florian.github.io/bloom-filters/
/// https://llimllib.github.io/bloomfilter-tutorial/

use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash, Hasher};
use std::iter::Map;
use std::slice::Iter;

use crate::bitset::BitSet;

pub struct Bloom {
    bitset: BitSet,
    random_states: Vec<RandomState>,
}

impl Bloom {
    pub fn new(bitset_len: usize, num_hashes: u32) -> Bloom {
        let bitset = BitSet::new(bitset_len);
        let random_states: Vec<RandomState> = (0..num_hashes)
            .map(|_| RandomState::new())
            .collect();
        Bloom { bitset, random_states }
    }

    pub fn insert<T: Hash>(&mut self, value: T) {
        for i in self.get_indices(value) {
            self.bitset.set(i).unwrap();
        }
    }

    // TODO: rename to "maybe contains"
    pub fn contains<T: Hash>(&self, value: T) -> bool {
        for i in self.get_indices(value) {
            // If any of the bits are 0, the value is guaranteed to not be in the set
            if !self.bitset.get(i).unwrap() {
                return false;
            }
        }

        // The value may be in the set
        return true;
    }

    fn get_indices<T: Hash>(&self, value: T) -> Vec<usize> {
        self.random_states.iter().map(|random_state| {
            let mut hasher = random_state.build_hasher();
            value.hash(&mut hasher);
            (hasher.finish() % self.bitset.size as u64) as usize
        }).collect::<Vec<usize>>()
    }
}

#[test]
fn test_insert() {
    let x = "hello there";
    let y = "goodbye";

    let mut bloom = Bloom::new(32, 1);

    assert!(!bloom.contains(x));
    assert!(!bloom.contains(y));

    bloom.insert(x);
    assert!(bloom.contains(x));
    bloom.insert(x);
    assert!(bloom.contains(x));
    bloom.insert(y);
    assert!(bloom.contains(y));
}

#[test]
fn test_collision() {
    // TODO: simulate a collision
}
