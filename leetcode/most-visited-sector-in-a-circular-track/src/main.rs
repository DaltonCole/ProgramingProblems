struct Solution;

impl Solution {
    pub fn most_visited(n: i32, rounds: Vec<i32>) -> Vec<i32> {
        // Running total of visited squares
        let mut totals = vec![0; n as usize];

        // Start
        // 0 index instead of 1 index
        let mut previous = rounds[0] - 1;
        // Go to next space
        for &current in rounds.iter().skip(1) {
            // While we're not at the finish square
            while previous != current - 1 {
                // Add to visited list
                totals[previous as usize] += 1;
                // Go to next space, keeping track of finishing the circle
                previous = (previous + 1) % n;
            }
        }
        // Account for finish square
        totals[previous as usize] += 1;

        // Get number of visits from the most visited location
        let max_val = *totals.iter().max().unwrap();

        // Get index of all most visited spaces. Go back to 1-indexing
        totals
            .iter()
            .enumerate()
            .filter(|(i, x)| **x == max_val)
            .map(|(i, x)| i as i32 + 1)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(vec![1, 2], Solution::most_visited(4, vec![1, 3, 1, 2]));
    }

    #[test]
    fn test2() {
        assert_eq!(
            vec![2],
            Solution::most_visited(2, vec![2, 1, 2, 1, 2, 1, 2, 1, 2])
        );
    }
    #[test]
    fn test3() {
        assert_eq!(
            vec![1, 2, 3, 4, 5, 6, 7],
            Solution::most_visited(7, vec![1, 3, 5, 7])
        );
    }
}
