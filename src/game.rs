use super::board;

pub struct Game {
    board: board::Board,
    current_player: board::Player,
    winner: Option<board::Player>,
}

const ROWS: usize = 6;
const COLS: usize = 7;

impl Game {
    pub fn new(first_player: board::Player) -> Self {
        Game {
            board: board::Board::new(ROWS, COLS),
            current_player: first_player,
            winner: None,
        }
    }

    pub fn play(&mut self, col: usize) -> Result<(), board::PlayErr> {
        self.board.play(col, self.current_player)?;
        self.current_player = match self.current_player {
            board::Player::YELLOW => board::Player::RED,
            board::Player::RED => board::Player::YELLOW,
        };
        Ok(())
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
}
