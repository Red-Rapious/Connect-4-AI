use std::fs::File;
use std::io::{BufReader, BufRead};
use std::time::{Duration, Instant};

use lib_game_board::{Solver, Position};
use lib_game_board::sequence_position::SequencePosition;
use lib_game_board::grid_position::GridPosition;
use statistics::Statistics;

pub mod statistics;

pub struct Benchmark 
{
    test_sets: Vec<TestSet>
}

impl Benchmark
{
    pub fn new(test_sets: Vec<TestSet>) -> Self {
        Self { test_sets }
    }

    pub fn benchmark<P: Position>(&self, solver: &mut impl Solver) -> Vec<Statistics> {
        self.test_sets
            .iter()
            .map(|test| test.test_solver::<P>(solver))
            .collect()
    }
}

pub struct TestSet
{
    games_moves: Vec<(SequencePosition, i32)>
}

impl TestSet
{
    pub fn new(length: usize, rating: usize, datasets_path: &str, games_number: Option<usize>) -> Self 
    {
        assert!(1 <= rating && rating <= 3);
        assert!(1 <= length && length <= 3);
        if length == 3 {
            assert!(rating == 1);
        }
        if length == 2 {
            assert!(rating != 3);
        }

        let games_moves = TestSet::load_test(length, rating, datasets_path);
        let games_moves = match games_number {
            None => games_moves,
            Some(n) => games_moves.into_iter().take(n).collect()
        };

        Self { games_moves }
    }

    fn load_test(length: usize, rating: usize, datasets_path: &str) -> Vec<(SequencePosition, i32)> {
        //let file_path = format!("./datasets/Test_L{}_R{}", length, rating);
        let file_path = format!("{}/datasets/Test_L{}_R{}", datasets_path, length, rating);
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

    pub fn test_solver<P: Position>(&self, solver: &mut impl Solver) -> Statistics {
        let mut execution_times: Vec<Duration> = Vec::with_capacity(self.games_moves.len());
        let mut explored_positions_nb: Vec<usize> = Vec::with_capacity(self.games_moves.len());

        let results: Vec<bool> = self.games_moves
            .iter()
            .map(|(position, expected_score)| {
                solver.reset_explored_positions();

                let now = Instant::now();
                let solved_score = solver.solve(&mut GridPosition::from(position));
                execution_times.push(now.elapsed());

                let explored_positions = solver.explored_positions();
                explored_positions_nb.push(explored_positions);


                solved_score == *expected_score
            })
            .collect();


        Statistics::new(results, execution_times, explored_positions_nb)
    }
}

#[cfg(test)]
mod benchmark_tests {
    use super::*;
    use lib_game_board::Position;

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
                &vec![false; test_set.games_moves.len()]
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
}