use lib_game_board::{Solver, WeakSolver};

pub struct AlphaBetaSolver {
    move_order: Vec<usize>,
    explored_positions: usize
}

impl AlphaBetaSolver {
    pub fn new(move_order: Vec<usize>) -> Self {
        Self { move_order, explored_positions: 0 }
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

        let max_score = ((position.width()*position.height() - 1 - position.nb_moves()) / 2) as i32;
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

        alpha
    }
}

impl Solver for AlphaBetaSolver {
    /// Uses negamax to solve the position.
    fn solve(&mut self, position: &(impl lib_game_board::Position + Clone)) -> i32 {
        let best_score = (position.width() * position.height()) as i32;
        self.solve_range(position, -best_score, best_score)
    }

    fn explored_positions(&self) -> usize {
        self.explored_positions
    }

    fn reset_explored_positions(&mut self) {
        self.explored_positions = 0;
    }
}

impl WeakSolver for AlphaBetaSolver {
    fn weak_solve(&mut self, position: &(impl lib_game_board::Position + Clone)) -> i32 {
        self.solve_range(position, -1, 1)
    }

    fn explored_positions(&self) -> usize {
        self.explored_positions
    }

    fn reset_explored_positions(&mut self) {
        self.explored_positions = 0;
    }
}

#[cfg(test)]
mod alpha_beta_tests {
    use super::*;
    use lib_game_board::{grid_position::GridPosition, sequence_position::SequencePosition, stack_position::StackPosition};

    #[test]
    fn grid_correctness() {
        let mut alpha_beta_solver = AlphaBetaSolver::new((0..7).collect());

        assert_eq!(alpha_beta_solver.solve(
        &mut GridPosition::from(
                    &SequencePosition::from(
                        &"2252576253462244111563365343671351441".to_string()
                    ))),
                -1);
    }

    #[test]
    fn stack_correctness() {
        let mut alpha_beta_solver = AlphaBetaSolver::new((0..7).collect());

        assert_eq!(alpha_beta_solver.solve(
        &mut StackPosition::from(
                    &SequencePosition::from(
                        &"2252576253462244111563365343671351441".to_string()
                    ))),
                -1);
    }
}