use lib_game_board::{Solver, WeakSolver};
use crate::simple_transposition_table::{SimpleTranspositionTable, TABLE_SIZE};

pub struct AlphaBetaWithIterativeDeepening {
    move_order: Vec<usize>,
    explored_positions: usize,
    transposition_table: SimpleTranspositionTable
}

impl AlphaBetaWithIterativeDeepening {
    pub fn new(move_order: Vec<usize>) -> Self {
        Self { move_order, explored_positions: 0, transposition_table: SimpleTranspositionTable::new(TABLE_SIZE) }
    }

    fn solve_range(&mut self, position: &(impl lib_game_board::Position + Clone), mut alpha: i32, mut beta: i32) -> i32 {
        self.explored_positions += 1;
        // Draw
        if position.nb_moves() == position.width() * position.height() {
            return 0;
        }

        // Next move is winnable
        for column in 0..position.width() {
            if position.can_play(column) && position.is_winning_move(column) {
                return ((position.width()*position.height() + 1 - position.nb_moves()) / 2) as i32;
            }
        }

        let position_min_score = - ((position.width()*position.height()) as i32) /2 + 3;
        let max_score = match self.transposition_table.get(position.key()) {
            None => ((position.width()*position.height() - 1 - position.nb_moves()) / 2) as i32,
            Some(val) => val as i32 + position_min_score // val + min_score
        };

        if beta > max_score {
            beta = max_score;

            if alpha >= beta {
                return beta;
            }
        }

        // For each possible move
        for column in self.move_order.clone().iter() {
            if position.can_play(*column) {
                let mut position2 = position.clone();
                position2.play(*column);

                let score = - self.solve_range(&mut position2, -beta, -alpha);
                
                if score >= beta {
                    return score;
                }
                if score > alpha {
                    alpha = score;
                }
            }
        }

        self.transposition_table.insert(position.key(), (alpha - position_min_score + 1) as u8);
        alpha
    }
}

impl Solver for AlphaBetaWithIterativeDeepening {
    fn solve(&mut self, position: &(impl lib_game_board::Position + Clone)) -> i32 {
        let mut min = - ((position.width()*position.height()) as i32 - position.nb_moves() as i32)/2;
        let mut max = ((position.width()*position.height() + 1) as i32 - position.nb_moves() as i32)/2;

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

impl WeakSolver for AlphaBetaWithIterativeDeepening {
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
mod alpha_beta_with_iterative_deepening_tests {
    use super::*;
    use lib_game_board::bitboard_position::BitboardPosition;
    use lib_game_board::sequence_position::SequencePosition;

    #[test]
    fn bitboard_correctness() {
        let mut alpha_beta_with_transposition = AlphaBetaWithIterativeDeepening::new((0..7).collect());

        assert_eq!(alpha_beta_with_transposition.solve(
        &mut BitboardPosition::from(
                    &SequencePosition::from(
                        &"2252576253462244111563365343671351441".to_string()
                    ))),
                -1);
    }
}