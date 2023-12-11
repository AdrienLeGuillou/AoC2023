use std::fs;

fn main() {
    let input = read_input("input.txt");
    let mut answer = 0;

    let mut symbols: Vec<Symbol> = Vec::new();
    let mut numbers: Vec<Number> = Vec::new();
    let mut l_num: i32 = 0;

    for l in input.lines() {
        symbols.append(&mut find_symbols(l, l_num));
        numbers.append(&mut find_numbers(l, l_num));
        l_num += 1;
    }
    // println!("deb:{:?}", symbols);
    // println!("deb:{:?}", numbers);

    // // Part 1
    // let answer = numbers.iter()
    //                     .filter(|x| x.is_part(&symbols))
    //                     .fold(0, |acc, x| acc + x.value);

    // Part 2
    let answer = symbols.iter()
                        .fold(0, |acc, x| acc + x.get_gear_ratio(&numbers).unwrap_or(0));



    println!("Answer:{}", answer);
}

fn read_input(file_path: &str) -> String {
    fs::read_to_string(file_path)
        .expect("Should have been able to read the file")
}

#[derive(Debug)]
struct Symbol {
    value: char,
    l: i32,
    c: i32,
}

impl Symbol {
    fn new(value: char, l: i32, c: i32) -> Self {
        Self {value, l, c}
    }

    fn get_gear_ratio(&self, numbers: &Vec<Number>) -> Option<u32> {
        if self.value != '*' {
            return None
        }

        let mut ratio = 1;
        let mut n_adj = 0;
        for n in numbers {
            if n.is_adjacent(self) {
                ratio *= n.value;
                n_adj += 1;
            }
        }

        if n_adj == 2 {
            return Some(ratio)
        }

        None
    }
}

fn is_symbol(c: char) -> bool {
    !c.is_digit(10) && c != '.'
}

fn find_symbols(s: &str, l_num: i32) -> Vec<Symbol> {
    let mut ret = Vec::new();
    let mut c_num: i32 = 0;
    for c in s.chars() { if is_symbol(c) {
            ret.push(Symbol::new(c, l_num, c_num));
        }
        c_num += 1;
    }
    ret
}

fn find_numbers(s: &str, l_num: i32) -> Vec<Number> {
    let mut ret = Vec::new();
    let mut c_num: i32 = 0;
    let mut width: i32 = 0;
    let mut value: u32 = 0;

    for c in s.chars() {
        if c.is_digit(10) {
            value = value * 10 + c.to_digit(10).unwrap();
            width += 1;
        } else if width > 0 {
            ret.push(Number::new(value, l_num, c_num - width, width));
            width = 0;
            value = 0;
        }
        c_num += 1;
    }
    if width > 0 {
        ret.push(Number::new(value, l_num, c_num - width, width));
    }
    ret
}

#[derive(Debug)]
struct Number {
    value: u32,
    l: i32,
    c: i32,
    width: i32,
}

impl Number {
    fn new(value: u32, l: i32, c: i32, width: i32) -> Self {
        Self {value, l, c, width}
    }

    fn is_adjacent(&self, s: &Symbol) -> bool {
        s.l >= self.l - 1 && s.l <= self.l + 1 &&
        s.c >= self.c - 1 && s.c <= self.c + self.width
    }

    fn is_part(&self, symbols: &Vec<Symbol>) -> bool {
        for s in symbols {
            if self.is_adjacent(s) {
                return true
            }
        }
        false
    }
}

