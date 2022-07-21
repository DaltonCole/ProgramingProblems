use std::collections::HashMap;

fn get_inputs() -> Vec<u32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().split(" ")
        .map(|x| x.parse().unwrap())
        .collect()
}

fn are_equal(map: &mut HashMap<u32, i32>, a: u32, b: u32) -> bool {
    let count = map.entry(a).or_insert(0);
    *count += 1;

    let count = map.entry(b).or_insert(0);
    *count -= 1;

    for (_, value) in map {
        if *value != 0 {
            return false;
        }
    }

    return true;
}

fn insert_partitions(a: &Vec<u32>, b: &Vec<u32>) -> Vec<usize> {
    let mut map: HashMap<u32, i32> = HashMap::new();
    let mut parts = Vec::<usize>::new();
    let mut start = 0;
    let mut end = 1;
    while start < a.len() {
        if are_equal(&mut map, a[end-1], b[end-1]) {
            map.clear();
            parts.push(end);
            start = end;
            end = start + 1;
        } else {
            end += 1;
        }
    }

    parts
}

fn main() {
    get_inputs();
    let a = get_inputs();
    let b = get_inputs();

    let splits = insert_partitions(&a, &b);
    let mut splits_index = 0;

    for (pos, e) in b.iter().enumerate() {
        if splits.len() != splits_index {
            if pos == splits[splits_index] {
                print!("# ");
                splits_index += 1;
            }
        }
        print!("{} ", e);
    }
    println!();
}
