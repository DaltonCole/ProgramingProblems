struct Solution;

use std::cell::Cell;
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut min_max_of_chars = HashMap::new();

        for (i, letter) in s.chars().enumerate() {
            let mut pair = min_max_of_chars
                .entry(letter)
                .or_insert(Cell::new((usize::MAX, usize::MIN)))
                .get_mut();

            pair.0 = (pair.0).min(i);
            pair.1 = (pair.1).max(i);
        }

        loop {
            let mut changed = false;
            let mut remove = HashSet::new();
            for first in min_max_of_chars.keys() {
                if let Some(_) = remove.get(first) {
                    continue;
                }
                for second in min_max_of_chars.keys() {
                    if let Some(_) = remove.get(second) {
                        continue;
                    }
                    if first != second {
                        let pair1 = min_max_of_chars.get(&first).unwrap();
                        let pair2 = min_max_of_chars.get(&second).unwrap();

                        if Self::intersection(
                            pair1.get().0,
                            pair1.get().1,
                            pair2.get().0,
                            pair2.get().1,
                        ) {
                            let my_min = pair1.get().0.min(pair2.get().0);
                            let my_max = pair1.get().1.max(pair2.get().1);
                            changed = true;
                            pair1.set((my_min, my_max));
                            remove.insert(*second);
                        }
                    }
                }
            }

            for r in remove {
                min_max_of_chars.remove(&r);
            }

            if !changed {
                break;
            }
        }

        let mut pairs = min_max_of_chars
            .values()
            .cloned()
            .map(|x| x.into_inner())
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
