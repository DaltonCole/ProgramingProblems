fn get_input() -> i32 {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer.trim().parse::<i32>().unwrap()
}

fn main() {
    let presses = get_input();
    let mut total = 0;
    if presses % 2 == 1 {
        println!("still running");
    } else {
        for _ in (0..presses).step_by(2) {
            let start = get_input();
            let end = get_input();
            total += end - start;
        }
        println!("{}", total);
    }
}
