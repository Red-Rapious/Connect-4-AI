use lib_min_max_solver::MinMaxSolver;
use lib_alpha_beta_solver::AlphaBetaSolver;
use lib_benchmark::{Benchmark, TestSet};
use lib_game_board::grid_position::GridPosition;

fn main() {
    benchmark_alpha_beta();
    benchmark_alpha_beta_weak()
}

fn _benchmark_min_max() {
    let mut min_max_solver = MinMaxSolver::new();
    let test_sets = vec![TestSet::new(3, 1, &"libs/benchmark", None)];
    let benchmark = Benchmark::new(test_sets);

    let stats = benchmark.benchmark::<GridPosition>(&mut min_max_solver);
    println!("Min-Max - Test Set L3 R1:\n\t{}", stats[0]);
}

fn benchmark_alpha_beta() {
    let move_order: &'static [usize] = &[3, 4, 2, 5, 1, 6, 0];
    let mut alpha_beta_solver = AlphaBetaSolver::new(move_order);
    let test_sets = vec![
        TestSet::new(3, 1, &"libs/benchmark", None),
        //TestSet::new(2, 1, &"libs/benchmark", None),
        //TestSet::new(2, 2, &"libs/benchmark", None)
        ];
    let benchmark = Benchmark::new(test_sets);

    let stats = benchmark.benchmark::<GridPosition>(&mut alpha_beta_solver);
    println!("AlphaBeta - Test Set L3 R1:\n\t{}", stats[0]);
    //println!("AlphaBeta - Test Set L2 R1:\n\t{}", stats[1]);
    //println!("AlphaBeta - Test Set L2 R2:\n\t{}", stats[2]);
}

fn benchmark_alpha_beta_weak() {
    let move_order: &'static [usize] = &[3, 4, 2, 5, 1, 6, 0];
    let mut alpha_beta_solver = AlphaBetaSolver::new(move_order);
    let test_sets = vec![
        TestSet::new(3, 1, &"libs/benchmark", None),
        //TestSet::new(2, 1, &"libs/benchmark", None),
        //TestSet::new(2, 2, &"libs/benchmark", None)
        ];
    let benchmark = Benchmark::new(test_sets);

    let stats = benchmark.benchmark_weak::<GridPosition>(&mut alpha_beta_solver);
    println!("AlphaBeta (Weak) - Test Set L3 R1:\n\t{}", stats[0]);
    //println!("AlphaBeta (Weak) - Test Set L2 R1:\n\t{}", stats[1]);
    //println!("AlphaBeta (Weak) - Test Set L2 R2:\n\t{}", stats[2]);
}