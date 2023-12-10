fn main() {
    // Part 1
    let input = std::fs::read_to_string("input.txt").unwrap();
    let sequences: Vec<Vec<i64>> = input.lines()
                                   .map(parse_seq)
                                   .collect();

    let ans_p1: i64 = sequences.iter().map(get_next).fold(0, |a, x| a + x);
    println!("Part 1: {}", ans_p1);

    let ans_p2: i64 = sequences.iter().map(get_prev).fold(0, |a, x| a + x);
    println!("Part 2: {}", ans_p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, 4);
    }
}

fn parse_seq(s: &str) -> Vec<i64> {
    s.split(" ").map(|x| x.parse::<i64>().unwrap())
                .collect()
}

fn get_next(s: &Vec<i64>) -> i64 {
    match s[s.len() - 1] - s[s.len() - 2] {
        0 => s[s.len() - 1],
        _ => s[s.len() - 1] + get_next(&seq_diff(s)),
    }
}

fn get_prev(s: &Vec<i64>) -> i64 {
    match s[s.len() - 1] - s[s.len() - 2] {
        0 => s[0],
        _ => s[0] - get_prev(&seq_diff(s)),
    }
}

fn seq_diff(s: &Vec<i64>) -> Vec<i64> {
    let mut sd = Vec::with_capacity(s.len() - 1);
    for i in 0..(s.len() - 1) {
        sd.push(s[i + 1] - s[i]);
    }
    sd
}
