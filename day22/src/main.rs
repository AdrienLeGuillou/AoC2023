use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    // let input = std::fs::read_to_string("example1.txt").unwrap();
    // let input = std::fs::read_to_string("example2.txt").unwrap();
    let input = std::fs::read_to_string("input.txt").unwrap();

    // parse the bricks and sort them lowest first
    let mut bricks: Vec<Brick> = input.lines().map(Brick::parse).collect();
    bricks.sort();

    // make a volume with `true` when something, `false` if empty
    let space_max = get_max_space(&bricks);
    println!("space_max: {:?}", space_max);
    let mut space =
        vec![vec![vec![false; space_max[2]]; space_max[1]]; space_max[0]];

    // Fill z == 0 with `true` -> presence of something. Here, the floor
    for x in 0..space_max[0] {
        for y in 0..space_max[1] {
            space[x][y][0] = true;
        }
    }

    // make the brick fall down
    for b in bricks.iter_mut() {
        while b.move_down(&mut space) {}
        b.fill_space(&mut space);
    }

    // store the relations between bricks in a HashMap
    let mut relations = HashMap::new();
    let mut n = 0;
    // store the key of each bricks on the level it starts on
    let space_max = get_max_space(&bricks);
    let mut levels = vec![Vec::new(); space_max[2]];
    // fill the HashMap and the levels
    for b in bricks.iter() {
        relations.insert(n, BrickRel::new(&b));
        levels[b.start[2] as usize].push(n);
        n += 1;
    }

    // get the relation between each bricks
    for lvl in 1..levels.len() { // lvl 0 is the floor
        for b_ind in levels[lvl].iter() {
            let cur_brick = relations.get(b_ind).unwrap().brick;
            let mut top_blks = Vec::new();

            if cur_brick.axis == Axis::Z {
                top_blks.push(cur_brick.get_end());
            } else {
                top_blks.append(&mut cur_brick.blocks());
            }

            let above = (top_blks[0][2] + 1) as usize;
            if above >= levels.len() {continue;}

            for b in top_blks.iter_mut() {
                b[2] += 1;
            }

            let mut supports = Vec::new();
            for a_i in levels[above].iter() {
                let a = relations.get_mut(&a_i).unwrap();
                for a_blk in a.brick.blocks() {
                    if top_blks.contains(&a_blk) {
                        supports.push(*a_i);
                        a.rest_on.push(*b_ind);
                        break;
                    }
                }
            }

            let cur_rel = relations.get_mut(b_ind).unwrap();
            cur_rel.support.append(&mut supports);
        }
    }

    // // print bricks
    // for b in bricks.iter() {
    //     println!("{b:?} end: {:?}", b.get_end());
    // }
    //
    // // print relations
    // for r in relations.iter() {
    //     println!("{r:?}");
    // }


   // Part1 -------------------------------------------------------------------
    let mut answ = 0;
    for (_, r) in relations.iter() {
        let mut can_rm = true;

        for s_i in r.support.iter() {
            let s = relations.get(&s_i).unwrap();
            if s.rest_on.len() == 1 {
                can_rm = false;
                break;
            }
        }

        if can_rm {
            answ += 1;
        }
    }
    println!("P1: {}", answ);

    // Part2 -------------------------------------------------------------------
    let mut answ = 0;
    for (ci, r) in relations.iter() {
        let mut fallen = HashSet::new();
        let mut supported = VecDeque::new();

        fallen.insert(ci);
        for s_i in r.support.iter() {
            supported.push_back(s_i);
        }

        while let Some(s_i) = supported.pop_front() {
            if fallen.contains(s_i) {continue;}

            let s = relations.get(&s_i).unwrap();

            let mut fall = true;
            for b in s.rest_on.iter() {
                if !fallen.contains(b) {
                    fall = false;
                    break;
                }
            }

            if fall {
                answ += 1;
                fallen.insert(s_i);
                for n_i in s.support.iter() {
                    supported.push_back(n_i);
                }
            }
        }
    }

    println!("P2: {}", answ);
}

#[derive(Debug)]
struct BrickRel<'a> {
    brick: &'a Brick,
    support: Vec<u32>,
    rest_on: Vec<u32>,
}

impl <'a>BrickRel<'a> {
    fn new(b: &'a Brick) -> Self {
        Self {
            brick: b,
            support: Vec::new(),
            rest_on: Vec::new(),
        }
    }
}

fn get_max_space(bricks: &Vec<Brick>) -> [usize; 3] {
    let mut space_max = [0, 0, 0];
    for b in bricks {
        let e = b.get_end();
        for i in 0..e.len() {
            if e[i] as usize > space_max[i] {
                space_max[i] = e[i] as usize;
            }
        }
    }
    // add 1 as the max elts must be indexable
    for i in 0..space_max.len() {
        space_max[i] += 1;
    }
    space_max
}

#[derive(Debug, Eq, PartialEq)]
enum Axis {
    X, Y, Z
}

impl Axis {
    fn as_usize(&self) -> usize {
        match self {
            Self::X => 0,
            Self::Y => 1,
            Self::Z => 2,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Brick {
    start: [i32; 3],
    axis: Axis,
    length: i32,
}

impl Brick {
    fn parse(s: &str) -> Self {
        let (start, end) = s.split_once("~").unwrap();
        let start =  Self::parse_xyz(start);
        let end = Self::parse_xyz(end);

        let axis = if start[0] != end[0] {
            Axis::X
        } else if start[1] != end[1] {
            Axis::Y
        } else {
            Axis::Z
        };

        let length =  end[axis.as_usize()] - start[axis.as_usize()] + 1;

        Self { start, axis, length, }
    }

    fn parse_xyz(s: &str) -> [i32; 3] {
        let elts: Vec<i32> =s.split(",").map(|x| x.parse().unwrap()).collect();
        elts[0..3].try_into().unwrap()
    }

    fn move_n(&mut self, dir: Axis, n: i32) {
        self.start[dir.as_usize()] += n;
    }

    fn blocks(&self) -> Vec<[i32; 3]> {
        let mut blocks = Vec::new();
        for i in 0..self.length {
            let mut blk = self.start;
            blk[self.axis.as_usize()] += i;
            blocks.push(blk);
        }
        blocks
    }

    fn get_end(&self) -> [i32; 3] {
        let mut blk = self.start;
        blk[self.axis.as_usize()] += self.length - 1;
        blk
    }

    fn fill_space(&mut self, space: &mut Vec<Vec<Vec<bool>>>) {
        for blk in self.blocks() {
            space[blk[0] as usize][blk[1] as usize][blk[2] as usize] = true
        }
    }

    fn move_down(&mut self, space: &Vec<Vec<Vec<bool>>>) -> bool {
        for blk in self.blocks() {
            if space[blk[0] as usize][blk[1] as usize][(blk[2] - 1) as usize] {
                return false;
            }
        }
        self.move_n(Axis::Z, -1);
        true
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start[2].cmp(&other.start[2])
            .then_with(|| self.start[1].cmp(&other.start[1]))
            .then_with(|| self.start[0].cmp(&other.start[0]))
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

