
fn get_input() -> (u32, u32) {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let v: Vec<u32> = line.trim().split(" ")
        .map(|x| x.parse().unwrap())
        .collect();
    (v[0], v[1])
}


fn main() {
    let (a, b) = get_input();
    if a > b {
        println!("1");
    } else {
        println!("0");

    }
}
