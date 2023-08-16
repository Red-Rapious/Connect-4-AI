use lib_game_board::Position;
use crate::lower_bound_transposition_table::{LowerBoundTranspositionTable, TABLE_SIZE};

use std::io::Read;
use std::io::BufReader;
use std::fs::File;

pub struct OpeningBook {
    width: usize,
    height: usize,
    depth: usize,
    transposition_table: LowerBoundTranspositionTable
}

impl OpeningBook {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            depth: 0,
            transposition_table: LowerBoundTranspositionTable::new(TABLE_SIZE)
        }
    }

    pub fn load(&mut self, filename: &str) {
        let file = File::open(&filename).expect(format!("Unable to read file: {}", filename).as_str());
        let mut reader = BufReader::new(&file);

        let mut width = [0u8; 1];
        reader.read_exact(&mut width).unwrap();
        assert_eq!(width[0] as usize, self.width);

        let mut height = [0u8; 1];
        reader.read_exact(&mut height).unwrap();
        assert_eq!(height[0] as usize, self.height);

        let mut max_depth = [0u8; 1];
        reader.read_exact(&mut max_depth).unwrap();
        self.depth = max_depth[0] as usize;

        let mut key_size = [0u8; 1];
        reader.read_exact(&mut key_size).unwrap();
        let key_size = key_size[0] as usize;
        assert!(key_size <= 32); // Keys are stored as u32 in LowerBoundTranspositionTable

        let mut value_size = [0u8; 1];
        reader.read_exact(&mut value_size).unwrap();
        let value_size = value_size[0] as usize;
        assert!(value_size <= 8); // Values are stored as u8 in LowerBoundTranspositionTable

        let mut log_size = [0u8; 1];
        reader.read_exact(&mut log_size).unwrap();
        //let log_size = log_size[0];

        let number_values = ((file.metadata().unwrap().len() - 6) / (key_size + value_size) as u64) as usize;

        let mut keys = Vec::with_capacity(number_values);
        for _ in 0..number_values {
            let mut key = vec![0u8; key_size];
            reader.read_exact(&mut key).unwrap();
            
            let key = match key_size {
                1 => u8::from_le_bytes(key.try_into().unwrap()) as u64,
                2 => u16::from_le_bytes(key.try_into().unwrap()) as u64,
                _ => {
                    for _ in 0..(64/8-key_size) {
                        key.push(0u8);
                    }
                    u64::from_le_bytes(key.try_into().unwrap())
                }
            };

            keys.push(key);
        }
        
        let mut values = Vec::with_capacity(number_values);
        for _ in 0..number_values {
            let mut value = vec![0u8; value_size];
            reader.read_exact(&mut value).unwrap();

            let value = match value_size {
                1 => u8::from_le_bytes(value.try_into().unwrap()) as u16,
                _ => {
                    for _ in 0..(64/8-value_size) {
                        value.push(0u8);
                    }
                    u16::from_le_bytes(value.try_into().unwrap())
                }
            };

            values.push(value);
        }

        for i in 0..number_values {
            if i == 0 {
                dbg!(keys[i], values[i]);
            }
            self.transposition_table.insert(keys[i], values[i]);
        }
    }

    pub fn get(&self, position: &impl Position) -> Option<u16> {
        if position.nb_moves() > self.depth {
            None
        } else {
            self.transposition_table.get(position.key())
        }
    }
}

#[cfg(test)]
mod opening_book_tests {
    use super::*;

    mod loading {
        use super::*;

        #[test]
        fn load_small() {
            let mut book = OpeningBook::new(7, 6);
            book.load("./opening-books/7x6_small.book");
        }

        #[test]
        //#[ignore]
        fn load_large() {
            let mut book = OpeningBook::new(7, 6);
            book.load("./opening-books/7x6.book");
        }
    }

    mod informations {
        use super::*;

        #[test]
        fn info_small() {
            let mut book = OpeningBook::new(7, 6);
            book.load("./opening-books/7x6_small.book");

            assert_eq!(book.width, 7);
            assert_eq!(book.height, 6);
            assert_eq!(book.depth, 16);
        }

        #[test]
        #[ignore]
        fn info_large() {
            let mut book = OpeningBook::new(7, 6);
            book.load("./opening-books/7x6.book");

            assert_eq!(book.width, 7);
            assert_eq!(book.height, 6);
            assert_eq!(book.depth, 14);
        }
    }
}