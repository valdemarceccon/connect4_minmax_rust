use std::io;
use std::io::Write;

use colored::Colorize;
use connect4_core::ai;
use connect4_core::board;
use connect4_core::game;

const STARTING_PLAYER: board::Player = board::Player::Yellow;

fn main() {
    let ai_player = STARTING_PLAYER;

    let mut game = game::Game::new(board::Player::Red);
    print!("\x1B[2J");
    display_board(game.get_board(), None);

    while game.state == game::GameState::Playing {
        let msg = if game.current_player == ai_player {
            print!("thinking...");
            io::stdout().flush().unwrap();
            let ai_move = ai::get_ai_move(&game);
            println!();
            make_play(&mut game, ai_move)
        } else {
            match read_input(game.current_player) {
                Ok(0) => Some(String::from("Jogada invalida")),
                Ok(v) => make_play(&mut game, v - 1),
                Err(e) => Some(String::from(e)),
            }
        };

        if game.state != game::GameState::Playing {
            print!("\x1B[2J");
            display_board(game.get_board(), msg);
            print!("Again? (Y|n): ");
            io::stdout().flush().unwrap();
            let buf = &mut String::new();
            if io::stdin().read_line(buf).is_ok() {
                match buf.trim() {
                    "Y" | "y" | "" => game = game::Game::new(STARTING_PLAYER),
                    _ => (),
                }
            }

            print!("\x1B[2J");
            display_board(game.get_board(), None);
        } else {
            print!("\x1B[2J");
            display_board(game.get_board(), msg);
        }
    }
}

fn make_play(game: &mut game::Game, m: usize) -> Option<String> {
    match game.play(m) {
        Ok(()) => match game.state {
            game::GameState::Winner(p) => match p {
                board::Player::Red => Some(format!("{} ganhou", CIRCLE.red())),
                board::Player::Yellow => Some(format!("{} ganhou", CIRCLE.yellow())),
            },
            game::GameState::Tie => Some(String::from("Empate")),
            game::GameState::Playing => None,
        },
        Err(e) => match e {
            board::PlayErr::FullColumn => Some(String::from("coluna cheia")),
            board::PlayErr::OutOfBounds => Some(String::from("jogada invalida")),
        },
    }
}

fn read_input<'a>(player: board::Player) -> Result<usize, &'a str> {
    let p = match player {
        board::Player::Yellow => format!("{}", CIRCLE.yellow()),
        board::Player::Red => format!("{}", CIRCLE.red()),
    };
    print!("{} turn to play: ", p);
    io::stdout().flush().unwrap();
    let buf = &mut String::new();
    match io::stdin().read_line(buf) {
        Ok(_) => buf.trim().parse().map_err(|_| "invalid play"),
        Err(_) => Err("problema"),
    }
}

const HOR_LINE: &str = "\u{2500}";
const VER_LINE: &str = "\u{2502}";
const CORNER_TOP_LEFT: &str = "\u{250c}";
const CORNER_TOP_RIGHT: &str = "\u{2510}";
const CORNER_BOTTOM_RIGHT: &str = "\u{2518}";
const CORNER_BOTTOM_LEFT: &str = "\u{2514}";
const TOP_SPLIT: &str = "\u{252c}";
const BOTTOM_SPLIT: &str = "\u{2534}";
const LEFT_SPLIT: &str = "\u{251c}";
const RIGHT_SPLIT: &str = "\u{2524}";
const MIDDLE_SPLIT: &str = "\u{253c}";
const CIRCLE: &str = "\u{2B24}";

fn display_board(board: &board::Board, msg: Option<String>) {
    if let Some(msg) = msg {
        println!("{}", msg);
    }
    print_header(board);
    for row in 0..board.get_rows() {
        for col in 0..board.get_columns() {
            match board.get_piece_at(row, col) {
                None => print!("{}  ", VER_LINE),
                Some(board::Player::Red) => print!("{}{} ", VER_LINE, CIRCLE.red()),
                Some(board::Player::Yellow) => print!("{}{} ", VER_LINE, CIRCLE.yellow()),
            }
        }
        print!("{}", VER_LINE);
        println!();
        if row < board.get_rows() - 1 {
            print_inner_row(board);
        }
    }

    print_bottom(board);
}

fn print_bottom(board: &board::Board) {
    print!("{}", CORNER_BOTTOM_LEFT);
    for c in 1..board.get_columns() * 3 {
        if c % 3 == 0 {
            print!("{}", BOTTOM_SPLIT);
        } else {
            print!("{}", HOR_LINE);
        }
    }
    print!("{}", CORNER_BOTTOM_RIGHT);
    println!();
}

fn print_inner_row(board: &board::Board) {
    print!("{}", LEFT_SPLIT);
    for c in 1..board.get_columns() * 3 {
        if c % 3 != 0 {
            print!("{}", HOR_LINE);
        } else {
            print!("{}", MIDDLE_SPLIT);
        }
    }
    println!("{}", RIGHT_SPLIT);
}

fn print_header(board: &board::Board) {
    print!(" ");
    for c in 0..board.get_columns() {
        print!(" {} ", c + 1);
    }
    println!();
    print!("{}", CORNER_TOP_LEFT);
    for c in 1..board.get_columns() * 3 {
        if c % 3 == 0 {
            print!("{}", TOP_SPLIT);
        } else {
            print!("{}", HOR_LINE);
        }
    }
    print!("{}", CORNER_TOP_RIGHT);
    println!();
}
