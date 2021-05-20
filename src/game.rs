use super::board;
use super::validator;
use crate::game::GameState::Playing;
use crate::board::PlayErr;

pub struct Game {
    board: board::Board,
    pub state: GameState,
    pub current_player: board::Player,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GameState {
    Playing,
    Tie,
    Winner(board::Player),
}

const ROWS: usize = 6;
const COLS: usize = 7;

impl Game {
    pub fn new(first_player: board::Player) -> Self {
        Game {
            board: board::Board::new(ROWS, COLS),
            current_player: first_player,
            state: Playing,
        }
    }

    pub fn with_size(rows: usize, cols: usize, first_player: board::Player) -> Self {
        Game {
            board: board::Board::new(rows, cols),
            current_player: first_player,
            state: Playing,
        }
    }

    pub fn play(&mut self, col: usize) -> Result<(), PlayErr> {
        if self.state != Playing {
            return Ok(());
        }
        self.board.play(col, self.current_player)?;

        if let Some(player) = validator::get_winner(&self.board) {
            self.state = GameState::Winner(player);
        }

        if self.board.is_board_full() {
            self.state = GameState::Tie;
        }

        self.current_player = match self.current_player {
            board::Player::Yellow => board::Player::Red,
            board::Player::Red => board::Player::Yellow,
        };

        Ok(())
    }

    pub fn get_board(&self) -> &board::Board {
        &self.board
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn game_construction() {
        let game = Game::new(board::Player::Yellow);
        assert_eq!(game.current_player, board::Player::Yellow);
    }

    #[test]
    fn toggle_player_on_play() -> Result<(), board::PlayErr> {
        let mut game = Game::new(board::Player::Yellow);
        let col = 1;
        game.play(col)?;
        assert_eq!(game.current_player, board::Player::Red);

        game.play(col)?;
        assert_eq!(game.current_player, board::Player::Yellow);

        Ok(())
    }

    #[test]
    fn game_finish_in_tie() -> Result<(), board::PlayErr> {
        let mut game = Game::with_size(3, 3, board::Player::Yellow);
        let mut game_state = GameState::Playing;
        for c in 0..game.board.get_columns() {
            for _ in 0..game.board.get_rows() {
                assert_eq!(game_state, GameState::Playing);
                game.play(c)?;
                game_state = game.state;
            }
        }

        assert_eq!(game_state, GameState::Tie);

        Ok(())
    }

    #[test]
    fn does_not_change_state_when_some_error_occours() -> Result<(), board::PlayErr> {
        let mut game = Game::with_size(3, 3, board::Player::Yellow);
        game.play(0)?;
        game.play(0)?;
        game.play(0)?;
        let r = game.play(0);
        let current_player = game.current_player;
        let current_winner = game.state;
        let current_board = game.board.clone();
        assert_eq!(Err(board::PlayErr::FullColumn), r);

        assert_eq!(current_player, game.current_player);
        assert_eq!(current_winner, game.state);
        assert_eq!(current_board.get_pieces(), game.board.get_pieces());

        Ok(())
    }
}
