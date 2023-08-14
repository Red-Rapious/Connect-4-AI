use crate::{*, sequence_position::SequencePosition};

#[derive(Debug, PartialEq, Clone)]
pub struct BitboardPositionWithOrdering {
    player_turn: Cell,
    width: usize,
    height: usize,
    board: u64,
    mask: u64,
    nb_moves: usize,
    bottom_mask: u64,
    board_mask: u64
}

impl BitboardPositionWithOrdering {
    pub fn new(width: usize, height: usize) -> Self {
        assert!(width*(height+1) <= 64, "The board does not fit inside a 64bits bitboard.");
        let bottom_mask = BitboardPositionWithOrdering::bottom(width, height);
        Self { 
            player_turn: FIRST_PLAYER, 
            width, height, 
            board: 0, mask: 0, nb_moves: 0, 
            bottom_mask,
            board_mask: bottom_mask * ((1 << height)-1)
        }
    }

    fn top_mask_col(&self, column: usize) -> u64 {
        (1 << (self.height - 1)) << (column * (self.height + 1))
    }

    fn bottom_mask_col(&self, column: usize) -> u64 {
        1 << (column * (self.height + 1))
    }

    fn bottom(width: usize, height: usize) -> u64 {
        if width == 0 {
            0
        } else {
            BitboardPositionWithOrdering::bottom(width-1, height) | 1 << (width-1)*(height+1)
        }
    }

    fn column_mask(&self, column: usize) -> u64 {
        ((1 << self.height) - 1) << (column * (self.height + 1))
    }

    fn winning_positions(&self) -> u64 {
        self.compute_winning_positions(self.board, self.mask)
    }

    fn opponent_winning_positions(&self) -> u64 {
        self.compute_winning_positions(self.board ^ self.mask, self.mask)
    }
    
    fn possible_positions(&self) -> u64 {
        (self.mask + self.bottom_mask) & self.board_mask
    }


    fn compute_winning_positions(&self, board: u64, mask: u64) -> u64 {
        // Vertical
        let mut r = (board << 1) & (board << 2) & (board << 3);

        // Horizontal
        let mut p = (board << (self.height+1)) & (board << 2*(self.height+1));
        r |= p & (board << 3*(self.height+1));
        r |= p & (board >> (self.height+1));
        p = (board >> (self.height+1)) & (board >> 2*(self.height+1));
        r |= p & (board << (self.height+1));
        r |= p & (board >> 3*(self.height+1));

        //diagonal 1
        p = (board << self.height) & (board << 2*self.height);
        r |= p & (board << 3*self.height);
        r |= p & (board >> self.height);
        p = (board >> self.height) & (board >> 2*self.height);
        r |= p & (board << self.height);
        r |= p & (board >> 3*self.height);

        //diagonal 2
        p = (board << (self.height+2)) & (board << 2*(self.height+2));
        r |= p & (board << 3*(self.height+2));
        r |= p & (board >> (self.height+2));
        p = (board >> (self.height+2)) & (board >> 2*(self.height+2));
        r |= p & (board << (self.height+2));
        r |= p & (board >> 3*(self.height+2));

        r & (self.board_mask ^ mask)
    }

    fn population_count(&self, mut m: u64) -> usize {
        let mut c = 0;
        while m != 0 {
            m &= m - 1;
            c += 1;
        }
        c
    }
}

impl Position for BitboardPositionWithOrdering {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn can_play(&self, column: usize) -> bool {
        self.mask & self.top_mask_col(column) == 0
    }

    fn play(&mut self, column: usize) {
        self.nb_moves += 1;
        self.player_turn = self.player_turn.swap_turn();

        self.board ^= self.mask;
        self.mask |= self.mask + self.bottom_mask_col(column);
    }

    fn nb_moves(&self) -> usize {
        self.nb_moves
    }

    fn player_turn(&self) -> Cell {
        self.player_turn
    }

    fn is_winning_move(&self, column: usize) -> bool {
        self.winning_positions() & self.possible_positions() & self.column_mask(column) != 0
    }

    fn from_seq(sequence: &SequencePosition) -> Self {
        Self::from(sequence)
    }

    fn key(&self) -> u64 {
        self.board + self.mask
    }

    fn can_win_next(&self) -> bool {
        self.winning_positions() & self.possible_positions() != 0
    }

    fn possible_non_loosing_moves(&self) -> u64 {
        assert!(!self.can_win_next());

        let mut possible_mask = self.possible_positions();
        let opponent_win = self.opponent_winning_positions();
        let forced_moves = possible_mask & opponent_win;

        if forced_moves != 0 {
            if forced_moves & (forced_moves - 1) != 0 {
                return 0; // unable to win since the opponent has two winning moves
            } else {
                possible_mask = forced_moves; // forced to play the forced move
            }
        }

        possible_mask & !(opponent_win >> 1)
    }

    fn move_score(&self, move_bit: u64) -> usize {
        self.population_count(self.compute_winning_positions(self.board | move_bit, self.mask))
    }

    fn play_move(&mut self, move_bit: u64) {
        self.nb_moves += 1;
        self.player_turn = self.player_turn.swap_turn();
        
        self.board ^= self.mask;
        self.mask |= move_bit;
    }
}

impl From<&SequencePosition> for BitboardPositionWithOrdering {
    fn from(sequence_position: &SequencePosition) -> Self {
        let mut grid_position = BitboardPositionWithOrdering::new(7, 6);        
        let mut player = FIRST_PLAYER;

        for column in sequence_position.sequence() {
            grid_position.play(column-1);
            player = player.swap_turn();
        }

        grid_position
    }
}

#[cfg(test)]
mod bitboard_position_with_ordering_tests {
    use super::*;

    mod can_play {
        use super::*;

        #[test]
        fn empty_position() {
            let position = BitboardPositionWithOrdering::new(7, 6);

            for column in 0..7 {
                assert!(position.can_play(column), "Cannot play in column {} while one should be able to.", column);
            }
        }

        #[test]
        fn full_position() {
            let mut position = BitboardPositionWithOrdering::new(7, 6);

            for column in 0..7 {
                for _ in 0..6 {
                    position.play(column);
                }
                assert!(!position.can_play(column), "Can play in column {} while one should not be able to.", column);
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
            let mut position = BitboardPositionWithOrdering::new(7, 6);
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
            let mut position = BitboardPositionWithOrdering::new(7, 6);
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
            let expected_result = BitboardPositionWithOrdering::new(7, 6);
            assert_eq!(
                BitboardPositionWithOrdering::from(&SequencePosition::from(&"".to_string())),
                expected_result
            )
        } 

        #[test]
        fn sequence_line1() {
            let mut expected_result = BitboardPositionWithOrdering::new(7, 6);
            expected_result.play(0);
            expected_result.play(1);
            expected_result.play(2);
            expected_result.play(3);
            expected_result.play(4);
            expected_result.play(5);
            expected_result.play(6);

            assert_eq!(
                BitboardPositionWithOrdering::from(&SequencePosition::from(&"1234567".to_string())),
                expected_result
            )
        } 
    }
}