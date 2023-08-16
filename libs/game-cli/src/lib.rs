use lib_game_board::{bitboard_position_with_ordering::BitboardPositionWithOrdering, grid_position::GridPosition, Position};

pub struct GameCLI {
    position: BitboardPositionWithOrdering
}

impl GameCLI {
    pub fn new(width: usize, height: usize) -> Self {
        Self { position: BitboardPositionWithOrdering::new(width, height) }
    }

    pub fn run_game(&mut self) {
        println!(r"{}   ___                            _           _ _    ", Self::left_shift(53));
        println!(r"{}  / __| ___  _ _   _ _   ___  __ | |_        | | |   ", Self::left_shift(53));
        println!(r"{} | (__ / _ \| ' \ | ' \ / -_)/ _||  _|  ===  |_  _|  ", Self::left_shift(53));
        println!(r"{}  \___|\___/|_||_||_||_|\___|\__| \__|         |_|   ", Self::left_shift(53));
        println!("\n\n");

        let mut turn = 0;

        while turn < 42 {
            loop {
                let column = match turn % 2 {
                    0 => { self.display_board(); Self::ask_position() },
                    1 => 0,
                    _ => panic!()
                };

                match self.play(column) {
                    Ok(()) => break,
                    Err(()) => println!("You cannot play in the column {}.", column + 1)
                };
            }

            turn += 1;
        }
    }

    fn display_board(&self) {
        use lib_game_board::Cell::*;
        let grid_position = GridPosition::from(&self.position);

        let left_shift = Self::left_shift(self.position.width()*4+1);

        for line in (0..self.position.height()).rev() {
            print!("{}", left_shift);
            for column in 0..self.position.width() {
                print!("| {} ",
                    match grid_position.grid()[line][column] {
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

    fn play(&mut self, column: usize) -> Result<(), ()> {
        match self.position.can_play(column) {
            true => Ok(self.position.play(column)),
            false => Err(())
        }
    }

    fn ask_position() -> usize {
        let mut column: usize;
        loop {
            println!("Choose a column to play (between 1 and 7):");
            let mut input = String::new();
            match std::io::stdin().read_line(&mut input) {
                Ok(_) => (),
                Err(_) => {
                    println!("Please enter something.");
                    continue
                }
            }
            match input.replace("\n", "").parse() {
                Ok(value) => column = value,
                Err(_) => {
                    println!("Please enter a number.");
                    continue
                }
            }
    
            if 1 <= column && column <= 7 { 
                break; 
            } else {
                println!("Please enter a column number that is between 1 and 7.\n");
            }
        }
    
        column - 1 // start indexing the array at 0
    }
    
    fn left_shift(element_width: usize) -> String {
        let terminal_columns = termsize::get().unwrap().cols;
        let number_spaces_before = (terminal_columns as usize - element_width)/2;
        " ".repeat(number_spaces_before)
    }
}