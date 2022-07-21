use std::env;
use std::fs;
use std::io;

// Better way, instead of doing them 1 by 1, since there are only 9 possible numbers, keep track of
// the 9 possible numbers

const FISH_TIME: usize = 6;
const BABY_TIME: usize = 8;
const NUM_DAYS: usize = 256;

fn read_from_file(f_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(f_name)
}

fn get_fish(file: &str) -> Vec<usize> {
    file.trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn bucket_fish(fish: &Vec<usize>) -> Vec<usize> {
    let mut fish_buckets = vec![0; BABY_TIME + 1];

    for f in fish {
        fish_buckets[*f] += 1;
    }

    fish_buckets
}

fn simulate_day(fish: &mut Vec<usize>) {
    let new_babies = fish[0];
    fish.rotate_left(1);

    fish[FISH_TIME] += new_babies;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let contents = read_from_file(file_name).unwrap();
    let mut fish = get_fish(&contents);
    fish = bucket_fish(&fish);

    for _ in 0..NUM_DAYS {
        simulate_day(&mut fish);
    }
    
    println!("{:?}", fish);
    println!("{}", fish.iter().sum::<usize>());
}
