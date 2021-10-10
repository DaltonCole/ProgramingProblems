fn read_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer
}

fn split_input(s: &String) -> Vec<(u32, u32)> {
    let v1: Vec<&str> = s.trim().split(";").collect();
    let v2: Vec<(u32, u32)> = v1.iter()
        .map(|x| {
            let y: Vec<u32> = x.split("-")
                .map(|z| z.parse().unwrap())
                .collect();
            if y.len() == 2 {
                (y[0], y[1])
            } else {
                (y[0], y[0])
            }
        })
        .collect();
    v2
}

fn answer(v: &Vec<(u32, u32)>) -> u32 {
    let mut total = 0;
    for range in v {
        total += (range.1 - range.0) + 1;
    }
    total
}


fn main() {
    let input = read_input();

    let v = split_input(&input);

    let x = answer(&v);

    println!("{}", x);
}
