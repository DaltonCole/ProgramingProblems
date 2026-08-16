struct Solution;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn num_buses_to_destination(mut routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        if source == target {
            return 0;
        }

        let mut stop_to_route = HashMap::new();
        for i in 0..routes.len() {
            for &stop in routes[i].iter() {
                stop_to_route
                    .entry(stop)
                    .and_modify(|routes: &mut Vec<usize>| routes.push(i))
                    .or_insert(Vec::from([i]));
            }
        }

        if !stop_to_route.contains_key(&source) || !stop_to_route.contains_key(&target) {
            return -1;
        }

        let mut queue = VecDeque::from([(0, source)]);
        let mut visited = HashSet::from([source]);

        while let Some((depth, stop)) = queue.pop_front() {
            for &next_route in stop_to_route.get(&stop).unwrap().iter() {
                for &next_stop in routes[next_route].iter() {
                    if next_stop == target {
                        return depth + 1;
                    }
                    if !visited.contains(&next_stop) {
                        visited.insert(next_stop);
                        queue.push_back((depth + 1, next_stop));
                    }
                }
                routes[next_route].clear();
            }
        }

        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            2,
            Solution::num_buses_to_destination(vec![vec![1, 2, 7], vec![3, 6, 7]], 1, 6)
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            -1,
            Solution::num_buses_to_destination(
                vec![
                    vec![7, 12],
                    vec![4, 5, 15],
                    vec![6],
                    vec![15, 19],
                    vec![9, 12, 13]
                ],
                15,
                12
            )
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            2,
            Solution::num_buses_to_destination(
                vec![vec![1, 2, 7], vec![3, 6, 7, 8], vec![7, 8, 9]],
                1,
                9
            )
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            3,
            Solution::num_buses_to_destination(
                vec![vec![1, 2, 7], vec![3, 6, 7, 8], vec![8, 9]],
                1,
                9
            )
        );
    }

    #[test]
    fn test5() {
        assert_eq!(
            0,
            Solution::num_buses_to_destination(vec![vec![1, 2, 7], vec![3, 6, 7]], 1, 1)
        );
    }
    #[test]
    fn test6() {
        assert_eq!(
            0,
            Solution::num_buses_to_destination(vec![vec![1, 2, 7], vec![3, 6, 7]], 1, 1)
        );
    }

    #[test]
    fn test7() {
        assert_eq!(
            -1,
            Solution::num_buses_to_destination(vec![vec![1, 2, 7], vec![3, 6, 7]], 8, 6)
        );
    }
}
