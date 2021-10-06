fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let v: Vec<u32> = buffer.trim().split(" ")
        .map(|x| x.parse().unwrap())
        .collect();
    println!("{}", v[0] + v[1]);
}
