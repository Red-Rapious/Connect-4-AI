pub const TABLE_SIZE: usize = (1 << 23) + 9; // first prime greater than 2^23

pub struct LowerBoundTranspositionTable {
    size: usize,
    keys: Vec<u32>,
    values: Vec<Option<u16>>
}

impl LowerBoundTranspositionTable {
    pub fn new(size: usize) -> Self {
        assert_eq!(size % 2, 1, "LowerBoundTranspositionTable size must be odd, but it is equal to {size}.");

        Self { 
            size, 
            keys: vec![0; size], 
            values: vec![None; size]
        }
    }
    
    fn index(&self, key: u64) -> usize {
        key as usize % self.size
    }

    pub fn insert(&mut self, key: u64, value: u16) {
        let index = self.index(key);
        self.keys[index] = key as u32; // possibly truncated
        self.values[index] = Some(value);
    }

    pub fn get(&self, key: u64) -> Option<u16> {
        let index = self.index(key);
        if self.keys[index] == key as u32 {
            self.values[index]
        } else {
            None
        }
    }
}

#[cfg(test)]
mod optimised_transposition_table_tests {
    use super::*;

    #[test]
    fn insert_get() {
        let mut table = LowerBoundTranspositionTable::new(11);

        table.insert(42, 21);
        assert_eq!(table.get(42), Some(21));
    }

    #[test]
    fn insert_get_index() {
        let mut table = LowerBoundTranspositionTable::new(11);

        table.insert(10, 21);
        assert_eq!(table.get(0), None);
    }

    #[test]
    fn index_override() {
        let mut table = LowerBoundTranspositionTable::new(11);

        table.insert(10, 21);
        table.insert(21, 22);
        assert_eq!(table.get(21), Some(22));
        assert_eq!(table.get(10), None);
    }
}