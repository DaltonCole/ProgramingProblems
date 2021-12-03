fn get_num() -> u16 {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().parse().unwrap()
}

fn num_printers(num: u16) -> u16 {
    let mut printers = 1;
    let mut count = 1;
    while num > printers {
        printers *= 2;
        count += 1;
    }
    count 
}

fn main() {
    let num = get_num();
    println!("{}", num_printers(num));
}
