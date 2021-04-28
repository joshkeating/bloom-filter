/*
  A Bloom filter library
  Authors: Josh Keating, Phillip Quinn, Sam Gehman

  https://florian.github.io/bloom-filters/
  https://llimllib.github.io/bloomfilter-tutorial/
*/

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

    pub fn contains<T: Hash>(&self, value: T) {}

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
    bloom.insert(x);
    bloom.insert(x);
    bloom.insert(y);
}
