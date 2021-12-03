fn get_num() -> u64 {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().parse().unwrap()
}

fn get_time() -> u64 {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut v: Vec<u64> = line.trim().split(" ")
        .map(|x| x.parse().unwrap())
        .collect();
    v.drain(0..=0);
    v.iter().sum()
}

fn main() {
    for _ in 0..get_num() {
        let mut customers = Vec::new();
        for _ in 0..get_num() {
            customers.push(get_time());
        }
        customers.sort();
        for i in 1..(customers.len()) {
            customers[i] += customers[i - 1];
        }
        let ave = customers.iter().sum::<u64>() as f64 / customers.len() as f64;
        println!("{:.8}", ave);
    }
}
