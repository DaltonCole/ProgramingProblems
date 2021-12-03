use std::str::FromStr;

struct Game {
    players: i32,
    lives: i32,
    heads_prob: f32,
}

impl FromStr for Game {
    type Err = std::string::ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = line.trim().split(" ").collect();
        let players = v[0].parse().unwrap();
        let lives = v[1].parse().unwrap();
        let heads_prob = v[2].parse().unwrap();

        Ok(Game { players, lives, heads_prob })
    }
}

fn get_game() -> Game {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().parse().unwrap()
}

fn count_combinations(n: u64, r: u64) -> u64 {
    if r > n {
        0
    } else {
        (1..=r).fold(1, |acc, val| acc * (n - val + 1) / val)
    }
}


fn draw_probability(game: &Game) -> f32 {
    let mut before: f32 = 0.0;
    let mut current;
    let mut total = 0.0;

    for i in (game.lives - 1)..10 {
        let com = count_combinations((i - 1) as u64, (game.lives - 1) as u64) as f32;
        current = com * game.heads_prob.powi(i - game.lives) * (1.0 - game.heads_prob).powi(game.lives);
        total += current + before.powi(game.players - 1);
        before = current;
        println!("{}", total);
    }

    1.0 - (game.players as f32 * total)


    /*
    let mut prob;

    let prob_head = game.heads_prob;
    let prob_tail = 1.0 - prob_head;
    
    // First
    let mut current_prob = prob_tail.powi(game.lives);
    prob = current_prob;

    // Lots more
    for i in (game.lives + 1)..10 {
        let n = game.lives;
        let total_trials = i - game.lives;
        let com = count_combinations(n as u64, total_trials as u64) as f32;
        current_prob = com * prob_tail.powi(n) * prob_head.powi(total_trials) * prob_tail;
        prob += current_prob;
        println!("{}", current_prob);
    }

    prob.powi(game.players)
    */
}


fn main() {
    println!("{:.6}", draw_probability(&get_game()));
}
