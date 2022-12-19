struct Solution;

#[derive(Debug)]
enum FoundStates {
    NotFound,
    Found,
    Done,
}

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut merged = Vec::new();
        let mut found = FoundStates::NotFound;
        let mut prev_start = None;
        let mut prev_end = None;

        for interval in &intervals {
            let start = interval[0];
            let end = interval[1];

            match &found {
                FoundStates::NotFound => {
                    // Complete overlap
                    if start == new_interval[0] && end == new_interval[1] {
                        found = FoundStates::Done;
                        merged.push(interval.to_vec());
                    }
                    // We passed the new_interval
                    else if new_interval[1] < start {
                        found = FoundStates::Done;
                        merged.push(new_interval.clone());
                        merged.push(interval.to_vec());
                    }
                    // Interval is within new_interval
                    else if new_interval[0] <= end {
                        found = FoundStates::Found;
                        prev_start = Some(start.min(new_interval[0]));
                        prev_end = Some(end.max(new_interval[1]));
                    }
                    // This interval encompases the new_interval
                    else if new_interval[1] <= end {
                        found = FoundStates::Done;
                        merged.push(vec![
                            prev_start.unwrap_or(start).min(new_interval[0]),
                            prev_end.unwrap_or(end).max(new_interval[1]),
                        ]);
                    }
                    // Interval is not within new interval
                    else {
                        merged.push(interval.to_vec());
                    }
                }
                FoundStates::Found => {
                    // Last interval was large enough
                    if new_interval[1] < start {
                        merged.push(vec![prev_start.unwrap(), prev_end.unwrap()]);
                        merged.push(interval.to_vec());
                        found = FoundStates::Done;
                    }
                    // Update known interval and go to the next iteration
                    else {
                        prev_end = Some(prev_end.unwrap().max(end));
                    }
                }
                FoundStates::Done => {
                    merged.push(interval.to_vec());
                }
            }

            #[cfg(test)]
            println!("Found: {:?}, {:?}, {:?}", found, prev_start, prev_end);
            #[cfg(test)]
            println!("{:?}", merged);
        }

        match &found {
            // If Found, but not Done, last interval needed to be expaned
            FoundStates::NotFound => {
                merged.push(new_interval);
            }
            FoundStates::Found => {
                merged.push(vec![
                    prev_start.unwrap().min(new_interval[0]),
                    prev_end.unwrap().max(new_interval[1]),
                ]);
            }
            FoundStates::Done => {}
        }

        merged
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
            vec![vec![1, 5], vec![6, 9]]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::insert(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16]
                ],
                vec![4, 8]
            ),
            vec![vec![1, 2], vec![3, 10], vec![12, 16]]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::insert(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16]
                ],
                vec![3, 5]
            ),
            vec![
                vec![1, 2],
                vec![3, 5],
                vec![6, 7],
                vec![8, 10],
                vec![12, 16]
            ]
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::insert(vec![vec![1, 2], vec![3, 5]], vec![4, 8]),
            vec![vec![1, 2], vec![3, 8]]
        );
    }

    #[test]
    fn test5() {
        assert_eq!(
            Solution::insert(vec![vec![1, 2], vec![8, 9]], vec![3, 5]),
            vec![vec![1, 2], vec![3, 5], vec![8, 9]]
        );
    }

    #[test]
    fn test6() {
        assert_eq!(
            Solution::insert(vec![vec![1, 5]], vec![5, 7]),
            vec![vec![1, 7]]
        );
    }

    #[test]
    fn test7() {
        assert_eq!(Solution::insert(vec![], vec![5, 7]), vec![vec![5, 7]]);
    }

    #[test]
    fn test8() {
        assert_eq!(
            Solution::insert(vec![vec![1, 5]], vec![0, 5]),
            vec![vec![0, 5]]
        );
    }

    #[test]
    fn test9() {
        assert_eq!(
            Solution::insert(vec![vec![1, 5]], vec![0, 6]),
            vec![vec![0, 6]]
        );
    }
}
