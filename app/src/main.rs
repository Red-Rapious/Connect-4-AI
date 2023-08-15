fn main() {
    // /target/... solver weak position move_ordering L R
    let args: Vec<String> = std::env::args().collect();
    lib_benchmark::run_benchmark(args);
}