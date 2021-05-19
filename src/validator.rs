use super::board;
use super::board_iterators;

pub fn get_winner(board: &board::Board) -> Option<board::Player> {
    find_winner_in_rows(board)
        .or_else(|| find_winner_in_cols(board))
        .or_else(|| find_winner_in_diagonals(board))
}

fn find_winner_in_rows(board: &board::Board) -> Option<board::Player> {
    let rit = board_iterators::RowIterator::new(board);
    for row in rit {
        for w in row.windows(4) {
            if four_player_in_a_row(w, board::Player::Yellow) {
                return Some(board::Player::Yellow);
            }
            if four_player_in_a_row(w, board::Player::Red) {
                return Some(board::Player::Red);
            }
        }
    }
    None
}

fn find_winner_in_cols(board: &board::Board) -> Option<board::Player> {
    let rit = board_iterators::ColIterator::new(board);
    for row in rit {
        for w in row.windows(4) {
            if four_player_in_a_row(w, board::Player::Yellow) {
                return Some(board::Player::Yellow);
            }
            if four_player_in_a_row(w, board::Player::Red) {
                return Some(board::Player::Red);
            }
        }
    }
    None
}

fn find_winner_in_diagonals(board: &board::Board) -> Option<board::Player> {
    let it = board_iterators::MainDiagonalIterator::new(board)
        .chain(board_iterators::SecondaryDiagonalIterator::new(board));
    for row in it {
        for w in row.windows(4) {
            if four_player_in_a_row(w, board::Player::Yellow) {
                return Some(board::Player::Yellow);
            }
            if four_player_in_a_row(w, board::Player::Red) {
                return Some(board::Player::Red);
            }
        }
    }
    None
}

fn four_player_in_a_row(w: &[&Option<board::Player>], p: board::Player) -> bool {
    w == [&Some(p), &Some(p), &Some(p), &Some(p)]
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup_board(plays: Vec<usize>) -> Result<board::Board, board::PlayErr> {
        let mut b = board::Board::new(5, 5);
        let mut last_player = board::Player::Yellow;
        for p in plays {
            b.play(p, last_player)?;
            last_player = match last_player {
                board::Player::Yellow => board::Player::Red,
                board::Player::Red => board::Player::Yellow,
            };
        }
        Ok(b)
    }

    #[test]
    fn there_is_no_winner() -> Result<(), board::PlayErr> {
        let board = setup_board(vec![1, 1, 1, 1, 1, 2, 2, 2, 2, 2])?;
        let winner = get_winner(&board);

        assert_eq!(winner, None);

        Ok(())
    }

    #[test]
    fn there_is_a_winner_in_a_colunm() -> Result<(), board::PlayErr> {
        let board = setup_board(vec![1, 2, 1, 2, 1, 2, 1])?;
        let winner = find_winner_in_cols(&board);

        assert_ne!(winner, None);

        Ok(())
    }

    #[test]
    fn there_is_a_winner_in_a_row() -> Result<(), board::PlayErr> {
        let board = setup_board(vec![1, 1, 2, 2, 3, 3, 4])?;
        let winner = find_winner_in_rows(&board);

        assert_ne!(winner, None);

        Ok(())
    }

    #[test]
    fn there_is_a_winner_in_the_main_diagonal() -> Result<(), board::PlayErr> {
        let board = setup_board(vec![1, 2, 2, 3, 3, 1, 3, 4, 4, 4, 4])?;
        let winner = find_winner_in_diagonals(&board);

        assert_ne!(winner, None);

        Ok(())
    }

    #[test]
    fn there_is_a_winner_in_the_secundary_diagonal() -> Result<(), board::PlayErr> {
        let board = setup_board(vec![1, 1, 1, 1, 2, 2, 3, 2, 1, 3, 3, 4])?;
        let winner = find_winner_in_diagonals(&board);

        assert_ne!(winner, None);

        Ok(())
    }
}
