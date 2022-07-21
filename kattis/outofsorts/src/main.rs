fn get_inputs() -> Vec<u64> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().split(" ")
        .map(|x| x.parse().unwrap())
        .collect()
}

fn binary_search(v: &[u64], n: u64) -> bool {
    if v.len() == 0 {
        return false;
    }
    // Get mid point, rounded down
    let mid = (v.len() - 1) / 2;
    // If mid is our number, return true
    if v[mid] == n {
        return true;
    } 
    // If only number is wrong, return false
    else if v.len() == 1 {
        return false;
    } else {
        // Go right
        if v[mid] < n {
            return binary_search(&v[mid+1..v.len()], n);
        } else { // Go left
            return binary_search(&v[0..mid], n);
        }
    }

}

fn main() {
    let tmp = get_inputs();
    let (n, m, a, c, mut x) = (tmp[0], tmp[1], tmp[2], tmp[3], tmp[4]);
    let mut v = Vec::<u64>::new();
    for _ in 0..n {
        v.push(((a * x) + c) % m);
        x = *v.last().unwrap();
    }
    let mut count = 0;
    for i in &v {
        if binary_search(&v[0..v.len()], *i) {
            count += 1;
        }
    }
    println!("{}", count);
}
