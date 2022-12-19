struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut start_end_index_map = HashMap::new();
        let mut letters = Vec::new();
        let mut partitions = Vec::<i32>::new();

        for (index, c) in s.chars().enumerate() {
            match start_end_index_map.get(&c) {
                None => {
                    start_end_index_map.insert(c, [index, index]);
                    letters.push(c);
                }
                Some(v) => {
                    start_end_index_map.insert(c, [v[0], index]);
                }
            }
        }

        #[cfg(test)]
        println!("{:#?}", start_end_index_map);

        let mut max = start_end_index_map.get(&letters[0]).unwrap()[1];
        for c in letters.into_iter().skip(1) {
            if let Some([start, end]) = start_end_index_map.get(&c) {
                if start < &max {
                    max = max.max(*end);
                } else {
                    partitions.push(max as i32 + 1);
                    max = *end;
                }
            }
        }

        partitions.push(max as i32 + 1);

        #[cfg(test)]
        println!("{:?}", partitions);

        for i in (1..(partitions.len())).rev() {
            println!("{}", i);
            partitions[i] -= partitions[i - 1];
        }

        #[cfg(test)]
        println!("{:?}", partitions);

        partitions
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

    #[test]
    fn test3() {
        assert_eq!(Solution::partition_labels("e".to_string()), vec![1]);
    }
}
