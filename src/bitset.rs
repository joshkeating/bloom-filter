pub struct BitSet {
    size: u32,
    data: Vec<u8>,
}

impl BitSet {
    pub fn new(size: u32) -> BitSet {
        let num_bytes: u32 = (size as f32 / 8.0).ceil() as u32;
        BitSet { size: num_bytes, data: vec![0; size as usize] }
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

#[test]
fn test1() {
    assert!(true);
}

#[test]
fn test2() {
    assert!(false);
}
