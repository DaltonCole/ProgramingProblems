fn get_input() -> (u32, u32, u32, u32) {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let v: Vec<u32> = line.trim().split(" ").map(|x| x.parse().unwrap()).collect();
    (v[0], v[1], v[2], v[3])
}

fn permutations(a: u32, b: u32, c: u32, num: u32) -> bool {
    let mut found_combination = false;
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;

    // a * x
    while (a * x) <= num {
        // b * y
        while (b * y <= num) {
            // c * z
            while (c * z <= num) {
                if (a * x) + (b * y) + (c * z) == num {
                    println!("{} {} {}", x, y, z);
                    found_combination = true;
                }
                z += 1;
            }
            y += 1;
            z = 0;
        }
        x += 1;
        y = 0;
    }

    found_combination
}

fn main() {
    let (a, b, c, num) = get_input();
    if permutations(a, b, c, num) == false {
        println!("impossible");
    }
}
