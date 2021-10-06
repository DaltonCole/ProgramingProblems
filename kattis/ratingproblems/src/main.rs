fn get_input() -> f32 {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer.trim().parse::<f32>().unwrap()
}

fn get_two_inputs() -> (f32, f32) {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    let vec: Vec<f32> =  buffer.trim().split(" ")
        .map(|x| x.parse().expect("Not an integer!"))
        .collect();
    (vec[0], vec[1])
}

fn main() {
    let (count, num) = get_two_inputs();
    let mut total = 0.0;
    for _ in 0..num as i32 {
        total += get_input();
    }

    println!("{} {} ", (total - ((count - num) * 3.0)) / count, (total + ((count - num) * 3.0)) / count);

}
