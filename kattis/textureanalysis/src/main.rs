use std::collections::HashSet;

fn get_input() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn even(s: &str) -> bool {
    let x: Vec<char> = s.chars().collect();
    let r: Vec<char> = s.chars().rev().collect();

    for i in 0..x.len() {
        if x[i] != r[i] {
            return false;
        }
    }
    true
}

fn even2(s: &str) -> bool {
    //println!("{}", s);
    let mut split: Vec<&str> = s.trim().split("*").collect();
    split.pop();
    split.drain(0..=0);
    let split: HashSet<usize> = split.iter()
        .map(|x| x.matches(".").count())
        .collect();
    //println!("{:?}", split);
    match split.len() {
        0 | 1 => true,
        _ => false,
    }
}

fn main() {
    let mut count = 1;
    loop {
        let line = get_input();
        if line == "END" {
            break;
        }

        match even2(&line) {
            true => {
                println!("{} EVEN", count);
            },
            false => {
                println!("{} NOT EVEN", count);
            }
        }
        count += 1;
    }
}
