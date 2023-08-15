use lib_game_board::{grid_position::GridPosition, Position};

pub struct GameCLI {
    position: GridPosition
}

impl GameCLI {
    pub fn new(width: usize, height: usize) -> Self {
        Self { position: GridPosition::new(width, height) }
    }

    pub fn display_board(&self) {
        use lib_game_board::Cell::*;
        let left_shift = "      ";

        for line in (0..self.position.height()).rev() {
            print!("{}", left_shift);
            for column in 0..self.position.width() {
                print!("| {} ",
                    match self.position.grid()[line][column] {
                        Empty => " ",
                        Red => "X",
                        Yellow => "O"
                    }
                );
            }
            println!("|");
            print!("{}", left_shift);
            for _ in 0..self.position.width()*4+1 {
                print!("-");
            }
            println!("");
        }

        print!("{}", left_shift);
        for column in 0..self.position.width() {
            print!("  {} ", column+1);
        }
        println!("");
    }

    pub fn play(&mut self, column: usize) -> Result<(), ()> {
        match self.position.can_play(column) {
            true => Ok(self.position.play(column)),
            false => Err(())
        }
    }
}

#[cfg(test)]
mod game_cli_tests {
    use super::*;

    #[test]
    fn test_display() {
        let mut game_cli = GameCLI::new(7, 6);
        game_cli.position.play(0);
        game_cli.position.play(0);
        game_cli.position.play(3);
        game_cli.display_board();
    }
}