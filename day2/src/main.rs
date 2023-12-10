use std::fs;

fn main() {
    let input = read_input("input.txt");
    let mut answer = 0;


    let games: Vec<Game> = input.lines()
                     .map(parse_game)
                     .collect();

    // Part 1
    let valid_sets: [u32; 3] = [12, 13, 14];
    for g in games {
        if is_valid_game(&g, valid_sets) {
            answer += g.id;
        }
    }

    // // Part 2
    // for g in games {
    //     answer += power_cube(&g);
    // }

    println!("Answer:{}", answer);
}

fn read_input(file_path: &str) -> String {
    fs::read_to_string(file_path)
        .expect("Should have been able to read the file")
}

fn parse_game(s: &str) -> Game {
    let (game_sub, sets) = s.split_once(": ").unwrap();
    let id: u32 = game_sub.replace("Game ", "").parse().unwrap();
    let mut game = Game::new(id);

    for set_sub in sets.split("; ") {
        game.sets.push(parse_set(set_sub));
    }

    game
}

fn parse_set(s: &str) -> [u32; 3] {
    let mut set = [0, 0, 0];
    for c in s.split(", ") {
        let g: Vec<&str> = c.split(" ").collect();
        match g[1] {
            "red" => set[0] = g[0].parse().unwrap(),
            "green" => set[1] = g[0].parse().unwrap(),
            "blue" => set[2] = g[0].parse().unwrap(),
            _ => {}
        }
    }

    set
}

fn is_valid_game(game: &Game, stock: [u32; 3]) -> bool {
    for set in &game.sets {
        if !is_valid_set(*set, stock) {
            return false
        }
    }
    true
}

fn is_valid_set(set: [u32; 3], stock: [u32; 3]) -> bool {
    for i in 0..stock.len() {
        if set[i] > stock[i] {
            return false
        }
    }
    true
}

fn min_cubes(game: &Game) -> [u32; 3] {
    let mut amount = [0, 0, 0];
    for set in &game.sets {
        for i in 0..amount.len() {
            amount[i] = amount[i].max(set[i]);
        }
    }
    amount
}

fn power_cube(game: &Game) -> u32 {
    min_cubes(game).into_iter()
                   .fold(1, |acc, x| acc * x.max(1))
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<[u32; 3]>
}

impl Game {
    fn new(id: u32) -> Self {
        Game {
            id,
            sets: Vec::<[u32; 3]>::new(),
        }
    }
}
