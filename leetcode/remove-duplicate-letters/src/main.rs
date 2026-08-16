struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut letter_counts: HashMap<char, i32> =
            s.to_lowercase().chars().fold(HashMap::new(), |mut map, c| {
                *map.entry(c).or_insert(0) += 1;
                map
            });

        let mut solution: Vec<char> = s.chars().collect();

        for c in ('a'..'z').rev() {
            if let Some(&count) = letter_counts.get(&c) {
                if count > 1 {
                    // Find first occurrence of c
                    println!("{}: {} - {:?}", c, count, solution);

                    let mut index = 0;
                    let mut original_count = *letter_counts.get(&c).unwrap();
                    while letter_counts.get(&c).unwrap() > &0 {
                        if solution[index] == c {
                            *letter_counts.entry(c).or_default() -= 1;

                            if letter_counts.get(&c).unwrap() == &0
                                || solution[index] < solution[index + 1]
                            {
                                solution.retain(|&x| {
                                    if x == c {
                                        original_count -= 1;
                                        if &original_count != letter_counts.get(&c).unwrap() {
                                            return false;
                                        }
                                    }
                                    true
                                });
                                break;
                            }
                        }
                        index += 1;
                    }
                }
            }
        }

        solution.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            "abc".to_string(),
            Solution::remove_duplicate_letters("bcabc".to_string())
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            "acdb".to_string(),
            Solution::remove_duplicate_letters("cbacdcbc".to_string())
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            "bacd".to_string(),
            Solution::remove_duplicate_letters("cbbacdcbc".to_string())
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            "acb".to_string(),
            Solution::remove_duplicate_letters("ccacbaba".to_string())
        );
    }
}
