use std::fs;

fn main() {
    let input = read_input("input.txt");
    let mut result = 0;

    // // Solution to part 1
    // for l in input.lines() {
    //     let first = l.chars()
    //                  .nth(first_i).unwrap()
    //                  .to_digit(10).unwrap();
    //
    //     let last_i = l.rfind(is_09).unwrap();
    //     let last = l.chars()
    //                  .nth(last_i).unwrap()
    //                  .to_digit(10).unwrap();
    //
    //     result += first * 10 + last;
    // }

    for l in input.lines() {
        let first = get_first(l);
        let last = get_last(l);
        result += first * 10 + last;
    }

    println!("{}", result);
}

fn read_input(file_path: &str) -> String {
    fs::read_to_string(file_path)
        .expect("Should have been able to read the file")
}

fn is_09(c: char) -> bool {
    char::is_digit(c, 10)
}

fn get_first(s: &str) -> u32 {
    let pats = ["1", "2", "3", "4", "5", "6", "7", "8", "9",
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut min_index = usize::MAX;
    let mut out = 0;
    for i in 0..pats.len() {
        if let Some(index) = s.find(pats[i]) {
            if index <= min_index {
                min_index = index;
                out = (i % 9 + 1) as u32
            }
        }
    }
    out
}

fn get_last(s: &str) -> u32 {
    let pats = ["1", "2", "3", "4", "5", "6", "7", "8", "9",
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut max_index = 0;
    let mut out = 0;
    for i in 0..pats.len() {
        if let Some(index) = s.rfind(pats[i]) {
            if index >= max_index {
                max_index = index;
                out = (i % 9 + 1) as u32
            }
        }
    }
    out
}
