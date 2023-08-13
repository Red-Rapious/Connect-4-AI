use sequence_position::SequencePosition;

pub mod grid_position;
pub mod sequence_position;
pub mod stack_position;
pub mod bitboard_position;

const FIRST_PLAYER: Cell = Cell::Red;

pub trait Position {
    fn player_turn(&self) -> Cell;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn can_play(&self, column: usize) -> bool;
    fn play(&mut self, column: usize);
    //fn winning(&self) -> Cell;
    fn is_winning_move(&self, column: usize) -> bool;
    fn nb_moves(&self) -> usize;
    fn from_seq(sequence: &SequencePosition) -> Self;
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Cell {
    Red,
    Yellow,
    Empty
}

impl Cell {
    pub fn swap_turn(&self) -> Self {
        assert_ne!(*self, Cell::Empty);
        use Cell::*;

        match *self {
            Empty => panic!(),
            Red => Yellow,
            Yellow => Red
        }
    }
}

pub trait Solver{
    fn solve(&mut self, position: &(impl Position + Clone)) -> i32;
    fn explored_positions(&self) -> usize;
    fn reset_explored_positions(&mut self);
}

pub trait WeakSolver{
    fn weak_solve(&mut self, position: &(impl Position + Clone)) -> i32;
    fn explored_positions(&self) -> usize;
    fn reset_explored_positions(&mut self);
}