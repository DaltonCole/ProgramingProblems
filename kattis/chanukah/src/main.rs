fn get_num() -> u32 {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().parse().unwrap()
}

fn get_second_num() -> u32 {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let v: Vec<u32> = buffer.trim().split(" ")
        .map(|x| x.parse().unwrap())
        .collect();
    v[1]
}

fn sum() -> u32 {
    let num = get_second_num();
    let mut total: u32 = 0;
    for i in 0..num {
        total += i + 2;
    }
    total
}

fn main() {
    let num = get_num();
    for i in 0..num {
        println!("{} {}", i+1, sum());
    }
}
