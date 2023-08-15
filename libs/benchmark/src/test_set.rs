use std::fs::File;
use std::io::{BufReader, BufRead};
use std::time::{Duration, Instant};
use progress_bar::*;

use lib_game_board::{Solver, Position, WeakSolver, sequence_position::SequencePosition};
use crate::statistics::Statistics;


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

    pub fn test_solver<P: Position + Clone>(&self, solver: &mut impl Solver) -> Statistics {
        let mut execution_times: Vec<Duration> = Vec::with_capacity(self.games_moves.len());
        let mut explored_positions_nb: Vec<usize> = Vec::with_capacity(self.games_moves.len());

        init_progress_bar(self.games_moves.len());
        set_progress_bar_action("Testing", Color::LightBlue, Style::Normal);
        let results: Vec<bool> = self.games_moves
            .iter()
            .map(|(position, expected_score)| {
                solver.reset_explored_positions();

                let now = Instant::now();
                let solved_score = solver.solve(&mut P::from_seq(position));
                execution_times.push(now.elapsed());
                inc_progress_bar();

                let explored_positions = solver.explored_positions();
                explored_positions_nb.push(explored_positions);


                if solved_score == *expected_score {
                    true
                } else {
                    println!("Test failed: expected score was {}, but solved score is {}.", *expected_score, solved_score);
                    false
                }
            })
            .collect();
        finalize_progress_bar();

        Statistics::new(results, execution_times, explored_positions_nb)
    }

    pub fn test_weak_solver<P: Position + Clone>(&self, solver: &mut impl WeakSolver) -> Statistics {
        let mut execution_times: Vec<Duration> = Vec::with_capacity(self.games_moves.len());
        let mut explored_positions_nb: Vec<usize> = Vec::with_capacity(self.games_moves.len());

        let results: Vec<bool> = self.games_moves
            .iter()
            .map(|(position, expected_score)| {
                let expected_score = 
                    if *expected_score == 0 { 0 } 
                    else if *expected_score < 0 { -1 } 
                    else { 1 };
                    
                solver.reset_explored_positions();

                let now = Instant::now();
                let solved_score = solver.weak_solve(&mut P::from_seq(position));
                execution_times.push(now.elapsed());

                let solved_score = 
                    if solved_score == 0 { 0 } 
                    else if solved_score < 0 { -1 } 
                    else { 1 };

                let explored_positions = solver.explored_positions();
                explored_positions_nb.push(explored_positions);


                if solved_score == expected_score {
                    true
                } else {
                    println!("Test failed: expected score was {}, but solved score is {}.", expected_score, solved_score);
                    false
                }
            })
            .collect();


        Statistics::new(results, execution_times, explored_positions_nb)
    }
}