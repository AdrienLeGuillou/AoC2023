fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let map = Map::parse_input(&input);
    let galaxies = map.find_galaxies();

    let mut empty_cols: Vec<usize> = Vec::new();
    let mut empty_rows: Vec<usize> = Vec::new();

    let mut n = 0;
    for c in map.cols() {
        if ! c.contains(&Space::Galaxy) {
            empty_cols.push(n);
        }
        n += 1;
    }

    let mut n = 0;
    for r in map.rows() {
        if ! r.contains(&Space::Galaxy) {
            empty_rows.push(n);
        }
        n += 1;
    }

    let mut answ = 0;
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            let dist = galaxies[i].dist_exp(&galaxies[j], &empty_cols, &empty_rows);
            answ += dist;
        }
    }
    println!("Part1: {}", answ);

    let mut answ = 0;
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            let dist = galaxies[i].dist_exp_old(&galaxies[j], &empty_cols, &empty_rows);
            answ += dist;
        }
    }
    println!("Part2: {}", answ);
}

#[derive(Debug)]
struct Map {
    m: Vec<Vec<Space>>,
    n_cols: usize,
    n_rows: usize,
}

impl Map {
    fn new(m: Vec<Vec<Space>>, n_rows: usize) -> Self {
        let n_cols = m.len() / n_rows;
        Self {
            m,
            n_cols,
            n_rows,
        }
    }

    fn get(&self, pos: Position) -> Result<Space, &'static str> {
        if pos.row >= self.n_rows || pos.col >= self.n_cols {
            Err("Subscript out of bounds")
        } else {
            Ok(self.m[self.n_cols * pos.row][pos.col])
        }
    }

    fn parse_input(s: &str) -> Self {
        let mut m: Vec<Vec<Space>> = Vec::new();
        for l in s.lines() {
            m.push(l.chars().map(Space::parse).collect());
        }
        let n_rows = m.len();
        let n_cols = m[0].len();
        Self {
            m,
            n_cols,
            n_rows,
        }
    }

    fn iter<'a>(&'a self) -> IterMap<'a> {
        IterMap{
            inner: self,
            pos: Position::new(0, 0),
        }
    }

    fn cols<'b>(&'b self) -> IterMapCols<'b> {
        IterMapCols {
            inner: self,
            col: 0,
        }
    }

    fn rows<'b>(&'b self) -> IterMapRows<'b> {
        IterMapRows {
            inner: self,
            row: 0,
        }
    }

    fn find_galaxies(&self) -> Vec<Position> {
        let mut out = Vec::new();
        for row in 0..self.n_rows {
            for col in 0..self.n_cols {
                if self.m[row][col] == Space::Galaxy {
                    out.push(Position::new(row, col))
                }
            }
        }
        out
    }
}

struct IterMap<'a> {
    inner: &'a Map,
    pos: Position,
}

impl<'a> Iterator for IterMap<'a> {
    type Item = &'a Space;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos.col < self.inner.n_cols - 1 {
            self.pos.col += 1;
        } else {
            if self.pos.row < self.inner.n_rows - 1 {
                self.pos.row += 1;
                self.pos.col = 0;
            } else {
                return None
            }
        }
        Some(&self.inner.m[self.pos.row][self.pos.col])
    }
}

struct IterMapCols<'a> {
    inner: &'a Map,
    col: usize,
}

impl<'b> Iterator for IterMapCols<'b> {
    type Item = Vec<Space>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.col >= self.inner.n_cols - 1 {
            return None
        }

        let mut out = Vec::new();
        for i in 0..self.inner.n_rows {
            out.push(self.inner.m[i][self.col])
        }
        self.col += 1;
        Some(out)
    }
}

struct IterMapRows<'b> {
    inner: &'b Map,
    row: usize,
}

impl<'b> Iterator for IterMapRows<'b> {
    type Item = Vec<Space>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.inner.n_rows - 1 {
            return None
        }
        let out = self.inner.m[self.row].clone();
        self.row += 1;
        Some(out)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Galaxy,
    Empty,
}

impl Space {
    fn parse(c: char) -> Self {
        match c {
            '#' => Self::Galaxy,
            _ => Self::Empty,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn new(row: usize, col: usize) -> Self {
        Self {row, col}
    }

    fn dist(&self, other: &Self) -> usize {
        let vert = (self.row as i64 - other.row as i64).abs();
        let hori = (self.col as i64 - other.col as i64).abs();
        (vert + hori) as usize
    }

    fn dist_exp(&self, other: &Self,
                e_cols: &Vec<usize>, e_rows: &Vec<usize>) -> usize {
        let vert = (self.row as i64 - other.row as i64).abs();
        let hori = (self.col as i64 - other.col as i64).abs();

        let dup_r = count_between(e_rows, self.row, other.row);
        let dup_c = count_between(e_cols, self.col, other.col);


        (vert + hori) as usize + dup_r + dup_c
    }

    fn dist_exp_old(&self, other: &Self,
                e_cols: &Vec<usize>, e_rows: &Vec<usize>) -> u64 {
        let vert = (self.row as i64 - other.row as i64).abs() as u64;
        let hori = (self.col as i64 - other.col as i64).abs() as u64;

        let dup_r = count_between(e_rows, self.row, other.row) as u64;
        let dup_c = count_between(e_cols, self.col, other.col) as u64;


        (vert + hori) + (dup_r + dup_c) * 999_999
    }
}

fn between(x: &usize, a: usize, b: usize) -> bool {
    if a > b {
        *x > b && *x < a
    } else {
        *x > a && *x < b
    }
}

fn count_between(v: &Vec<usize>, a: usize, b: usize) -> usize {
    let mut count = 0_usize;
    for elt in v {
        if between(elt, a, b) {
            count += 1;
        }
    }
    count
}
