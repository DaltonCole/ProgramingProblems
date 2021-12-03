use std::collections::HashSet;
use std::collections::HashMap;
use std::iter::FromIterator;

struct ToyCategory {
    pub toys: HashSet<u8>,
    pub available: u8,
    pub main_toy: u8,
}

fn get_line() -> Vec<u8> {
let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().split(" ")
        .map(|x| x.parse().unwrap())
        .collect()
}

fn get_first_line() -> (u8, u8, u8) {
    let v = get_line();
    (v[0], v[1], v[2])
}

fn get_favorite_toys() -> HashSet<u8> {
    let mut v = get_line();
    v.remove(0);
    HashSet::from_iter(v)
}

fn get_toy_categories() -> ToyCategory {
    let mut v = get_line();
    v.remove(0);

    let mut toy = ToyCategory{ 
        toys: HashSet::new(), 
        available: v.pop().unwrap(),
        main_toy: 0,
    };
    for i in v {
        toy.toys.insert(i);
        toy.main_toy = i;
    }
    toy
}

// Doesn't work
fn simplify(children: &mut Vec<HashSet<u8>>, toys: &mut Vec<ToyCategory>) {
    // Map toys in the same category to their representative toy
    let mut map = HashMap::new();
    for toy in toys {
        for t in &toy.toys {
            map.insert(t, toy.main_toy);
        }
    }
    // Update children so only the representative toy from each category remains
    for child in children {
        for (key, value) in &map {
            if *key != value {
                child.remove(&key);
            }
        }
        println!("{:?}", child);
    }
}



fn main() {
    let (num_children, num_toys, num_categories) = get_first_line();
    let mut children = Vec::new();
    for _ in 0..num_children {
        children.push(get_favorite_toys());
    }
    let mut toys = Vec::new();
    for _ in 0..num_categories {
        toys.push(get_toy_categories());
    }
}
