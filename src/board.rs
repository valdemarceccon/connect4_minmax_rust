pub const ROWS: usize = 7;
pub const COLS: usize = 6;

#[derive(Debug)]
pub struct Board {
    state: Vec<Option<Players>>,
}

impl Board {
    pub fn new() -> Self {
        let mut state = Vec::with_capacity(ROWS * COLS);

        for _ in 0..ROWS * COLS {
            state.push(None);
        }

        Board { state }
    }

    pub fn piece_at(&self, row: usize, col: usize) -> Option<Players> {
        self.state[index(row, col)].clone()
    }

    pub fn play(&mut self, player: Players, col: usize) {
        for i in (0..ROWS).rev() {
            if self.piece_at(i, col) == None {
                self.state[index(i, col)] = Some(player);
                break;
            }
        }
    }

    pub fn get_row(&self, row: usize) -> Vec<Option<Players>> {
        let mut ret = Vec::new();

        for i in 0..row {
            ret.push(self.piece_at(row, i));
        }

        ret
    }

    pub fn get_col(&self, col: usize) -> Vec<Option<Players>> {
        let mut ret = Vec::new();

        for i in 0..col {
            ret.push(self.piece_at(i, col));
        }

        ret
    }

    fn is_col_full(&self, col: usize) -> bool {
        self.piece_at(0, col) != None
    }

    pub fn is_play_valid(&self, col: i8) -> bool {
        if col < 0 {
            return false;
        }

        let col = col as usize;

        if col >= COLS {
            return false;
        }

        if self.is_col_full(col) {
            return false;
        }

        true
    }
}

fn index(row: usize, col: usize) -> usize {
    row * COLS + col
}

#[derive(Debug, PartialEq, Clone)]
pub enum Players {
    RED,
    YELLOW,
}
