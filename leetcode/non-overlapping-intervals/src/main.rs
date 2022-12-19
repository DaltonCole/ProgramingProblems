struct Solution;

impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        let mut removed = 0;
        let mut max = i32::MIN;

        intervals.sort();

        #[cfg(test)]
        println!("{:?}", intervals);

        for interval in &intervals {
            if interval[0] < max {
                removed += 1;
                max = max.min(interval[1]);
            } else {
                max = interval[1];
            }
            #[cfg(test)]
            println!("{}, {}, {:?}", removed, max, interval);
        }
        removed
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]]),
            1
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![1, 2], vec![1, 2]]),
            2
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3]]),
            0
        );
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::erase_overlap_intervals(vec![]), 0);
    }

    #[test]
    fn test5() {
        assert_eq!(
            Solution::erase_overlap_intervals(vec![vec![0, 501], vec![500, 502], vec![501, 1000]]),
            1
        );
    }

    #[test]
    fn test6() {
        assert_eq!(
            Solution::erase_overlap_intervals(vec![vec![1, 3], vec![2, 3], vec![3, 4], vec![1, 2]]),
            1
        );
    }

    #[test]
    fn test7() {
        assert_eq!(
            Solution::erase_overlap_intervals(vec![
                vec![-52, 31],
                vec![-73, -26],
                vec![82, 97],
                vec![-65, -11],
                vec![-62, -49],
                vec![95, 99],
                vec![58, 95],
                vec![-31, 49],
                vec![66, 98],
                vec![-63, 2],
                vec![30, 47],
                vec![-40, -26]
            ]),
            7
        );
    }

    #[test]
    fn test9() {
        assert_eq!(
            Solution::erase_overlap_intervals(vec![
                vec![-1000, 1000],
                vec![0, 1],
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![4, 5],
                vec![5, 6],
            ]),
            1
        );
    }
}
