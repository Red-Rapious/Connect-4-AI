use lib_min_max_solver::MinMaxSolver;
use lib_benchmark::{Benchmark, TestSet};

fn main() {
    benchmark_minmax();
}

fn benchmark_minmax() {
    let mut minmax_solver = MinMaxSolver::new();
    let test_sets = vec![TestSet::new(3, 1, &"libs/benchmark", None)];
    let benchmark = Benchmark::new(test_sets);

    let stats = benchmark.benchmark(&mut minmax_solver);
    println!("Minimax - Test Set L3 R1:\n\t{}", stats[0]);
}