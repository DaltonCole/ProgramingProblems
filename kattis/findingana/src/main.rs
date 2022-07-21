fn get_input() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn main() {
    let line = get_input();
    let mut found_a = false;
    for c in line.chars() {
        if c == 'a' {
            found_a = true;
        }
        if found_a {
            print!("{}", c);
        }
    }
    println!("");
}
