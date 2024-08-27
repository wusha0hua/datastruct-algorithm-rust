
#[derive(Debug, Clone)]
pub struct BitMap {
    max: usize,
    bits: usize,
    map: Vec<usize>,
}

impl BitMap {
    pub fn new(max: usize) -> Self {
        Self {
            max,
            bits: usize::BITS as usize,
            map: vec![0; (max + usize::BITS as usize - 1) / usize::BITS as usize],
        }
    }
    pub fn add(&mut self, index: usize) -> bool {
        let (x, y) = Self::get_position(index);
        if self.map[x] & (1 << y) == 0 {
            self.map[x] |= 1 << y;
            true
        } else {
            false
        }
    }
    pub fn remove(&mut self, index: usize) -> bool {
        let (x, y) = Self::get_position(index);
        if self.map[x] & (1 << y) == 0 {
            false
        } else {
            self.map[x] &= !(1 << y);
            true    
        }
    }
    pub fn reverse(&mut self, index: usize) {
        let (x, y) = Self::get_position(index);
        self.map[x] ^= 1 << y;
    }
    pub fn contains(&mut self, index: usize) -> bool {
        let (x, y) = Self::get_position(index);
        !((self.map[x] & (1 << y)) == 0)
    }
    fn get_position(index: usize) -> (usize, usize) {
        (index / usize::BITS as usize, index % usize::BITS as usize)
    }
}

#[cfg(test)]
mod test_bitmap {
    use super::BitMap;
    #[test]
    fn test_bitmap() {
        let mut bitmap = BitMap::new(1000);
        bitmap.add(10);
        bitmap.add(50);
        bitmap.add(1000);
        assert_eq!(bitmap.contains(10), true);
        assert_eq!(bitmap.contains(100), false);
        assert_eq!(bitmap.remove(50), true);
        assert_eq!(bitmap.contains(50), false);
    }
}
