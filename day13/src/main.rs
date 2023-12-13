fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let patterns: Vec<Pattern> = input.split("\n\n").map(Pattern::parse).collect();
    // println!("{:?}", patterns);
    //
    // println!("{:?}", patterns[0].find_ref_row());
    // println!("{:?}", patterns[0].find_ref_col());
    //
    // println!("{:?}", patterns[3].find_ref_row());
    // println!("{:?}", patterns[3].find_ref_col());

    // for i in 0..10 {
    //     println!("{i}, {}", patterns[i].ref_val())
    // }

    let answ = patterns.iter().fold(0, |a, x| a + x.ref_val());
    println!("P1: {}", answ);

    let answ = patterns.iter().fold(0, |a, x| a + x.ref_val2());
    println!("P2: {}", answ);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Floor {
    Ash,
    Rock,
}

impl Floor {
    fn parse(c: char) -> Self {
        match c {
            '.' => Self::Ash,
            _ => Self::Rock,
        }
    }
}


#[derive(Debug)]
struct Pattern {
    inner: Vec<Floor>,
    n_rows: usize,
    n_cols: usize,
}

impl Pattern {
    fn parse(s: &str) -> Self {
        let mut inner: Vec<Floor> = Vec::new();
        let mut n_rows = 0;
        for l in s.lines() {
            inner.append(&mut l.chars().map(Floor::parse).collect());
            n_rows += 1;
        }
        let n_cols = inner.len() / n_rows;
        Self {
            inner, n_rows, n_cols
        }
    }

    fn get(&self, row: usize, col: usize) -> Floor {
        self.inner[row * self.n_cols + col]
    }

    // rows
    fn eq_row(&self, a: usize, b: usize) -> Option<bool> {
        if a >= self.n_rows || b >= self.n_rows {
            return None
        }

        for i in 0..self.n_cols {
            if self.get(a, i) != self.get(b, i) {
                return Some(false)
            }
        }
        Some(true)
    }

    fn eq_rows2(&self, a: usize, b: usize) -> usize {
        let mut err = 0;
        for i in 0..self.n_cols {
            if self.get(a, i) != self.get(b, i) {
                err += 1
            }
            if err >= 2 {
                break
            }
        }
        err
    }

    fn eq_cols(&self, a: usize, b: usize) -> Option<bool> {
        if a >= self.n_cols || b >= self.n_cols {
            return None
        }

        for i in 0..self.n_rows {
            if self.get(i, a) != self.get(i, b) {
                return Some(false)
            }
        }
        Some(true)
    }

    fn eq_cols2(&self, a: usize, b: usize) -> usize {
        let mut err = 0;
        for i in 0..self.n_rows {
            if self.get(i, a) != self.get(i, b) {
                err += 1
            }
            if err >= 2 {
                break
            }
        }
        err
    }

    // return the row above of the reflection
    fn find_ref_row(&self) -> Option<usize> {
        for r in 0..(self.n_rows - 1) {
            if self.is_ref_row(r) {
                return Some(r)
            }
        }
        None
    }

    fn find_ref_row2(&self) -> Option<usize> {
        for r in 0..(self.n_rows - 1) {
            if self.is_ref_row2(r) {
                return Some(r)
            }
        }
        None
    }

    fn is_ref_row(&self, row: usize) -> bool {
        let mut a = row;
        let mut b = row + 1;
        while b < self.n_rows {
            if !self.eq_row(a, b).unwrap() {
                return false
            } else {
                if a == 0 {break}
                a -= 1;
                b += 1;
            }
        }
        true
    }

    fn is_ref_row2(&self, row: usize) -> bool {
        let mut a = row;
        let mut b = row + 1;
        let mut errs = 0;
        while b < self.n_rows {
            errs += self.eq_rows2(a, b);
            if errs > 1 {
                return false
            } else {
                if a == 0 {break}
                a -= 1;
                b += 1;
            }
        }
        errs == 1
    }

    // return the col left of the reflection
    fn find_ref_col(&self) -> Option<usize> {
        for c in 0..(self.n_cols - 1) {
            if self.is_ref_col(c) {
                return Some(c)
            }
        }
        None
    }

    fn find_ref_col2(&self) -> Option<usize> {
        for c in 0..(self.n_cols - 1) {
            if self.is_ref_col2(c) {
                return Some(c)
            }
        }
        None
    }

    fn is_ref_col(&self, col: usize) -> bool {
        let mut a = col;
        let mut b = col + 1;
        while b < self.n_cols {
            if !self.eq_cols(a, b).unwrap() {
                return false
            } else {
                if a == 0 {break}
                a -= 1;
                b += 1;
            }
        }
        true
    }

    fn is_ref_col2(&self, col: usize) -> bool {
        let mut a = col;
        let mut b = col + 1;
        let mut errs = 0;
        while b < self.n_cols {
            errs += self.eq_cols2(a, b);
            if errs > 1 {
                return false
            } else {
                if a == 0 {break}
                a -= 1;
                b += 1;
            }
        }
        errs == 1
    }

    fn ref_val(&self) -> usize {
        if let Some(c) = self.find_ref_col() {
            return c + 1;
        }

        if let Some(r) = self.find_ref_row() {
            return (r + 1) * 100;
        }

        0
    }

    fn ref_val2(&self) -> usize {
        if let Some(c) = self.find_ref_col2() {
            return c + 1;
        }

        if let Some(r) = self.find_ref_row2() {
            return (r + 1) * 100;
        }

        0
    }
}
