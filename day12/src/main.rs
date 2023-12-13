fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    // Part1 -------------------------------------------------------------------
    let records: Vec<Record> = input.lines().map(Record::parse).collect();

    let answ = records.iter().fold(0, |a, x| a + x.count_argm());
    println!("p1: {answ}");

    // Part2 -------------------------------------------------------------------
    let records: Vec<Record> = records.iter().map(|x| x.part2()).collect();

    let answ = records.iter().fold(0, |a, x| a + x.count_argm());
    println!("p2: {answ}");
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Spring {
    Operational,
    Damaged,
    Uknown,
}

impl Spring {
    fn parse_c(c: char) -> Self {
        match c {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            _   => Self::Uknown,
        }
    }
}

#[derive(Debug)]
struct Record {
    inner: Vec<Spring>,
    groups: Vec<usize>,
}

impl Record {
    fn parse<'a>(s: &str) -> Self {
        let (rec, grps) = s.split_once(" ").unwrap();
        let inner: Vec<Spring> = rec.chars().map(Spring::parse_c).collect();
        let groups = grps.split(",")
                         .map(|x| x.parse::<usize>().unwrap())
                         .collect();
        Self {
            inner,
            groups,
        }
    }

    fn part2(&self) -> Self {
        let mut inner2 = self.inner.clone();
        let mut groups2 = self.groups.clone();

        for _ in 0..4 {
            inner2.push(Spring::Uknown);
            inner2.append(&mut self.inner.clone());
            groups2.append(&mut self.groups.clone());
        }

        Self {
            inner: inner2,
            groups: groups2,
        }


    }

    fn count_argm(&self) -> usize {
        count_permutations(&self.inner, &self.groups)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct State {
    group: usize,
    count: usize,
    permutation: usize,
}

impl State {
    fn new(group: usize, count: usize) -> Self {
        Self {group, count, permutation: 1}
    }

    fn add_one(&mut self) {
        self.count += 1;
    }

    fn check_op(&mut self, groups: &[usize]) {
        if self.group < groups.len() && self.count == groups[self.group] {
            self.group += 1;
            self.count = 0;
        } else if self.count > 0 {
            // invalidate this state
            self.group = groups.len() + 1;
        }
    }

    fn is_valid(&self, groups: &[usize]) -> bool {
        (self.group < groups.len() && self.count <= groups[self.group]) ||
        (self.is_done(groups))
    }

    fn is_done(&self, groups: &[usize]) -> bool {
        (self.group == groups.len() && self.count == 0) ||
        (self.group == groups.len() - 1 && self.count == groups[self.group])
    }
}

fn count_permutations(rec: &[Spring], groups: &[usize]) -> usize {
    let mut states = Vec::new();
    states.push(State::new(0, 0));

    for s in rec {
        match s {
            Spring::Damaged => {
                for s in states.iter_mut() {
                    s.add_one();
                }
            },
            Spring::Operational => {
                for s in states.iter_mut() {
                    s.check_op(groups);
                }
            },
            Spring::Uknown => {
                // new states where ? = #
                let mut s1 = states.clone();
                for s in s1.iter_mut() {
                    s.add_one();
                }
                // originals where ? = .
                for s in states.iter_mut() {
                    s.check_op(groups);
                }
                // combine
                states.append(&mut s1);
            },
        }
        // keep only valid states
        states = states.into_iter().filter(|x| x.is_valid(groups)).collect();
        states = combine(states);
    }

    states.iter().filter(|x| x.is_done(groups))
                 .fold(0, |a, x| a + x.permutation)
}

fn combine(states: Vec<State>) -> Vec<State> {
    let mut new_states = Vec::new();
    let mut removed: Vec<usize> = Vec::new();

    for i in 0..states.len() {
        if removed.contains(&i) {
            continue;
        }

        let mut ns = states[i];
        for j in (i + 1)..states.len() {
            if states[i].group == states[j].group &&
               states[i].count == states[j].count &&
                   states[i].count == 0 {
                ns.permutation += states[j].permutation;
                removed.push(j);
            }
        }
        new_states.push(ns);
    }
    new_states
}
