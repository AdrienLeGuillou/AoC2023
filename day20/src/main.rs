// for part 2:
//  the input to rx is a `conv(4)`. I checked (manually) if there was cycles in
//  each 4 components.
//  Yes, and they even start at 1. Then -> lcm -> success
//
// I feel it's quite ugly but it works
use num::Integer;
use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    // let input = std::fs::read_to_string("example1.txt").unwrap();
    // let input = std::fs::read_to_string("example2.txt").unwrap();
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut nodes: HashMap<String, Node> = HashMap::new();
    let mut conjs = Vec::new();
    for l in input.lines() {
        let (in_node, outs) = l.split_once(" -> ").unwrap();
        let (in_mod, in_name) = Module::parse(in_node);

        if let Module::Conj {in_names: _, in_pulses: _} =  in_mod {
            conjs.push(in_name.clone());
        }

        let mut outs: Vec<String> = outs.split(", ")
                                    .map(|x| x.to_owned())
                                    .collect();

        for o in outs.iter() {
            if let Some(node) = nodes.get_mut(o) {
                node.ins.push(in_name.clone());
            } else {
                let node = Node {
                    ins: vec![in_name.clone()],
                    outs: Vec::new(),
                    module: Module::Untyped,
                };
                nodes.insert(o.clone(), node);
            }
        }

        if let Some(node) = nodes.get_mut(&in_name) {
            node.module = in_mod;
            node.outs.append(&mut outs);
        } else {
            let node = Node {
                ins: Vec::new(),
                outs,
                module: in_mod,
            };
            nodes.insert(in_name, node);
        }
    }

    for c in conjs {
        if let Some(node) = nodes.get_mut(&c) {
            node.module = Module::Conj{
                in_names: node.ins.clone(),
                in_pulses: vec![Pulse::L; node.ins.len()],
            }
        }
    }

    // println!("{:?}", nodes);

    let mut nodes_sav = nodes.clone();

    // Part1 -------------------------------------------------------------------
    let mut n_low = 0;
    let mut n_high = 0;

    for _ in 0..1000 {
        let mut pulses = VecDeque::new();
        pulses.push_back((
            Pulse::L,
            "broadcaster".to_owned(),
            "button".to_owned()
        ));
        while let Some(p) = pulses.pop_front() {
            if p.0 == Pulse::L {
                n_low += 1;
            } else {
                n_high += 1;
            }

            let cur_node = nodes.get_mut(&p.1).unwrap();
             // println!("{:?}", pulses);
             // println!("cur: {:?}, pulse: {:?}", cur_node, p);
             // println!("");

            if let Some(p_next) = cur_node.module.process_input(p.0, &p.2) {
                for o in cur_node.outs.iter() {
                    pulses.push_back((p_next, o.clone(), p.1.clone()));
                }
            }
        }
    }
    let answ = n_low * n_high;
    println!("P1: {}", answ);


    // Part2 -------------------------------------------------------------------
    let mut nodes = nodes_sav;
    let mut answ = 0;
    let mut done = false;

    let mut hs: Vec<Vec<u64>> = vec![Vec::new(); 4];

    // while answ < 10000 {
    while !done {
        answ += 1;
        let mut pulses = VecDeque::new();
        pulses.push_back((
            Pulse::L,
            "broadcaster".to_owned(),
            "button".to_owned()
        ));

        while let Some(p) = pulses.pop_front() {
            let cur_node = nodes.get_mut(&p.1).unwrap();

            if let Some(p_next) = cur_node.module.process_input(p.0, &p.2) {
                for o in cur_node.outs.iter() {
                    pulses.push_back((p_next, o.clone(), p.1.clone()));
                }
            }

            if  p.1 == "zh" && p.0 == Pulse::H {
                // println!("n: {answ} - {:?}", p);
                // println!("{:?}", cur_node);
                if let Module::Conj{in_names: _, in_pulses} = &cur_node.module {
                    let mut ok = true;
                    for i in 0..in_pulses.len() {
                        if in_pulses[i] == Pulse::H {
                            hs[i].push(answ);
                        }
                        if hs[i].len() < 1 {
                            ok = false;
                        }
                    }

                    if ok {
                        println!("{:?}", hs);
                        done = true;
                        break;
                    }
                }
            }

        }
    }
    let mut out = hs[0][0];
    for i in 1..hs.len() {
        out = out.lcm(&hs[i][0]);
    }
    println!("P2: {}", out);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pulse {
    H,
    L,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Module {
    Broad,
    Flip(bool),
    Conj{in_names: Vec<String>, in_pulses: Vec<Pulse>},
    Untyped,
}

impl Module {
    fn process_input(&mut self, p: Pulse, in_node: &str) -> Option<Pulse> {
        match self {
            Self::Flip(on) if p == Pulse::L => {
                *on = !*on;
                if *on { Some(Pulse::H) } else { Some(Pulse::L) }
            },
            Self::Conj{in_names, in_pulses} => {
                let mut all_highs = true;
                for i in 0..in_names.len() {
                    if in_names[i] == in_node {
                        in_pulses[i] = p;
                    }

                    if in_pulses[i] == Pulse::L {
                        all_highs = false;
                    }
                }

                if all_highs { Some(Pulse::L) } else { Some(Pulse::H) }
            },
            Self::Broad => Some(p),
            _ => None,
        }
    }

    fn parse(s: &str) -> (Self, String) {
        match s.chars().next() {
            Some('%') => (Self::Flip(false), s[1..].to_owned()),
            Some('&') => (
                Self::Conj{in_names: Vec::new(), in_pulses: Vec::new()},
                s[1..].to_owned()
            ),
            _ => {
                if s == "broadcaster" {
                    (Self::Broad, s.to_owned())
                } else {
                    (Self::Untyped, s.to_owned())
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Node {
    ins: Vec<String>,
    outs: Vec<String>,
    module: Module,
}

impl Node {

}

