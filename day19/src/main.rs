use std::collections::VecDeque;
use std::collections::HashMap;

fn main() {
    // let input = std::fs::read_to_string("example1.txt").unwrap();
    // let input = std::fs::read_to_string("example2.txt").unwrap();
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut workflows: HashMap<String, Vec<Rule>> = HashMap::new();
    let mut n = 0;

    for l in input.lines() {
        // done with the wfs
        n += 1;
        if l == "" {
            break;
        }

        let (name, rule_s) = l.split_once("{").unwrap();
        let rule_s = rule_s.replace("}", "");
        let rules = rule_s.split(",").map(Rule::parse1).collect();

        workflows.insert(name.to_owned(), rules);
    }

    // Part1 -------------------------------------------------------------------
    let mut answ = 0;
    for l in input.lines().skip(n) {
// println!("{:?}", Part::parse1(l));
        let p = Part::parse1(l);
        if let Some(v) = all_wfs(&p, &workflows, "in") {
            answ += v;
        }
    }
    println!("P1: {}", answ);


    // Part2 -------------------------------------------------------------------

    println!("P2: {}", all_wfs_ranges(State::default(), &workflows));
}

#[derive(Debug, Clone)]
struct State {
    wf: String,
    rule_n: usize,
    ranges: [[u32; 2]; 4],
}


impl State {
    fn default() -> Self {
        Self {
            wf: "in".to_owned(),
            rule_n: 0,
            ranges: [[1, 4000], [1, 4000], [1, 4000], [1, 4000]],
        }
    }

    fn copy_to(&self, s: &str) -> Self {
        Self {
            wf: s.to_owned(),
            rule_n: self.rule_n,
            ranges: self.ranges,
        }
    }

    fn count_perm(&self) -> u64 {
        let mut out = 1;
        for i in 0..4 {
            out *= self.ranges[i][1] as u64 - self.ranges[i][0] as u64 + 1;
        }
        out
    }

    fn apply_range(&self, r: &Rule) -> Vec<State> {
        let mut out = Vec::new();

        let mut s_true = self.clone();
        s_true.rule_n = 0;

        let mut s_false = self.clone();
        s_false.rule_n += 1;

        match r {
            Rule::Out(s) => {
                s_true.wf = s.to_owned();
                out.push(s_true);
            },
            Rule::Gt(pos, val, s) => {
                s_true.wf = s.to_owned();

                s_true.ranges[*pos][0] = s_true.ranges[*pos][0].max(*val + 1);
                s_false.ranges[*pos][1] = s_false.ranges[*pos][1].min(*val);

                if s_true.ranges[*pos][0] <= s_true.ranges[*pos][1] {
                    out.push(s_true);
                }
                if s_false.ranges[*pos][0] <= s_false.ranges[*pos][1] {
                    out.push(s_false);
                }
            },
            Rule::Lt(pos, val, s) => {
                s_true.wf = s.to_owned();

                s_true.ranges[*pos][1] = s_true.ranges[*pos][1].min(*val - 1);
                s_false.ranges[*pos][0] = s_false.ranges[*pos][0].max(*val);

                if s_true.ranges[*pos][0] <= s_true.ranges[*pos][1] {
                    out.push(s_true);
                }
                if s_false.ranges[*pos][0] <= s_false.ranges[*pos][1] {
                    out.push(s_false);
                }
            },
        }
        out
    }

    fn apply_wf(&self, wfs: &HashMap<String, Vec<Rule>>) -> Vec<Self> {
        let wf = wfs.get(&self.wf).unwrap();
        self.apply_range(&wf[self.rule_n])
    }
}

fn all_wfs_ranges(s: State, wfs: &HashMap<String, Vec<Rule>>) -> u64 {
    let mut out = 0;
    let mut states = VecDeque::new();
    states.push_back(s);

    while let Some(s) = states.pop_front() {
        if s.wf == "R" {
            continue;
        } else if s.wf == "A" {
            out += s.count_perm();
            continue;
        }

        let new_states = s.apply_wf(wfs);
        for ns in new_states {
            states.push_back(ns);
        }
    }
    out
}




fn apply_wf(p: &Part, wf: &Vec<Rule>) -> String {
    for r in wf {
        if let Some(s) = r.apply(p) {
            return s
        }
    }
    "".to_owned()
}

fn all_wfs(p: &Part, wfs: &HashMap<String, Vec<Rule>>, first: &str) -> Option<u32> {
    let mut next_name = first.to_owned();
    loop {
// println!("{next_name}");
        let cur_wf = wfs.get(&next_name).unwrap();
        next_name = apply_wf(&p, cur_wf);
        if next_name == "A" {
            return Some(p.get_val());
        } else if next_name == "R" {
            return None
        }
    }
}


#[derive(Debug)]
enum Rule {
    Gt(usize, u32, String),
    Lt(usize, u32, String),
    Out(String),
}

impl Rule {
    fn parse1(s: &str) -> Self {
        if let Some((beg, end)) = s.split_once(":") {
            let pos = match &beg[0..1] {
                "x" => 0,
                "m" => 1,
                "a" => 2,
                _   => 3,
            };

            let val: u32 = beg[2..].parse().unwrap();
            if &beg[1..2] == "<" {
                Self::Lt(pos, val, end.to_owned())
            } else {
                Self::Gt(pos, val, end.to_owned())
            }
        } else {
            Self::Out(s.to_owned())
        }
    }

    fn apply(&self, p: &Part) -> Option<String> {
        match self {
            Self::Out(s) => Some(s.to_owned()),
            Self::Gt(pos, val, s) if p.quals[*pos] > *val => Some(s.to_owned()),
            Self::Lt(pos, val, s) if p.quals[*pos] < *val => Some(s.to_owned()),
            _ => None
        }
    }

}

#[derive(Debug)]
struct Part {
    quals: [u32; 4],
}

impl Part {
    fn parse1(s: &str) -> Self {
        let s = &s[1..(s.len() - 1)];
        let quals: Vec<u32> = s.split(",").map(|x| {
            let (_, c) = x.split_once("=").unwrap();
            c.parse::<u32>().unwrap()
        }).collect();

        Self {quals: quals.try_into().unwrap()}
    }

    fn get_val(&self) -> u32 {
        let mut val = 0;
        for i in 0..4 {
            val += self.quals[i];
        }
        val
    }
}
