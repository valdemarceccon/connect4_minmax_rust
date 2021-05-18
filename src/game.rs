use super::board;

pub struct Game {
    board: board::Board,
    current_player: board::Player,
}

const ROWS: usize = 6;
const COLS: usize = 7;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GameState {
    Playing,
    Tie,
    Winner,
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
            board::Player::Yellow => board::Player::Red,
            board::Player::Red => board::Player::Yellow,
        };
        if self.board.is_board_full() {
            return Ok(GameState::Tie);
        }
        Ok(GameState::Playing)
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
        let mut game = Game::new(board::Player::Yellow);
        let mut game_state = GameState::Playing;
        for c in 0..COLS {
            for _ in 0..ROWS {
                assert_eq!(game_state, GameState::Playing);
                game_state = game.play(c)?;
            }
        }

        assert_eq!(game_state, GameState::Tie);

        Ok(())
    }
}
