struct Solution;

impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut i1 = 0;
        let mut i2 = 0;

        while nums1[i1] != nums2[i2] {
            if nums1[i1] < nums2[i2] {
                i1 += 1;
            } else {
                i2 += 1;
            }

            if i1 >= nums1.len() || i2 >= nums2.len() {
                return -1;
            }
        }

        nums1[i1]
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test1() {
        assert_eq!(2, Solution::get_common(vec![1, 2, 3], vec![2, 4]));
    }

    #[test]
    fn test2() {
        assert_eq!(2, Solution::get_common(vec![1, 2, 3, 6], vec![2, 3, 4, 5]));
    }

    #[test]
    fn test3() {
        assert_eq!(-1, Solution::get_common(vec![1, 2, 3,], vec![4, 5]));
    }
}
