use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

fn main() {
    // let input = std::fs::read_to_string("example1.txt").unwrap();
    // let input = std::fs::read_to_string("example2.txt").unwrap();
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut map_inner: Vec<Block> = Vec::new();
    let mut n_rows = 0;

    for l in input.lines() {
        n_rows += 1;
        let mut part: Vec<Block> = l.chars().map(Block::parse).collect();
        map_inner.append(&mut part);
    }

    let mut map = Map::new(map_inner.clone(), n_rows);
    map.get_mut(0, 0).unwrap().total_cost = [0; 4];

    // Part1 -------------------------------------------------------------------
    let answ = shortest_p1((0, 0), (map.n_rows - 1, map.n_cols -1), &mut map);
    println!("P1: {}", answ);

    // Part2 -------------------------------------------------------------------
    let mut map = Map::new(map_inner.clone(), n_rows);
    let answ = shortest_p2((0, 0), (map.n_rows - 1, map.n_cols -1), &mut map);
    println!("P2: {}", answ);
}

fn shortest_p1(start_pos: (usize, usize), end_pos: (usize, usize),
               map: &mut Map<Block>) -> u64 {

    let mut states: BinaryHeap<State> = BinaryHeap::new();
    states.push(State::new(start_pos.0, start_pos.1, 0, Dir::Right));
    states.push(State::new(start_pos.0, start_pos.1, 0, Dir::Down));

    while let Some(cur_state) = states.pop() {
        let visited = map.get(cur_state.row, cur_state.col)
            .unwrap()
            .get_dir_visited(cur_state.dir);
        if visited {continue;}
        map.get_mut(cur_state.row, cur_state.col)
            .unwrap()
            .set_dir_visited(cur_state.dir);

        let cur_pos = (cur_state.row, cur_state.col);
        let cur_cost = cur_state.cost;

        let adjacents = get_adj(&cur_state, &map);

        for (r, c, d) in adjacents {
            let new_cost = cur_cost + calc_cost(cur_pos, (r, c), map);
            let mut blk = map.get_mut(r, c).unwrap();
            if blk.get_dir_cost(d) > new_cost {
                blk.set_dir_cost(d, new_cost);
                states.push(State::new(r, c, new_cost, d));
            }
        }
    }

    map.get_p(end_pos).unwrap().total_cost.iter().fold(u64::MAX, |a, &x| a.min(x))
}

fn get_adj(state: &State, map: &Map<Block>) -> Vec<(usize, usize, Dir)> {
    let pos = (state.row, state.col);
    let max = (map.n_rows, map.n_cols);
    let mut out = Vec::new();

    for dir in Dir::iter() {
        if dir == state.dir || dir == state.dir.opposite() {
            continue;
        }
        for n in 1..=3 {
            if let Some(prop) = dir.move_n(pos, n, max) {
                out.push((prop.0, prop.1, dir));
            }
        }
    }
    out
}

fn shortest_p2(start_pos: (usize, usize), end_pos: (usize, usize),
               map: &mut Map<Block>) -> u64 {

    let mut states: BinaryHeap<State> = BinaryHeap::new();
    states.push(State::new(start_pos.0, start_pos.1, 0, Dir::Right));
    states.push(State::new(start_pos.0, start_pos.1, 0, Dir::Down));

    while let Some(cur_state) = states.pop() {
        let visited = map.get(cur_state.row, cur_state.col)
            .unwrap()
            .get_dir_visited(cur_state.dir);
        if visited {continue;}
        map.get_mut(cur_state.row, cur_state.col)
            .unwrap()
            .set_dir_visited(cur_state.dir);

        let cur_pos = (cur_state.row, cur_state.col);
        let cur_cost = cur_state.cost;

        let adjacents = get_adj_p2(&cur_state, &map);

        for (r, c, d) in adjacents {
            let new_cost = cur_cost + calc_cost(cur_pos, (r, c), map);
            let mut blk = map.get_mut(r, c).unwrap();
            if blk.get_dir_cost(d) > new_cost {
                blk.set_dir_cost(d, new_cost);
                states.push(State::new(r, c, new_cost, d));
            }
        }
    }

    map.get_p(end_pos).unwrap().total_cost.iter().fold(u64::MAX, |a, &x| a.min(x))
}

fn get_adj_p2(state: &State, map: &Map<Block>) -> Vec<(usize, usize, Dir)> {
    let pos = (state.row, state.col);
    let max = (map.n_rows, map.n_cols);
    let mut out = Vec::new();

    for dir in Dir::iter() {
        if dir == state.dir || dir == state.dir.opposite() {
            continue;
        }
        for n in 4..=10 {
            if let Some(prop) = dir.move_n(pos, n, max) {
                out.push((prop.0, prop.1, dir));
            }
        }
    }
    out
}

fn calc_cost(from_p: (usize, usize), to_p: (usize, usize),
             map: &Map<Block>) -> u64 {
    let mut cost = 0;
    // row move
    if from_p.0 < to_p.0 {
        let start = from_p.0 + 1;
        let end = to_p.0;
        for r in start..=end {
            cost += map.get(r, from_p.1).unwrap().cost;
        }
    } else if from_p.0 > to_p.0 {
        let start = to_p.0;
        let end = from_p.0 - 1;
        for r in start..=end {
            cost += map.get(r, from_p.1).unwrap().cost;
        }
    } else if from_p.1 < to_p.1 {
        let start = from_p.1 + 1;
        let end = to_p.1;
        for c in start..=end {
            cost += map.get(from_p.0, c).unwrap().cost;
        }
    } else {
        let start = to_p.1;
        let end = from_p.1 - 1;
        for c in start..=end {
            cost += map.get(from_p.0, c).unwrap().cost;
        }
    }

    cost
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    row: usize,
    col: usize,
    cost: u64,
    dir: Dir, // entered from
}

impl State {
    fn new(row: usize, col: usize, cost: u64, dir: Dir) -> Self {
        Self {row, col, cost, dir}
    }

    fn get_pos(&self) -> (usize, usize) {
        (self.row, self.col)
    }
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.col.cmp(&other.col))
            .then_with(|| self.row.cmp(&other.row))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Block {
    cost: u64,
    total_cost: [u64; 4],
    visited: [bool; 4],
}

impl Block {
    fn new(cost: u64) -> Self {
        Self {
            cost,
            total_cost: [u64::MAX; 4],
            visited: [false; 4],
        }
    }

    fn parse(c: char) -> Self {
        let cost = c.to_digit(10).unwrap();
        Self::new(cost as u64)
    }

    fn get_dir_cost(&self, dir: Dir) -> u64 {
        self.total_cost[dir.as_usize()]
    }

    fn set_dir_cost(&mut self, dir: Dir, cost: u64) {
        self.total_cost[dir.as_usize()] = cost;
    }

    fn get_dir_visited(&self, dir: Dir) -> bool {
        self.visited[dir.as_usize()]
    }

    fn set_dir_visited(&mut self, dir: Dir) {
        self.visited[dir.as_usize()] = true;
    }

    fn print(&self) {
        for m in self.total_cost {
            if m > 99 {
                print!{"--,"};
            } else {
                print!{"{m:2},"}
            }
        }
        print!{" "}
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, EnumIter)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
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

fn print_map(m: &Map<Block>) {
    for r in 0..m.n_rows {
        for c in 0..m.n_cols {
            m.get(r, c).unwrap().print();
        }
        print!("\n");
    }
    print!("\n");
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

    fn move_n(&self, from: (usize, usize), n: usize,
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
