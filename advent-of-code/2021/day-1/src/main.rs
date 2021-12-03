use std::env;
use std::fs;
use std::io;

fn read_from_file(f_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(f_name)
}

fn convert_contents(contents: &str) -> Vec<u32> {
    contents.split("\n")
        .filter(|x| *x != "")
        .map(|x| x.parse().unwrap())
        .collect()
}

fn count_increase(v: &Vec<u32>) -> u32 {
    let mut count = 0;
    for i in 1..v.len() {
        if v[i] > v[i-1] {
            count += 1;
        }
    }
    count
}

fn count_window_increases(v: &Vec<u32>) -> u32 {
    let mut count = 0;
    for i in 3..v.len() {
        let first = v[i-3] + v[i-2] + v[i-1];
        let second = v[i-2] + v[i-1] + v[i];
        if second > first {
            count += 1;
        }
    }
    count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let contents = read_from_file(file_name).unwrap();
    let numbers = convert_contents(&contents);
    let increases = count_window_increases(&numbers);
    println!("{}", increases);
}
