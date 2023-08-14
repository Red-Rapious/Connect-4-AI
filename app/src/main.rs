use lib_min_max_solver::MinMaxSolver;
use lib_alpha_beta_solver::{alpha_beta_solver::AlphaBetaSolver, alpha_beta_with_transposition::AlphaBetaWithTransposition, alpha_beta_with_iterative_deepening::AlphaBetaWithIterativeDeepening, anticipating_alpha_beta::AnticipatingAlphaBeta};
use lib_benchmark::{Benchmark, TestSet};
use lib_game_board::{grid_position::GridPosition, bitboard_position::BitboardPosition, Solver, stack_position::StackPosition, WeakSolver, anticipating_bitboard_position::AnticipatingBitboardPosition};

use std::{io::Write, time::Instant};

fn main() {
    // /target/... solver weak position move_ordering L R
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 1+6 {
        println!("\n\nInvalid arguments list. The argument list should be as follow:");
        println!("\tcargo run solver weak position move_ordering length rating");
        println!("where:");
        println!("\t- 'solver': the solver type. Choose between 'min_max', 'alpha_beta', 'alpha_beta_with_transposition', 'alpha_beta_with_iterative_deepening', and 'anticipating_alpha_beta'.");
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

    let move_ordering = match move_ordering_string.as_str() {
        "left_to_right" => (0..7).collect(),
        "center_first" => vec![3, 4, 2, 5, 1, 6, 0],
        _ => { assert!(solver_string.as_str() == "min_max", "Unknown move ordering."); vec![] }
    };

    let test_sets = vec![TestSet::new(length, rating, &"libs/benchmark", None)];
    let benchmark = Benchmark::new(test_sets);

    let mut solver: AllowedSolver = match solver_string.as_str() {
        "min_max" => AllowedSolver::MinMaxSolver(MinMaxSolver::new()),
        "alpha_beta" => AllowedSolver::AlphaBetaSolver(AlphaBetaSolver::new(move_ordering)),
        "alpha_beta_with_transposition" => AllowedSolver::AlphaBetaWithTransposition(AlphaBetaWithTransposition::new(move_ordering)),
        "alpha_beta_with_iterative_deepening" => AllowedSolver::AlphaBetaWithIterativeDeepening(AlphaBetaWithIterativeDeepening::new(move_ordering)),
        "anticipating_alpha_beta" => AllowedSolver::AnticipatingAlphaBeta(AnticipatingAlphaBeta::new(move_ordering)),
        _ => panic!("Unknown solver name.")
    };


    println!("\n\nSelected arguments:");
    println!("\t- Solver: {}", solver_string);
    println!("\t- Solving type: {}", weak_string);
    println!("\t- Move ordering: {}", move_ordering_string);
    println!("\t- Test set: L{} R{}", length, rating);
    print!("\nBeginning of the benchmark... ");
    std::io::stdout().flush().unwrap();

    let now = Instant::now();

    let stats: Vec<lib_benchmark::statistics::Statistics> = 
    if weak_string.as_str() == "strong" { 
        match position_string.as_str() {
            "grid" => benchmark.benchmark::<GridPosition>(&mut solver),
            "stack" => benchmark.benchmark::<StackPosition>(&mut solver),
            "bitboard" => if solver_string == "anticipating_alpha_beta" { benchmark.benchmark::<AnticipatingBitboardPosition>(&mut solver) } else { benchmark.benchmark::<BitboardPosition>(&mut solver) },
            _ => panic!("Unknown position name.")
        }
    } else if weak_string.as_str() == "weak" {
        assert_ne!(solver_string, "min_max", "MinMax solver does not implement weak solving.");
        match position_string.as_str() {
            "grid" => benchmark.benchmark_weak::<GridPosition>(&mut solver),
            "stack" => benchmark.benchmark_weak::<StackPosition>(&mut solver),
            "bitboard" => if solver_string == "anticipating_alpha_beta" { benchmark.benchmark_weak::<AnticipatingBitboardPosition>(&mut solver) } else { benchmark.benchmark_weak::<BitboardPosition>(&mut solver) },
            _ => panic!("Unknown position name.")
        }
    } else {
        panic!("Unknown weak/strong argument.")
    };

    println!("done in {:?}.\n", now.elapsed());

    println!("Benchmark results:\n   {}", stats[0]);
}

enum AllowedSolver {
    MinMaxSolver(MinMaxSolver),
    AlphaBetaSolver(AlphaBetaSolver),
    AlphaBetaWithTransposition(AlphaBetaWithTransposition),
    AlphaBetaWithIterativeDeepening(AlphaBetaWithIterativeDeepening),
    AnticipatingAlphaBeta(AnticipatingAlphaBeta)
}

impl Solver for AllowedSolver {
    fn solve(&mut self, position: &(impl lib_game_board::Position + Clone)) -> i32 {
        use AllowedSolver::*;
        match self {
            MinMaxSolver(ref mut solver) => solver.solve(position),
            AlphaBetaSolver(ref mut solver) => solver.solve(position),
            AlphaBetaWithTransposition(ref mut solver) => solver.solve(position),
            AlphaBetaWithIterativeDeepening(ref mut solver) => solver.solve(position),
            AnticipatingAlphaBeta(ref mut solver) => solver.solve(position)
        }
    }

    fn explored_positions(&self) -> usize {
        use AllowedSolver::*;
        match self {
            MinMaxSolver(solver) => Solver::explored_positions(solver),
            AlphaBetaSolver(solver) => Solver::explored_positions(solver),
            AlphaBetaWithTransposition(solver) => Solver::explored_positions(solver),
            AlphaBetaWithIterativeDeepening(solver) => Solver::explored_positions(solver),
            AnticipatingAlphaBeta(solver) => Solver::explored_positions(solver)
        }
    }

    fn reset_explored_positions(&mut self) {
        use AllowedSolver::*;
        match self {
            MinMaxSolver(ref mut solver) => Solver::explored_positions(solver),
            AlphaBetaSolver(ref mut solver) => Solver::explored_positions(solver),
            AlphaBetaWithTransposition(ref mut solver) => Solver::explored_positions(solver),
            AlphaBetaWithIterativeDeepening(ref mut solver) => Solver::explored_positions(solver),
            AnticipatingAlphaBeta(ref mut solver) => Solver::explored_positions(solver)
        };
    }
}

impl WeakSolver for AllowedSolver {
    fn weak_solve(&mut self, position: &(impl lib_game_board::Position + Clone)) -> i32 {
        use AllowedSolver::*;
        match self {
            MinMaxSolver(_) => panic!("MinMaxSolver does not implement WeakSolver trait."),
            AlphaBetaSolver(ref mut solver) => solver.weak_solve(position),
            AlphaBetaWithTransposition(ref mut solver) => solver.weak_solve(position),
            AlphaBetaWithIterativeDeepening(ref mut solver) => solver.weak_solve(position),
            AnticipatingAlphaBeta(ref mut solver) => solver.weak_solve(position)
        }
    }

    fn explored_positions(&self) -> usize {
        use AllowedSolver::*;
        match self {
            MinMaxSolver(_) => panic!("MinMaxSolver does not implement WeakSolver trait."),
            AlphaBetaSolver(solver) => WeakSolver::explored_positions(solver),
            AlphaBetaWithTransposition(solver) => WeakSolver::explored_positions(solver),
            AlphaBetaWithIterativeDeepening(solver) => WeakSolver::explored_positions(solver),
            AnticipatingAlphaBeta(solver) => WeakSolver::explored_positions(solver)
        }
    }

    fn reset_explored_positions(&mut self) {
        use AllowedSolver::*;
        match self {
            MinMaxSolver(_) => panic!("MinMaxSolver does not implement WeakSolver trait."),
            AlphaBetaSolver(ref mut solver) => WeakSolver::reset_explored_positions(solver),
            AlphaBetaWithTransposition(ref mut solver) => WeakSolver::reset_explored_positions(solver),
            AlphaBetaWithIterativeDeepening(ref mut solver) => WeakSolver::reset_explored_positions(solver),
            AnticipatingAlphaBeta(ref mut solver) => WeakSolver::reset_explored_positions(solver)
        }
    }
}