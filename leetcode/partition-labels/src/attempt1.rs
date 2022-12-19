struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut min_max_of_chars = HashMap::new();

        for (i, letter) in s.chars().enumerate() {
            let mut pair = min_max_of_chars
                .entry(letter)
                .or_insert((usize::MAX, usize::MIN));
            pair.0 = (pair.0).min(i);
            pair.1 = (pair.1).max(i);
        }

        loop {
            let mut changed = false;
            let keys = min_max_of_chars.keys().cloned().collect::<Vec<char>>();
            for first in &keys {
                if let None = min_max_of_chars.get(&first) {
                    continue;
                }
                for second in &keys {
                    if let None = min_max_of_chars.get(&second) {
                        continue;
                    }
                    if first != second {
                        let mut pair1 = min_max_of_chars.get(&first).unwrap().clone();
                        let pair2 = min_max_of_chars.get(&second).unwrap().clone();

                        #[cfg(test)]
                        println!(
                            "{} {} - {},{} - {},{}",
                            first, second, pair1.0, pair1.1, pair2.0, pair2.1
                        );
                        if Self::intersection(pair1.0, pair1.1, pair2.0, pair2.1) {
                            pair1.0 = pair1.0.min(pair2.0);
                            pair1.1 = pair1.1.max(pair2.1);
                            changed = true;
                            min_max_of_chars.insert(*first, pair1);
                            min_max_of_chars.remove(&second);
                        }

                        #[cfg(test)]
                        println!("{} {} - {},{}\n", first, second, pair1.0, pair1.1);
                    }
                }
            }
            if !changed {
                break;
            }
        }

        let mut pairs = min_max_of_chars
            .values()
            .cloned()
            .collect::<Vec<(usize, usize)>>();

        pairs.sort();

        let mut answer = Vec::new();
        for pair in pairs {
            answer.push((pair.1 - pair.0 + 1) as i32);
        }

        #[cfg(test)]
        println!("{:#?}", min_max_of_chars);

        answer
    }

    fn intersection(start1: usize, end1: usize, start2: usize, end2: usize) -> bool {
        if (start1 <= start2 && start2 <= end1)
            || (start2 <= start1 && start1 <= end2)
            || (start1 <= end2 && end2 <= end1)
            || (start2 <= end1 && end1 <= end2)
        {
            return true;
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::partition_labels("ababcbacadefegdehijhklij".to_string()),
            vec![9, 7, 8]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::partition_labels("eccbbbbdec".to_string()),
            vec![10]
        );
    }
}
