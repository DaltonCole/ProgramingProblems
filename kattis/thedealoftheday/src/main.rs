fn get_input() -> Vec<u128> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().split(" ")
        .map(|x| x.parse().unwrap())
        .collect()
}

fn count(nums: &Vec<u128>, amount: u128, index: usize) -> u128 {
    if amount == 0 {
        println!("11111111111111");
        return 1;
    }

    if index >= nums.len() {
        println!("0000000000000000000");
        return 0;
    }

    // Last item to check
    if amount == 1 {
        println!("hi {}", nums[index]);
        return nums[index];
    }

    // Keep checking
    let mut total : u128 = nums[index];
    for i in index..nums.len() {
        total *= count(&nums, amount - 1, i + 1);
    }
    total
}

fn main() {
    let mut nums = get_input();
    let amount = get_input()[0];
    nums.retain(|&x| x != 0);
    let mut total : u128 = 0;
    for i in 0..=(nums.len()-amount as usize) {
        total += count(&nums, amount, i)
    }
    println!("{}", total);
}
