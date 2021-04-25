/*
  A Bloom filter library
  Authors: Josh Keating, Phillip Quinn, Sam Gehman

  https://florian.github.io/bloom-filters/
  https://llimllib.github.io/bloomfilter-tutorial/
*/

use crate::bitset::BitSet;
use std::hash::{Hash, BuildHasher, Hasher};
use std::collections::hash_map::{RandomState, DefaultHasher};
use std::slice::Iter;
use std::iter::Map;

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

    pub fn insert<T: Hash>(&mut self, x: T) {
        for i in self.get_indices(x) {
            self.bitset.set(i).unwrap();
        }
    }

    pub fn contains<T: Hash>(&self, x: T) {}

    fn get_indices<T: Hash>(&self, x: T) -> Vec<usize> {
        self.random_states.iter().map(|random_state| {
            println!("New hash!");
            let mut hasher = random_state.build_hasher();
            x.hash(&mut hasher);
            (hasher.finish() % self.bitset.size as u64) as usize
        }).collect::<Vec<usize>>()
    }
}

#[test]
fn test_fmt() {
}
