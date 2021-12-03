fn get_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    String::from(line.trim())
}

fn substring(s: &String, i: usize, j: usize) -> &String {
    let ss: String = s.chars().skip(i).take(j - i).collect();
    &ss
}

fn max_string(input: &String) -> u32 {
    let mut max = 100;

    for i in 0..input.len() {
        for j in i..input.len() {
            let mut tmp_str = input.clone();
            let sub_str = substring(&input, i, j);

        }
    }

    max
}

fn main() {
    let input = get_line();

    println!("{}", max_string(&input));
}
