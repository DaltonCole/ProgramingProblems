use std::collections::HashSet;

fn get_num() -> u8 {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().parse().unwrap()
}

fn repeated_char(s: &str) -> bool {
    let mut chars = HashSet::new();
    for c in s.chars() {
        match chars.contains(&c) {
            true => {
                return true;
            }
            false => {
                chars.insert(c);
            }
        }
    }
    false
}

fn get_names(num: u8) -> Vec<String> {
    let mut names = Vec::new();
    for _ in 0..num {
        let mut name = String::new();
        std::io::stdin().read_line(&mut name).unwrap();
        name = name.trim().to_string();

        if name.chars().count() >= 5 && repeated_char(&name) == false {
            names.push(name);
        }
    }

    names.sort();
    names.sort_by(|a, b| {
        if a.chars().count() == b.chars().count() {
            return b.cmp(&a);
        } else {
            return a.chars().count().cmp(&b.chars().count());
        }
    });

    // println!("{:?}", names);

    names
}

fn main() {
    let num = get_num();
    let names = get_names(num);

    match names.first() {
        None => {
            println!("neibb!");
        }
        Some(name) => {
            println!("{}", name);
        }
    }
}
