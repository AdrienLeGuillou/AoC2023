fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let records: Vec<Record> = input.lines().map(Record::parse).collect();

    // println!("{:?}", car(&records[0].inner, &records[0].groups));
    // println!("{:?}", get_ranges(&records[2].inner, 1));

    // println!("e1: {:?} / 1", records[0].count_argm());
    // println!("e2: {:?} / 4", records[1].count_argm());
    // println!("e3: {:?} / 1", records[2].count_argm());
    // println!("e4: {:?} / 1", records[3].count_argm());
    // println!("e5: {:?} / 4", records[4].count_argm());
    // println!("e6: {:?} / 10", records[5].count_argm());
    // println!("e7: {:?} / ?", records[6].count_argm());

    // println!("{:?}", get_ranges(&records[998].inner[0..15], 2));

    // println!("done: {:?}", records[25 - 1].count_argm());

    let answ = records.iter().fold(0, |a, x| a + x.count_argm());
    println!("p1: {answ}");

    // Part2 -------------------------------------------------------------------
    let records: Vec<Record> = records.iter().map(|x| x.part2()).collect();
    // println!("{:?}", records[0]);
    //
    // println!("e1: {:?} / 1", records[0].count_argm());
    // println!("e2: {:?} / 4", records[1].count_argm());
    // println!("e3: {:?} / 1", records[2].count_argm());
    // println!("e4: {:?} / 1", records[3].count_argm());
    // println!("e5: {:?} / 4", records[4].count_argm());
    // println!("e6: {:?} / 10", records[5].count_argm());

    for i in 0..records.len() {
        println!("{i}: count {}", records[i].count_argm());
    }

    // let answ = records.iter().fold(0, |a, x| a + x.count_argm());
    // println!("p2: {answ}");
}

fn car(rec: &[Spring], gs: &[usize]) -> usize {
    // println!("entering grp {:?}, with rec len {:?}", gs[0], rec.len());
    let mut count = 0;
    let rs = rec.len();
    let ranges = get_ranges(rec, gs[0]);

    // println!("grps rem: {:?}, ranges: {:?}", gs, ranges);

    if gs.len() == 1 {
        // println!("");
        // println!("count: {}", ranges.len());
        return ranges.iter().fold(0, |a, x| a + all_accounted_for(rec, x) as usize);
    }

    for r in ranges {
        let beg = r[1] + 2;
        if beg < rs {
            // println!("chosen: {:?}", r);
            count += car(&rec[beg..rs], &gs[1..gs.len()]);
            // println!("<-- back to: {:?}", gs);
        }
    }

    count
}

fn all_accounted_for(rec: &[Spring], r: &[usize; 2]) -> bool {
    if r[1] + 1 < rec.len() {
        ! rec[(r[1] + 1)..rec.len()].contains(&Spring::Damaged)
    } else {
        true
    }
}

fn get_ranges(rec: &[Spring], g_size: usize) -> Vec<[usize; 2]> {
    let mut ranges = Vec::new();
    let mut count = 0;
    let mut first_d = usize::MAX;

    for i in 0..rec.len() {
        match rec[i] {
            Spring::Operational => count = 0,
            Spring::Damaged => {
                count += 1;
                first_d = first_d.min(i);
            },
            Spring::Uknown => count += 1,
        }

        if count >= g_size {
            if i == rec.len() - 1 || rec[i + 1] != Spring::Damaged {
                ranges.push([i + 1 - g_size, i]);
            }
        }
    }
    ranges.into_iter().filter(|x| x[0] <= first_d).collect()
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
        car(&self.inner, &self.groups)
    }
}

