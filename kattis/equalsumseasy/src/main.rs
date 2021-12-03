use std::collections::HashMap;
use std::collections::HashSet;

enum Solution {
    Found {first: HashSet<u32>, second: HashSet<u32>},
    Impossible,
    None,
}

fn get_number() -> u32 {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().parse().unwrap()
}

fn get_numbers() -> Vec<u32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut v: Vec<u32> = line.trim().split(" ")
        .map(|x| x.parse().unwrap())
        .collect();
    v.remove(0);
    v
}


fn add_subset(x: u32, map: &mut HashMap<u32, HashSet<u32>>) -> Solution {
    // Temporary map to store new sets
    let mut new_items: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut new_entries = false;
    // For each key
    for key in map.keys() {
        // If x is not part of the set
        if let None = map.get(key).unwrap().get(&x) {
            // Add up old set + new item
            let sum: u32 = key + x;
            // Add new element to set
            let mut new_set: HashSet<u32> = map.get(key).unwrap().clone();
            new_set.insert(x);

            // If sum already exists
            if let Some(other_set) = map.get(&sum) {
                // Make sure they are not the same set
                if *other_set == new_set {
                    continue;
                }

                // Return other set and new set
                return Solution::Found{
                    first: other_set.clone(),
                    second: new_set,
                }
            }
            // Else, add to sum: set to new items
            new_items.insert(sum, new_set);
            // Added new element
            new_entries = true;
        }
    }
    // Add new items to map
    map.extend(new_items);

    match new_entries {
        true => Solution::None,
        false => Solution::Impossible,
    }
}


fn solution(v: &Vec<u32>) {
    let mut sum: HashMap<u32, HashSet<u32>> = HashMap::new();

    // Initialize
    for x in v {
        sum.insert(*x, vec![*x].into_iter().collect());
    }

    loop {
        for x in v {
            match add_subset(*x, &mut sum) {
                Solution::Found { first, second }=> {
                    let mut a = first.into_iter().collect::<Vec<u32>>();
                    let mut b = second.into_iter().collect::<Vec<u32>>();
                    a.sort();
                    b.sort();
                    println!("{}", a.into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
                    println!("{}", b.into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
                    return;
                },
                Solution::Impossible => {
                    println!("Impossible");
                    return;
                }
                Solution::None => {}
            }
        }
    }

}


fn main() {
    for i in 0..get_number() {
        println!("Case #{}", i + 1);
        solution(&get_numbers());
    }
}
