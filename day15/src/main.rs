use std::collections::VecDeque;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    // let input = std::fs::read_to_string("example1.txt").unwrap();

    println!("{}", hash_string("qp"));

    // Part 1
    let answ = input.trim_end().split(",").fold(0, |a, x| a + hash_string(x));
    println!("P1: {answ}");

    // Part 2
    let mut boxes: Vec<VecDeque<Lens>> = Vec::with_capacity(256);
    for _ in 0..256 {
        boxes.push(VecDeque::new());
    }

    for s in input.trim_end().split(",") {
        parse_instr(s, &mut boxes);
    }
    // println!("{:?}", boxes);

    let mut answ = 0;
    for i in 0..boxes.len() {
        answ += fp(&boxes[i]) * (i + 1) as u64;
    }
    println!("P2: {answ}");
}

fn parse_instr(s: &str, boxes: &mut Vec<VecDeque<Lens>>) {
    if s.contains('=') {
        add_lens(s, boxes);
    } else {
        rm_lens(s, boxes);
    }
}

fn fp(boxe: &VecDeque<Lens>) -> u64 {
    let mut out = 0;
    let mut n = 1;
    for l in boxe {
        out += n * l.focal;
        n += 1;
    }
    out
}

fn rm_lens(s: &str, boxes: &mut Vec<VecDeque<Lens>>) {
    let (lab, _) = s.split_once("-").unwrap();
    let box_i = hash_string(lab) as usize;

    let lab_i = find_label(&boxes[box_i], lab);
    if let Some(i) = lab_i {
        boxes[box_i].remove(i);
    }
}

fn add_lens(s: &str, boxes: &mut Vec<VecDeque<Lens>>) {
    let (lab, focal_s) = s.split_once("=").unwrap();
    let focal: u64 = focal_s.parse().unwrap();
    let box_i = hash_string(lab) as usize;
    let lab_i = find_label(&boxes[box_i], lab);
    if let Some(i) = lab_i {
        boxes[box_i].get_mut(i).unwrap().focal = focal;
    } else {
        boxes[box_i].push_back(Lens {label: lab.to_owned(), focal})
    }
}

fn find_label(boxe: &VecDeque<Lens>, lab: &str) -> Option<usize> {
    for i in 0..boxe.len() {
        if let Some(lens) = boxe.get(i) {
            if lab == lens.label {
                return Some(i);
            }
        }
    }
    None
}

fn hash_char(c: char, cur: u64) -> u64 {
    ((c as u64 + cur) * 17) % 256
}

fn hash_string(s: &str) -> u64 {
    s.chars().fold(0, |a, x| hash_char(x, a))
}

#[derive(Debug)]
struct Lens {
    label: String,
    focal: u64,
}

