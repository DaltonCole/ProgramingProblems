struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, mut cost: Vec<i32>) -> i32 {
        for i in 0..cost.len() {
            cost[i] = gas[i] - cost[i];
        }

        if cost.iter().sum::<i32>() < 0 {
            return -1;
        }

        let mut total = 0;
        let mut index = 0;

        for i in 0..cost.len() {
            total += cost[i];
            if total < 0 {
                index = i + 1;
                total = 0;
            }
        }

        index as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            3,
            Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2])
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            -1,
            Solution::can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3])
        );
    }
}
