use std::fmt;
use std::fmt::{Formatter, Display};
use std::slice::Iter;
use std::iter::Map;

#[derive(Debug)]
pub struct BitSet {
    pub size: usize,
    // TODO: does this leave unused memory (word size ptrs)?
    data: Vec<u8>,
}

impl BitSet {
    pub fn new(size: usize) -> BitSet {
        let num_bytes: usize = (size as f32 / 8.0).ceil() as usize;
        BitSet { size, data: vec![0; num_bytes] }
    }

    pub fn set(&mut self, index: usize) -> Result<(), String> {
        match self.get_position(index) {
            Ok((index, offset)) => {
                Ok((self.data[index as usize] |= 1 << offset))
            }
            Err(msg) => {
                Err(msg)
            }
        }
    }

    pub fn get(&self, index: usize) -> Result<bool, String> {
        match self.get_position(index) {
            Ok((index, offset)) => {
                Ok((self.data[index] >> offset) & 1 != 0)
            }
            Err(msg) => {
                Err(msg)
            }
        }
    }

    fn get_position(&self, i: usize) -> Result<(usize, usize), String> {
        if i < 0 || i >= self.size {
            Err(format!("index out of bounds: {i}", i = i))
        } else {
            let index = i / 8;
            let offset = i % 8;
            Ok((index, offset))
        }
    }
}

// https://doc.rust-lang.org/std/fmt/trait.Display.html
impl fmt::Display for BitSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // https://doc.rust-lang.org/std/fmt/trait.Binary.html
        let bit_string = self.data.iter()
            .map(|byte| format!("{:08b}", byte))
            .map(|byte_string| byte_string.chars().rev().collect::<String>())
            .collect::<String>();
        write!(f, "{}", &bit_string[0..self.size])
    }
}

#[test]
fn test_fmt() {
    let mut bs = BitSet::new(10);
    let values = [
        "1000000000",
        "1100000000",
        "1110000000",
        "1111000000",
        "1111100000",
        "1111110000",
        "1111111000",
        "1111111100",
        "1111111110",
        "1111111111"
    ];

    for i in 0..10 {
        bs.set(i);
        assert_eq!(values[i], format!("{}", bs));
    }
}

#[test]
fn test_set() {
    // let size = 10;
    // let bs = BitSet::new(size);
    //
    // (0..size).iter().all()
    // // bs.iter().all(|x| x)
}

#[test]
fn test_get() {
    let mut bs = BitSet::new(10);

    // Out of bounds is Err
    assert!(bs.get(11).is_err());

    // Default value is false
    assert_eq!(bs.get(0).unwrap(), false);

    // Value is true after setting
    bs.set(0);
    assert_eq!(bs.get(0).unwrap(), true);

    // Value is still true after setting again
    bs.set(0);
    assert_eq!(bs.get(0).unwrap(), true);
}
