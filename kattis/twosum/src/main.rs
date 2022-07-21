fn get_input_and_sum() -> u32 {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().split(" ")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u32>>()
        .iter()
        .sum()
}

fn main() {
    println!("{}", get_input_and_sum());
}
