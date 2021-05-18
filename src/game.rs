use super::board;

pub struct Game {
    board: board::Board,
    current_player: board::Player,
}

const ROWS: usize = 6;
const COLS: usize = 7;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GameState {
    PLAYING,
    TIE,
    WINNER,
}

impl Game {
    pub fn new(first_player: board::Player) -> Self {
        Game {
            board: board::Board::new(ROWS, COLS),
            current_player: first_player,
        }
    }

    pub fn play(&mut self, col: usize) -> Result<GameState, board::PlayErr> {
        self.board.play(col, self.current_player)?;
        self.current_player = match self.current_player {
            board::Player::YELLOW => board::Player::RED,
            board::Player::RED => board::Player::YELLOW,
        };
        if self.board.is_board_full() {
            return Ok(GameState::TIE);
        }
        Ok(GameState::PLAYING)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn game_construction() {
        let game = Game::new(board::Player::YELLOW);
        assert_eq!(game.current_player, board::Player::YELLOW);
    }

    #[test]
    fn toggle_player_on_play() -> Result<(), board::PlayErr> {
        let mut game = Game::new(board::Player::YELLOW);
        let col = 1;
        game.play(col)?;
        assert_eq!(game.current_player, board::Player::RED);

        game.play(col)?;
        assert_eq!(game.current_player, board::Player::YELLOW);

        Ok(())
    }

    #[test]
    fn game_finish_in_tie() -> Result<(), board::PlayErr> {
        let mut game = Game::new(board::Player::YELLOW);
        let mut game_state = GameState::PLAYING;
        for c in 0..COLS {
            for _ in 0..ROWS {
                assert_eq!(game_state, GameState::PLAYING);
                game_state = game.play(c)?;
            }
        }

        assert_eq!(game_state, GameState::TIE);

        Ok(())
    }
}
