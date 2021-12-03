use std::env;
use std::fs;
use std::io;

fn read_from_file(f_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(f_name)
}

fn convert_contents(contents: &str) -> Vec<Vec<char>> {
    let lines: Vec<String> = contents.split("\n")
        .filter(|x| *x != "")
        .map(|x| x.to_string())
        .collect();
    let mut v = Vec::<Vec<char>>::new();
    for line in lines {
        v.push(line.chars().collect());
    }
    v
}

fn most_least_popular_part_1(nums: &Vec<Vec<char>>) -> (u32, u32) {
    let mut count_ones = vec![0; nums[0].len()];
    for num in nums {
        for i in 0..num.len() {
            if num[i] == '1' {
                count_ones[i] += 1;
            }
        }
    }
    let mut most = 0;
    let mut least = 0;
    let threshold = nums.len() / 2;

    for i in 0..count_ones.len() {
        // Bit shift by 1
        most <<= 1;
        least <<= 1;
        // 1 is the majority
        if count_ones[i] > threshold {
            most += 1;
        } else { // 1 is the minority
            least += 1;
        }
    }
    (most, least)
} 

fn find_most_popular_bit(nums: &Vec<&Vec<char>>, index: usize) -> char {
    let mut ones = 0;
    let mut zeros = 0;
    for num in nums {
        if num[index] == '1' {
            ones += 1;
        } else {
            zeros += 1;
        }
    }
    match ones >= zeros {
        true => '1',
        false => '0',
    }
}

fn find_least_popular_bit(nums: &Vec<&Vec<char>>, index: usize) -> char {
    match find_most_popular_bit(nums, index) {
        '1' => '0',
        _ => '1'
    }
}
fn make_num(v: &Vec<char>) -> u32 {
    let mut num = 0;
    for n in v {
        num <<= 1;
        if *n == '1' {
            num += 1;
        }
    }
    num
}

fn part2(nums: &Vec<Vec<char>>) -> (u32, u32) {
    let mut most = Vec::<&Vec<char>>::new();
    let mut least = Vec::<&Vec<char>>::new();
    for num in nums {
        most.push(num);
        least.push(num);
    }

    for i in 0..nums[0].len() {
        // Most
        let most_popular_bit = find_most_popular_bit(&most, i);
        most.retain(|x| x[i] == most_popular_bit);
        // Least
        if least.len() > 1 {
            let least_popular_bit = find_least_popular_bit(&least, i);
            least.retain(|x| x[i] == least_popular_bit);
        }
    }

    (make_num(most[0]), make_num(least[0]))
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let contents = read_from_file(file_name).unwrap();
    let nums = convert_contents(&contents);
    let (most, least) = part2(&nums);
    println!("{} {} ", most, least);
    println!("{}", most * least);
}
