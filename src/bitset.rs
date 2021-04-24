pub struct BitSet {
    size: u32,
    data: Vec<u8>,
}

impl BitSet {
    pub fn new(size: u32) {
        let size: usize = usize(ceil(f32(size) / 8));
        let data: Vec<u8, size> = vec![0; size];
        Bitset { size, data }
    }

    fn index

    pub fn set(&mut self, index: u32) -> Result<(), E> {
        if index >= self.size {
            Ok(())
        } else {
            Err("index out of bounds")
        }
    }

    pub fn get(&self, index: u32) -> Some<Bool> {
        if index >= self.size {
            None
        } else {
            Some()
        }
    }
}
