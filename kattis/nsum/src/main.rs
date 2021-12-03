fn get_nothing() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
}

fn get_numbers_and_sum() -> u32 {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let v: Vec<u32> = line.trim().split(" ")
        .map(|x| x.parse().unwrap())
        .collect();
    v.iter().sum()
}

fn main() {
    get_nothing();
    println!("{}", get_numbers_and_sum());
}
