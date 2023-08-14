# Connect-4-AI
An artificial intelligence to perfectly solve the game of Connect Four.

## About
My goal is to create a perfect Connect Four bot, capable of predicting the outcome of a game in a reasonable computing time. 


## Results
I am using [Pascal Pons's test sets](http://blog.gamesolver.org/solving-connect-four/02-test-protocol/) for Connect 4 AIs to benchmark my different solvers.

### Min-Max *(using Negamax)*

| Test Set | Accuracy | Position representation | Execution time (mean) | Explored positions (mean) |
| -------- | -------- | ----------------------- | --------------------- | ------------------------- |
| L3 R1    | 100%     | `GridPosition`          | 32.7ms                | 11 024                    |
| L3 R1    | 100%     | `StackPosition`         | 32.4ms                | 11 024                    |

### Alpha-Beta *(using Negamax)*

| Test Set | Type   | Move order    | Accuracy | Position representation | Execution time (mean) | Explored positions (mean) |
| -------- | ------ | ------------- | -------- | ----------------------- | --------------------- | ------------------------- |
| L3 R1    | Strong | Left to right | 100%     | `GridPosition`          | 996μs                 | 283                       |
| L3 R1    | Strong | Center-first  | 100%     | `GridPosition`          | 525μs                 | 142                       |
| L3 R1    | Weak   | Left to right | 100%     | `GridPosition`          | 783μs                 | 222                       |
| L3 R1    | Weak   | Center-first  | 100%     | `GridPosition`          | 400μs                 | 110                       |

### Bitboard *(with Alpha-Beta)*
| Test Set | Type   | Move order    | Position representation | Execution time (mean) | Explored positions (mean) |
| -------- | ------ | ------------- | ----------------------- | --------------------- | ------------------------- |
| L3 R1    | Strong | Center-first  | `BitboardPosition`      | 50μs                  | 142                       |
| L2 R1    | Strong | Center-first  | `BitboardPosition`      | 403ms                 | 1 183 210                 |
| L3 R1    | Weak   | Center-first  | `BitboardPosition`      | 40μs                  | 110                       |
| L2 R1    | Weak   | Center-first  | `BitboardPosition`      | 277ms                 | 795 053                   |

### Transposition table *(with Alpha-Beta, Bitboard)*
| Test Set | Type   | Move order    | Position representation | Execution time (mean) | Explored positions (mean) |
| -------- | ------ | ------------- | ----------------------- | --------------------- | ------------------------- |
| L3 R1    | Strong | Center-first  | `BitboardPosition`      | 64μs                  | 140                       |
| L2 R1    | Strong | Center-first  | `BitboardPosition`      | 265ms                 | 696 568                   |
| L3 R1    | Weak   | Center-first  | `BitboardPosition`      | 48μs                  | 108                       |
| L2 R1    | Weak   | Center-first  | `BitboardPosition`      | 250ms                 | 639 428                   |

### Iterative deepening *(with Alpha-Beta, Bitboard, Transposition table)*
| Test Set | Type   | Move order    | Position representation | Execution time (mean) | Explored positions (mean) |
| -------- | ------ | ------------- | ----------------------- | --------------------- | ------------------------- |
| L3 R1    | Strong | Center-first  | `BitboardPosition`      | 82μs                  | 91 554                    |
| L2 R1    | Strong | Center-first  | `BitboardPosition`      | 5.7ms                 | 7 299 614                 |
| L3 R1    | Weak   | Center-first  | `BitboardPosition`      | 53μs                  | 113                       |
| L2 R1    | Weak   | Center-first  | `BitboardPosition`      | 273ms                 | 714 680                   |

### Loosing move anticipation *(with Alpha-Beta, Bitboard, Transposition table, Iterative deepening)*
| Test Set | Type   | Move order    | Position representation             | Execution time (mean) | Explored positions (mean) |
| -------- | ------ | ------------- | ----------------------------------- | --------------------- | ------------------------- |
| L3 R1    | Strong | Center-first  | `AnticipatingBitboardPosition`      | 60μs                  | 57 122                    |
| L2 R1    | Strong | Center-first  | `AnticipatingBitboardPosition`      | 3.3ms                 | 3 934 303                 |
<!--| L3 R1    | Weak   | Center-first  | `AnticipatingBitboardPosition`      | μs                  |                        |-->
<!--| L2 R1    | Weak   | Center-first  | `AnticipatingBitboardPosition`      | ms                 |                    |-->

## Workspace description
- [`game-board`](libs/game-board/) defines some basic traits: the `Position` trait, which represents a Connect 4 grid, and the `Solver` trait, that can play the game.
  - `GridPosition` is a naive implementation of `Position` using a bi-dimensionnal vector
  - `StackPosition` is a similar implementation, using a list-based approach instead of an array-based one. Performances are extremely close.
  - `BitboardPosition` is an optimised implementation using two `u64` to represent the board, and bitwise operations to speed up the needed functions. Performances are around 10 times faster than with the naive `GridPosition`.
- [`benchmark`](libs/benchmark/) is responsible for loading test sets and to execute the tests on a given `Solver`.
- [`min-max-solver`](libs/min-max-solver/) is the first solver that I implemented, using the Negamax variant of the Min-Max algorithm.
- [`alpha-beta-solver`](libs/alpha-beta-solver/) contains all variants of the Alpha-Beta algorithm. The different solvers are:
  - [Default](libs/alpha-beta-solver/alpha_beta_solver.rs): vanilla Alpha-Beta.
  - [Transposition table](libs/alpha-beta-solver/alpha_beta_with_transposition_table.rs): uses a Transposition table to save previously explored positions. Increases both memory usage but decreases execution time.
  - [Iterative deepening](libs/alpha-beta-solver/alpha_beta_with_iterative_deepening.rs): uses a dichotomic approach to progressively increase the depth of search. The possible range for the score is narrowed using the Null Window  Search method.


## Running
You can check that everything is working by running:
```console
$ cargo test
```
This will execute unit tests for each component of the workspace.

The main program can be executed using:
```console
$ cargo run [args]
```
where `[args]` is the list of the arguments needed for the benchmark. 

The general arguments list goes as follows:
```console
$ cargo run solver weak position move_ordering L R
```
With:
- `solver`: the solver type. Choose between `min_max`, `alpha_beta`, `alpha_beta_with_transposition`, and `alpha_beta_with_iterative_deepening`.
- `weak`: compute the numbers of move until the end (strong) or only the winner (weak). Choose between `strong` and `weak`.
- `position`: the representation of the board. Choose between `grid`, `stack` and `bitboard`.
- `move_ordering`: the order of the moves. Impactful only for Alpha-Beta-based solvers. Choose between `left_to_right`, and `center_first`.
- `L`: the overall state of the game in the test dataset. Choose between 1, 2 and 3, where 3 is the easiest.
- `R`: the overall difficulty of the game in the test dataset. Choose between 1, 2 and 3, where 3 is the easiest. Some ratings aren't available depending on `L`.

For instance:
```console
$ cargo run alpha_beta_with_transposition weak bitboard center_first 3 1
```
launches a benchmark of the Alpha-Beta solver that uses a transposition table, in Weak mode. Positions will be represented with a Bitboard, and moves will be explored starting from the center columns. The benchmark will execute the dataset `L3 R1`.

## License
This work is licensed under the [CC-BY-NC-SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/) license.