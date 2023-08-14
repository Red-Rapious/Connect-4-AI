use crate::{*, sequence_position::SequencePosition};

const FIRST_PLAYER: Cell = Cell::Red;

#[derive(Debug, PartialEq, Clone)]
pub struct StackPosition {
    player_turn: Cell,
    width: usize,
    height: usize,
    stacks: Vec<Vec<Cell>>,
    nb_moves: usize
}

impl StackPosition {
    pub fn new(width: usize, height: usize) -> Self {
        let stacks = (0..width).map(|_| vec![]).collect();

        Self { player_turn: FIRST_PLAYER, width, height, stacks, nb_moves: 0 }
    }
}

impl Position for StackPosition {
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

        self.stacks[column].len() < self.height
    }

    fn play(&mut self, column: usize) {
        assert!(column < self.width);
        assert_ne!(self.player_turn, Cell::Empty);
        
        assert!(self.can_play(column));
        self.stacks[column].push(self.player_turn);

        self.nb_moves += 1;
        self.player_turn = self.player_turn.swap_turn();
    }

    fn is_winning_move(&self, column: usize) -> bool {
        let line = self.stacks[column].len();

        // Vertical align: check if the 3 cells below are of the player's color
        if line >= 3
            && self.stacks[column][line-3] == self.player_turn
            && self.stacks[column][line-2] == self.player_turn
            && self.stacks[column][line-1] == self.player_turn {
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
                 && 0 <= y && y < self.stacks[x as usize].len() as i32
                 && self.stacks[x as usize][y as usize] == self.player_turn {
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
        todo!("key not implemented for StackPosition");
    }

    fn can_win_next(&self) -> bool {
        todo!("can_win_next not implemented for StackPosition");
    }

    fn possible_non_loosing_moves(&self) -> u64 {
        todo!("possible_non_loosing_moves not implemented for StackPosition");
    }

    fn move_score(&self, _move_bit: u64) -> usize {
        todo!("move_score not implemented for StackPosition")
    }

    fn play_move(&mut self, _move_bit: u64) {
        todo!("play_move not implemented for StackPosition")
    }
}

impl From<&SequencePosition> for StackPosition {
    fn from(sequence_position: &SequencePosition) -> Self {
        let mut grid_position = StackPosition::new(7, 6);        
        let mut player = FIRST_PLAYER;

        for column in sequence_position.sequence() {
            grid_position.play(column-1);
            player = player.swap_turn();
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
            let position = StackPosition::new(7, 6);

            for column in 0..7 {
                assert!(position.can_play(column));
            }
        }

        #[test]
        fn grid_position_full() {
            let mut position = StackPosition::new(7, 6);

            for column in 0..7 {
                for _ in 0..6 {
                    position.play(column);
                }
                assert!(!position.can_play(column));
            }
        }
    }

    mod play {
        use super::*;

        #[test]
        fn grid_position_1() {
            let mut position = StackPosition::new(7, 6);
            position.play(0);

            assert_eq!(position.stacks[0][0], FIRST_PLAYER);
        }

        #[test]
        fn grid_position_2() {
            let mut position = StackPosition::new(7, 6);
            position.play(0);
            position.play(0);

            assert_eq!(position.stacks[0][0], Cell::Red);
            assert_eq!(position.stacks[0][1], Cell::Yellow);
            assert_eq!(position.stacks[1].len(), 0);
        }
    }

    // TODO: test `winning`

    // TODO: test `unplay`

    mod is_winning_move {
        use super::*;

        #[test]
        fn test_vertical() {
            let mut position = StackPosition::new(7, 6);
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
            let mut position = StackPosition::new(7, 6);
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
            let expected_result = StackPosition::new(7, 6);
            assert_eq!(
                StackPosition::from(&SequencePosition::from(&"".to_string())),
                expected_result
            )
        } 

        #[test]
        fn sequence_line1() {
            let mut expected_result = StackPosition::new(7, 6);
            expected_result.play(0);
            expected_result.play(1);
            expected_result.play(2);
            expected_result.play(3);
            expected_result.play(4);
            expected_result.play(5);
            expected_result.play(6);

            assert_eq!(
                StackPosition::from(&SequencePosition::from(&"1234567".to_string())),
                expected_result
            )
        } 
    }
}