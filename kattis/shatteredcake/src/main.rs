fn get_area() -> u32 {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let v: Vec<u32> = buffer.trim().split(" ")
        .map(|x| x.parse().unwrap())
        .collect();
    v[0] * v[1]
}

fn get_num() -> u32 {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().parse().unwrap()
}

fn main() {
    let width = get_num();
    let n = get_num();
    let mut total: u32 = 0;
    for _ in 0..n {
        total += get_area();
    }
    println!("{}", total / width);
}
