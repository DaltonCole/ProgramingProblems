fn get_input() -> Vec<u32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().split(" ")
        .map(|x| x.parse().unwrap())
        .collect()
}

fn get_first_line() -> (u32, u32) {
    let v = get_input();
    (v[0], v[1])
}

fn get_stops() -> (u32, u32, u32) {
    let v = get_input();
    (v[0], v[1], v[2])
}

fn main() {
    let (stops, total_dist) = get_first_line();
    let mut cur_time = 0;
    let mut prev_dist = 0;
    for _ in 0..stops {
        let (dist, red, green) = get_stops();
        cur_time += dist - prev_dist;
        let extra = cur_time % (red + green);
        if extra < red {
            cur_time += red - extra;
        }
        prev_dist = dist;
    }
    cur_time += total_dist - prev_dist;

    println!("{}", cur_time);
}
