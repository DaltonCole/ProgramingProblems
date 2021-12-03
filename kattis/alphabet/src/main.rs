use std::cmp::max;

fn get_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn longest_common_subsequence(a: &str, b: &str) -> u8 {
    // Converts strings to a vector of chars
    let x: Vec<char> = a.chars().collect();
    let y: Vec<char> = b.chars().collect();
    // Create table (len(a)+1 by len(b)+1)
    //   Need buffer filled with 0s before each string (hence the extra dimensions)
    let mut table = vec![vec![0; y.len() + 1]; x.len() + 1];

    for i in 1..(x.len() + 1) {
        for j in 1..(y.len() + 1) {
            if x[i-1] == y[j-1] {
                table[i][j] = 1 + table[i - 1][j - 1];
            } else {
                table[i][j] = max(table[i-1][j], table[i][j-1]);
            }
        }
    }

    table[x.len()][y.len()]
}

fn main() {
    let line = get_line();

    let alph = "abcdefghijklmnopqrstuvwxyz";

    let answer = 26 - longest_common_subsequence(&line, &alph);

    println!("{}", answer);
}
