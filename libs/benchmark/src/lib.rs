use std::fs::File;
use std::io::{BufReader, BufRead};

use lib_game_board::Solver;
use lib_game_board::sequence_position::SequencePosition;
use lib_game_board::grid_position::GridPosition;

pub struct Benchmark<S> 
where 
    S: Solver 
{
    test_sets: Vec<TestSet<S>>
}

impl<S> Benchmark<S>
where
    S: Solver
{
    pub fn new(test_sets: Vec<TestSet<S>>) -> Self {
        Self { test_sets }
    }

    pub fn benchmark(&self) {
        let _ = self.test_sets
            .iter()
            .map(|test| test.test_solver());
    }
}

pub struct TestSet<S>
where 
    S: Solver
{
    solver: S,
    games_moves: Vec<(SequencePosition, i32)>
}

impl<S> TestSet<S>
where 
    S: Solver
{
    pub fn new(solver: S, length: usize, rating: usize) -> Self 
    where 
        S: Solver 
    {
        assert!(1 <= rating && rating <= 3);
        assert!(1 <= length && length <= 3);

        let games_moves = TestSet::<S>::load_test(length, rating);

        Self { solver, games_moves }
    }

    fn load_test(length: usize, rating: usize) -> Vec<(SequencePosition, i32)> {
        let file_path = format!("./datasets/Test_L{}_R{}", length, rating);
        let file = File::open(&file_path).expect(format!("Unable to read file: {}", file_path).as_str());
        let reader = BufReader::new(file);
        let mut games_moves = Vec::with_capacity(1_000);

        for line in reader.lines() {
            let line_content = line.expect("Unable to read line");
            let elements: Vec<String> = line_content
                .split_whitespace()
                .map(str::parse)
                .collect::<Result<_, _>>()
                .expect("Unable to parse line.");

            assert_eq!(elements.len(), 2);

            let position = SequencePosition::from(&elements[0]);
            let score: i32 = elements[1].parse().expect("Unable to parse score.");

            games_moves.push((position, score));
        }
        
        games_moves
    }

    pub fn games_moves(&self) -> &Vec<(SequencePosition, i32)> {
        &self.games_moves
    }

    pub fn test_solver(&self) -> Vec<bool> {
        self.games_moves
            .iter()
            .map(|(position, expected_score)| 
                <S as Solver>
                    ::solve(&self.solver, &GridPosition::from(position)) 
                    
                == *expected_score)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct TestSolver {
        value: i32
    }
    impl TestSolver {
        pub fn new(value: i32) -> Self {
            Self { value }
        }
    }
    impl Solver for TestSolver {
        fn solve<P>(&self, _position: &P) -> i32 where P: lib_game_board::Position {
            self.value
        }
    }

    mod load {
        use super::*;

        #[test]
        #[allow(non_snake_case)]
        fn load_L1_R1() {
            let test_set = TestSet::new(TestSolver::new(0), 1, 1);
            
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
            let solver = TestSolver::new(0);
            let test_set = TestSet::new(solver, 1, 1);

            assert_eq!(
                test_set.test_solver(),
                vec![false; test_set.games_moves.len()]
            )
        }

        #[test]
        fn test_test_solver_11() {
            let solver = TestSolver::new(11);
            let test_set = TestSet::new(solver, 1, 1);

            let correctly_solved: usize = test_set
                .test_solver()
                .iter()
                .map(|b| if *b { 1 } else { 0 })
                .sum();

            assert_eq!(correctly_solved, 104);
        }
    }
}