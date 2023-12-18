use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

fn main() {
    // let input = std::fs::read_to_string("example1.txt").unwrap();
    // let input = std::fs::read_to_string("example2.txt").unwrap();
    let input = std::fs::read_to_string("input.txt").unwrap();

    let instructions: Vec<Instr> = input.lines().map(Instr::parse).collect();
    // println!("{:?}", instructions);


    // Part1 -------------------------------------------------------------------
    println!("{:?}", p1(&instructions));

    // Part2 -------------------------------------------------------------------
    let instructions: Vec<Instr> = input.lines().map(Instr::parse_2).collect();
    // println!("{:?}", instructions);
    println!("{:?}", p1(&instructions));

    // println!("P2: {}", answ);
}


fn p1(instructions: &Vec<Instr>) -> f64 {
    let mut pos_cur = (0, 0);
    let mut area = 0_f64;
    let mut i = 0;
    let mut border = 0.0;

    loop {
        let instr_cur = &instructions[i];
        border += instr_cur.len as f64;
        let pos_prev = pos_cur;
        pos_cur = instr_cur.do_move(pos_prev);

        // shoelace algorith to calculate the area of the polygon
        area += 0.5
              * (pos_prev.1 as f64 + pos_cur.1 as f64)
              * (pos_prev.0 as f64 - pos_cur.0 as f64);

        i += 1;
        if i >= instructions.len() {
            break;
        }
    }
// picks theorem: WARNING. Uses the borders and not the corners (they were the
// same on day 10)
    let area = area.abs() - border / 2.0 + 1.0;
    area + border // here I want the borders as well
}

fn parse_dir(s: &str) -> Dir {
    match s {
        "U" => Dir::Up,
        "D" => Dir::Down,
        "L" => Dir::Left,
        _ => Dir::Right,
    }
}

#[derive(Debug, Clone)]
struct Instr {
    dir: Dir,
    len: usize,
}

impl Instr {
    fn parse(s: &str) -> Self {
        let elts: Vec<String> = s.split(" ").map(|x| x.to_owned()).collect();
        let dir = parse_dir(&elts[0]);
        let len: usize = elts[1].parse().unwrap();
        // let col = elts[2].replace("(", "").replace(")", "");

        Self {dir, len}
    }

    fn parse_2(s: &str) -> Self {
        let (_, s) = s.split_once("(#").unwrap();
        let s = s.replace(")", "");
        let mut len = 0;
        let mut n = 0;
        let mut dir = Dir::Up;

        for c in s.chars() {
            if n < 5 {
                let d = c.to_digit(16).unwrap();
                len *= 16;
                len += d;
            } else {
                dir = match c {
                    '0' => Dir::Right,
                    '1' => Dir::Down,
                    '2' => Dir::Left,
                    _ => Dir::Up,
                }
            }

            n += 1;
        }

        Self {dir, len: len as usize}
    }

    fn do_move(&self, pos: (i32, i32)) -> (i32, i32) {
        self.dir.move_any(pos, self.len as i32)
    }
}



#[derive(Debug)]
struct Map<T> {
    inner: Vec<T>,
    n_cols: usize,
    n_rows: usize,
}

impl <T> Map<T> {
    fn new(inner: Vec<T>, n_rows: usize) -> Self {
        let n_cols = inner.len() / n_rows;
        Self {
            inner,
            n_cols,
            n_rows,
        }
    }

    fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row >= self.n_rows || col >= self.n_cols {
            None
        } else {
            Some(&self.inner[self.n_cols * row + col])
        }
    }

    fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        if row >= self.n_rows || col >= self.n_cols {
            None
        } else {
            Some(&mut self.inner[self.n_cols * row + col])
        }
    }

    fn get_p(&self, pos: (usize, usize)) -> Option<&T> {
        self.get(pos.0, pos.1)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, EnumIter)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }

    fn as_usize(&self) -> usize {
        match self {
            Self::Up => 0,
            Self::Right => 1,
            Self::Down => 2,
            Self::Left => 3,
        }
    }

    fn move_any(&self, from: (i32, i32), n: i32) -> (i32, i32) {
        match self {
            Self::Up => (from.0 - n, from.1) ,
            Self::Right => (from.0, from.1 + n),
            Self::Down => (from.0 + n, from.1),
            Self::Left => (from.0, from.1 - n),
        }
    }

    fn move_map(&self, from: (usize, usize), n: usize,
            max: (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Self::Up => {
                if let Some(r) = from.0.checked_sub(n) {
                    Some((r, from.1))
                } else {
                    None
                }
            } ,
            Self::Right => {
                let c = from.1 + n;
                if c < max.1 {
                    Some((from.0, c))
                } else {
                    None
                }
            },
            Self::Down => {
                let r = from.0 + n;
                if r < max.0 {
                    Some((r, from.1))
                } else {
                    None
                }
            },
            Self::Left => {
                if let Some(c) = from.1.checked_sub(n) {
                    Some((from.0, c))
                } else {
                    None
                }
            } ,
        }
    }
}
