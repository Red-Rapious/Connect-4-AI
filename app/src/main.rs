use lib_benchmark::run_benchmark;
use lib_game_cli::GameCLI;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("\n\n`cargo run`: invalid arguments. Run either `cargo run benchmark ...` or `cargo run game`");
        return;
    }

    if args[1] == "benchmark" {
        // /target/... benchmark solver weak position move_ordering L R
        if args.len() != 2+6 && args.len() != 2+6+1 {
            println!("\n\ncargo run benchmark: invalid arguments list. The argument list should be as follow:");
            println!("\tcargo run benchmark [solver] [weak] [position] [move_ordering] [L] [R] [games_number]");
            println!("where:");
            println!("\t- 'solver': the solver type. Choose between 'min_max', 'alpha_beta', 'alpha_beta_with_transposition', 'alpha_beta_with_iterative_deepening', 'anticipating_alpha_beta', 'alpha_beta_with_ordering', 'alpha_beta_with_optimised_transposition', and 'alpha_beta_with_lower_bound_transposition'.");
            println!("\t- 'weak': compute the numbers of move until the end (strong) or only the winner (weak). Choose between 'strong' and 'weak'.");
            println!("\t- 'position': the representation of the board. Choose between 'grid', 'stack' and 'bitboard'.");
            println!("\t- 'move_ordering': the order of the moves. Impactful only for Alpha-Beta-based solvers. Choose between 'left_to_right', and 'center_first'.");
            println!("\t- 'L': the overall state of the game in the test dataset. Choose between 1, 2 and 3, where 3 is the easiest.");
            println!("\t- 'R': the overall difficulty of the game in the test dataset. Choose between 1, 2 and 3, where 3 is the easiest. Some ratings aren't available depending on L.");
            println!("\t- 'games_number': the number of games that the solver will be tested on. Let empty to test on all games.");
            return;
        }
    
        let solver_string = &args[2+0];
        let weak_string = &args[2+1];
        let position_string = &args[2+2];
        let move_ordering_string = &args[2+3];
        let length: usize = args[2+4].trim().parse().expect("6th argument, 'length', is not a number.");
        let rating: usize = args[2+5].trim().parse().expect("7th argument, 'rating', is not a number.");
        let mut games_number = match args.len() {
            8 => None,
            9 => Some(args[2+6].trim().parse().expect("8th argument, 'games_number', is not a number.")),
            _ => panic!()
        };
        if games_number == Some(0) {
            games_number = None;
        }
    
        run_benchmark(&solver_string, &weak_string, &position_string, &move_ordering_string, length, rating, games_number);
    } else if args[1] == "game" {

        let mut game_cli = GameCLI::new(7, 6);
        game_cli.run_game();

    } else {
        println!("\n\n`cargo run`: invalid argument. Run either `cargo run benchmark ...` or `cargo run game ...`");
        return;
    }
}