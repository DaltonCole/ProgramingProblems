fn get_count() -> u32 {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().parse().unwrap()
}

fn get_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn main() {
    let num = get_count();
    let mut v: Vec<String> = Vec::new();

    for _ in 0..num {
        v.push(get_line());
    }

    for line in v.iter().step_by(2) {
        println!("{}", line);
    }
}
