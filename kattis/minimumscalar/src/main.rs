fn get_num() -> i64 {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().parse().unwrap()
}

fn get_vec() -> Vec<i64> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().split(" ")
        .map(|x| x.parse().unwrap())
        .collect()
}

fn scalar_product(x: &Vec<i64>, y: &Vec<i64>) -> i64 {
    let mut total = 0;
    for (a, b) in x.iter().zip(y.iter()) {
        total += a * b;
    }
    total
}

fn main() {
    let cases = get_num();
    for i in 1..=cases {
        get_num();
        let mut x = get_vec();
        let mut y = get_vec();
        x.sort();
        y.sort();
        y.reverse();
        println!("Case #{}: {}", i, scalar_product(&x, &y));
    }
}
