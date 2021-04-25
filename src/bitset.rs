use std::fmt;
use std::fmt::Formatter;
use std::slice::Iter;
use std::iter::Map;

#[derive(Debug)]
pub struct BitSet {
    size: u32,
    data: Vec<u8>,
}

impl BitSet {
    pub fn new(size: u32) -> BitSet {
        let num_bytes: usize = (size as f32 / 8.0).ceil() as usize;
        BitSet { size, data: vec![0; num_bytes] }
    }

    fn get_position(&self, i: u32) -> Option<(u32, u32)> {
        if i >= self.size {
            None
        } else {
            let index = i / 8;
            let offset = i % 8;
            Some((index, offset))
        }
    }

    pub fn set(&mut self, index: u32) -> Result<(), String> {
        if let Some((index, offset)) = self.get_position(index) {
            self.data[index as usize] |= 1 << offset;
            Ok(())
        } else {
            Err(format!("index out of bounds: {i}", i = index))
        }
    }

    pub fn get(&self, index: u32) -> Option<bool> {
        if index >= self.size {
            None
        } else {
            Some(false)
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
        write!(f, "{}", bit_string)
    }
}

#[test]
fn test1() {
    let mut bs = BitSet::new(10);
    bs.set(0);

    assert_eq!(bs.size, 10);
    println!("{}", bs);
    println!("{:?}", bs);
}

#[test]
fn test2() {
    // assert!(false);
}
