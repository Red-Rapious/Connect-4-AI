use lib_game_board::{bitboard_position_with_ordering::BitboardPositionWithOrdering, grid_position::GridPosition, Position, sequence_position::SequencePosition, Cell};
use lib_alpha_beta_solver::final_alpha_beta::FinalAlphaBeta;

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

        println!("Loading game files...");
        let mut solver = FinalAlphaBeta::new(7, 6, vec![3, 4, 2, 5, 1, 6, 0]);
        solver.load_opening_book("libs/alpha-beta-solver/opening-books/7x6_small.book");

        self.position = Position::from_seq(
            &SequencePosition::from(&
                "37313333717124171162542"
                .to_string())
        );

        while self.position.nb_moves() < 42 {
            loop {
                let column = match self.position.nb_moves() % 2 {
                    // Human's turn
                    0 => { 
                        self.display_board(); 
                        let column = Self::ask_position();
                        println!("You played in column {}.", column + 1);
                        column
                    }, 
                    // AI's turn
                    1 => 
                    {
                        let (_score, column) = solver.solve(&self.position);
                        println!("AI played in column {}.", column + 1);
                        column
                    },
                    _ => panic!()
                };

                match self.play(column) {
                    Ok(()) => break,
                    Err(()) => println!("You cannot play in the column {}.", column + 1)
                };
            }

            let winner = GridPosition::from(&self.position).winning();
            if winner == Cell::Red {
                self.display_board();
                println!("\n\n{}\x1b[31;1mRED PLAYER WINS!\x1b[0m", Self::left_shift(29));
                return;
            } else if winner == Cell::Yellow {
                self.display_board();
                println!("\n\n{}\x1b[93;1m ===\x1b[0m YELLOW PLAYER WINS! \x1b[93;1m=== \x1b[0m", Self::left_shift(29));
                return;
            }
        }
    }

    fn display_board(&self) {
        use lib_game_board::Cell::*;
        let grid_position = GridPosition::from(&self.position);

        let left_shift = Self::left_shift(self.position.width()*4+1);

        for line in (0..self.position.height()).rev() {
            print!("{}", left_shift);
            for column in 0..self.position.width() {
                print!("{} {} ",
                    if column == 0 {
                        "\x1b[1m|\x1b[0m"
                    } else {
                        "|"
                    },
                    match grid_position.grid()[line][column] {
                        Empty => " ",
                        Red => "\x1b[31;1mX\x1b[0m",
                        Yellow => "\x1b[93;1mO\x1b[0m"
                    }
                );
            }
            println!("\x1b[1m|\x1b[0m");
            print!("{}", left_shift);
            if line == 0 {
                print!("\x1b[1m{}\x1b[0m", "-".repeat(self.position.width()*4+1));
            } else {
                print!("{}", "-".repeat(self.position.width()*4+1));
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