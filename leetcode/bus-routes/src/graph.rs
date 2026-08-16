struct Solution;

use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        if source == target {
            return 0;
        }

        let graph = Self::generate_graph(&routes, source, target);

        if !graph.last().unwrap().iter().any(|&x| x == true) {
            return -1;
        }

        let mut depth = 0;
        let mut queue = VecDeque::from([0]);
        let mut new_queue = VecDeque::new();
        let mut visited = HashSet::from([0]);

        while let Some(node) = queue.pop_front() {
            for index in graph[node]
                .iter()
                .enumerate()
                .filter(|&(_, edge_exist)| *edge_exist)
                .map(|(index, _)| index)
            {
                if !visited.contains(&index) {
                    if index == graph.len() - 1 {
                        return depth;
                    }

                    visited.insert(index);
                    new_queue.push_back(index);
                }
            }

            // New level
            if queue.is_empty() {
                (new_queue, queue) = (queue, new_queue);
                depth += 1;
            }
        }

        -1
    }

    fn generate_graph(routes: &Vec<Vec<i32>>, source: i32, target: i32) -> Vec<Vec<bool>> {
        let mut route_set: Vec<HashSet<&i32>> = Vec::new();
        let mut graph = vec![vec![false; routes.len() + 2]; routes.len() + 2];

        for route in routes.iter() {
            route_set.push(HashSet::from_iter(route));
        }

        for (i, node1) in route_set.iter().enumerate() {
            if node1.contains(&source) {
                graph[0][i + 1] = true;
                graph[i + 1][0] = true;
            }
            if node1.contains(&target) {
                graph.last_mut().unwrap()[i + 1] = true;
                *graph[i + 1].last_mut().unwrap() = true;
            }

            for (j, node2) in route_set.iter().enumerate() {
                if !node1.is_disjoint(&node2) {
                    graph[i + 1][j + 1] = true;
                    graph[j + 1][i + 1] = true;
                }
            }
        }

        graph
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
}
