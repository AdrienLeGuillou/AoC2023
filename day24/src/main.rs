fn main() {
    // let input = std::fs::read_to_string("example1.txt").unwrap();
    // let (min, max) = (7_f64, 27_f64);

    let input = std::fs::read_to_string("input.txt").unwrap();
    let (min, max) = (200000000000000_f64, 400000000000000_f64);

    let hails: Vec<Hail> = input.lines().map(Hail::parse).collect();

    let mut answ = 0;
    for i in 0..(hails.len() - 1) {
        for j in (i + 1)..hails.len() {
            if hails[i].do_intersect_p1(&hails[j], min, max) {
                answ += 1;
            }
        }
    }

    println!("P1: {answ}");
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

    fn get_3d_coefs(&self) -> (f64, f64, f64, f64) {
        let y_slope = self.vy / self.vx;
        let y_intercept = self.y - self.x * y_slope;
        let z_slope = self.vz / self.vx;
        let z_intercept = self.z - self.x * z_slope;

        (y_slope, y_intercept, z_slope, z_intercept)
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
