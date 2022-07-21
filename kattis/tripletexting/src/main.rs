use std::collections::HashMap;

fn get_input() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn split_input(line: &String) -> Vec<&str> {
    let split_pos: usize = line.len() / 3;

    let mut v: Vec<&str> = Vec::new();

    v.push(&line[0..split_pos]);
    v.push(&line[split_pos..(2*split_pos)]);
    v.push(&line[(2*split_pos)..line.len()]);

    return v;
}

fn find_most_occuring_string<'a>(s: &'a Vec<&str>) -> &'a str {
    let mut map = HashMap::new();

    for word in s {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }


    for (key, value) in map.into_iter() {
        if value == 2 {
            return key;
        }
    }

    return s[1];
}

fn main() {
    let line = get_input();
    let split = split_input(&line);
    println!("{}", find_most_occuring_string(&split));
}
