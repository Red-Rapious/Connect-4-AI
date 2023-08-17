use std::collections::HashMap;

use crate::lower_bound_transposition_table::{LowerBoundTranspositionTable, TABLE_SIZE};
use crate::move_sorter::MoveSorter;
use crate::opening_book::OpeningBook;

pub struct FinalAlphaBeta {
    move_order: Vec<usize>,
    explored_positions: usize,
    transposition_table: LowerBoundTranspositionTable,
    move_table: HashMap<u32, Option<u8>>,
    opening_book: OpeningBook
}

impl FinalAlphaBeta {
    pub fn new(width: usize, height: usize, move_order: Vec<usize>) -> Self {
        Self { 
            move_order, 
            explored_positions: 0, 
            transposition_table: LowerBoundTranspositionTable::new(TABLE_SIZE),
            move_table: HashMap::with_capacity(TABLE_SIZE),
            opening_book: OpeningBook::new(width, height)
        }
    }

    pub fn load_opening_book(&mut self, book_path: &str) {
        self.opening_book.load(book_path);
    }

    fn solve_range(&mut self, position: &(impl lib_game_board::Position + Clone), mut alpha: i32, mut beta: i32) -> (i32, Option<u8>) {
        self.explored_positions += 1;

        // Anticipate loosing move
        let next = position.possible_non_loosing_moves();
        if next == 0 {
            // if no possible non losing move, opponent wins next move
            return (
                -((position.width()*position.height() - position.nb_moves()) as i32)/2,
                None
            );
        }

        // Check for a draw game
        if position.nb_moves() >= position.width() * position.height() - 2 {
            return (0, None);
        }

        let mut min = - ((position.width()*position.height() - 2 - position.nb_moves()) as i32)/2;
        if alpha < min {
            alpha = min; // there is no need to keep alpha below our min possible score.
            if alpha >= beta {
                return (alpha, None);
            } 
        }

        let mut max = ((position.width()*position.height() - 1 - position.nb_moves()) as i32)/2;
        if beta > max {
            beta = max; // there is no need to keep beta above our max possible score.
            if alpha >= beta {
                return (beta, None);
            } 
        }

        let position_min_score = -((position.width()*position.height()) as i32)/2 + 3;
        let position_max_score = (position.width()*position.height()+1) as i32/2 - 3;

        // Compare lower and upper bound to the transposition table content
        if let Some(val) = self.transposition_table.get(position.key()) {
            if val > (position_max_score - position_min_score + 1) as u16 { // we have a lower bound
                min = val as i32 + 2*position_min_score - position_max_score - 2;
                if alpha < min {
                    alpha = min;
                    if alpha >= beta {
                        return (alpha, *self.move_table.get(&(position.key() as u32)).unwrap());
                    }
                }
            } else { // we have an upper bound
                max = val as i32 + position_min_score - 1;
                if beta > max {
                    beta = max;
                    if alpha >= beta { 
                        return (beta, *self.move_table.get(&(position.key() as u32)).unwrap());
                    }  
                }
            }
        };

        if let Some(val) = self.opening_book.get(position) {
            return (val as i32 + position_min_score - 1, None); // TODO: retrieve move from opening book
        }

        // Hash Map to retrieve the column from a given move bitboard
        let mut move_to_column_map = HashMap::with_capacity(position.width());
        // Sort the moves by score
        let mut move_sorter = MoveSorter::new(position.width());
        for column in self.move_order.clone().iter().rev() {
            let column_mask = ((1 << position.height()) - 1) << (column * (position.height() + 1));
            let move_bit = next & column_mask;
            if move_bit != 0 {
                move_sorter.add(move_bit, position.move_score(move_bit));
                move_to_column_map.insert(move_bit, *column as u8);
            }
        }
        

        // For each move, apply the basic negamax principle
        let mut best_move: Option<u8> = None;
        loop {
            let next = move_sorter.get_next();
            if next == 0 { break; }

            let mut position2 = position.clone();
            position2.play_move(next);

            let (score, _) = self.solve_range(&mut position2, -beta, -alpha);
            let score = -score;
            
            if score >= beta {
                self.transposition_table.insert(position.key(), (score + position_max_score - 2*position_min_score + 2) as u16);
                self.move_table.insert(position.key() as u32, Some(*move_to_column_map.get(&next).unwrap()));
                return (score, Some(*move_to_column_map.get(&next).unwrap()));
            }
            if score > alpha {
                alpha = score;
                best_move = Some(*move_to_column_map.get(&next).unwrap());
            }
        }

        self.transposition_table.insert(position.key(), (alpha - position_min_score + 1) as u16);
        self.move_table.insert(position.key() as u32, best_move);
        (alpha, best_move)
    }

    pub fn solve(&mut self, position: &(impl lib_game_board::Position + Clone)) -> (i32, usize) {
        if position.can_win_next() {
            for column in 0..position.width() {
                if position.is_winning_move(column) {
                    return (((position.width()*position.height() + 1 - position.nb_moves()) as i32)/2, column);
                }
            }
        }

        let mut min = - ((position.width()*position.height() - position.nb_moves()) as i32)/2;
        let mut max = ((position.width()*position.height() + 1 - position.nb_moves()) as i32)/2;
        let mut best_move = None;

        while min < max {
            let mut med = min + (max - min)/2;
            
            if med <= 0 && min/2 < med { med = min/2; }
            else if med >= 0 && max/2 > med { med = max/2; }

            let (r, column_played) = self.solve_range(position, med, med+1);
            best_move = match column_played {
                None => best_move,
                Some(val) => Some(val as usize)
            };
            if r <= med {
                max = r;
            } else {
                min = r;
            }
        }

        if let Some(column) = best_move {
            return (min, column);
        } else {
            println!("[WARNING] `best_move` is `None`: a random move was instead replaced.");
            for column in 0..position.width() {
                if position.can_play(column) {
                    return (min, column);
                }
            }
            panic!("Impossible to play anywhere. The game should trigger a draw.")
        }
    }

    pub fn explored_positions(&self) -> usize {
        self.explored_positions
    }

    pub fn reset_explored_positions(&mut self) {
        self.explored_positions = 0;
        self.transposition_table = LowerBoundTranspositionTable::new(TABLE_SIZE);
    }

    pub fn weak_solve(&mut self, position: &(impl lib_game_board::Position + Clone)) -> (i32, Option<usize>) {
        let mut min = -1;
        let mut max = 1;
        let mut best_move = None;

        while min < max {
            let mut med = min + (max - min)/2;
            
            if med <= 0 && min/2 < med { med = min/2; }
            else if med >= 0 && max/2 > med { med = max/2; }

            let (r, column_played) = self.solve_range(position, med, med+1);
            best_move = match column_played {
                None => best_move,
                Some(val) => Some(val as usize)
            };

            if r <= med {
                max = r;
            } else {
                min = r;
            }
        }

        (min, best_move)
    }
}

#[cfg(test)]
mod final_alpha_beta_tests {
    use super::*;
    use lib_game_board::bitboard_position_with_ordering::BitboardPositionWithOrdering;
    use lib_game_board::sequence_position::SequencePosition;

    #[test]
    fn bitboard_correctness() {
        let mut solver = FinalAlphaBeta::new(7, 6, (0..7).collect());

        assert_eq!(solver.solve(
        &mut BitboardPositionWithOrdering::from(
                    &SequencePosition::from(
                        &"2252576253462244111563365343671351441".to_string()
                    ))).0,
                -1);
    }

    #[test]
    fn load_small_opening_book() {
        let mut solver = FinalAlphaBeta::new(7, 6, (0..7).collect());
        solver.load_opening_book("./opening-books/7x6_small.book");

        assert_eq!(solver.solve(
        &mut BitboardPositionWithOrdering::from(
                    &SequencePosition::from(
                        &"2252576253462244111563365343671351441".to_string()
                    ))).0,
                -1);
    }

    #[test]
    #[ignore]
    fn load_large_opening_book() {
        let mut solver = FinalAlphaBeta::new(7, 6, (0..7).collect());
        solver.load_opening_book("./opening-books/7x6.book");

        assert_eq!(solver.solve(
        &mut BitboardPositionWithOrdering::from(
                    &SequencePosition::from(
                        &"2252576253462244111563365343671351441".to_string()
                    ))).0,
                -1);
    }
}