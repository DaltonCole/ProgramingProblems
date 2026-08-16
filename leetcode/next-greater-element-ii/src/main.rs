struct Solution;

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut greater_num = vec![-1; nums.len()];
        let mut stack = vec![0];

        for index in 1..nums.len() {
            while let Some(&smallest_elem_index) = stack.last() {
                if nums[smallest_elem_index] < nums[index] {
                    greater_num[smallest_elem_index] = nums[index];
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(index);
        }

        for index in 0..=stack[0] {
            while let Some(&smallest_elem_index) = stack.last() {
                if nums[smallest_elem_index] < nums[index] {
                    greater_num[smallest_elem_index] = nums[index];
                    stack.pop();
                } else {
                    break;
                }
            }
        }

        greater_num
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            vec![2, -1, 2],
            Solution::next_greater_elements(vec![1, 2, 1])
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            vec![2, 3, 4, -1, 4],
            Solution::next_greater_elements(vec![1, 2, 3, 4, 3])
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            vec![4, -1, 4, -1, 4],
            Solution::next_greater_elements(vec![1, 4, 3, 4, 3])
        );
    }
}
