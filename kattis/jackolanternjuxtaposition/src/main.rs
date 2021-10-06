fn get_three_inputs() -> Vec<i32> {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer.trim().split(" ")
        .map(|x| x.parse().expect("Not an integer!"))
        .collect()
}

fn main() {
    let v = get_three_inputs();
    let mut total = 1;
    for i in &v {
        total *= i;
    }

    println!("{}", total);
}
