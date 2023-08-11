use lib_min_max_solver::MinMaxSolver;
use lib_benchmark::{Benchmark, TestSet};

fn main() {
    benchmark_minmax();
}

fn benchmark_minmax() {
    let mut minmax_solver = MinMaxSolver::new();
    let test_sets = vec![TestSet::new(3, 1, &"libs/benchmark", Some(10))];
    let benchmark = Benchmark::new(test_sets);

    let stats = benchmark.benchmark(&mut minmax_solver);
    println!("Minimax - Test Set L3 R1: {}", stats[0]);
    println!("Explored positions: {}", minmax_solver.explored_positions() / 10);
}