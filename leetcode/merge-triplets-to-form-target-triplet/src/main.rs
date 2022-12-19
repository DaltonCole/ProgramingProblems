struct Solution;

impl Solution {
    pub fn merge_triplets(triplets: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        let mut found = [false, false, false];

        for triplet in triplets {
            if triplet[0] <= target[0] && triplet[1] <= target[1] && triplet[2] <= target[2] {
                found[0] |= triplet[0] == target[0];
                found[1] |= triplet[1] == target[1];
                found[2] |= triplet[2] == target[2];
            }
        }

        !found.contains(&false)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::merge_triplets(
            vec![vec![2, 5, 3], vec![1, 8, 4], vec![1, 7, 5]],
            vec![2, 7, 5],
        ));
    }

    #[test]
    fn test2() {
        assert!(!Solution::merge_triplets(
            vec![vec![3, 4, 5], vec![4, 5, 6]],
            vec![3, 2, 5],
        ));
    }

    #[test]
    fn test3() {
        assert!(Solution::merge_triplets(
            vec![vec![2, 5, 3], vec![2, 3, 4], vec![1, 2, 5], vec![5, 2, 3]],
            vec![5, 5, 5],
        ));
    }
}
