fn get_input() -> i32 {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer.trim().parse::<i32>().unwrap()
}

fn main() {
    let num = get_input();
    let mut total = 0;
    for _ in 0..num {
        total += get_input();
    }
    total -= num - 1;
    println!("{}", total);
}
