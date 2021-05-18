use super::board;
use std::iter::FromIterator;

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
create_board_iterator!(DiaIterator);

impl<'a> Iterator for RowIterator<'a> {
    type Item = Vec<Option<board::Player>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.board.rows {
            return None;
        }

        Some(self.board.pieces.iter().take(self.board.cols).collect())
    }
}

impl<'a> FromIterator<&'a Vec<Option<board::Player>>> for RowIterator<'a> {
    fn from_iter<T: IntoIterator<Item = &'a Vec<Option<board::Player>>>>(iter: T) -> Self {
        todo!()
    }
}
