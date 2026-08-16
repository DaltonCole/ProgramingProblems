struct Solution;

use std::collections::HashMap;

impl Solution {
    fn include_or_do_not_include(current_end_time: i32, start_time: i32)


    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        // Convert to tuples
        let mut jobs = Vec::new();
        for ((&start, end), money) in start_time.iter().zip(end_time).zip(profit) {
            jobs.push((start, end, money));
        }
        jobs.sort();

        // Memorization
        let mut memory: HashMap<i32, i32> = HashMap::new();
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            120,
            Solution::job_scheduling(vec![1, 2, 3, 3], vec![3, 4, 5, 6], vec![50, 10, 40, 70])
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            150,
            Solution::job_scheduling(
                vec![1, 2, 3, 4, 6],
                vec![3, 5, 10, 6, 9],
                vec![20, 20, 100, 70, 60]
            )
        );
    }
    #[test]
    fn test3() {
        assert_eq!(
            6,
            Solution::job_scheduling(vec![1, 1, 1], vec![2, 3, 4], vec![5, 6, 4])
        );
    }
}
