use lib_min_max_solver::MinMaxSolver;
use lib_alpha_beta_solver::AlphaBetaSolver;
use lib_benchmark::{Benchmark, TestSet};
use lib_game_board::grid_position::GridPosition;

fn main() {
    benchmark_alpha_beta();
}

fn _benchmark_min_max() {
    let mut min_max_solver = MinMaxSolver::new();
    let test_sets = vec![TestSet::new(3, 1, &"libs/benchmark", None)];
    let benchmark = Benchmark::new(test_sets);

    let stats = benchmark.benchmark::<GridPosition>(&mut min_max_solver);
    println!("Minimax - Test Set L3 R1:\n\t{}", stats[0]);
}

fn benchmark_alpha_beta() {
    let mut alpha_beta_solver = AlphaBetaSolver::new();
    let test_sets = vec![
        TestSet::new(3, 1, &"libs/benchmark", None),
        TestSet::new(2, 1, &"libs/benchmark", Some(1)),
        TestSet::new(2, 2, &"libs/benchmark", Some(1))
        ];
    let benchmark = Benchmark::new(test_sets);

    let stats = benchmark.benchmark::<GridPosition>(&mut alpha_beta_solver);
    println!("Minimax - Test Set L3 R1:\n\t{}", stats[0]);
    println!("Minimax - Test Set L2 R1:\n\t{}", stats[1]);
    println!("Minimax - Test Set L2 R2:\n\t{}", stats[2]);
}