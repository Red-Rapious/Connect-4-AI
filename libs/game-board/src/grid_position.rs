use crate::{*, sequence_position::SequencePosition, bitboard_position_with_ordering::BitboardPositionWithOrdering};

#[derive(Debug, PartialEq, Clone)]
pub struct GridPosition {
    player_turn: Cell,
    width: usize,
    height: usize,
    grid: Vec<Vec<Cell>>,
    nb_moves: usize
}

impl GridPosition {
    pub fn new(width: usize, height: usize) -> Self {
        assert!(height > 0);

        let grid = (0..height).map(|_| vec![Cell::Empty; width]).collect();

        Self { player_turn: FIRST_PLAYER, width, height, grid, nb_moves: 0 }
    }

    pub fn grid(&self) -> &Vec<Vec<Cell>> {
        &self.grid
    }

    /*fn is_align(&self, line: usize, column: usize, incrementer: (i32, i32)) -> Cell {
        let (i0, i1) = incrementer;
        if self.grid[line][column] == self.grid[(line as i32 + i0) as usize][(column as i32 + i1) as usize] 
        && self.grid[line][column] == self.grid[(line as i32 + 2*i0) as usize][(column as i32 + 2*i1) as usize]
        && self.grid[line][column] == self.grid[(line as i32 + 3*i0) as usize][(column as i32 + 3*i1) as usize]
        {
            self.grid[line][column]
        } else {
            Cell::Empty
        }
    }*/

    /*fn _unplay(&mut self, column: usize) {
        self.nb_moves -= 1;
        self.player_turn = self.player_turn.swap_turn();

        for line in (0..self.height).rev() {
            if self.grid[line][column] != Cell::Empty {
                self.grid[line][column] = Cell::Empty;
                return;
            }
        }
    }*/
}

impl Position for GridPosition {
    fn player_turn(&self) -> Cell {
        self.player_turn
    }

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

    fn play(&mut self, column: usize) {
        assert!(column < self.width);
        assert!(self.can_play(column));
        assert_ne!(self.player_turn, Cell::Empty);

        let mut line = 0;
        while self.grid[line][column] != Cell::Empty {
            line += 1
        }
        self.grid[line][column] = self.player_turn;

        self.nb_moves += 1;
        self.player_turn = self.player_turn.swap_turn();
    }

    /*fn winning(&self) -> Cell {
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
    }*/

    /*fn is_winning_move(&mut self, column: usize, player: Cell) -> bool {
        self.play(column, player);
        let is_winning_move = self.winning();
        self.unplay(column);

        is_winning_move != Cell::Empty
    }*/

    fn is_winning_move(&self, column: usize) -> bool {
        let mut line = 0;
        while self.grid[line][column] != Cell::Empty {
            line += 1;
        }

        // Vertical align: check if the 3 cells below are of the player's color
        if line >= 3
            && self.grid[line-3][column] == self.player_turn
            && self.grid[line-2][column] == self.player_turn
            && self.grid[line-1][column] == self.player_turn {
                return true;
        }

        // Other aligns
        for dy in [-1, 0, 1] {
            let mut nb_nearby = 0;
            for dx in [-1, 1] {
                let mut x = column as i32 + dx;
                let mut y = line as i32 + dx*dy;

                while 
                    0 <= x && x < self.width as i32
                 && 0 <= y && y < self.height as i32
                 && self.grid[y as usize][x as usize] == self.player_turn {
                    x += dx;
                    y += dx*dy;
                    nb_nearby += 1;
                 }
            }

            if nb_nearby >= 3 {
                return true;
            }
        }

        false
    }

    fn nb_moves(&self) -> usize {
        self.nb_moves
    }

    fn from_seq(sequence: &SequencePosition) -> Self {
        Self::from(sequence)
    }

    fn key(&self) -> u64 {
        todo!("key not implemented for GridPosition");
    }

    fn can_win_next(&self) -> bool {
        todo!("can_win_next not implemented for GridPosition");
    }

    fn possible_non_loosing_moves(&self) -> u64 {
        todo!("possible_non_loosing_moves not implemented for GridPosition");
    }

    fn move_score(&self, _move_bit: u64) -> usize {
        todo!("move_score not implemented for GridPosition")
    }

    fn play_move(&mut self, _move_bit: u64) {
        todo!("play_move not implemented for GridPosition")
    }
}

impl From<&SequencePosition> for GridPosition {
    fn from(sequence_position: &SequencePosition) -> Self {
        let mut grid_position = GridPosition::new(7, 6);        
        let mut player = FIRST_PLAYER;

        for column in sequence_position.sequence() {
            grid_position.play(column-1);
            player = player.swap_turn();
        }

        grid_position
    }
}

impl From<&BitboardPositionWithOrdering> for GridPosition {
    fn from(bitboard_position: &BitboardPositionWithOrdering) -> Self {
        let width = bitboard_position.width();
        let height = bitboard_position.height();
        let player_turn = bitboard_position.player_turn();
        let nb_moves = bitboard_position.nb_moves();

        // Initialise empty grid
        let mut grid: Vec<Vec<Cell>> = (0..height)
            .map(|_| vec![Cell::Empty; width])
            .collect();

        let mut bit_index = 0;
        for column in 0..width {
            for line in 0..height {
                let occupied = bitboard_position.mask() & (1 << bit_index) != 0;
                if occupied {
                    let is_current_player = bitboard_position.board() & (1 << bit_index) != 0;
                    match is_current_player {
                        true => grid[line][column] = player_turn,
                        false => grid[line][column] = player_turn.swap_turn()
                    };
                }

                bit_index += 1;
            }
            bit_index += 1; // extra bit on top of each column
        }

        Self { player_turn, width, height, grid, nb_moves }
    }
}

#[cfg(test)]
mod grid_position_tests {
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
                for _ in 0..6 {
                    grid_position.play(column);
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
            grid_position.play(0);

            assert_eq!(grid_position.grid[0][0], FIRST_PLAYER);
        }

        #[test]
        fn grid_position_2() {
            let mut grid_position = GridPosition::new(7, 6);
            grid_position.play(0);
            grid_position.play(0);

            assert_eq!(grid_position.grid[0][0], Cell::Red);
            assert_eq!(grid_position.grid[1][0], Cell::Yellow);
            assert_eq!(grid_position.grid[0][1], Cell::Empty);
        }
    }

    // TODO: test `winning`

    // TODO: test `unplay`

    mod is_winning_move {
        use super::*;

        #[test]
        fn test_vertical() {
            let mut position = GridPosition::new(7, 6);
            for _ in 0..3 {
                assert!(!position.is_winning_move(0));
                position.play(0); // red player play in 0
                position.play(1); // yellow player play in 1
            }
            assert_eq!(position.player_turn, Cell::Red);
            assert!(position.is_winning_move(0));
            assert_eq!(position.nb_moves, 6);
        }

        #[test]
        fn test_horizontal() {
            let mut position = GridPosition::new(7, 6);
            for column in 0..3 {
                assert!(!position.is_winning_move(column));
                // both player play on the same column
                position.play(column);
                position.play(column);
            }
            assert_eq!(position.player_turn, Cell::Red);
            assert!(position.is_winning_move(3));
            assert_eq!(position.nb_moves, 6);
        }
    }

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
            expected_result.play(0);
            expected_result.play(1);
            expected_result.play(2);
            expected_result.play(3);
            expected_result.play(4);
            expected_result.play(5);
            expected_result.play(6);

            assert_eq!(
                GridPosition::from(&SequencePosition::from(&"1234567".to_string())),
                expected_result
            )
        } 
    }

    mod from_bitboard_position {
        use super::*;

        #[test]
        fn bitboard_bottom_left() {
            let mut bitboard_position = BitboardPositionWithOrdering::new(7, 6);
            bitboard_position.play(0);
            bitboard_position.play(0);

            let mut expected_grid_position = GridPosition::new(7, 6);
            expected_grid_position.play(0);
            expected_grid_position.play(0);

            let converted_grid_position = GridPosition::from(&bitboard_position);

            assert_eq!(converted_grid_position, expected_grid_position);
        }

        #[test]
        fn bitboard_full() {
            let mut bitboard_position = BitboardPositionWithOrdering::new(7, 6);
            let mut expected_grid_position = GridPosition::new(7, 6);
            for c in 0..7 {
                for _ in 0..6 {
                    bitboard_position.play(c);
                    expected_grid_position.play(c);
                }
            }

            let converted_grid_position = GridPosition::from(&bitboard_position);

            assert_eq!(converted_grid_position, expected_grid_position);
        }
    }
}