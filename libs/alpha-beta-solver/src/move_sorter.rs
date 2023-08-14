pub struct MoveSorter {
    size: usize,
    entries: Vec<(u64, usize)>
}

impl MoveSorter {
    pub fn new(width: usize) -> Self {
        Self {
            size: 0,
            entries: vec![(0, 0); width]
        }
    }

    pub fn add(&mut self, move_bit: u64, score: usize) {
        let mut index = self.size;
        
        while index != 0 && self.entries[index-1].1 > score {
            self.entries[index] = self.entries[index-1];
            index -= 1;
        }
        self.entries[index].0 = move_bit;
        self.entries[index].1 = score;

        self.size += 1;
    }

    pub fn get_next(&mut self) -> u64 {
        if self.size > 0 {
            self.size -= 1;
            self.entries[self.size].0
        } else {
            0
        }
    }
}

#[cfg(test)]
mod move_sorter_test {
    use super::*;

    #[test]
    fn in_order() {
        let mut move_sorter = MoveSorter::new(7);

        for i in 0..7 {
            move_sorter.add(i as u64, i)
        }
        for i in 0..7 {
            assert_eq!(move_sorter.get_next(), 7-1-i as u64);
        }
    }

    #[test]
    fn reverse_order() {
        let mut move_sorter = MoveSorter::new(7);

        for i in (0..7).rev() {
            move_sorter.add(i as u64, i)
        }
        for i in 0..7 {
            assert_eq!(move_sorter.get_next(), 7-1-i as u64);
        }
    }

    #[test]
    fn different_moves_and_scores() {
        let mut move_sorter = MoveSorter::new(7);

        for i in (0..7).rev() {
            move_sorter.add(i as u64, 7-1-i)
        }
        for i in 0..7 {
            assert_eq!(move_sorter.get_next(), i as u64);
        }
    }
}