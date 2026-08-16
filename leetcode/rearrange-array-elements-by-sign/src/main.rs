struct Solution;

impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; nums.len()];
        let mut pos_index = 0;
        let mut neg_index = 1;

        for num in nums {
            if num > 0 {
                res[pos_index] = num;
                pos_index += 2;
            } else {
                res[neg_index] = num;
                neg_index += 2;
            }
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test1() {
        assert_eq!(
            vec![3, -2, 1, -5, 2, -4],
            Solution::rearrange_array(vec![3, 1, -2, -5, 2, -4])
        );
    }
}
