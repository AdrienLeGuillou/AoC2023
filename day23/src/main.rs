use strum::IntoEnumIterator;
use std::collections::{HashMap, HashSet, LinkedList, VecDeque};
mod map;
use crate::map::{Dir, Position};

fn main() {
    // let input = std::fs::read_to_string("example1.txt").unwrap();
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut map = Vec::new();
    for l in input.lines() {
        map.push(l.chars().collect::<Vec<char>>());
    }
    let max_map = Position::new(map.len(), map[0].len());
    let start_pos = get_ends(&map, 0);
    let end_pos = get_ends(&map, map.len() - 1);

    // Part1 -------------------------------------------------------------------
    let mut visited: HashSet<Position> = HashSet::new();
    let mut answ = 0;
    visited.insert(start_pos);

    let next_pos = Dir::Down.move_n(&start_pos, 1, &max_map).unwrap();

    get_max(&next_pos, &end_pos, &start_pos,
            &map, &mut visited, 0, &mut answ);
    println!("P1: {}", answ);

    // Part2 -------------------------------------------------------------------
    let mut mem_map: HashMap<Position, [NodeTo; 4]> = HashMap::new();
    populate_mem_map(&mut mem_map, &start_pos, &end_pos, &map);

    // for m in mem_map.iter() { println!("{:?}", m); }

    let mut visited: HashSet<Position> = HashSet::new();
    let mut answ = 0;
    get_max_p2(&start_pos, &Dir::Right, &end_pos, &mem_map, &mut visited, 0, &mut answ);
    println!("P2: {}", answ);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NodeTo {
    None,
    To(Position, usize),
    Unknown,
}

fn populate_mem_map(mem_map: &mut HashMap<Position, [NodeTo; 4]>,
                    start_pos: &Position, end_pos: &Position,
                    map: &Vec<Vec<char>>) {
    let mut list_pos = VecDeque::new();
    list_pos.push_front(*start_pos);
    if !mem_map.contains_key(&start_pos) {
        mem_map.insert(*start_pos, surroundings(&start_pos, &map));
    }

    while let Some(p) = list_pos.pop_back() {
        let node = mem_map.get(&p).unwrap().clone();

        for i in 0..node.len() {
            if node[i] != NodeTo::Unknown {continue;}

            let next_int = to_next_intersect(&p, Dir::from_usize(i),
                                             &map, &end_pos);
            if let Some((n_p, n_d, n_n)) = next_int {
                let node = mem_map.get_mut(&p).unwrap();
                node[i] = NodeTo::To(n_p, n_n);

                if !mem_map.contains_key(&n_p) {
                    mem_map.insert(n_p, surroundings(&n_p, &map));
                }
                let node = mem_map.get_mut(&n_p).unwrap();
                node[n_d.opposite().as_usize()] = NodeTo::To(p, n_n);

                list_pos.push_back(n_p);
            } else {
                let node = mem_map.get_mut(&p).unwrap();
                node[i] = NodeTo::None;
            }
        }
    }
}

fn surroundings(pos: &Position, map: &Vec<Vec<char>>) -> [NodeTo; 4] {
    let max_map = Position::new(map.len(), map[0].len());
    let mut out = [NodeTo::None; 4];
    for d in Dir::iter() {
        if let Some(p) = d.move_n(pos, 1, &max_map) {
            if map[p.row][p.col] != '#' {
                out[d.as_usize()] = NodeTo::Unknown;
            }
        }
    }
    out
}

fn to_next_intersect(start_pos: &Position, d: Dir, map: &Vec<Vec<char>>,
                     end_pos: &Position) -> Option<(Position, Dir, usize)> {
    let max_map = Position::new(map.len(), map[0].len());
    let mut n = 0;
    let mut prev_pos = *start_pos;
    let mut cur_pos = d.move_n(&prev_pos, 1, &max_map).unwrap();
    let mut cur_dir = Dir::Down;

    loop {
        n += 1;
        let adjs = get_adj_d(&cur_pos, map, &prev_pos);
        // Path
        if adjs.len() == 1 {
            prev_pos = cur_pos;
            cur_pos = adjs[0].0;
            cur_dir = adjs[0].1;
        // intersection
        } else if adjs.len() > 1 || cur_pos == *end_pos {
            return Some((cur_pos, cur_dir, n));
        // dead end
        } else {
            return None;
        }
    }
}

fn get_adj_d(pos: &Position, map: &Vec<Vec<char>>,
           prev: &Position) -> Vec<(Position, Dir)> {
    let max_map = Position::new(map.len(), map[0].len());
    let mut out = Vec::new();

    let tile = map[pos.row][pos.col];
    for d in get_dirs(&tile) {
        if let Some(p) =  d.move_n(&pos, 1, &max_map) {
            if map[p.row][p.col] != '#' &&  p != *prev {
                out.push((p, d));
            }
        }
    }
    out
}

fn get_next_dir(pos: &Position, map: &Vec<Vec<char>>,
           prev: &Position) -> Vec<Dir> {
    let max_map = Position::new(map.len(), map[0].len());
    let mut out = Vec::new();
    for d in Dir::iter() {
        if let Some(p) =  d.move_n(&pos, 1, &max_map) {
            if map[p.row][p.col] != '#' &&  p != *prev {
                out.push(d);
            }
        }
    }
    out
}

fn get_max(start_pos: &Position, end_pos: &Position, prev_pos: &Position,
           map: &Vec<Vec<char>>, visited: &mut HashSet<Position>,
           cur_len: u64, max_len: &mut u64) {

    let mut cur_pos = *start_pos;
    let mut prev_pos = *prev_pos;
    let mut cur_vis: LinkedList<Position> = LinkedList::new();
    let mut cur_len = cur_len; // we start one after the start

    loop {
        cur_len += 1;

        if visited.contains(&cur_pos) { break; }
        visited.insert(cur_pos);
        cur_vis.push_back(cur_pos);

        let adjs = get_adj(&cur_pos, map, &prev_pos);

        match adjs.len() {
            0 => break,// done
            1 => {
                prev_pos = cur_pos;
                cur_pos = adjs[0];
            },
            _ => {
                // HashMap could be added here
                for a in adjs {

                    get_max(&a, end_pos, &cur_pos, map, visited,
                            cur_len, max_len);
                }
            },
        };
    }

    // when the fn exits, rm from the visited HashSet what was visited in this
    // part
    for v in cur_vis {
        visited.remove(&v);
    }

    if &cur_pos == end_pos && cur_len > *max_len {
        *max_len = cur_len;
    }
}

fn get_ends(map: &Vec<Vec<char>>, row: usize) -> Position {
    let mut i = 0;
    while i < map[row].len() {
        if map[row][i] == '.' {
            break;
        }
        i += 1;
    }
    return Position::new(row, i);
}

fn get_adj(pos: &Position, map: &Vec<Vec<char>>,
           prev: &Position) -> Vec<Position> {
    let max_map = Position::new(map.len(), map[0].len());
    let mut out = Vec::new();

    let tile = map[pos.row][pos.col];
    for d in get_dirs(&tile) {
        if let Some(p) =  d.move_n(&pos, 1, &max_map) {
            if map[p.row][p.col] != '#' &&  p != *prev {
                out.push(p);
            }
        }
    }

    out
}


fn get_dirs(c: &char) -> Vec<Dir> {
    match c {
        '>' => vec![Dir::Right],
        '<' => vec![Dir::Left],
        'v' => vec![Dir::Down],
        '^' => vec![Dir::Up],
        _   => vec![Dir::Up, Dir::Right, Dir::Down, Dir::Left,],
    }
}

fn get_max_p2(start_pos: &Position, prev_dir: &Dir, end_pos: &Position,
              mem_map: &HashMap<Position, [NodeTo; 4]>,
              visited: &mut HashSet<Position>,
              cur_len: u64, max_len: &mut u64) {

    // println!("{start_pos:?}, {cur_len}");
    if start_pos == end_pos {
        if cur_len > *max_len {
            *max_len = cur_len;
        }
        return;
    }

    if visited.contains(start_pos) { return; }
    visited.insert(*start_pos);

    let node = mem_map.get(start_pos).unwrap();

    for i in 0..node.len() {
        if node[i] == NodeTo::None {continue;}

        if let NodeTo::To(p, n) = node[i] {
            get_max_p2(&p, &Dir::from_usize(i), end_pos,
                       mem_map, visited, cur_len + n as u64, max_len);
        }
    }
    visited.remove(start_pos);
}
