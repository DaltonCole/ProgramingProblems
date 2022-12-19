struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.len() == 0 {
            return vec![];
        }

        intervals.sort();

        let mut merged = Vec::new();
        let mut prev_start = intervals[0][0];
        let mut prev_end = intervals[0][1];

        for interval in intervals.into_iter().skip(1) {
            let start = interval[0];
            let end = interval[1];

            // New does not overlap with old
            if prev_end < start {
                merged.push(vec![prev_start, prev_end]);
                prev_start = start;
                prev_end = end;
            }
            // New overlaps with old
            else {
                prev_start = prev_start.min(start);
                prev_end = prev_end.max(end);
            }
        }

        merged.push(vec![prev_start, prev_end]);

        merged
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::merge(vec![vec![1, 4], vec![4, 5]]),
            vec![vec![1, 5]]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::merge(vec![]), Vec::<Vec::<i32>>::new());
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::merge(vec![vec![1, 4], vec![0, 4]]),
            vec![vec![0, 4]]
        );
    }

    #[test]
    fn test5() {
        assert_eq!(
            Solution::merge(vec![vec![1, 4], vec![0, 0]]),
            vec![vec![0, 0], vec![1, 4]]
        );
    }
}
