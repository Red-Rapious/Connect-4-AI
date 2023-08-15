use lib_game_board::{Solver, Position, WeakSolver};
use crate::statistics::Statistics;
use crate::test_set::TestSet;

pub struct Benchmark 
{
    test_sets: Vec<TestSet>
}

impl Benchmark
{
    pub fn new(test_sets: Vec<TestSet>) -> Self {
        Self { test_sets }
    }

    pub fn benchmark<P: Position + Clone>(&self, solver: &mut impl Solver) -> Vec<Statistics> {
        self.test_sets
            .iter()
            .map(|test| test.test_solver::<P>(solver))
            .collect()
    }

    pub fn benchmark_weak<P: Position + Clone>(&self, solver: &mut impl WeakSolver) -> Vec<Statistics> {
        self.test_sets
            .iter()
            .map(|test| test.test_weak_solver::<P>(solver))
            .collect()
    }
}



#[cfg(test)]
mod benchmark_tests {
    use super::*;
    use lib_game_board::{grid_position::GridPosition, sequence_position::SequencePosition};

    pub struct TestSolver {
        value: i32
    }
    impl TestSolver {
        pub fn new(value: i32) -> Self {
            Self { value }
        }
    }
    impl Solver for TestSolver {
        fn solve(&mut self, _position: &impl Position) -> i32{
            self.value
        }

        fn explored_positions(&self) -> usize {
            0
        }

        fn reset_explored_positions(&mut self) {
            ()
        }
    }
    impl WeakSolver for TestSolver {
        fn weak_solve(&mut self, _position: &impl Position) -> i32{
            self.value
        }

        fn explored_positions(&self) -> usize {
            0
        }

        fn reset_explored_positions(&mut self) {
            ()
        }
    }

    mod load {
        use super::*;

        #[test]
        #[allow(non_snake_case)]
        fn load_L1_R1() {
            let test_set = TestSet::new(1, 1, &".", None);
            
            assert_eq!(
                test_set.games_moves()[0],
                (SequencePosition::from(&"32164625".to_string()), 11)
            )
        }
    }

    mod test_solver {
        use super::*;

        #[test]
        fn test_test_solver_0() {
            let test_set = TestSet::new(1, 1, &".", None);
            let mut solver = TestSolver::new(0);

            assert_eq!(
                test_set.test_solver::<GridPosition>(&mut solver).results(),
                &vec![false; test_set.games_moves().len()]
            )
        }

        #[test]
        fn test_test_solver_11() {
            let test_set = TestSet::new(1, 1, &".", None);
            let mut solver = TestSolver::new(11);

            let correctly_solved: usize = test_set
                .test_solver::<GridPosition>(&mut solver)
                .results()
                .iter()
                .map(|b| if *b { 1 } else { 0 })
                .sum();

            assert_eq!(correctly_solved, 104);
        }
    }

    mod test_weak_solver {
        use super::*;

        #[test]
        fn test_test_solver_0() {
            let test_set = TestSet::new(1, 1, &".", None);
            let mut solver = TestSolver::new(0);

            assert_eq!(
                test_set.test_weak_solver::<GridPosition>(&mut solver).results(),
                &vec![false; test_set.games_moves().len()]
            )
        }

        #[test]
        fn test_test_solver_1() {
            let test_set = TestSet::new(1, 1, &".", None);
            let mut solver = TestSolver::new(1);

            let correctly_solved: usize = test_set
                .test_weak_solver::<GridPosition>(&mut solver)
                .results()
                .iter()
                .map(|b| if *b { 1 } else { 0 })
                .sum();

            assert_eq!(correctly_solved, 723);
        }
    }
}