pub mod grid_position;
pub mod sequence_position;

pub trait Position {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn can_play(&self, column: usize) -> bool;
    fn play(&mut self, column: usize, player: Cell);
    fn winning(&self) -> Cell;
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Cell {
    Red,
    Yellow,
    Empty
}

pub trait Solver{
    fn solve(&self, position: &impl Position) -> i32;
}