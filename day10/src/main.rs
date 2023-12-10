fn main() {
    // Part 1
    let input = std::fs::read_to_string("input.txt").unwrap();
    let map = Map::parse_input(&input);

    let pos_start = map.get_start();
    let pipe_cur = get_start_pipe(&map, pos_start);
    // let pipe_cur = Pipe::from_u8(4);
    let mut pos_cur = pipe_cur.go_through(pos_start);
    let mut pipe_prev = pipe_cur;
    let mut n_steps = 1;
    let mut area = 0_f64;

    loop {
        let pos_prev = pos_cur;
        let pipe_cur = Pipe::from_u8(map.get(pos_cur).unwrap());
        let pipe_cur = pipe_cur.align(&pipe_prev);
        pos_cur = pipe_cur.go_through(pos_cur);

        // shoelace algorith to calculate the area of the polygon
        area += 0.5
              * (pos_prev.1 as f64 + pos_cur.1 as f64)
              * (pos_prev.0 as f64 - pos_cur.0 as f64);
        pipe_prev = pipe_cur;

        n_steps += 1;

        if pos_cur.0 == pos_start.0 && pos_cur.1 == pos_start.1 {
            break
        }
    }
    println!("Part 1: {}", n_steps / 2);

    // pick's theorem for area of polygon's with integer vertex
    let answ = area.abs() - (n_steps as f64) / 2.0 + 1.0;
    println!("Part 2: {}", answ);
}

fn parse_pipe(c: char) -> u8 {
    match c {
        '-' => 0,
        '|' => 1,
        'L' => 2,
        '7' => 3,
        'J' => 4,
        'F' => 5,
        'S' => 6,
        _ => 9,
    }
}

// fn get_start_pipe(pos: (usize, usize), map: &Map) -> Pipe {
//
// }

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir { N, S, E, W }

impl Dir {
    fn to_move(&self) -> (i32, i32) {
        match self {
            Self::N => (-1, 0),
            Self::S => (1, 0),
            Self::E => (0, 1),
            Self::W => (0, -1),
        }
    }

    fn reverse(&self) -> Self {
        match self {
            Self::N => Self::S,
            Self::S => Self::N,
            Self::E => Self::W,
            Self::W => Self::E,
        }
    }

    fn apply_move(&self, pos: (usize, usize)) -> (usize, usize) {
        let dir = self.to_move();
        ((pos.0 as i32 + dir.0) as usize, (pos.1 as i32 + dir.1) as usize)
    }
}

#[derive(Debug)]
struct Pipe {
    entry: Dir,
    exit: Dir,
}

impl Pipe {
    fn new(entry: Dir, exit: Dir) -> Self {
        Self {entry, exit}
    }

    fn reverse(self) -> Self {
        Self {
            entry: self.exit,
            exit: self.entry,
        }
    }

    fn from_u8(u: u8) -> Self {
        match u {
            0 => Self::new(Dir::W, Dir::E),
            1 => Self::new(Dir::N, Dir::S),
            2 => Self::new(Dir::N, Dir::E),
            3 => Self::new(Dir::W, Dir::S),
            4 => Self::new(Dir::N, Dir::W),
            _ => Self::new(Dir::E, Dir::S),
        }

    }

    fn align(self, prev: &Self) -> Self {
        if self.entry == prev.exit.reverse() {
            self
        } else {
            self.reverse()
        }
    }

    fn go_through(&self, pos: (usize, usize)) -> (usize, usize) {
        self.exit.apply_move(pos)
    }

    // is tile X connected to pipe from dir
    fn is_connected(&self, dir: Dir) -> bool {
        self.entry == dir.reverse() || self.exit == dir.reverse()
    }
}

fn get_start_pipe(map: &Map, pos: (usize, usize)) -> Pipe {
    let drs = [Dir::N, Dir::S, Dir::E, Dir::W];
    let mut io = Vec::new();
    for dir in drs {
        let t_pos = dir.apply_move(pos);
        if let Some(v) = map.get(t_pos) {
            if Pipe::from_u8(v).is_connected(dir) {
                io.push(dir);
            }
        }
    }
    Pipe::new(io[0], io[1])
}

#[derive(Debug)]
struct Map {
    m: Vec<u8>,
    n_cols: usize,
    n_rows: usize,
}

impl Map {
    fn new(m: Vec<u8>, n_rows: usize) -> Self {
        let n_cols = m.len() / n_rows;
        Self {
            m,
            n_cols,
            n_rows,
        }
    }

    fn get(&self, pos: (usize, usize)) -> Option<u8> {
        if pos.0 >= self.n_rows || pos.1 >= self.n_cols {
            None
        } else {
            Some(self.m[self.n_cols * pos.0 + pos.1])
        }
    }

    fn parse_input(s: &str) -> Self {
        let mut map = Vec::new();
        let mut n_rows = 0;
        for l in s.lines() {
            n_rows += 1;
            for c in l.chars() {
                map.push(parse_pipe(c));
            }
        }
        Self::new(map, n_rows)
    }

    fn get_start(&self) -> (usize, usize) {
        let pos = self.m.iter().position(|&x| x == 6).unwrap();
        (pos / self.n_cols, pos % self.n_cols)
    }
}

