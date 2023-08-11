# Connect-4-AI
An artificial intelligence to perfectly solve the game of Connect Four.

## About
My goal is to create a perfect Connect Four bot, capable of predicting the outcome of a game in a reasonable computing time. 

## Results
I am using [Pascal Pons's test sets](http://blog.gamesolver.org/solving-connect-four/02-test-protocol/) for Connect 4 AIs to benchmark my different solvers.

### Min-Max (using Negamax)

| Test Set | Accuracy | Position representation | Execution time (mean) | Explored positions (mean) |
| -------- | -------- | ----------------------- | --------------------- | ------------------------- |
| L3 R1    | 100%     | `GridPosition`          | 32.7ms                | 11 024                    |
| L3 R1    | 100%     | `StackPosition`         | 32.4ms                | 11 024                    |

### Alpha-Beta (using Negamax)

| Test Set | Type   | Move order   |Accuracy | Position representation | Execution time (mean) | Explored positions (mean) |
| -------- | ------ | ------------ | ------- | ----------------------- | --------------------- | ------------------------- |
| L3 R1    | Strong | Default      | 100%    | `GridPosition`          | 996μs                 | 283                       |
| L3 R1    | Weak   | Default      | 100%    | `GridPosition`          | 783μs                 | 222                       |
| L3 R1    | Strong | Center-first |100%     | `GridPosition`          | 514μs                 | 142                       |
| L3 R1    | Weak   | Center-first |100%     | `GridPosition`          | 383μs                 | 110                       |

<!--| L2 R1 | Strong | 100% | `GridPosition` | TODO | TODO |-->
<!--| L2 R1 | Weak | 100% | `GridPosition` | TODO | TODO |-->

## Workspace description
- [`game-board`](libs/game-board/) defines some basic traits: the `Position` trait, which represents a Connect 4 grid, and the `Solver` trait, that can play the game.
  - `GridPosition` is a naive implementation of `Position` using a bi-dimensionnal vector
  - `StackPosition` is a similar implementation, using a list-based approach instead of an array-based one. Performances are extremely close.
- [`benchmark`](libs/benchmark/) is responsible for loading test sets and to execute the tests on a given `Solver`.
- [`min-max-solver`](libs/min-max-solver/) is the first solver that I implemented, using the Negamax variant of the Min-Max algorithm.


## Running
You can check that everything is working by running:
```console
$ cargo test
```
This will execute unit tests for each component of the workspace.

To launch the main program, run:
```console
$ cargo run
```

## License
This work is licensed under the [CC-BY-NC-SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/) license.