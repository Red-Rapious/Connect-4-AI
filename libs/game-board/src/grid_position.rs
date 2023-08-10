use crate::{*, sequence_position::SequencePosition};

#[derive(Debug, PartialEq)]
pub struct GridPosition {
    width: usize,
    height: usize,
    grid: Vec<Vec<Cell>>
}

impl GridPosition {
    pub fn new(width: usize, height: usize) -> Self {
        let grid = (0..height).map(|_| vec![Cell::Empty; width]).collect();

        Self { width, height, grid }
    }

    fn is_align(&self, line: usize, column: usize, incrementer: (i32, i32)) -> Cell {
        let (i0, i1) = incrementer;
        if self.grid[line][column] == self.grid[(line as i32 + i0) as usize][(column as i32 + i1) as usize] 
        && self.grid[line][column] == self.grid[(line as i32 + 2*i0) as usize][(column as i32 + 2*i1) as usize]
        && self.grid[line][column] == self.grid[(line as i32 + 3*i0) as usize][(column as i32 + 3*i1) as usize]
        {
            self.grid[line][column]
        } else {
            Cell::Empty
        }
    }
}

impl Position for GridPosition {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn can_play(&self, column: usize) -> bool {
        assert!(column < self.width);

        self.grid[self.height-1][column] == Cell::Empty
    }

    fn play(&mut self, column: usize, player: Cell) {
        assert!(column < self.width);
        assert!(self.can_play(column));
        assert_ne!(player, Cell::Empty);

        let mut line = 0;
        while self.grid[line][column] != Cell::Empty {
            line += 1
        }
        self.grid[line][column] = player;
    }

    fn winning(&self) -> Cell {
        // Horizontal
        for line in 0..self.height {
            for column in 0..self.width-3 {
                let align = self.is_align(line, column, (0, 1));
                if align != Cell::Empty { return align; }
            }
        }

        // Vertical
        for line in 0..self.height-3 {
            for column in 0..self.width {
                let align = self.is_align(line, column, (1, 0));
                if align != Cell::Empty { return align; }
            }
        }

        // Diagonals
        for line in 0..self.height-3 {
            for column in 0..self.width-3 {
                let align = self.is_align(line, column, (1, 1));
                if align != Cell::Empty { return align; }
            }
        }
        for line in 0..self.height-3 {
            for column in 3..self.width {
                let align = self.is_align(line, column, (1, -1));
                if align != Cell::Empty { return align; }
            }
        }

        Cell::Empty
    }
}

impl From<&SequencePosition> for GridPosition {
    fn from(sequence_position: &SequencePosition) -> Self {
        let mut grid_position = GridPosition::new(7, 6);
        let mut player = Cell::Red;

        for column in sequence_position.sequence() {
            grid_position.play(column-1, player);
            player = match player {
                Cell::Red => Cell::Yellow,
                Cell::Yellow => Cell::Red,
                Cell::Empty => panic!()
            };
        }

        grid_position
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod can_play {
        use super::*;

        #[test]
        fn grid_position_empty() {
            let grid_position = GridPosition::new(7, 6);

            for column in 0..7 {
                assert!(grid_position.can_play(column));
            }
        }

        #[test]
        fn grid_position_single() {
            let mut grid_position = GridPosition::new(7, 6);
            grid_position.grid[5][0] = Cell::Red;

            assert!(!grid_position.can_play(0));
        }

        #[test]
        fn grid_position_full() {
            let mut grid_position = GridPosition::new(7, 6);

            for column in 0..7 {
                for count in 0..6 {
                    dbg!(column);
                    dbg!(count);
                    grid_position.play(column, Cell::Red);
                }
                assert!(!grid_position.can_play(column));
            }
        }
    }

    mod play {
        use super::*;

        #[test]
        fn grid_position_1() {
            let mut grid_position = GridPosition::new(7, 6);
            grid_position.play(0, Cell::Red);

            assert_eq!(grid_position.grid[0][0], Cell::Red);
        }

        #[test]
        fn grid_position_2() {
            let mut grid_position = GridPosition::new(7, 6);
            grid_position.play(0, Cell::Red);
            grid_position.play(0, Cell::Yellow);

            assert_eq!(grid_position.grid[0][0], Cell::Red);
            assert_eq!(grid_position.grid[1][0], Cell::Yellow);
            assert_eq!(grid_position.grid[0][1], Cell::Empty);
        }
    }

    // TODO: test `winning`

    mod from_sequence_position {
        use super::*;

        #[test]
        fn sequence_empty() {
            let expected_result = GridPosition::new(7, 6);
            assert_eq!(
                GridPosition::from(&SequencePosition::from(&"".to_string())),
                expected_result
            )
        } 

        #[test]
        fn sequence_line1() {
            let mut expected_result = GridPosition::new(7, 6);
            expected_result.play(0, Cell::Red);
            expected_result.play(2, Cell::Red);
            expected_result.play(4, Cell::Red);
            expected_result.play(6, Cell::Red);
            expected_result.play(1, Cell::Yellow);
            expected_result.play(3, Cell::Yellow);
            expected_result.play(5, Cell::Yellow);

            assert_eq!(
                GridPosition::from(&SequencePosition::from(&"1234567".to_string())),
                expected_result
            )
        } 
    }
}