use crate::{*, sequence_position::SequencePosition};

#[derive(Debug, PartialEq, Clone)]
pub struct BitboardPosition {
    player_turn: Cell,
    width: usize,
    height: usize,
    board: u64,
    mask: u64,
    nb_moves: usize
}

impl BitboardPosition {
    pub fn new(width: usize, height: usize) -> Self {
        assert!(width*(height+1) <= 64, "The board does not fit inside a 64bits bitboard.");
        Self { player_turn: FIRST_PLAYER, width, height, board: 0, mask: 0, nb_moves: 0 }
    }

    fn top_mask(&self, column: usize) -> u64 {
        (1 << (self.height - 1)) << (column * (self.height - 1))
    }

    fn bottom_mask(&self, column: usize) -> u64 {
        1 << (column * (self.height + 1))
    }

    fn column_mask(&self, column: usize) -> u64 {
        ((1 << self.height) - 1) << (column * (self.height + 1))
    }

    fn is_winning_board(&self, board: u64) -> bool {
        // Horizontal 
        let mut m = board & (board >> (self.height+1));
        if m & (m >> (2 * (self.height + 1))) != 0 {
            return true;
        }

        // Vertical
        m = board & (board >> 1);
        if (m & (m >> 2)) != 0 {
            return true;
        }

        // Diagonals
        m = board & (board >> self.height);
        if m & (m >> (2 * self.height)) != 0 {
            return true;
        }

        m = board & (board >> (self.height+2));
        if m & (m >> (2*(self.height+2))) != 0 {
            return true;
        }

        false
    }
}

impl Position for BitboardPosition {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn can_play(&self, column: usize) -> bool {
        self.mask & self.top_mask(column) == 0
    }

    fn play(&mut self, column: usize) {
        self.nb_moves += 1;
        self.player_turn = self.player_turn.swap_turn();

        self.board ^= self.mask;
        self.mask |= self.mask + self.bottom_mask(column);
    }

    fn nb_moves(&self) -> usize {
        self.nb_moves
    }

    fn player_turn(&self) -> Cell {
        self.player_turn
    }

    fn is_winning_move(&self, column: usize) -> bool {
        let mut board_after = self.board;
        board_after |= (self.mask + self.bottom_mask(column)) & self.column_mask(column);
        self.is_winning_board(board_after)
    }

    fn from_seq(sequence: &SequencePosition) -> Self {
        Self::from(sequence)
    }
}

impl From<&SequencePosition> for BitboardPosition {
    fn from(sequence_position: &SequencePosition) -> Self {
        let mut grid_position = BitboardPosition::new(7, 6);        
        let mut player = FIRST_PLAYER;

        for column in sequence_position.sequence() {
            grid_position.play(column-1);
            player = player.swap_turn();
        }

        grid_position
    }
}

#[cfg(test)]
mod bitboard_position_tests {
    use super::*;

    mod can_play {
        use super::*;

        #[test]
        fn grid_position_empty() {
            let position = BitboardPosition::new(7, 6);

            for column in 0..7 {
                assert!(position.can_play(column));
            }
        }

        #[test]
        fn grid_position_full() {
            let mut position = BitboardPosition::new(7, 6);

            for column in 0..7 {
                for _ in 0..6 {
                    position.play(column);
                }
                assert!(!position.can_play(column));
            }
        }
    }

    // TODO: test `play`

    // TODO: test `winning`

    // TODO: test `unplay`

    mod is_winning_move {
        use super::*;

        #[test]
        fn test_vertical() {
            let mut position = BitboardPosition::new(7, 6);
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
            let mut position = BitboardPosition::new(7, 6);
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
            let expected_result = BitboardPosition::new(7, 6);
            assert_eq!(
                BitboardPosition::from(&SequencePosition::from(&"".to_string())),
                expected_result
            )
        } 

        #[test]
        fn sequence_line1() {
            let mut expected_result = BitboardPosition::new(7, 6);
            expected_result.play(0);
            expected_result.play(1);
            expected_result.play(2);
            expected_result.play(3);
            expected_result.play(4);
            expected_result.play(5);
            expected_result.play(6);

            assert_eq!(
                BitboardPosition::from(&SequencePosition::from(&"1234567".to_string())),
                expected_result
            )
        } 
    }
}