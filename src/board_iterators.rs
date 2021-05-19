use super::board;
use std::cmp;

macro_rules! create_board_iterator {
    ($name:ident) => {
        pub struct $name<'a> {
            board: &'a board::Board,
            pos: usize,
        }

        impl<'a> $name<'a> {
            pub fn new(board: &'a board::Board) -> Self {
                $name { board, pos: 0 }
            }
        }
    };
}

create_board_iterator!(RowIterator);
create_board_iterator!(ColIterator);
create_board_iterator!(MainDiagonalIterator);
create_board_iterator!(SecondaryDiagonalIterator);

impl<'a> Iterator for RowIterator<'a> {
    type Item = Vec<&'a Option<board::Player>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.board.get_rows() {
            return None;
        }
        let ret = self
            .board
            .get_pieces()
            .iter()
            .skip(self.pos * self.board.get_columns())
            .take(self.board.get_columns())
            .collect();
        self.pos += 1;
        Some(ret)
    }
}

impl<'a> Iterator for ColIterator<'a> {
    type Item = Vec<&'a Option<board::Player>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.board.get_columns() {
            return None;
        }
        let ret = self
            .board
            .get_pieces()
            .iter()
            .skip(self.pos)
            .step_by(self.board.get_columns())
            .collect();
        self.pos += 1;
        Some(ret)
    }
}

impl<'a> Iterator for MainDiagonalIterator<'a> {
    type Item = Vec<&'a Option<board::Player>>;

    fn next(&mut self) -> Option<Self::Item> {
        let diag_n = self.board.get_rows() + self.board.get_columns() - 1;
        if self.pos >= diag_n {
            return None;
        }

        let h = self.board.get_rows();
        let w = self.board.get_columns();
        let mut ret = Vec::new();

        if self.pos < h + w - 1 {
            for q in (cmp::max(self.pos as i8 + 1 - h as i8, 0) as usize)..cmp::min(self.pos + 1, w)
            {
                let row = h + q - 1 - self.pos;
                let col = q;
                let idx = row * self.board.get_columns() + col;
                ret.push(&self.board.get_pieces()[idx]);
            }
        } else {
            return None;
        }

        self.pos += 1;

        Some(ret)
    }
}

impl<'a> Iterator for SecondaryDiagonalIterator<'a> {
    type Item = Vec<&'a Option<board::Player>>;

    fn next(&mut self) -> Option<Self::Item> {
        let diag_n = self.board.get_rows() + self.board.get_columns() - 1;
        if self.pos >= diag_n {
            return None;
        }

        let h = self.board.get_rows();
        let w = self.board.get_columns();
        let mut ret = Vec::new();

        if self.pos < h + w - 1 {
            for q in (cmp::max(self.pos as i8 + 1 - h as i8, 0) as usize)..cmp::min(self.pos + 1, w)
            {
                let row = self.pos - q;
                let col = q;
                let idx = row * self.board.get_columns() + col;
                ret.push(&self.board.get_pieces()[idx]);
            }
        } else {
            return None;
        }

        self.pos += 1;

        Some(ret)
    }
}

#[cfg(test)]
mod test {
    const ROWS: usize = 3;
    const COLS: usize = 3;
    use super::board::Player;
    use super::*;
    #[test]
    fn test_row_iterator() -> Result<(), board::PlayErr> {
        let board = &mut board::Board::new(ROWS, COLS);
        board.play(0, Player::Yellow)?;
        board.play(0, Player::Yellow)?;
        board.play(0, Player::Yellow)?;
        board.play(1, Player::Yellow)?;

        let it = RowIterator::new(&board);
        let expected_it = vec![
            vec![&Some(Player::Yellow), &None, &None],
            vec![&Some(Player::Yellow), &None, &None],
            vec![&Some(Player::Yellow), &Some(Player::Yellow), &None],
        ]
        .into_iter();
        assert!(Iterator::eq(it, expected_it));

        Ok(())
    }

    #[test]
    fn test_col_iterator() -> Result<(), board::PlayErr> {
        let board = &mut board::Board::new(ROWS, COLS);
        board.play(0, Player::Yellow)?;
        board.play(0, Player::Yellow)?;
        board.play(0, Player::Yellow)?;
        board.play(1, Player::Yellow)?;

        let it = ColIterator::new(&board);
        let expected_it = vec![
            vec![
                &Some(Player::Yellow),
                &Some(Player::Yellow),
                &Some(Player::Yellow),
            ],
            vec![&None, &None, &Some(Player::Yellow)],
            vec![&None, &None, &None],
        ]
        .into_iter();

        assert!(Iterator::eq(it, expected_it));

        Ok(())
    }

    #[test]
    fn test_diagonal_iterator() -> Result<(), board::PlayErr> {
        let board = &mut board::Board::new(ROWS, COLS);
        board.play(0, Player::Yellow)?;
        board.play(0, Player::Yellow)?;
        board.play(0, Player::Yellow)?;
        board.play(1, Player::Yellow)?;

        let d1_it = MainDiagonalIterator::new(&board);
        let d2_it = SecondaryDiagonalIterator::new(&board);
        let mut it = d1_it.chain(d2_it);
        let mut expected_it = vec![
            vec![&Some(Player::Yellow)],
            vec![&Some(Player::Yellow), &Some(Player::Yellow)],
            vec![&Some(Player::Yellow), &None, &None],
            vec![&None, &None],
            vec![&None],
            vec![&Some(Player::Yellow)],
            vec![&Some(Player::Yellow), &None],
            vec![&Some(Player::Yellow), &None, &None],
            vec![&Some(Player::Yellow), &None],
            vec![&None],
        ]
        .into_iter();

        for d in 0..(board.get_columns() + board.get_rows() - 1) * 2 {
            let val = it.next();
            let exp = expected_it.next();

            assert_eq!(val, exp, "diag: {}", d);
        }

        assert_eq!(it.next(), None);

        Ok(())
    }
}
