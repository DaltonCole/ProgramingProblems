use std::cmp::min;

fn get_num() -> u32 {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().parse().unwrap()
}

fn get_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn get_diff(a: &str, b: &str) -> (u32, u32) {
    let mut same = 0;
    let mut diff = 0;
    for (x, y) in a.chars().zip(b.chars()) {
        match x == y {
            true => {same += 1; },
            false => {diff += 1; },
        }
    }
    (same, diff)
}

fn find_answer(same: u32, diff: u32, correct: u32) -> u32 {
    min(same, correct) + min(diff, same + diff - correct)
}

fn main() {
    let correct = get_num();
    let mine = get_line();
    let their = get_line();
    let (same, diff) = get_diff(&mine, &their);

    println!("{}", find_answer(same, diff, correct));
}
