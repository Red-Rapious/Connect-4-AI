use lib_game_board::Solver;

pub struct MinMaxSolver;

impl MinMaxSolver {
    pub fn new() -> Self {
        Self
    }
}

impl Solver for MinMaxSolver {
    /// Uses negamax to solve the position.
    fn solve(&self, position: &mut (impl lib_game_board::Position + Clone)) -> i32 {
        // Draw
        if position.nb_moves() == position.width() * position.height() {
            return 0;
        }

        // Next move is winnable
        for column in 0..position.width() {
            if position.can_play(column) && position.is_winning_move(column, position.player_turn()) {
                return ((position.width()*position.height() + 1 - position.nb_moves()) / 2) as i32;
            }
        }

        // Worse possible score by default (lower bound)
        let mut best_score = - ((position.width() * position.height()) as i32);

        // For each possible move
        for column in 0..position.width() {
            if position.can_play(column) {
                let mut position2 = position.clone();
                position2.play(column, position.player_turn());

                let score = - self.solve(&mut position2);
                if score > best_score {
                    best_score = score;
                }
            }
        }

        best_score
    }
}