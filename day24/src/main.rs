// Not happy with my code here
//
// Quit botched and I don't understand everything as much as I want...
// Will bo better next year
//
// I am happy with the use of `nalgebra` and the intercept shift code
// I learned a ton here

use nalgebra::{Matrix2, Vector2, Vector3};

fn main() {
    let input = std::fs::read_to_string("example1.txt").unwrap();
    // let (min, max) = (7_f64, 27_f64);

    let input = std::fs::read_to_string("input.txt").unwrap();
    // let (min, max) = (200000000000000_f64, 400000000000000_f64);
    //
    // let hails: Vec<Hail> = input.lines().map(Hail::parse).collect();
    //
    // // Part 1 ------------------------------------------------------------------
    // let mut answ = 0;
    // for i in 0..(hails.len() - 1) {
    //     for j in (i + 1)..hails.len() {
    //         if hails[i].do_intersect_p1(&hails[j], min, max) {
    //             answ += 1;
    //         }
    //     }
    // }
    // println!("P1: {answ}");

    // Part 2 ------------------------------------------------------------------
    // let input = std::fs::read_to_string("example2.txt").unwrap();
    let hails: Vec<NdHail> = input.lines().map(NdHail::parse).collect();

    let mut igns = [Vec::new(), Vec::new(), Vec::new()];
    for i in 0..(hails.len() - 1) {
        for j in (i + 1)..hails.len() {
            for k in 0..3 {
                if hails[i].x[k] > hails[j].x[k] && hails[i].v[k] > hails[j].v[k] {
                    igns[k].push([hails[j].v[k] as i64, hails[i].v[k] as i64]);
                }
            }
        }
    }

    for i in 0..3 {
        combine(&mut igns[i]);
    }
    // // combine(&mut igns[2]);
    // println!("{:?}", igns[0]);
    // println!("{:?}", igns[1]);
    // println!("{:?}", igns[2]);
    //
    // println!("skip test {}", skip(-700.0, &igns[1]));

    // my solution: -3, 1, 2
    let min = -500_f64;
    let max = -min;

    let mut x = min;
    let mut done = false;

    while !done {
        x = skip(x, &igns[0]);
        if x > max {break;}
        // println!("x: {x}");
        let mut y = min;
        while !done {
            if y > max {break;}
            y = skip(y, &igns[1]);
            let mut z = min;
            while !done {
                // println!("enter z");
                if z > max {break;}
                z = skip(z, &igns[2]);
                let rock_speed = Vector3::new(x as f64, y as f64, z as f64);
                for i in 1..hails.len() {
                    if let Some(p) = hails[0].intersect_shift(&hails[i], rock_speed) {
                        println!("{i}: intersect, v: {x}, {y}, {z}");
                        println!("p: {}", p[0] + p[0] + p[2]);
                        if i >= (hails.len() - 1) {
                            done = true;
                            println!("p: {p:?}");
                            break;
                        }
                    } else {
                        // println!("{i}: do not intersect");
                        break;
                    }
                }
                z += 1.0;
            }
             y += 1.0;
        }
        x += 1.0;
    }

    // let rock_speed = Vector3::new(-3.0, 1.0, 2.0);
    // for i in 1..hails.len() {
    //     if let Some(p) = hails[0].intersect_shift(&hails[i], rock_speed) {
    //         println!("{i}: intersect");
    //         println!("p: {p:?}");
    //     } else {
    //         println!("{i}: do not intersect");
    //         break;
    //     }
    // }


    // // test intersect
    // let a = NdHail{
    //     x: Vector3::new(-2.0, -1.0, 0.0),
    //     v: Vector3::new(1.0, 1.0, 1.0),
    // };
    // let b = NdHail{
    //     x: Vector3::new(8.0, -6.0, -11.0),
    //     v: Vector3::new(-2.0, 3.0, 5.0),
    // };
    //
    // let a = NdHail{
    //     x: Vector3::new(5.0, 1.0, 8.0),
    //     v: Vector3::new(1.0, -1.0, -3.0),
    // };
    // let b = NdHail{
    //     x: Vector3::new(1.0, -4.0, 8.0),
    //     v: Vector3::new(2.0, 1.0, -2.0),
    // };
    //
    // println!("{:?}", a.intersect(&b));
}

#[derive(Debug)]
struct NdHail {
    x: Vector3<f64>,
    v: Vector3<f64>,
}

impl NdHail {
    fn parse(s: &str) -> Self {
        let (ps, vs) = s.split_once(" @ ").unwrap();
        let x: Vec<f64> = ps.split(", ").map(|x| x.trim().parse().unwrap()).collect();
        let v: Vec<f64> = vs.split(", ").map(|x| x.trim().parse().unwrap()).collect();

        let x = Vector3::new(x[0], x[1], x[2]);
        let v = Vector3::new(v[0], v[1], v[2]);

        Self {x, v}
    }

    fn intersect(&self, other: &Self) -> Option<Vector3<f64>> {
        let a_mat = Matrix2::new(
            self.v[0], -other.v[0],
            self.v[1], -other.v[1],
        );

        let b_vec = Vector2::new(
            other.x[0] - self.x[0],
            other.x[1] - self.x[1],
        );

        if let Some(coefs) = a_mat.lu().solve(&b_vec) {
            // println!("{:?}", coefs);
            let intersect = self.x[2] + coefs[0] * self.v[2];
            if intersect == other.x[2] + coefs[1] * other.v[2] {
                return Some(self.x + coefs[0] * self.v)
            }
        }

        None
    }

    fn intersect_shift(&self, other: &Self, rock: Vector3<f64>) ->
        Option<Vector3<f64>> {

        let a1 = self.x;
        let b1 = self.v - rock;
        let a2 = other.x;
        let b2 = other.v - rock;

        let a_mat = Matrix2::new(
            b1[0], -b2[0],
            b1[1], -b2[1],
        );

        let b_vec = Vector2::new(
            a2[0] - a1[0],
            a2[1] - a1[1],
        );

        if let Some(coefs) = a_mat.lu().solve(&b_vec) {
            // println!("{:?}", coefs);
            let intersect = a1[2] + coefs[0] * b1[2];
            if intersect == a2[2] + coefs[1] * b2[2] {
                return Some(a1 + coefs[0] * b1)
            }
        }
        None
    }

    fn intersect_shift_xy(&self, other: &Self, rock: Vector3<f64>) ->
        bool {

        let a1 = self.x;
        let b1 = self.v - rock;
        let a2 = other.x;
        let b2 = other.v - rock;

        let a_mat = Matrix2::new(
            b1[0], -b2[0],
            b1[1], -b2[1],
        );

        let b_vec = Vector2::new(
            a2[0] - a1[0],
            a2[1] - a1[1],
        );

        if let Some(_) = a_mat.lu().solve(&b_vec) {
            return true
        }
        false
    }
}

#[derive(Debug)]
struct Hail {
    x: f64, y: f64, z: f64,
    vx: f64, vy: f64, vz: f64,
}

impl Hail {
    fn parse(s: &str) -> Self {
        let (ps, vs) = s.split_once(" @ ").unwrap();
        let p: Vec<f64> = ps.split(", ").map(|x| x.trim().parse().unwrap()).collect();
        let v: Vec<f64> = vs.split(", ").map(|x| x.trim().parse().unwrap()).collect();

        Self {
            x: p[0], y: p[1], z: p[2],
            vx: v[0], vy: v[1], vz: v[2],
        }
    }

    fn get_ab(&self) -> (f64, f64) {
        let slope = self.vy / self.vx;
        let intercept = self.y - self.x * slope;
        (slope, intercept)
    }


    fn calc_y(&self, x: f64) -> f64 {
        let (a, b) = self.get_ab();
        a * x + b
    }

    fn get_x_range(&self, min: f64, max: f64) -> (f64, f64) {
        let (mut xmin, mut xmax) = (0.0, 0.0);
        if self.vx < 0.0 {
            (xmin, xmax) = (min, self.x);
        } else {
            (xmin, xmax) = (self.x, max);
        }
        (xmin, xmax)
    }

    fn is_future(&self, x: f64) -> bool {
        (self.x > x && self.vx < 0.0) || (self.x < x && self.vx > 0.0)
    }

    fn do_intersect_p1(&self, other: &Self, min: f64, max: f64) -> bool {
        if let Some((x, y)) = self.intersect(other) {
            return x <= max && x >= min &&
                   y <= max && y >= min &&
                        self.is_future(x) && other.is_future(x)
        }
        false
    }

    fn intersect(&self, other: &Self) -> Option<(f64, f64)> {
        let p1 = self.get_ab();
        let p2 = other.get_ab();

        if p1.0 == p2.0 {
            return None
        }

        let x_int = (p1.1 - p2.1) / (p2.0 - p1.0);
        let y_int = x_int * p1.0 + p1.1;

        Some((x_int, y_int))
    }
}

fn reorder(r: (f64, f64)) -> (f64, f64) {
    if r.0 > r.1 {
        (r.1, r.0)
    } else {
        r
    }
}

fn skip(x: f64, igns: &Vec<[i64; 2]>) -> f64 {
    for i in 0..igns.len() {
        if x > igns[i][0] as f64 && x < igns[i][1] as f64{
            return igns[i][1] as f64 + 1.0;
        }
    }
    x
}

fn combine(r_in: &mut Vec<[i64; 2]>) {
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
    *r_in = out;
}
