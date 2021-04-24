/*
  A Bloom filter library
  Authors: Josh Keating, Phillip Quinn, Sam Gehman

  https://florian.github.io/bloom-filters/
  https://llimllib.github.io/bloomfilter-tutorial/
*/

pub struct Bloom {
    m: u32,
    k: u32,
}

impl Bloom {
    // m: length of the bit set
    // k: number of hash functions
    pub fn new(m: u32, k: u32) -> Bloom {
        Bloom { m, k }
    }

    pub fn insert(&self, x: i32) {}

    pub fn contains(&self, x: i32) {}
}
