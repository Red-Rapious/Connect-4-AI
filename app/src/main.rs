use lib_min_max_solver::MinMaxSolver;
use lib_benchmark::{Benchmark, TestSet};

fn main() {
    benchmark_minimax();
}

fn benchmark_minimax() {
    let solver = MinMaxSolver::new();
    let test_sets = vec![TestSet::new(1, 1, &"libs/benchmark", Some(1))];
    let benchmark = Benchmark::new(test_sets);

    let stats = benchmark.benchmark(&solver);
    println!("Minimax - Test Set L1 R1: {}", stats[0]);
}