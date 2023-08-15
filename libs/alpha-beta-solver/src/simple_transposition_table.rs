pub const TABLE_SIZE: usize = 10_000_000;

#[derive(Debug)]
pub struct SimpleTranspositionTable {
    table: Vec<TableEntry>
}

impl SimpleTranspositionTable {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        Self { table: vec![TableEntry::default(); size] }
    }

    fn index(&self, key: u64) -> usize {
        key as usize % self.table.len()
    }

    pub fn insert(&mut self, key: u64, val: u8) {
        let index = self.index(key);
        self.table[index] = TableEntry::new(key, val);
    }

    pub fn get(&self, key: u64) -> Option<u8> {
        let index = self.index(key);

        if self.table[index].key() == key {
            self.table[index].val()
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
struct TableEntry {
    key: u64,
    val: Option<u8>
}

impl TableEntry {
    pub fn new(key: u64, val: u8) -> Self {
        Self { key, val: Some(val) }
    }

    pub fn key(&self) -> u64 {
        self.key
    }

    pub fn val(&self) -> Option<u8> {
        self.val
    }
}

impl std::default::Default for TableEntry {
    fn default() -> Self {
        Self { key: 0, val: None }
    }
}

#[cfg(test)]
mod transposition_table_tests {
    use super::*;

    #[test]
    fn insert_get() {
        let mut table = SimpleTranspositionTable::new(10);

        table.insert(42, 21);
        assert_eq!(table.get(42), Some(21));
    }

    #[test]
    fn insert_get_index() {
        let mut table = SimpleTranspositionTable::new(10);

        table.insert(10, 21);
        assert_eq!(table.get(0), None);
    }

    #[test]
    fn index_override() {
        let mut table = SimpleTranspositionTable::new(10);

        table.insert(10, 21);
        table.insert(20, 22);
        assert_eq!(table.get(20), Some(22));
        assert_eq!(table.get(10), None);
    }
}