// for part 2:
//
// choice of values is not random:
//      65 is the distance from border
//      131 is the length of the grid
//
// can also be solved with:
// https://www.dcode.fr/lagrange-interpolating-polynomial

use strum::IntoEnumIterator;
use std::collections::HashSet;

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

    let map = Map::new(map_inner.clone(), n_rows);
    let start_pos = map.find(Tile::Start).unwrap();
    println!("start: {start_pos:?}");

    let start_pos = RPos {row: start_pos.row as i32, col: start_pos.col as i32};

    // print_map(&map);

    // Part1 -------------------------------------------------------------------
    let answ = get_n_reach(&map, start_pos, 64);
    println!("P1: {}", answ);

    // Part2 -------------------------------------------------------------------
    let target = (26501365.0 - 65.0) / 131.0;
    let ps = [
        get_n_reach(&map, start_pos, 65) as f64,
        get_n_reach(&map, start_pos, 196) as f64,
        get_n_reach(&map, start_pos, 327) as f64,
    ];

    println!("Ps: {:?}", ps);

    let answ = lagrange(ps, target);
    println!("P2: {}", answ);
}

//  Lagrange's Interpolation formula for ax^2 + bx + c
//       with x=[0,1,2]
//       and  y=[y0,y1,y2]
//       we have f(x) = (x^2-3x+2) * y0/2 - (x^2-2x)*y1 + (x^2-x) * y2/2
//  so the coefficients are:
//  a = y0/2 - y1 + y2/2
//  b = -3*y0/2 + 2*y1 - y2/2
//  c = y0
fn lagrange(vals: [f64; 3], target: f64) -> f64 {
    let mut coef = [0.0; 3];
    coef[0] = vals[0] / 2.0 - vals[1] + vals[2] / 2.0;
    coef[1] = - 3.0 * (vals[0] / 2.0) + 2.0 * vals[1] - vals[2] / 2.0;
    coef[2] = vals[0];

    // println!("Coeffs: {:?}", coef);
    coef[0] * target * target + coef[1] * target + coef[2]
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Garden,
    Start,
    Rock,
}

impl Tile {
    fn parse(c: char) -> Self {
        match c {
            '.' => Self::Garden,
            'S' => Self::Start,
            _   => Self::Rock,
        }
    }

    fn print(&self) {
        match self {
            Self::Garden => print!("."),
            Self::Rock => print!(" # "),
            Self::Start => print!(" S "),
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

fn convert(x: i32, max: i32) -> usize {
    let new = x % max;
    if new < 0 {
        (new + max) as usize
    } else {
        new as usize
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct RPos {
    row: i32,
    col: i32,
}

impl RPos {
    fn convert(&self, max: Position) -> Position {
        Position::new(
            convert(self.row as i32, max.row as i32),
            convert(self.col as i32, max.col as i32)
        )
    }
}

fn move_rpos(dir: &Dir, pos: &RPos) -> RPos {
    let mut row = pos.row;
    let mut col = pos.col;
    match dir {
        Dir::Up => row -= 1,
        Dir::Right => col += 1,
        Dir::Down => row += 1,
        Dir::Left => col -= 1,
    }
    RPos{row, col}
}

fn get_adj_rpos(pos: &RPos) -> Vec<RPos> {
    let mut out = Vec::new();
    for dir in Dir::iter() {
        out.push(move_rpos(&dir, pos));
    }
    out
}

fn get_n_reach(map: &Map<Tile>, start: RPos, n_steps: u64) -> u64 {
    let mut reach = 0;
    let modulo = n_steps % 2;
    let mut visited = HashSet::new();
    let mut frontier = HashSet::new();

    frontier.insert(start);

    if modulo == 0 {
        reach += 1;
    }

    for i in 1..=n_steps {
        let mut new_frontier = HashSet::new();
        for pos in frontier.iter() {
            visited.insert(pos.clone());
            let adjs = get_adj_rpos(pos);
            for a in adjs {
                let a_pos = a.convert(map.max_position());
                if let Some(tile) = map.get_position(&a_pos) {
                    if *tile != Tile::Rock && !visited.contains(&a) {
                        if new_frontier.insert(a) && i % 2 == modulo {
                            reach += 1;
                        }
                    }
                }
            }
        }
        frontier = new_frontier;
    }
    reach
}

fn get_n_reach3(map: &Map<Tile>, start: RPos, n_steps: [u64; 3]) -> [f64; 3] {
    let mut count = 0.0;
    let mut reach = [0.0; 3];
    let mut n = 0;
    let modulo = n_steps[n] % 2;
    let mut visited = HashSet::new();
    let mut frontier = HashSet::new();

    frontier.insert(start);

    if modulo == 0 {
        count += 1.0;
    }

    for i in 1..=n_steps[2] {
        let mut new_frontier = HashSet::new();
        for pos in frontier.iter() {
            visited.insert(pos.clone());
            let adjs = get_adj_rpos(pos);
            for a in adjs {
                let a_pos = a.convert(map.max_position());
                if let Some(tile) = map.get_position(&a_pos) {
                    if *tile != Tile::Rock && !visited.contains(&a) {
                        if new_frontier.insert(a) && i % 2 == modulo {
                            count += 1.0;
                        }
                    }
                }
            }
        }
        frontier = new_frontier;
        if i == n_steps[n] {
            reach[n] = count;
            n += 1;
        }
    }
    reach
}
