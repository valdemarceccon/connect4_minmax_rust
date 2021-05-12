use super::board::{Board, Players, COLS};

#[derive(Debug)]
pub struct GameState {
    pub board: Board,
    current_player: Players,
    winner: Option<Players>,
}

pub enum GameResult {
    ENDED,
    RUNNING,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            board: Board::new(),
            current_player: Players::YELLOW,
            winner: None,
        }
    }

    pub fn play(&mut self, col: i8) -> Result<(), &str> {
        if !self.board.is_play_valid(col) {
            return Err("Invalid play");
        }

        self.board.play(self.current_player.clone(), col as usize);
        self.toggle_player();
        Ok(())
    }

    fn toggle_player(&mut self) {
        self.current_player = match self.current_player {
            Players::RED => Players::YELLOW,
            Players::YELLOW => Players::RED,
        }
    }

    pub fn get_valid_plays(&self) -> Vec<usize> {
        let mut ret = Vec::new();
        for col in 0..COLS {
            match self.board.piece_at(0, col) {
                None => ret.push(col),
                _ => (),
            }
        }
        ret
    }

    pub fn get_current_player(&self) -> &Players {
        &self.current_player
    }
}
