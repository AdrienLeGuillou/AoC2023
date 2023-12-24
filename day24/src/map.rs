use strum_macros::EnumIter;

#[derive(Debug)]
pub struct Map<T> {
    inner: Vec<T>,
    pub n_cols: usize,
    pub n_rows: usize,
}

impl <T> Map<T> where T: Eq + PartialEq {
    pub fn new(inner: Vec<T>, n_rows: usize) -> Self {
        let n_cols = inner.len() / n_rows;
        Self {
            inner,
            n_cols,
            n_rows,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row >= self.n_rows || col >= self.n_cols {
            None
        } else {
            Some(&self.inner[self.n_cols * row + col])
        }
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        if row >= self.n_rows || col >= self.n_cols {
            None
        } else {
            Some(&mut self.inner[self.n_cols * row + col])
        }
    }

    pub fn get_position(&self, pos: &Position) -> Option<&T> {
        self.get(pos.row, pos.col)
    }

    pub fn get_position_mut(&mut self, pos: &Position) -> Option<&mut T> {
        self.get_mut(pos.row, pos.col)
    }

    pub fn max_position(&self) -> Position {
        Position::new(self.n_rows, self.n_cols)
    }

    pub fn find(&self, val: T) -> Option<Position> {
        for i in 0..self.inner.len() {
            if self.inner[i] == val {
                return Some(Position::new(i / self.n_cols, i % self.n_rows))
            }
        }
        None
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Self {
        Self {row, col}
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, EnumIter)]
pub enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    pub fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }

    pub fn as_usize(&self) -> usize {
        match self {
            Self::Up => 0,
            Self::Right => 1,
            Self::Down => 2,
            Self::Left => 3,
        }
    }

    pub fn from_usize(x: usize) -> Self {
        match x {
            0 => Self::Up,
            1 => Self::Right,
            2 => Self::Down,
            _ => Self::Left,
        }
    }

    pub fn move_n(&self, from: &Position, n: usize, max: &Position) -> Option<Position> {
        let row = from.row;
        let col = from.col;

        match self {
            Self::Up => {
                if let Some(row) = row.checked_sub(n) {
                    Some(Position::new(row, col))
                } else {
                    None
                }
            } ,
            Self::Down => {
                let row = row + n;
                if row < max.row {
                    Some(Position::new(row, col))
                } else {
                    None
                }
            },
            Self::Right => {
                let col = col + n;
                if col < max.col {
                    Some(Position::new(row, col))
                } else {
                    None
                }
            },
            Self::Left => {
                if let Some(col) = col.checked_sub(n) {
                    Some(Position::new(row, col))
                } else {
                    None
                }
            } ,
        }
    }
}
