fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut pf = Plateform::parse(&input);

    // Part 1 ------------------------------------------------------------------
    pf.tilt(Dir::North);
    let answ = pf.calc_north_load();
    println!("P1: {}", answ);

    // Part 2 ------------------------------------------------------------------
    // I could have used a pathfinding algo like brent or tortoise instead
    let n_tot = 1000000000;
    let n_burnin = 1000;
    let n_rem = n_tot - n_burnin;

    // burn in: run some tilts to enter the cycle
    //
    for _ in 0..n_burnin {
        pf.tilt_cycle();
    }

    // find the cycle length
    let orig = pf.inner.clone();
    let mut n = 0;
    loop {
        pf.tilt_cycle();
        n += 1;
        if orig.eq(&pf.inner) {
            break
        }
    }

    println!("cycle length: {n}");
    println!("left to run: {}", n_rem % n);

    // run the remaining cycles (modulo)
    for i in 0..(n_rem % n) {
        pf.tilt_cycle();
        let answ = pf.calc_north_load();
        println!("{i}: P2: {}", answ);
    }

    let answ = pf.calc_north_load();
    println!("P2: {}", answ);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rock {
    Cubed,
    Round,
    None,
}

impl Rock {
    fn parse(c: char) -> Self {
        match c {
            '#' => Self::Cubed,
            'O' => Self::Round,
            _ => Self::None,
        }
    }

    fn print(&self) {
        match self {
            Self::Cubed => print!("#"),
            Self::Round => print!("O"),
            Self::None => print!("."),
        }
    }
}


#[derive(Debug)]
struct Plateform {
    inner: Vec<Rock>,
    n_rows: usize,
    n_cols: usize,
}

impl Plateform {
    fn parse(s: &str) -> Self {
        let mut inner: Vec<Rock> = Vec::new();
        let mut n_rows = 0;
        for l in s.lines() {
            inner.append(&mut l.chars().map(Rock::parse).collect());
            n_rows += 1;
        }
        let n_cols = inner.len() / n_rows;
        Self {
            inner, n_rows, n_cols
        }
    }

    fn get(&self, row: usize, col: usize) -> Rock {
        self.inner[row * self.n_cols + col]
    }

    fn set(& mut self, row: usize, col: usize, val: Rock) {
        self.inner[row * self.n_cols + col] = val;
    }

    fn tilt_cycle(&mut self) {
        self.tilt(Dir::North);
        self.tilt(Dir::West);
        self.tilt(Dir::South);
        self.tilt(Dir::East);
    }

    fn tilt(&mut self, dir: Dir) {
        match dir {
            Dir::North => {
                for col in 0..self.n_cols {
                    self.tilt_north_col(col);
                }
            },
            Dir::South => {
                for col in 0..self.n_cols {
                    self.tilt_south_col(col);
                }
            },
            Dir::West => {
                for row in 0..self.n_rows {
                    self.tilt_west_row(row);
                }
            },
            Dir::East => {
                for row in 0..self.n_rows {
                    self.tilt_east_row(row);
                }
            },
        }
    }

    fn tilt_north_col(&mut self, col: usize) {
        let mut first_empty = 0;
        for row in 0..self.n_rows {
            match self.get(row, col) {
                Rock::Cubed => {
                    first_empty = row + 1;
                },
                Rock::Round => {
                    self.set(row, col, Rock::None);
                    self.set(first_empty, col, Rock::Round);
                    first_empty += 1;
                },
                Rock::None => {},
            }
        }
    }

    fn tilt_south_col(&mut self, col: usize) {
        let mut first_empty = self.n_rows as i32 - 1;
        for row in (0..self.n_rows).rev() {
            match self.get(row, col) {
                Rock::Cubed => {
                    first_empty = row as i32 - 1;
                },
                Rock::Round => {
                    self.set(row, col, Rock::None);
                    self.set(first_empty as usize, col, Rock::Round);
                    first_empty -= 1;
                },
                Rock::None => {},
            }
        }
    }

    fn tilt_west_row(&mut self, row: usize) {
        let mut first_empty = 0;
        for col in 0..self.n_cols {
            match self.get(row, col) {
                Rock::Cubed => {
                    first_empty = col + 1;
                },
                Rock::Round => {
                    self.set(row, col, Rock::None);
                    self.set(row, first_empty, Rock::Round);
                    first_empty += 1;
                },
                Rock::None => {},
            }
        }
    }

    fn tilt_east_row(&mut self, row: usize) {
        let mut first_empty = self.n_cols as i32 - 1;
        for col in (0..self.n_cols).rev() {
            match self.get(row, col) {
                Rock::Cubed => {
                    first_empty = col as i32 - 1;
                },
                Rock::Round => {
                    self.set(row, col, Rock::None);
                    self.set(row, first_empty as usize, Rock::Round);
                    first_empty -= 1;
                },
                Rock::None => {},
            }
        }
    }

    fn print(&self) {
        for row in 0..self.n_rows {
            print!("{} - ", self.n_rows - row);
            for col in 0..self.n_cols {
                self.get(row, col).print();
            }
            print!("\n");
        }
    }

    fn calc_north_load(&self) -> usize {
        let mut count = 0;
        for col in 0..self.n_cols {
            count += self.count_north_col_load(col);
        }
        count
    }

    fn count_north_col_load(&self, col: usize) -> usize {
        let mut count = 0;
        for row in 0..self.n_rows {
            if self.get(row, col) == Rock::Round {
                count += self.n_rows - row;
            }
        }
        count
    }
}

enum Dir {
    North,
    South,
    East,
    West,
}
