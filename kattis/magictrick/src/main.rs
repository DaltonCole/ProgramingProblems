fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("Failed");

    let mut v = Vec::new();

    for c in line.chars() {
        if v.contains(&c) {
            println!("0");
            return;
        } else {
            v.push(c);
        }
    }
    println!("1");
}
