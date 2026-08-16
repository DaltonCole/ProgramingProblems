struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        let mut min = new_interval[0];
        let mut max = new_interval[1];

        for interval in intervals {
            // Interval should have ended last time
            if max < interval[0] && max != i32::MIN {
                result.push(vec![min, max]);
                result.push(interval);
                min = i32::MAX;
                max = i32::MIN;
            }
            // Interval should keep going
            else if min <= interval[1] {
                min = std::cmp::min(min, interval[0]);
                max = std::cmp::max(max, interval[1]);
            }
            // We're not in an interval
            else {
                result.push(interval);
            }
        }

        // If we haven't finished processing the interval yet, add in the final results
        if min != i32::MAX {
            result.push(vec![min, max]);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            vec![vec![1, 5], vec![6, 9]],
            Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5])
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            vec![vec![1, 2], vec![3, 10], vec![12, 16]],
            Solution::insert(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16]
                ],
                vec![4, 8]
            )
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            vec![vec![1, 7]],
            Solution::insert(vec![vec![1, 5]], vec![5, 7])
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            vec![vec![0, 0], vec![1, 5]],
            Solution::insert(vec![vec![1, 5]], vec![0, 0])
        );
    }

    #[test]
    fn test5() {
        assert_eq!(
            vec![vec![0, 1], vec![2, 5]],
            Solution::insert(vec![vec![2, 5]], vec![0, 1])
        );
    }
}
