fn main() {
    // /target/... solver weak position move_ordering L R
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 1+6 {
        println!("\n\nInvalid arguments list. The argument list should be as follow:");
        println!("\tcargo run solver weak position move_ordering length rating");
        println!("where:");
        println!("\t- 'solver': the solver type. Choose between 'min_max', 'alpha_beta', 'alpha_beta_with_transposition', 'alpha_beta_with_iterative_deepening', 'anticipating_alpha_beta', 'alpha_beta_with_ordering'.");
        println!("\t- 'weak': compute the numbers of move until the end (strong) or only the winner (weak). Choose between 'strong' and 'weak'.");
        println!("\t- 'position': the representation of the board. Choose between 'grid', 'stack' and 'bitboard'.");
        println!("\t- 'move_ordering': the order of the moves. Impactful only for Alpha-Beta-based solvers. Choose between 'left_to_right', and 'center_first'.");
        println!("\t- 'L': the overall state of the game in the test dataset. Choose between 1, 2 and 3, where 3 is the easiest.");
        println!("\t- 'R': the overall difficulty of the game in the test dataset. Choose between 1, 2 and 3, where 3 is the easiest. Some ratings aren't available depending on L.");
        return;
    }

    let solver_string = &args[1];
    let weak_string = &args[2];
    let position_string = &args[3];
    let move_ordering_string = &args[4];
    let length: usize = args[5].trim().parse().expect("5th argument, 'length', is not a number.");
    let rating: usize = args[6].trim().parse().expect("6th argument, 'rating', is not a number.");

    lib_benchmark::run_benchmark(&solver_string, &weak_string, &position_string, &move_ordering_string, length, rating);
}