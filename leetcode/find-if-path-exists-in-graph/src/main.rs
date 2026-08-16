struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut islands = HashMap::<usize, HashSet<i32>>::new();
        let mut node_house = HashMap::<i32, usize>::new();
        let mut next_island = 0;

        for edge in edges {
            let (start, end) = (edge[0], edge[1]);

            match (node_house.get(&start), node_house.get(&end)) {
                (Some(&a), Some(&b)) => {
                    if a == b {
                        continue;
                    }
                    // --- Join b into a --- //
                    // First join islands(b) into islands(a)
                    let island_members_b = islands.remove(&b).unwrap();
                    let mut island_members_a = islands.get_mut(&a).unwrap();

                    for &member_b in &island_members_b {
                        island_members_a.insert(member_b);
                    }

                    islands.remove(&b);
                    // Update all node houses
                    for &house in &island_members_b {
                        node_house.insert(house, a);
                    }
                }
                (Some(&a), None) | (None, Some(&a)) => {
                    node_house.insert(start, a);
                    node_house.insert(end, a);
                    if let Some(island_members) = islands.get_mut(&a) {
                        island_members.insert(start);
                        island_members.insert(end);
                    }
                }
                (None, None) => {
                    node_house.insert(start, next_island);
                    node_house.insert(end, next_island);
                    islands.insert(next_island, HashSet::from([start, end]));
                    next_island += 1;
                }
            }

            #[cfg(test)]
            {
                println!("({}, {})", start, end);
                println!("Node House: {:#?}", node_house);
                println!("Islands: {:#?}", islands);
            }
        }

        node_house.get(&source) == node_house.get(&destination)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::valid_path(
            3,
            vec![vec![0, 1], vec![1, 2], vec![2, 0]],
            0,
            2
        ));
    }

    #[test]
    fn test2() {
        assert!(!Solution::valid_path(
            6,
            vec![vec![0, 1], vec![0, 2], vec![3, 5], vec![5, 4], vec![4, 3]],
            0,
            5
        ));
    }

    #[test]
    fn test3() {
        assert!(Solution::valid_path(1, Vec::<Vec<i32>>::new(), 0, 0));
    }
}
