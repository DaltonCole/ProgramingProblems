
fn get_num() -> usize {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().parse().unwrap()
}

fn get_converted_line() -> Vec<char> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line = line.trim().replace(" ", "").to_uppercase();
    line.chars().collect()
}

fn encrypt(s: &Vec<char>, num: usize) -> String {
    let len = s.len();
    if num > len {
        return s.into_iter().collect();
    }

    let mut e: Vec<char> = vec![' '; len];

    let mut index = 0;
    e[0] = s[0];
    for i in 1..len {
        index = index + num;
        if index >= len {
            index = 0;
            while e[index] != ' ' {
                index += 1;
            }
        }
        e[index] = s[i];
    }

    e.into_iter().collect()
}

fn main() {
    let mut num;
    loop {
        num = get_num();
        if num == 0 {
            break;
        }

        let convert = get_converted_line();

        let answer = encrypt(&convert, num);

        println!("{}", answer);
    }
}
