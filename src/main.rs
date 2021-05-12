mod board;
mod game;

use board::Players;
use colored::*;
use game::GameState;

use io::Write;
use std::io;

const TABLE_HOR_LINE: char = '\u{2500}';
const TABLE_VER_LINE: char = '\u{2502}';
const TABLE_TOP_LEFT_CORNER: char = '\u{250c}';
const TABLE_TOP_RIGHT_CORNER: char = '\u{2510}';
const TABLE_BOTTOM_LEFT_CORNER: char = '\u{2514}';
const TABLE_BOTTOM_RIGHT_CORNER: char = '\u{2518}';
const TABLE_TOP_SPLIT: char = '\u{252c}';
const TABLE_BOTTOM_SPLIT: char = '\u{2534}';
const TABLE_LEFT_MIDDLE: char = '\u{251c}';
const TABLE_RIGHT_MIDDLE: char = '\u{2524}';
const TABLE_CROSS: char = '\u{253c}';
const P1_EMOJI: &str = "\u{1f535}";
const P2_EMOJI: &str = "\u{1f534}";

fn main() {
    let mut game = GameDisplay::new();
    loop {
        match game.run() {
            game::GameResult::RUNNING => (),
            game::GameResult::ENDED => {
                break;
            }
        }
    }
}

struct GameDisplay {
    game_state: GameState,
    message: Option<String>,
    last_input: Option<String>,
}

impl GameDisplay {
    pub fn new() -> Self {
        GameDisplay {
            game_state: GameState::new(),
            message: None,
            last_input: None,
        }
    }

    pub fn exec_play(&mut self) {
        match self.last_input.as_ref() {
            Some(input) => match input.parse::<i8>() {
                Ok(val) => {
                    let v = val - 1;
                    match self.game_state.play(v) {
                        Ok(()) => self.message = None,
                        Err(err) => self.message = Some(err.to_string()),
                    }
                }
                Err(_) => {
                    self.message = Some(String::from("Invalid play"));
                }
            },
            None => (),
        }
    }

    fn display_prompt(&mut self) {
        print!("{} turn to play: ", self.get_current_player_display());
        io::stdout().flush().unwrap();
        let buf = &mut String::new();
        match io::stdin().read_line(buf) {
            Ok(_) => self.last_input = Some(buf.trim().to_owned()),
            Err(err) => {
                self.message = Some(err.to_string());
            }
        }
    }

    fn get_current_player_display(&self) -> &str {
        match self.game_state.get_current_player() {
            Players::YELLOW => P1_EMOJI,
            Players::RED => P2_EMOJI,
        }
    }

    fn display_board(&mut self) {
        match self.message.as_mut() {
            Some(msg) => println!("{}", &msg),
            None => (),
        };
        print_header();
        for row_i in 0..board::ROWS {
            print!("{}", TABLE_VER_LINE);
            for col_i in 0..board::COLS {
                print!(
                    "{}{}",
                    match self.game_state.board.piece_at(row_i, col_i) {
                        Some(Players::YELLOW) => P1_EMOJI,
                        Some(Players::RED) => P2_EMOJI,
                        None => "  ",
                    },
                    TABLE_VER_LINE
                );
            }
            println!();
            if row_i != board::ROWS - 1 {
                print_inner_lines();
            }
        }
        print_footer();
    }

    pub fn run(&mut self) -> game::GameResult {
        print!("\x1B[2J");
        self.display_board();
        self.display_prompt();
        self.exec_play();
        game::GameResult::RUNNING
    }
}

fn print_header() {
    print!("{}", TABLE_TOP_LEFT_CORNER);
    for c in 1..board::COLS * 3 {
        if c % 3 == 0 {
            print!("{}", TABLE_TOP_SPLIT);
        } else {
            print!("{}", TABLE_HOR_LINE);
        }
    }
    print!("{}", TABLE_TOP_RIGHT_CORNER);
    println!();
}

fn print_inner_lines() {
    print!("{}", TABLE_LEFT_MIDDLE);
    for c in 1..board::COLS * 3 {
        if c % 3 == 0 {
            print!("{}", TABLE_CROSS);
        } else {
            print!("{}", TABLE_HOR_LINE);
        }
    }
    print!("{}", TABLE_RIGHT_MIDDLE);
    println!();
}

fn print_footer() {
    print!("{}", TABLE_BOTTOM_LEFT_CORNER);
    for c in 1..board::COLS * 3 {
        if c % 3 == 0 {
            print!("{}", TABLE_BOTTOM_SPLIT);
        } else {
            print!("{}", TABLE_HOR_LINE);
        }
    }
    print!("{}", TABLE_BOTTOM_RIGHT_CORNER);

    println!();
}
