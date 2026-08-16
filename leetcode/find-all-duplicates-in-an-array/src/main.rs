struct Solution;

impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut dups = Vec::new();
        let mut prev = [false; 100_000];

        for num in nums {
            if prev[num as usize] {
                dups.push(num);
            } else {
                prev[num as usize] = true;
            }
        }

        dups
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            vec![2, 3],
            Solution::find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1])
        );
    }
    #[test]
    fn test2() {
        assert_eq!(vec![1], Solution::find_duplicates(vec![1, 1, 2]));
    }
    #[test]
    fn test3() {
        assert_eq!(Vec::<i32>::new(), Solution::find_duplicates(vec![1]));
    }
}
