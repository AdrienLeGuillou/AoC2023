use std::fs;

fn main() {
    let input = read_input("input.txt");
    let mut answer = 0;

    let mut cards: Vec<Card> = input.lines()
                               .map(parse_card)
                               .collect();

    // Part 1
    let earnings = cards.iter().fold(0, |acc, x| acc + x.get_earnings());
    println!("Answer p1:{}", earnings);

    // Part 2
    for i in 0..cards.len() {
        let copies = cards[i].get_copies();
        let count = cards[i].count;
        for c in copies {
            if c <= cards.len() {
                cards[c - 1].add_copies(count)
            }
        }
    }
    let n_cards = cards.iter().fold(0, |acc, x| acc + x.count);
    println!("Answer p2:{}", n_cards);
}

fn read_input(file_path: &str) -> String {
    fs::read_to_string(file_path)
        .expect("Should have been able to read the file")
}

#[derive(Debug)]
struct Card {
    id: usize,
    count: usize,
    w_nums: Vec<u32>,
    s_nums: Vec<u32>,
}

impl Card {
    fn new(id: usize, w_nums: Vec<u32>, s_nums: Vec<u32>) -> Self{
        Self {id, count: 1, w_nums, s_nums}
    }

    fn get_earnings(&self) -> usize {
        let n_wins = self.get_n_matches();
        match n_wins {
            0 => 0,
            x => 2_usize.pow(x - 1) ,
        }
    }

    fn get_n_matches(&self) -> u32 {
        self.s_nums
            .iter()
            .fold(0, |acc, x| acc + self.w_nums.contains(x) as u32)
    }

    fn get_copies(&self) -> Vec<usize> {
        (
            (self.id + 1)..=(self.id + self.get_n_matches() as usize)
        ).collect()
    }

    fn add_copies(&mut self, copies: usize) {
        self.count += copies;
    }
}

fn parse_card(s: &str) -> Card {
    let (p_id, p_cards) = s.split_once(": ").unwrap();
    let id = p_id.rsplit_once(" ").unwrap().1.trim().parse::<usize>().unwrap();

    let (w_p, s_p) = p_cards.split_once("| ").unwrap();
    let w_nums = parse_numbers(w_p);
    let s_nums = parse_numbers(s_p);

    Card::new(id, w_nums, s_nums)
}

fn parse_numbers(s: &str) -> Vec<u32> {
    s.split(" ")
     .filter(|&x| x != "")
     .map(|x| x.parse::<u32>().unwrap())
     .collect()
}
