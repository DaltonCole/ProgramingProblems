fn get_num() -> u32 {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().parse().unwrap()
}


fn get_row() -> Vec<u32> {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().split(" ")
        .map(|x| x.parse().unwrap())
        .collect()
}

fn main() {
    let num = get_num();
    for _ in 0..num {
        let v = get_row();
        let total: u32 = v.iter().sum();
        println!("{}", total - v[0] - v[0] + 1);
    }
}
