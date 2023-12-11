use std::fs;

fn main() {
    let input = read_input("input.txt");
    // let input = read_input("example.txt");
    let times: Vec<i64> = input.lines().next().unwrap()
                               .split_once(":").unwrap().1
                               .split_ascii_whitespace()
                               .map(|x| x.parse::<i64>().unwrap())
                               .collect();

    let dists: Vec<i64> = input.lines().nth(1).unwrap()
                               .split_once(":").unwrap().1
                               .split_ascii_whitespace()
                               .map(|x| x.parse::<i64>().unwrap())
                               .collect();
    let mut answer_p1 = 1;


    for i in 0..times.len() {
        let sols = n_solutions(times[i], dists[i]);
        answer_p1 *= sols;
        // println!("t: {}, dt: {}, sols: {sols}", times[i], dists[i]);
    }

    println!("part1: {}", answer_p1);

    // part2
    let time = input.lines().next().unwrap().replace(" ", "").split_once(":")
                    .unwrap().1.parse::<i64>().unwrap();
    let dist = input.lines().nth(1).unwrap().replace(" ", "").split_once(":")
                    .unwrap().1.parse::<i64>().unwrap();

    // println!("t: {time}, dt: {dist}, sol: {}", n_solutions(time, dist));
    println!("part2: {}", n_solutions(time, dist));
}

fn read_input(file_path: &str) -> String {
    fs::read_to_string(file_path)
        .expect("Should have been able to read the file")
}

fn n_solutions(t: i64, dt: i64) -> i64 {
    let lb = lower_bound_float(t, dt);
    t - 2 * lb + 1
}

fn lower_bound(t: i64, dt: i64) -> i64 {
    match t % 2 {
        0 => lb_even(t, dt),
        _ => lb_odd(t, dt),
    }
}

// no need to check if odd of even
// do all calc in f64 and round up the lower_bound at the end
// just use target = old record + 1
// no need for quadratic formula
fn lower_bound_float(t: i64, dt: i64) -> i64 {
    let t2 = t as f64 / 2.0;
    let dmax = t2 * t2;
    let target = dt as f64 + 1.0;
    let rem = dmax - target;
    let n = rem.sqrt();
    (t2 - n).ceil() as i64
}

fn lb_even(t: i64, dt: i64) -> i64 {
    let dmax = (t / 2).pow(2);
    let rem = (dmax - dt - 1) as f64;
    let n = rem.sqrt().floor() as i64;
    (t / 2) - n
}

// quadratic formula: delta = b^2 - 4ac
// x = (-b +- sqrt(delta)) / 2a
fn lb_odd(t: i64, dt: i64) -> i64 {
    let dmax = (t / 2) * (t / 2 + 1);
    let rem = (dmax - dt - 1) as f64;
    let delta = 1.0 + 4.0 * rem;
    let root = (1.0 - delta.sqrt()) / -2.0;
    let n = root.floor() as i64;
    (t / 2) - n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        assert_eq!(4, n_solutions(7, 9));
        assert_eq!(8, lower_bound(15, 40));
        assert_eq!(9, lower_bound(30, 200));
    }
}
