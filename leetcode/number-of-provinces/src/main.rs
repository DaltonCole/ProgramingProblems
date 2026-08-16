struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        //Self::union_find(&is_connected)
        Self::bfs(&is_connected)
    }

    pub fn bfs(is_connected: &Vec<Vec<i32>>) -> i32 {
        let mut islands = 0;
        let mut processed_nodes = vec![false; is_connected.len()];

        for node in 0..is_connected.len() {
            if processed_nodes[node] {
                continue;
            }

            let mut queue = vec![node];

            while let Some(current_node) = queue.pop() {
                processed_nodes[current_node] = true;

                let connected_nodes = is_connected[current_node]
                    .iter()
                    .enumerate()
                    .filter(|(_, &value)| value == 1)
                    .map(|(index, _)| index)
                    .collect::<Vec<usize>>();

                for connected_node in connected_nodes {
                    if !processed_nodes[connected_node] {
                        queue.push(connected_node);
                        processed_nodes[connected_node] = true;
                    }
                }
            }

            islands += 1;
        }

        islands
    }

    pub fn union_find(is_connected: &Vec<Vec<i32>>) -> i32 {
        let mut islands: Vec<HashSet<usize>> = Vec::new();

        for edges in is_connected {
            let connected_nodes = edges
                .iter()
                .enumerate()
                .filter(|(_, &value)| value == 1)
                .map(|(index, _)| index)
                .collect::<Vec<usize>>();

            let mut joined_nodes = HashSet::new();
            for node in connected_nodes {
                let mut new_islands = Vec::new();
                if joined_nodes.contains(&node) {
                    continue;
                }
                // Find node in linked list
                while let Some(node_list) = islands.pop() {
                    if node_list.contains(&node) {
                        joined_nodes.extend(node_list);
                        break;
                    } else {
                        new_islands.push(node_list);
                    }
                }
                joined_nodes.insert(node);
                islands.append(&mut new_islands);
            }
            islands.push(joined_nodes);
        }

        islands.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            2,
            Solution::find_circle_num(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]])
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            3,
            Solution::find_circle_num(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]])
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            8,
            Solution::find_circle_num(vec![
                vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
                vec![0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0],
                vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]
            ])
        );
    }
}
