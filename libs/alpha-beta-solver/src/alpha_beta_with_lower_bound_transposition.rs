use lib_game_board::{Solver, WeakSolver};
use crate::lower_bound_transposition_table::{LowerBoundTranspositionTable, TABLE_SIZE};
use crate::move_sorter::MoveSorter;

pub struct AlphaBetaWithLowerBoundTransposition {
    move_order: Vec<usize>,
    explored_positions: usize,
    transposition_table: LowerBoundTranspositionTable
}

impl AlphaBetaWithLowerBoundTransposition {
    pub fn new(move_order: Vec<usize>) -> Self {
        Self { move_order, explored_positions: 0, transposition_table: LowerBoundTranspositionTable::new(TABLE_SIZE) }
    }

    fn solve_range(&mut self, position: &(impl lib_game_board::Position + Clone), mut alpha: i32, mut beta: i32) -> i32 {
        self.explored_positions += 1;

        // Anticipate loosing move
        let next = position.possible_non_loosing_moves();
        if next == 0 {
            // if no possible non losing move, opponent wins next move
            return -((position.width()*position.height() - position.nb_moves()) as i32)/2;
        }

        // Check for a draw game
        if position.nb_moves() >= position.width() * position.height() - 2 {
            return 0;
        }

        let mut min = - ((position.width()*position.height() - 2 - position.nb_moves()) as i32)/2;
        if alpha < min {
            alpha = min; // there is no need to keep alpha below our min possible score.
            if alpha >= beta {
                return alpha;
            } 
        }

        let mut max = ((position.width()*position.height() - 1 - position.nb_moves()) as i32)/2;
        if beta > max {
            beta = max; // there is no need to keep beta above our max possible score.
            if alpha >= beta {
                return beta;
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
                        return alpha;
                    }
                }
            } else { // we have an upper bound
                max = val as i32 + position_min_score - 1;
                if beta > max {
                    beta = max;
                    if alpha >= beta { 
                        return beta;
                    }  
                }
            }
        };

        // Sort the moves by score
        let mut move_sorter = MoveSorter::new(position.width());
        for column in self.move_order.clone().iter().rev() {
            let column_mask = ((1 << position.height()) - 1) << (column * (position.height() + 1));
            let move_bit = next & column_mask;
            if move_bit != 0 {
                move_sorter.add(move_bit, position.move_score(move_bit));
            }
        }
        

        // For each move, apply the basic negamax principle
        loop {
            let next = move_sorter.get_next();
            if next == 0 { break; }

            let mut position2 = position.clone();
            position2.play_move(next);

            let score = - self.solve_range(&mut position2, -beta, -alpha);
            
            if score >= beta {
                self.transposition_table.insert(position.key(), (score + position_max_score - 2*position_min_score + 2) as u16);
                return score;
            }
            if score > alpha {
                alpha = score;
            }
        }

        self.transposition_table.insert(position.key(), (alpha - position_min_score + 1) as u16);
        alpha
    }
}

impl Solver for AlphaBetaWithLowerBoundTransposition {
    fn solve(&mut self, position: &(impl lib_game_board::Position + Clone)) -> i32 {
        let mut min = - ((position.width()*position.height() - position.nb_moves()) as i32)/2;
        let mut max = ((position.width()*position.height() + 1 - position.nb_moves()) as i32)/2;

        while min < max {
            let mut med = min + (max - min)/2;
            
            if med <= 0 && min/2 < med { med = min/2; }
            else if med >= 0 && max/2 > med { med = max/2; }

            let r = self.solve_range(position, med, med+1);
            if r <= med {
                max = r;
            } else {
                min = r;
            }
        }

        min
    }

    fn explored_positions(&self) -> usize {
        self.explored_positions
    }

    fn reset_explored_positions(&mut self) {
        self.explored_positions = 0;
        self.transposition_table = LowerBoundTranspositionTable::new(TABLE_SIZE);
    }
}

impl WeakSolver for AlphaBetaWithLowerBoundTransposition {
    fn weak_solve(&mut self, position: &(impl lib_game_board::Position + Clone)) -> i32 {
        let mut min = -1;
        let mut max = 1;

        while min < max {
            let mut med = min + (max - min)/2;
            
            if med <= 0 && min/2 < med { med = min/2; }
            else if med >= 0 && max/2 > med { med = max/2; }

            let r = self.solve_range(position, med, med+1);
            if r <= med {
                max = r;
            } else {
                min = r;
            }
        }

        min
    }

    fn explored_positions(&self) -> usize {
        self.explored_positions
    }

    fn reset_explored_positions(&mut self) {
        self.explored_positions = 0;
    }
}

#[cfg(test)]
mod alpha_beta_with_lower_bound_transposition_tests {
    use super::*;
    use lib_game_board::bitboard_position_with_ordering::BitboardPositionWithOrdering;
    use lib_game_board::sequence_position::SequencePosition;

    #[test]
    fn bitboard_correctness() {
        let mut solver = AlphaBetaWithLowerBoundTransposition::new((0..7).collect());

        assert_eq!(solver.solve(
        &mut BitboardPositionWithOrdering::from(
                    &SequencePosition::from(
                        &"2252576253462244111563365343671351441".to_string()
                    ))),
                -1);
    }
}