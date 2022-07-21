use std::env;
use std::fs;
use std::io;

const PART: u32 = 2;

fn read_from_file(f_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(f_name)
}

fn get_output(contents: &str) -> (Vec<Vec<String>>, Vec<Vec<String>>) {
    let mut signals: Vec<Vec<String>> = Vec::new();
    let mut output: Vec<Vec<String>> = Vec::new();

    for line in contents.lines() {
        let split: Vec<String> = line.trim().split('|').map(str::to_string).collect();
        signals.push(split[0].split_whitespace().map(str::to_string).collect());
        output.push(split[1].split_whitespace().map(str::to_string).collect());
    }


    for i in 0..signals.len() {
        for j in 0..signals[i].len() {
            let mut chars: Vec<char> = signals[i][j].chars().collect();
            chars.sort_by(|a, b| a.cmp(b));
            signals[i][j] = chars.into_iter().collect::<String>()
        }
    }
    for i in 0..output.len() {
        for j in 0..output[i].len() {
            let mut chars: Vec<char> = output[i][j].chars().collect();
            chars.sort_by(|a, b| a.cmp(b));
            output[i][j] = chars.into_iter().collect::<String>()
        }
    }

    (signals, output)
}

fn count_char_matches(a: &String, b: &String) -> u32 {
    let mut count = 0;
    for c in a.chars() {
        for d in b.chars() {
            if c == d {
                count += 1;
            }
        }
    }
    count
}


fn decode(signal: &Vec<String>, out: &Vec<String>) -> u32 {
    // Found: 1, 2, 3, 4, 5, 6, 7, 8

    // 1, 4, 7, 8
    let one = signal.iter().find(|x| x.len() == 2).unwrap();
    let four = signal.iter().find(|x| x.len() == 4).unwrap();
    let seven = signal.iter().find(|x| x.len() == 3).unwrap();
    let eight = signal.iter().find(|x| x.len() == 7).unwrap();


    // 6 is missing 1 from 1, but has 6 segments
    let six = signal.iter().find(|x| x.len() == 6 && count_char_matches(&x, &one) == 1).unwrap();

    // 3 contains 1 and has 5 segments
    let three = signal.iter().find(|x| x.len() == 5 && count_char_matches(&x, &one) == 2).unwrap();

    // 5 is missing 1 from 6 and has 5 segments
    let five = signal.iter().find(|x| x.len() == 5 && count_char_matches(&x, &six) == 5).unwrap();

    // 2 has 3 of 5's segments and has 5 segments
    let two = signal.iter().find(|x| x.len() == 5 && count_char_matches(&x, &five) == 3).unwrap();

    // 9 has 5 of 5's segments and has 6 segments
    let nine = signal.iter().find(|x| x.len() == 6 && count_char_matches(&x, &five) == 5
        && count_char_matches(&x, &four) == 4).unwrap();

    // 0 has 6 segments 
    let zero = signal.iter().find(|x| x.len() == 6 && count_char_matches(&x, &two) == 4
        && count_char_matches(&x, &three) == 4 && count_char_matches(&x, &six) == 5 ).unwrap();

    let values = vec![zero, one, two, three, four, five, six, seven, eight, nine];
    //println!("{:?}", out);
    //println!("{:?}", values);

    let thou = values.iter().position(|&x| *x == out[0]).unwrap();
    let hund = values.iter().position(|&x| *x == out[1]).unwrap();
    let tens = values.iter().position(|&x| *x == out[2]).unwrap();
    let ones = values.iter().position(|&x| *x == out[3]).unwrap();

    let answer = (thou * 1000) + (hund * 100) + (tens * 10) + ones;

    //println!("{}", answer);

    answer as u32
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let contents = read_from_file(file_name).unwrap();
    let (signals, output) = get_output(&contents);

    let mut sum = 0;

    if PART == 1 {
        for nums in output {
            for num in nums {
                match num.len() {
                    2 | 4 | 3 | 7 => { sum += 1; },
                    _ => {}
                }
            }
        }
    } else {
        sum = signals.iter().zip(output.iter())
            .map(|it| decode(&it.0, &it.1))
            .sum();
    }

    //println!("{:#?}", signals);
    //println!("{:#?}", output);
    println!("{}", sum);
}
