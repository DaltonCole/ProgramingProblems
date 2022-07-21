use std::env;
use std::fs;
use std::io;

const PART: u32 = 2;

fn read_from_file(f_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(f_name)
}

fn get_positions(contents: &str) -> Vec<i32> {
    contents.trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn ideal_position(ships: &Vec<i32>) -> i32 {
    let min = *ships.iter().min().unwrap();
    let max = *ships.iter().max().unwrap();
    let mut ideal = min;
    let mut dist = i32::MAX;

    for i in min..=max {
        let new_dist = get_distance(&ships, i);
        //println!("{}: {}", i, new_dist);

        if new_dist <= dist {
            ideal = i;
            dist = new_dist;
        } else {
            break;
        }
    }

    ideal
}

fn get_distance(ships: &Vec<i32>, dist: i32) -> i32 {
    match PART {
        1 => {
            ships.iter()
                .map(|x| (x - dist).abs())
                .sum()
                
        }
        _ => {
            ships.iter()
                .map(|x| (x - dist).abs())
                .map(|x| x * (x + 1) / 2)
                .sum()
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let contents = read_from_file(file_name).unwrap();
    let mut ships = get_positions(&contents);
    //ships.sort();
    let pos = ideal_position(&ships);
    let dist = get_distance(&ships, pos);

    println!("{:?}", ships);
    println!("{}", pos);
    println!("{}", dist);
}
