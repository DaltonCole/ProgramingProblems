fn num555() -> bool {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.starts_with("555")
}

fn main() {
    match num555() {
        true => println!("1"),
        false => println!("0"),
    }
}
