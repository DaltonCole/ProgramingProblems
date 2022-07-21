fn get_input() -> Vec<u32> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().split(" ")
        .map(|x| x.parse().unwrap())
        .collect()
}

fn solution(v: &Vec<u32>, n: usize) -> bool {
    let mut k = 2;
    while (k * n) < v.len() {
        if v[(n * k) - 1] <= v[(n * (k - 1)) - 1] {
            return false;
        } else {
            //println!("{} > {} for {} in {:?}", v[(n * k) - 1], v[(n * (k - 1)) - 1], n, v);
        }
        k += 1;
    }

    return true;
}

fn solution2(v: &Vec<u32>, n: usize) -> bool {
    let mut old = v[n - 1];
    let mut index = n + n - 1;
    while index < v.len() {
        if v[index] < old {
            return false
        }
        old = v[index];
        index += n;
    }

    return true;
}

fn main() {
    get_input();
    let nums = get_input();
    for n in 1..(nums.len() / 2) {
        if solution2(&nums, n) {
            println!("{}", n);
            return;
        }
    }
    println!("ABORT!");
}
