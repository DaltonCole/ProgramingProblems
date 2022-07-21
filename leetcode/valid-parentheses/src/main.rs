struct Solution;

impl Solution {
    pub fn is_valid(s: &str) -> bool {
        let mut stack = Vec::new();

        for c in s.chars() {
            match c {
                '(' => {stack.push(c)},
                '[' => {stack.push(c)},
                '{' => {stack.push(c)},
                ')' => {
                    match stack.pop() {
                        Some(x) => {
                            if x != '(' {
                                return false;
                            }
                        },
                        None => { return false; },
                    }
                },
                ']' => {
                    match stack.pop() {
                        Some(x) => {
                            if x != '[' {
                                return false;
                            }
                        },
                        None => { return false; },
                    }
                },
                '}' => {
                    match stack.pop() {
                        Some(x) => {
                            if x != '{' {
                                return false;
                            }
                        },
                        None => { return false; },
                    }
                },
                _ => { panic!("Unexpected character: {}", c) },
            }

        }
        stack.is_empty()
    }
}

fn main() {
    println!("Hello, world!");
}
#[test]
fn test1() {
    assert_eq!(true, Solution::is_valid("()"));
}
#[test]
fn test2() {
    assert_eq!(true, Solution::is_valid("()[]{}"));
}
#[test]
fn test3() {
    assert_eq!(true, Solution::is_valid("({[]})"));
}
#[test]
fn test4() {
    assert_eq!(false, Solution::is_valid("(]"));
}
#[test]
fn test5() {
    assert_eq!(false, Solution::is_valid("("));
}
#[test]
fn test6() {
    assert_eq!(false, Solution::is_valid(")"));
}

// A better solution: https://leetcode.com/problems/valid-parentheses/discuss/207871/Rust
