use lib_min_max_solver::MinMaxSolver;
use lib_alpha_beta_solver::{
    alpha_beta_solver::AlphaBetaSolver, 
    alpha_beta_with_transposition::AlphaBetaWithTransposition, alpha_beta_with_iterative_deepening::AlphaBetaWithIterativeDeepening,
    anticipating_alpha_beta::AnticipatingAlphaBeta, 
    alpha_beta_with_ordering::AlphaBetaWithOrdering,
    alpha_beta_with_optimised_transposition::AlphaBetaWithOptimisedTransposition,
    alpha_beta_with_lower_bound_transposition::AlphaBetaWithLowerBoundTransposition,
    alpha_beta_with_opening_book::AlphaBetaWithOpeningBook
};
use lib_game_board::{
    Solver, 
    WeakSolver, 
    grid_position::GridPosition, 
    stack_position::StackPosition, 
    bitboard_position::BitboardPosition, 
    anticipating_bitboard_position::AnticipatingBitboardPosition, 
    bitboard_position_with_ordering::BitboardPositionWithOrdering
};
use crate::{benchmark::Benchmark, test_set::TestSet, statistics::Statistics};

use std::time::Instant;

pub mod statistics;
pub mod test_set;
pub mod benchmark;


pub fn run_benchmark(solver_string: &str, weak_string: &str, position_string: &str, move_ordering_string: &str, length: usize, rating: usize, games_number: Option<usize>) {
    let move_ordering = match move_ordering_string {
        "left_to_right" => (0..7).collect(),
        "center_first" => vec![3, 4, 2, 5, 1, 6, 0],
        _ => { assert!(solver_string == "min_max", "Unknown move ordering."); vec![] }
    };

    let test_sets = vec![TestSet::new(length, rating, &"libs/benchmark", games_number)];
    let benchmark = Benchmark::new(test_sets);

    println!("Loading solver...");
    let mut solver: AllowedSolver = match solver_string {
        "min_max" => AllowedSolver::MinMaxSolver(MinMaxSolver::new()),
        "alpha_beta" => AllowedSolver::AlphaBetaSolver(AlphaBetaSolver::new(move_ordering)),
        "alpha_beta_with_transposition" => AllowedSolver::AlphaBetaWithTransposition(AlphaBetaWithTransposition::new(move_ordering)),
        "alpha_beta_with_iterative_deepening" => AllowedSolver::AlphaBetaWithIterativeDeepening(AlphaBetaWithIterativeDeepening::new(move_ordering)),
        "anticipating_alpha_beta" => AllowedSolver::AnticipatingAlphaBeta(AnticipatingAlphaBeta::new(move_ordering)),
        "alpha_beta_with_ordering" => AllowedSolver::AlphaBetaWithOrdering(AlphaBetaWithOrdering::new(move_ordering)),
        "alpha_beta_with_optimised_transposition" => AllowedSolver::AlphaBetaWithOptimisedTransposition(AlphaBetaWithOptimisedTransposition::new(move_ordering)),
        "alpha_beta_with_lower_bound_transposition" => AllowedSolver::AlphaBetaWithLowerBoundTransposition(AlphaBetaWithLowerBoundTransposition::new(move_ordering)),
        "alpha_beta_with_opening_book" => {
            let mut solver = AlphaBetaWithOpeningBook::new(7, 6, move_ordering);
            solver.load_opening_book("libs/alpha-beta-solver/opening-books/7x6.book");
            AllowedSolver::AlphaBetaWithOpeningBook(solver)
        },
        _ => panic!("Unknown solver name.")
    };


    println!("\n\nSelected arguments:");
    println!("\t- Solver: {}", solver_string);
    println!("\t- Solving type: {}", weak_string);
    println!("\t- Move ordering: {}", move_ordering_string);
    println!("\t- Test set: L{} R{}", length, rating);
    println!("");

    let now = Instant::now();

    let stats: Vec<Statistics> = 
    if weak_string == "strong" { 
        match position_string {
            "grid" => benchmark.benchmark::<GridPosition>(&mut solver),
            "stack" => benchmark.benchmark::<StackPosition>(&mut solver),
            "bitboard" => 
            if solver_string == "anticipating_alpha_beta" { 
                benchmark.benchmark::<AnticipatingBitboardPosition>(&mut solver) 
            } else if solver_string == "alpha_beta_with_ordering" 
                || solver_string == "alpha_beta_with_optimised_transposition" 
                || solver_string == "alpha_beta_with_lower_bound_transposition"
                || solver_string == "alpha_beta_with_opening_book" 
            { 
                benchmark.benchmark::<BitboardPositionWithOrdering>(&mut solver) 
            } else { 
                benchmark.benchmark::<BitboardPosition>(&mut solver) 
            },
            _ => panic!("Unknown position name.")
        }
    } else if weak_string == "weak" {
        assert_ne!(solver_string, "min_max", "MinMax solver does not implement weak solving.");
        match position_string {
            "grid" => benchmark.benchmark_weak::<GridPosition>(&mut solver),
            "stack" => benchmark.benchmark_weak::<StackPosition>(&mut solver),
            "bitboard" => 
            if solver_string == "anticipating_alpha_beta" { 
                benchmark.benchmark_weak::<AnticipatingBitboardPosition>(&mut solver) 
            } else if solver_string == "alpha_beta_with_ordering" 
                || solver_string == "alpha_beta_with_optimised_transposition" 
                || solver_string == "alpha_beta_with_lower_bound_transposition"
                || solver_string == "alpha_beta_with_opening_book" 
            { 
                benchmark.benchmark_weak::<BitboardPositionWithOrdering>(&mut solver) 
            } else { 
                benchmark.benchmark_weak::<BitboardPosition>(&mut solver) 
            },
            _ => panic!("Unknown position name.")
        }
    } else {
        panic!("Unknown weak/strong argument.")
    };

    println!("Benchmark done in {:?}.\n", now.elapsed());

    println!("Benchmark results:\n   {}", stats[0]);
}

enum AllowedSolver {
    MinMaxSolver(MinMaxSolver),
    AlphaBetaSolver(AlphaBetaSolver),
    AlphaBetaWithTransposition(AlphaBetaWithTransposition),
    AlphaBetaWithIterativeDeepening(AlphaBetaWithIterativeDeepening),
    AnticipatingAlphaBeta(AnticipatingAlphaBeta),
    AlphaBetaWithOrdering(AlphaBetaWithOrdering),
    AlphaBetaWithOptimisedTransposition(AlphaBetaWithOptimisedTransposition),
    AlphaBetaWithLowerBoundTransposition(AlphaBetaWithLowerBoundTransposition),
    AlphaBetaWithOpeningBook(AlphaBetaWithOpeningBook)
}

impl Solver for AllowedSolver {
    fn solve(&mut self, position: &(impl lib_game_board::Position + Clone)) -> i32 {
        use AllowedSolver::*;
        match self {
            MinMaxSolver(ref mut solver) => solver.solve(position),
            AlphaBetaSolver(ref mut solver) => solver.solve(position),
            AlphaBetaWithTransposition(ref mut solver) => solver.solve(position),
            AlphaBetaWithIterativeDeepening(ref mut solver) => solver.solve(position),
            AnticipatingAlphaBeta(ref mut solver) => solver.solve(position),
            AlphaBetaWithOrdering(ref mut solver) => solver.solve(position),
            AlphaBetaWithOptimisedTransposition(ref mut solver) => solver.solve(position),
            AlphaBetaWithLowerBoundTransposition(ref mut solver) => solver.solve(position),
            AlphaBetaWithOpeningBook(ref mut solver) => solver.solve(position)
        }
    }

    fn explored_positions(&self) -> usize {
        use AllowedSolver::*;
        match self {
            MinMaxSolver(solver) => Solver::explored_positions(solver),
            AlphaBetaSolver(solver) => Solver::explored_positions(solver),
            AlphaBetaWithTransposition(solver) => Solver::explored_positions(solver),
            AlphaBetaWithIterativeDeepening(solver) => Solver::explored_positions(solver),
            AnticipatingAlphaBeta(solver) => Solver::explored_positions(solver),
            AlphaBetaWithOrdering(solver) => Solver::explored_positions(solver),
            AlphaBetaWithOptimisedTransposition(solver) => Solver::explored_positions(solver),
            AlphaBetaWithLowerBoundTransposition(solver) => Solver::explored_positions(solver),
            AlphaBetaWithOpeningBook(solver) => Solver::explored_positions(solver)
        }
    }

    fn reset_explored_positions(&mut self) {
        use AllowedSolver::*;
        match self {
            MinMaxSolver(ref mut solver) => Solver::explored_positions(solver),
            AlphaBetaSolver(ref mut solver) => Solver::explored_positions(solver),
            AlphaBetaWithTransposition(ref mut solver) => Solver::explored_positions(solver),
            AlphaBetaWithIterativeDeepening(ref mut solver) => Solver::explored_positions(solver),
            AnticipatingAlphaBeta(ref mut solver) => Solver::explored_positions(solver),
            AlphaBetaWithOrdering(ref mut solver) => Solver::explored_positions(solver),
            AlphaBetaWithOptimisedTransposition(ref mut solver) => Solver::explored_positions(solver),
            AlphaBetaWithLowerBoundTransposition(ref mut solver) => Solver::explored_positions(solver),
            AlphaBetaWithOpeningBook(ref mut solver) => Solver::explored_positions(solver)
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
            AnticipatingAlphaBeta(ref mut solver) => solver.weak_solve(position),
            AlphaBetaWithOrdering(ref mut solver) => solver.weak_solve(position),
            AlphaBetaWithOptimisedTransposition(ref mut solver) => solver.weak_solve(position),
            AlphaBetaWithLowerBoundTransposition(ref mut solver) => solver.weak_solve(position),
            AlphaBetaWithOpeningBook(ref mut solver) => solver.weak_solve(position)
        }
    }

    fn explored_positions(&self) -> usize {
        use AllowedSolver::*;
        match self {
            MinMaxSolver(_) => panic!("MinMaxSolver does not implement WeakSolver trait."),
            AlphaBetaSolver(solver) => WeakSolver::explored_positions(solver),
            AlphaBetaWithTransposition(solver) => WeakSolver::explored_positions(solver),
            AlphaBetaWithIterativeDeepening(solver) => WeakSolver::explored_positions(solver),
            AnticipatingAlphaBeta(solver) => WeakSolver::explored_positions(solver),
            AlphaBetaWithOrdering(solver) => WeakSolver::explored_positions(solver),
            AlphaBetaWithOptimisedTransposition(solver) => WeakSolver::explored_positions(solver),
            AlphaBetaWithLowerBoundTransposition(solver) => WeakSolver::explored_positions(solver),
            AlphaBetaWithOpeningBook(solver) => WeakSolver::explored_positions(solver)
        }
    }

    fn reset_explored_positions(&mut self) {
        use AllowedSolver::*;
        match self {
            MinMaxSolver(_) => panic!("MinMaxSolver does not implement WeakSolver trait."),
            AlphaBetaSolver(ref mut solver) => WeakSolver::reset_explored_positions(solver),
            AlphaBetaWithTransposition(ref mut solver) => WeakSolver::reset_explored_positions(solver),
            AlphaBetaWithIterativeDeepening(ref mut solver) => WeakSolver::reset_explored_positions(solver),
            AnticipatingAlphaBeta(ref mut solver) => WeakSolver::reset_explored_positions(solver),
            AlphaBetaWithOrdering(ref mut solver) => WeakSolver::reset_explored_positions(solver),
            AlphaBetaWithOptimisedTransposition(ref mut solver) => WeakSolver::reset_explored_positions(solver),
            AlphaBetaWithLowerBoundTransposition(ref mut solver) => WeakSolver::reset_explored_positions(solver),
            AlphaBetaWithOpeningBook(ref mut solver) => WeakSolver::reset_explored_positions(solver)
        }
    }
}