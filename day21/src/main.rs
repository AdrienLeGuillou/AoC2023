use strum::IntoEnumIterator;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

mod map;
use crate::map::{Map, Dir, Position};

fn main() {
    // let input = std::fs::read_to_string("example1.txt").unwrap();
    // let input = std::fs::read_to_string("example2.txt").unwrap();
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut map_inner: Vec<Tile> = Vec::new();
    let mut n_rows = 0;

    for l in input.lines() {
        n_rows += 1;
        let mut part: Vec<Tile> = l.chars().map(Tile::parse).collect();
        map_inner.append(&mut part);
    }

    let mut map = Map::new(map_inner.clone(), n_rows);
    let start_pos = map.find(Tile::Start).unwrap();
    if let Some(t) = map.get_position_mut(&start_pos) {
        *t = Tile::Garden(0, false);
    }
    println!("start: {start_pos:?}");

    // print_map(&map);

    // Part1 -------------------------------------------------------------------
    let answ = p1(start_pos, &mut map, 64);
    println!("P1: {}", answ);

    // Part2 -------------------------------------------------------------------
    // let mut map = Map::new(map_inner.clone(), n_rows);
    // let answ = shortest_p2((0, 0), (map.n_rows - 1, map.n_cols -1), &mut map);
    // println!("P2: {}", answ);
}

fn p1(start: Position, map: &mut Map<Tile>, n_steps: u64) -> u64 {
    let modulo = n_steps % 2;
    let mut n_out = 0;
    let mut states: BinaryHeap<State> = BinaryHeap::new();
    states.push(State::new(start, 0));

    while let Some(current) = states.pop() {
        let cur_tile = map.get_position_mut(&current.pos);
        let mut cur_cost = 0;

        if let Some(Tile::Garden(cost, visited)) = cur_tile {
            if *visited {
                continue;
            } else {
                *visited = true;
                cur_cost = *cost;
                if *cost > n_steps { break; }
                if *cost <= n_steps && *cost % 2 == modulo {
                    n_out += 1;
                }
           }
        }

        let adjacents = get_adj(&current.pos, map);
        for a in adjacents {
            let a_tile = map.get_position_mut(&a);
            if let Some(Tile::Garden(cost, visited)) = a_tile {
                if *visited { continue; }
                if *cost > cur_cost + 1 {
                    *cost = cur_cost + 1;
                    states.push(State::new(a, *cost));
                }
            }
        }
    }

    n_out
}

fn get_adj(pos: &Position, map: &Map<Tile>) -> Vec<Position> {
    let mut out = Vec::new();
    for dir in Dir::iter() {
        if let Some(p) = dir.move_n(pos, 1, &map.max_position()) {
            out.push(p);
        }
    }
    out
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    pos: Position,
    cost: u64,
}

impl State {
    fn new(pos: Position, cost: u64) -> Self {
        Self {pos, cost}
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
            .then_with(|| self.pos.col.cmp(&other.pos.col))
            .then_with(|| self.pos.row.cmp(&other.pos.row))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Garden(u64, bool),
    Start,
    Rock,
}

impl Tile {
    fn parse(c: char) -> Self {
        match c {
            '.' => Self::Garden(u64::MAX, false),
            'S' => Self::Start,
            _   => Self::Rock,
        }
    }

    fn print(&self) {
        match self {
            // Self::Garden(cost, _) => print!("{cost:1}"),
            Self::Garden(_, _) => print!("."),
            Self::Rock => print!("#"),
            Self::Start => print!("S"),
        }
    }
}


fn print_map(m: &Map<Tile>) {
    for r in 0..m.n_rows {
        for c in 0..m.n_cols {
            m.get(r, c).unwrap().print();
        }
        print!("\n");
    }
    print!("\n");
}
