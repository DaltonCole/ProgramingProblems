fn get_and_sort_two_numbers() -> (u32, u32) {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mut v: Vec<u32> = buffer.trim().split(" ")
        .map(|x| x.parse().unwrap())
        .collect();
    v.sort();
    (v[0], v[1])
}

fn main() {
    let (a, b) = get_and_sort_two_numbers();
    println!("{} {}", a, b);
}
