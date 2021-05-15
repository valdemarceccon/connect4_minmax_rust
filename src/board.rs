pub struct Board {
    rows: usize,
    cols: usize,
    pieces: Vec<Option<Player>>,
}

#[derive(Debug, PartialEq)]
pub enum PlayErr {
    FullColumn,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Player {
    YELLOW,
    RED,
}

impl Board {
    pub fn new(rows: usize, cols: usize) -> Self {
        let pieces = (0..cols * rows).map(|_| None).collect();
        Board { rows, cols, pieces }
    }

    pub fn play(&mut self, col: usize, p: Player) -> Result<(), PlayErr> {
        let first_not_empty = self.find_empty_row_in_column(col);
        match first_not_empty {
            Some(n) => {
                self.set_piece_at(n, col, p);
                Ok(())
            }
            None => Err(PlayErr::FullColumn),
        }
    }

    fn find_empty_row_in_column(&self, col: usize) -> Option<usize> {
        let row = self
            .pieces
            .iter()
            .skip(col)
            .step_by(self.cols)
            .position(|p| *p != None)
            .or(Some(self.rows));

        match row {
            Some(0) | None => None,
            Some(n) => Some(n - 1),
        }
    }

    pub fn get_piece_at(&self, row: usize, col: usize) -> Option<Player> {
        if row >= self.rows || col >= self.cols {
            return None;
        }

        match self.pieces[self.calc_index(row, col)] {
            Some(player) => Some(player),
            None => None,
        }
    }

    pub fn set_piece_at(&mut self, row: usize, col: usize, player: Player) {
        self.pieces[row * self.cols + col] = Some(player);
    }

    fn calc_index(&self, row: usize, col: usize) -> usize {
        (self.cols * row) + col
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const ROWS: usize = 6;
    const COLS: usize = 6;
    #[test]
    fn create_new_instance() {
        let b = Board::new(ROWS, COLS);
        assert_eq!(b.pieces.capacity(), ROWS * COLS);

        for i in 0..ROWS * COLS {
            assert_eq!(None, b.pieces[i]);
        }
    }
    #[test]
    fn find_row_when_searching_for_empty_col_in_a_empty_col() {
        let board = Board::new(ROWS, COLS);

        let found_row = board.find_empty_row_in_column(0);
        assert_eq!(found_row, Some(ROWS - 1));
    }

    fn target_index(row: usize, col: usize) -> usize {
        row * COLS + col
    }

    #[test]
    fn find_row_when_searching_for_non_empty_col_in_a_empty_col() {
        let mut board = Board::new(ROWS, COLS);
        board.pieces[target_index(ROWS - 1, 0)] = Some(Player::RED);
        let found_row = board.find_empty_row_in_column(0);
        assert_eq!(found_row, Some(ROWS - 2));
    }

    #[test]
    fn find_row_when_searching_for_full_col_in_a_empty_col() {
        let mut board = Board::new(ROWS, COLS);
        for r in 0..ROWS {
            board.pieces[target_index(r, 0)] = Some(Player::RED);
        }
        let found_row = board.find_empty_row_in_column(0);
        assert_eq!(found_row, None);
    }

    #[test]
    fn get_piece_at() -> Result<(), PlayErr> {
        let mut board = Board::new(ROWS, COLS);
        board.play(0, Player::YELLOW)?;
        assert_eq!(board.get_piece_at(ROWS - 1, 0), Some(Player::YELLOW));
        board.play(1, Player::YELLOW)?;
        assert_eq!(board.get_piece_at(ROWS - 1, 1), Some(Player::YELLOW));
        board.play(0, Player::YELLOW)?;
        assert_eq!(board.get_piece_at(ROWS - 2, 0), Some(Player::YELLOW));
        Ok(())
    }

    #[test]
    fn play_not_empty_column() -> Result<(), PlayErr> {
        let mut board = Board::new(ROWS, COLS);
        board.play(0, Player::YELLOW)?;
        board.play(0, Player::RED)?;

        if let Some(p) = board.pieces[(ROWS - 2) * COLS] {
            assert_eq!(p, Player::RED);
        } else {
            panic!("should not have failed {:?}", board.pieces);
        }

        Ok(())
    }
}
