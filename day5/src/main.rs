use std::fs;

fn main() {
    let input = read_input("input.txt");

    // read seeds
    let s_line = input.lines().nth(0).unwrap();
    let seeds = get_seeds(s_line) ;

    // parse converters
    let mut converters = Vec::new();
    for l in input.lines().skip(2) {
        if l.contains("map:") {
            converters.push(Converter::parse(l));
        } else if l != "" {
            let last = converters.len() - 1;
            converters[last].add_range(l);
        }
    }


    // part 1 - convert each seed to location, get MIN
    let answer_p1 = seeds.iter()
                         .map(|&x| full_conversion(x, &converters))
                         .fold(u64::MAX, |m, x| m.min(x));

    println!("Part 1: {answer_p1}");

    // part 2
    //
    // read seed ranges
    let seed_ranges = get_seed_ranges(s_line) ;
    // convert the ranges and not the individual seeds (1e8)
    let conv_ranges = full_ranges_conversion(seed_ranges, &converters);
    // get the MIN lower bound
    let answer_p2 = conv_ranges.iter().fold(u64::MAX, |a, x| a.min(x[0]));
    println!("Part 2: {answer_p2}");
}

fn read_input(file_path: &str) -> String {
    fs::read_to_string(file_path)
        .expect("Should have been able to read the file")
}

// read seeds  -----------------------------------------------------------------
fn get_seeds(s: &str) -> Vec<u64> {
    let (_, seeds) = s.split_once(": ").unwrap();
    seeds.split(" ").map(|x| x.parse::<u64>().unwrap()).collect()
}

fn get_seed_ranges(s: &str) -> Vec<[u64; 2]> {
    let (_, seeds) = s.split_once(": ").unwrap();
    let ranges: Vec<u64> = seeds.split(" ")
                                .map(|x| x.parse::<u64>().unwrap())
                                .collect();
    let mut seeds_ranges = Vec::new();
    let mut i = 0;

    while i < ranges.len() - 1 {
        seeds_ranges.push([ranges[i], ranges[i] + ranges[i + 1] - 1]);
        i += 2
    }

    seeds_ranges
}

// convert from seed to location - chain all converters ------------------------
fn full_conversion(v: u64, converters: &Vec<Converter>) -> u64 {
    converters.iter()
              .fold(v, |a, x| x.convert(a))
}

// range version
fn full_ranges_conversion(rs: Vec<[u64; 2]>, converters: &Vec<Converter>) -> Vec<[u64; 2]> {
    converters.iter()
              .fold(rs, |a, x| x.convert_ranges(a))
}

// Converters ------------------------------------------------------------------
#[derive(Debug)]
struct Converter {
    from: String,
    to: String,
    ranges: Vec<Range>,
}

impl Converter {
    fn parse(s: &str) -> Self {
        let (elts, _) = s.split_once(" ").unwrap();
        let (f, t) = elts.split_once("-to-").unwrap();
        Self::new(f, t)
    }

    fn new(from: &str, to: &str) -> Self {
        Self {
            from: from.to_owned(),
            to: to.to_owned(),
            ranges: Vec::new()
        }
    }

    fn add_range(&mut self, s: &str) {
        self.ranges.push(Range::parse(s));
    }

    // convert an input to the next type
    fn convert(&self, v: u64) -> u64 {
        for i in 0..self.ranges.len() {
            if let Some(x) = self.ranges[i].convert(v) {
                return x
            }
        }
        v
    }

    // convert ranges to the next type
    fn convert_ranges(&self, rs: Vec<[u64; 2]>) -> Vec<[u64;2]> {
        let mut inputs = rs.clone();
        let mut next = Vec::new();
        let mut out = Vec::new();

        for i in 0..self.ranges.len() {
            next = Vec::new();
            for part in inputs {
                let (c, r) = self.ranges[i].convert_range(part);
                if let Some(x) = c {
                    out.push(x);
                }
                if let Some(mut x) = r {
                    next.append(&mut x);
                }
            }
            inputs = next.clone();
        }

        out.append(&mut next);
        combine(out)
    }
}

// ranges converters -----------------------------------------------------------
#[derive(Debug)]
struct Range {
    from: u64,
    to: u64,
    offset: i64,
}

impl Range {
    fn parse(s: &str) -> Self {
        let ns: Vec<u64> = s.split(" ")
                            .map(|x| x.parse::<u64>()
                            .unwrap())
                            .collect();

        let from = ns[1];
        let to = ns[1] + ns[2] - 1;
        let offset = ns[0] as i64 - ns[1] as i64;

        Self::new(from, to, offset)
    }

    fn new(from: u64, to: u64, offset: i64) -> Self {
        Self {from, to, offset}
    }

    fn convert(&self, v: u64) -> Option<u64> {
        if v >= self.from && v <= (self.to) {
            Some((v as i64 + self.offset) as u64)
        } else {
            None
        }
    }

    // return (converted, remainder)
    fn convert_range(&self, rs: [u64; 2]) -> (Option<[u64; 2]>, Option<Vec<[u64; 2]>>) {
        let c_range = [self.from, self.to];
        if is_overlap(c_range, rs) {
            let convertible = intersect(c_range, rs);
            let conv = self.update_range(convertible);
            let rem = diff(convertible, rs);
            (Some(conv), rem)
        } else {
            (None, Some(vec![rs]))
        }
    }

    fn update_range(&self, rs: [u64; 2]) -> [u64;2] {
        [
            (rs[0] as i64 + self.offset) as u64,
            (rs[1] as i64 + self.offset) as u64
        ]
    }
}

// set functions ---------------------------------------------------------------
fn is_overlap(r1: [u64; 2], r2: [u64; 2]) -> bool {
    (r1[1] >= r2[0] && r1[0] <= r2[1]) || ((r2[1] >= r1[0] && r2[0] <= r1[1]))
}

fn intersect(r1: [u64; 2], r2: [u64; 2]) -> [u64; 2] {
    [r1[0].max(r2[0]), r1[1].min(r2[1])]
}

fn diff(r1: [u64; 2], r2: [u64; 2]) -> Option<Vec<[u64; 2]>> {
    let mut rem = Vec::new();

    if ! is_overlap(r1, r2) {
        return None;
    }

    // left remainder
    if r2[0] < r1[0] {
        rem.push([r2[0], r1[0] - 1]);
    }

    // right remainder
    if r2[1] > r1[1] {
        rem.push([r1[1] + 1, r2[1]]);
    }

    Some(rem)
}

// combine overlapping and contiguous sets
fn combine(mut r_in: Vec<[u64; 2]>) -> Vec<[u64; 2]> {
    r_in.sort();
    let mut cur =  r_in[0];
    let mut out = Vec::new();
    for r_c in r_in.iter().skip(1) {
        if cur[1] >= r_c[0] - 1 {
            cur[1] = r_c[1];
        } else {
            out.push(cur);
            cur = *r_c;
        }
    }
    out.push(cur);
    out
}
