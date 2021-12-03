fn get_line() -> Vec<u32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut v: Vec<u32> = line.trim().split(" ")
        .map(|x| x.parse().unwrap())
        .collect();
    v.sort();
    v
}

fn main() {
    get_line();
    let tasks = get_line();
    let times = get_line();

    let mut count = 0;
    for time in times {
        if tasks[count] <= time {
            count += 1;
        }
    }

    println!("{}", count);
}
