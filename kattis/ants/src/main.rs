use std::cmp::{min, max};

fn get_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn get_vec_line() -> Vec<u32> {
    get_line().split(" ")
        .map(|x| x.parse().unwrap())
        .collect()
}

fn get_cases() -> u32 {
    get_line().parse().unwrap()
}

fn get_ants() -> (u32, Vec<u32>) {
    let tmp = get_vec_line();
    let (dist, num_ants) = (tmp[0], tmp[1] as usize);

    let mut ants: Vec<u32> = Vec::new();
    while ants.len() < num_ants {
        let tmp = get_vec_line();
        for x in tmp {
            ants.push(x);
        }
    }

    (dist, ants)
}

fn min_dist(dist: u32, ants: &Vec<u32>) -> u32 {
    let mut min_dist = 0;
    let middle = dist as f32 / 2.0;
    for ant in ants {
        if (*ant as f32) < middle {
            min_dist = max(min_dist, *ant);
        } else {
            min_dist = max(min_dist, dist - ant);
        }
    }
    min_dist
}

fn max_dist(dist: u32, ants: &Vec<u32>) -> u32 {
    let mut min_dist = dist;
    let middle = dist as f32 / 2.0;
    for ant in ants {
        if (*ant as f32) < middle {
            min_dist = min(min_dist, *ant);
        } else {
            min_dist = min(min_dist, dist - ant);
        }
    }
    dist - min_dist
}

fn main() {
    for _ in 0..get_cases() {
        let (dist, ants) = get_ants();

        println!("{} {}", 
            min_dist(dist, &ants),
            max_dist(dist, &ants),
        )
    }
}
