fn get_input() -> (u32, u32) {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let v: Vec<u32> = line.trim().split(" ")
        .map(|x| x.parse().unwrap())
        .collect();
    (v[0], v[1])
}

fn main() {
    let (b, h) = get_input();
    println!("{}", (b * h) as f32 / 2.0)
}
