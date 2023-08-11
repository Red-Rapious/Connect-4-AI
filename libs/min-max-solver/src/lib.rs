use lib_game_board::Solver;

pub struct MinMaxSolver {
    explored_positions: usize
}

impl MinMaxSolver {
    pub fn new() -> Self {
        Self { explored_positions: 0 }
    }
}

impl Solver for MinMaxSolver {
    /// Uses negamax to solve the position.
    fn solve(&mut self, position: &(impl lib_game_board::Position + Clone)) -> i32 {
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

        // Worse possible score by default (lower bound)
        let mut best_score = - ((position.width() * position.height()) as i32);

        // For each possible move
        for column in 0..position.width() {
            if position.can_play(column) {
                let mut position2 = position.clone();
                position2.play(column);

                let score = - self.solve(&mut position2);
                if score > best_score {
                    best_score = score;
                }
            }
        }

        best_score
    }

    fn explored_positions(&self) -> usize {
        self.explored_positions
    }

    fn reset_explored_positions(&mut self) {
        self.explored_positions = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lib_game_board::{grid_position::GridPosition, sequence_position::SequencePosition};

    #[test]
    fn minimax_correctness() {
        let mut minmax_solver = MinMaxSolver::new();

        assert_eq!(minmax_solver.solve(
        &mut GridPosition::from(
                    &SequencePosition::from(
                        &"2252576253462244111563365343671351441".to_string()
                    ))),
                -1);
    }
}