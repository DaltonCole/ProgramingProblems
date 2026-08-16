struct Solution;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let total: i32 = nums.iter().sum();
        let total = total as usize;

        if total % 2 == 1 {
            return false;
        }

        let half = total / 2;

        let mut row = vec![0; half + 1];

        for num in nums {
            for col in (num as usize..=half).rev() {
                let dont_include = row[col];
                let mut include = 0;

                if col >= num as usize {
                    include = num + row[col - num as usize];
                }

                row[col] = std::cmp::max(include, dont_include);
            }
        }

        (*row.iter().last().unwrap()) as usize == half
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::can_partition(vec![1, 5, 11, 5]));
    }

    #[test]
    fn test2() {
        assert! {!Solution::can_partition(vec![1,2,3,5])};
    }

    #[test]
    fn test3() {
        assert! {!Solution::can_partition(vec![22, 2, 2])};
    }
}
