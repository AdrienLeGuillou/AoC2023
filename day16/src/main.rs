fn main() {
    // let input = std::fs::read_to_string("example1.txt").unwrap();
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut tmap_inner: Vec<Tile> = Vec::new();
    let mut nrows = 0;
    for l in input.lines() {
        let mut ts: Vec<Tile> = l.chars().map(Tile::parse).collect();
        tmap_inner.append(&mut ts);
        nrows += 1
    }

    let mut tmap = Map::new(tmap_inner.clone(), nrows);
    // print_map(&tmap);

    // Part1 -------------------------------------------------------------------
    println!("P1: {}", n_energized(Ray::new(0, -1, Dir::East), &mut tmap));

    // Part2 -------------------------------------------------------------------
    let mut best_answ = 0;

    for c in 0..tmap.n_cols {
        let mut tmap = Map::new(tmap_inner.clone(), nrows);
        best_answ = best_answ.max(n_energized(
                Ray::new(tmap.n_rows as i32, c as i32, Dir::North),
                &mut tmap)
            );
        let mut tmap = Map::new(tmap_inner.clone(), nrows);
        best_answ = best_answ.max(n_energized(
                Ray::new(-1, c as i32, Dir::South),
                &mut tmap)
            );
    }
    for r in 0..tmap.n_rows {
        let mut tmap = Map::new(tmap_inner.clone(), nrows);
        best_answ = best_answ.max(n_energized(
                Ray::new(r as i32, -1, Dir::East),
                &mut tmap)
            );
        let mut tmap = Map::new(tmap_inner.clone(), nrows);
        best_answ = best_answ.max(n_energized(
                Ray::new(r as i32, tmap.n_cols as i32, Dir::West),
                &mut tmap)
            );
    }

    println!("P2: {}", best_answ);
}

#[derive(Debug, Clone, Copy)]
struct Ray {
    row: i32,
    col: i32,
    dir: Dir,
}

impl Ray {
    fn new(row: i32, col: i32, dir: Dir) -> Self {
        Self {row, col, dir}
    }

    fn advance(&mut self) {
        match self.dir {
            Dir::North => self.row -= 1,
            Dir::East => self.col += 1,
            Dir::South => self.row += 1,
            Dir::West => self.col -= 1,
        }
    }

    fn update(&self, m: &mut Map<Tile>) -> (Option<Ray>, Option<Ray>) {
        if let Some(t) = m.get_mut(self.row as usize, self.col as usize) {
            if t.visited_from(self.dir) {
                return (None, None)
            } else {
                t.ray_visit(self);
            }

            let (d1, d2) = t.change_dir(self.dir);
            let r1 = Self::new(self.row, self.col, d1);
            if let Some(d) = d2 {
                let r2 = Self::new(self.row, self.col, d);
                (Some(r1), Some(r2))
            } else {
                (Some(r1), None)
            }
        } else {
            (None, None)
        }
    }
}


#[derive(Debug, Clone, Copy)]
enum TileType {
    Empty,
    FMirror,
    BMirror,
    HSplitter,
    VSplitter,
}

#[derive(Debug, Clone, Copy)]
struct Tile {
    t_type: TileType,
    visits: [bool; 4],
    visited: bool,
}

impl Tile {
    fn parse(c: char) -> Self {
        let visits = [false; 4];
        let t_type = match c {
            '-' => TileType::HSplitter,
            '|' => TileType::VSplitter,
            '/' => TileType::FMirror,
            '\\' => TileType::BMirror,
            _   => TileType::Empty,
        };
        Self {t_type, visits, visited: false}
    }

    fn ray_visit(&mut self, ray: &Ray) {
        self.add_visit(ray.dir);
    }

    fn add_visit(&mut self, dir: Dir) {
        match dir {
            Dir::North => self.visits[0] = true,
            Dir::East => self.visits[1] = true,
            Dir::South => self.visits[2] = true,
            Dir::West => self.visits[3] = true,
        }
        self.visited = true;
    }

    fn visited_from(&self, dir: Dir) -> bool {
        match dir {
            Dir::North => self.visits[0],
            Dir::East => self.visits[1],
            Dir::South => self.visits[2],
            Dir::West => self.visits[3],
        }
    }

    fn print(&self) {
        match self.t_type {
            TileType::HSplitter => print!("-"),
            TileType::VSplitter => print!("|"),
            TileType::FMirror => print!("/"),
            TileType::BMirror => print!("\\"),
            TileType::Empty => print!("."),
        }
    }

    fn change_dir(&self, dir: Dir) -> (Dir, Option<Dir>) {
        match self.t_type {
            TileType::HSplitter => self.change_dir_hs(dir),
            TileType::VSplitter => self.change_dir_vs(dir),
            TileType::FMirror => self.change_dir_fm(dir),
            TileType::BMirror => self.change_dir_bm(dir),
            TileType::Empty => (dir, None),
        }
    }

    fn change_dir_hs(&self, dir: Dir) -> (Dir, Option<Dir>) {
        if dir == Dir::West || dir == Dir::East {
            (dir, None)
        } else {
            (Dir::West, Some(Dir::East))
        }
    }

    fn change_dir_vs(&self, dir: Dir) -> (Dir, Option<Dir>) {
        if dir == Dir::West || dir == Dir::East {
            (Dir::North, Some(Dir::South))
        } else {
            (dir, None)
        }
    }

    fn change_dir_fm(&self, dir: Dir) -> (Dir, Option<Dir>) {
        match dir {
            Dir::North => (Dir::East, None),
            Dir::East => (Dir::North, None),
            Dir::South => (Dir::West, None),
            Dir::West => (Dir::South, None),
        }
    }

    fn change_dir_bm(&self, dir: Dir) -> (Dir, Option<Dir>) {
        match dir {
            Dir::North => (Dir::West, None),
            Dir::East => (Dir::South, None),
            Dir::South => (Dir::East, None),
            Dir::West => (Dir::North, None),
        }
    }
}

// direction of the ray: East means from W->East
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
struct Map<T> {
    inner: Vec<T>,
    n_cols: usize,
    n_rows: usize,
}

impl <T> Map<T> {
    fn new(inner: Vec<T>, n_rows: usize) -> Self {
        let n_cols = inner.len() / n_rows;
        Self {
            inner,
            n_cols,
            n_rows,
        }
    }

    fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row >= self.n_rows || col >= self.n_cols {
            None
        } else {
            Some(&self.inner[self.n_cols * row + col])
        }
    }

    fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        if row >= self.n_rows || col >= self.n_cols {
            None
        } else {
            Some(&mut self.inner[self.n_cols * row + col])
        }
    }
}

fn print_map(m: &Map<Tile>) {
    for r in 0..m.n_rows {
        for c in 0..m.n_cols {
            m.get(r, c).unwrap().print();
        }
        print!("\n");
    }
    print!("\n");
}

fn n_energized(start_ray: Ray, tmap: &mut Map<Tile>) -> u64 {
    let mut rays = vec![start_ray];

    loop {
        let mut uptd_rays = Vec::new();
        for r in rays.iter_mut() {
            r.advance();
            let (r1, r2) = r.update(tmap);
            if let Some(r) = r1 {
                uptd_rays.push(r);
            }
            if let Some(r) = r2 {
                uptd_rays.push(r);
            }
        }

        if uptd_rays.len() == 0 {
            break;
        }

        rays = uptd_rays.clone();
    }

    tmap.inner.iter().fold(0, |a, x| a + x.visited as u64)
}
