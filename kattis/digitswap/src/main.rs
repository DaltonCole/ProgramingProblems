fn get_and_swap() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().chars().rev().collect::<String>()
}

fn main() {
    println!("{}", get_and_swap());
}
