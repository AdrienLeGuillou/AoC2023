use std::collections::HashMap;
use num::Integer;

fn main() {
    // Part 1
    let input = std::fs::read_to_string("input.txt").unwrap();
    let directions = input.lines().next().unwrap();
    let directions = parse_direction(directions);

    let mut map: HashMap<String, [String; 2]> = HashMap::new();
    for s in input.lines().skip(2) {
        parse_map_node(s, &mut map);
    }
    let mut pos = "AAA".to_owned();
    let mut steps = 0;
    let mut dir_i = 0_usize;

    while pos != "ZZZ" {
        let dir = directions[dir_i];
        dir_i = (dir_i + 1) % directions.len();
        pos = get_next(&pos, &map, dir);
        steps += 1;
    }
    println!("part1: {steps}");

    // Part 2
    let input = std::fs::read_to_string("input.txt").unwrap();
    let directions = input.lines().next().unwrap();
    let directions = parse_direction(directions);

    let mut map: HashMap<String, [String; 2]> = HashMap::new();
    let mut positions = Vec::new();
    for s in input.lines().skip(2) {
        let pos = parse_map_node(s, &mut map);
        if node_end_with(&pos, 'A') {
            positions.push(pos.to_owned());
        }
    }
    println!("positions: {:?}", positions);

    // let mut steps = 0;
    // let mut dir_i = 0_usize;
    // while !is_arrived(&positions) {
    //     let dir = directions[dir_i];
    //     dir_i = (dir_i + 1) % directions.len();
    //     positions = positions.iter().map(|x| get_next(x, &map, dir)).collect();
    //     steps += 1;
    // }
    // println!("part2: {steps}");

    let mut periods = Vec::new();
    for p in positions {
        let mut steps = 0;
        let mut dir_i = 0_usize;
        let mut pos = p.clone();
        while !node_end_with(&pos, 'Z') {
            let dir = directions[dir_i];
            dir_i = (dir_i + 1) % directions.len();
            pos = get_next(&pos, &map, dir);
            steps += 1;
        }
        periods.push(steps as u64);
    }

    let mut out = periods[0];
    for i in 1..periods.len() {
        out = out.lcm(&periods[i]);
    }
    println!("part2: {out}");

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, 4);
    }
}

fn parse_map_node(s: &str, map: &mut HashMap<String, [String; 2]>) -> String {
    let (pos, dirs) = s.split_once(" = ").unwrap();
    let dirs = dirs.replace("(", "").replace(")", "").replace(" ", "");
    let (l, r) = dirs.split_once(",")
                     .unwrap();
    map.insert(pos.to_owned(), [l.to_owned(), r.to_owned()]);
    pos.to_owned()
}

fn parse_direction(s: &str) -> Vec<usize> {
    s.chars().map(|x| match x {
        'R' => 1,
        _  => 0,
    }).collect()
}

fn node_end_with(node: &str, c: char) -> bool {
    node.chars().nth(2).unwrap() == c
}

fn is_arrived(positions: &Vec<String>) -> bool {
    for p in positions.iter() {
        if !node_end_with(p, 'Z') {
            return false
        }
    }
    return true
}

fn get_next(p: &str, map: &HashMap<String, [String; 2]>, dir: usize) -> String {
    map.get(p).unwrap()[dir].clone()
}

fn advance_n(n: usize, p: &str, map: &HashMap<String, [String; 2]>,
             dir_i: usize, directions: &Vec<usize>) -> String {
    let mut pos = p.to_owned();
    let mut n = n;
    let mut dir_i = dir_i;
    while n > 0 {
        let dir = directions[dir_i];
        dir_i = (dir_i + 1) % directions.len();
        pos = get_next(&pos, map, dir);
        n -= 1;
    }
    pos
}
