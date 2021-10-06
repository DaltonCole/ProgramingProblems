fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer = buffer.trim().to_string();

    println!("{} {} {}", buffer, buffer, buffer);
}
