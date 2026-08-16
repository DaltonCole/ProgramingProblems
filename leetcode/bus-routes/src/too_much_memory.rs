struct Solution;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Eq, PartialEq)]
struct Journey {
    num_stops: i32,
    bus_station: i32,
    previous_stations: HashSet<i32>,
}

impl Journey {
    pub fn new(bus_station: i32) -> Journey {
        Journey {
            num_stops: 0,
            bus_station,
            previous_stations: HashSet::new(),
        }
    }

    pub fn add_stop(&self, next_station: i32) -> Option<Journey> {
        if self.previous_stations.contains(&next_station) {
            return None;
        }

        let mut new_journey = self.clone();

        new_journey.num_stops += 1;
        new_journey.bus_station = next_station;
        new_journey.previous_stations.insert(self.bus_station);

        Some(new_journey)
    }
}

impl Ord for Journey {
    fn cmp(&self, other: &Self) -> Ordering {
        other.num_stops.cmp(&self.num_stops)
    }
}

impl PartialOrd for Journey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        if source == target {
            return 0;
        }

        let source_to_stops = Self::map_source_to_stops(&routes);

        let mut pq = BinaryHeap::new(); // Priority Queue

        pq.push(Journey::new(source));

        while let Some(current_journey) = pq.pop() {
            let current_stop = current_journey.bus_station;

            for next_stop in source_to_stops.get(&current_stop).unwrap() {
                if let Some(new_journey) = current_journey.add_stop(*next_stop) {
                    if new_journey.bus_station == target {
                        return new_journey.num_stops;
                    }
                    pq.push(new_journey);
                }
            }
        }

        -1
    }

    fn map_source_to_stops(routes: &Vec<Vec<i32>>) -> HashMap<i32, HashSet<i32>> {
        let mut source_to_stops = HashMap::new();

        for route in routes.iter() {
            for source in route.iter() {
                for stop in route.iter() {
                    if source != stop {
                        source_to_stops
                            .entry(*source)
                            .and_modify(|stops: &mut HashSet<i32>| {
                                stops.insert(*stop);
                            })
                            .or_insert(HashSet::from([*stop]));
                    }
                }
            }
        }

        source_to_stops
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_map_source_to_stops() {
        let mut map = HashMap::new();
        map.insert(1, HashSet::from([2, 7]));
        map.insert(2, HashSet::from([1, 7]));
        map.insert(3, HashSet::from([6, 7]));
        map.insert(6, HashSet::from([3, 7]));
        map.insert(7, HashSet::from([1, 2, 3, 6]));
        assert_eq!(
            map,
            Solution::map_source_to_stops(&vec![vec![1, 2, 7], vec![3, 6, 7]])
        );
    }

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
