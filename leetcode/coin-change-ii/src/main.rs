struct Solution;

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let mut previous_combinations = vec![0; (amount + 1) as usize];
        previous_combinations[0] = 1;

        // Start by adding each coin individually. This prevents the case of 1,2 and 2,1 being
        // counted twice
        for &coin in coins.iter() {
            // Add this coin for each sub-amount all the way up to amount
            for sub_amount in coin..=amount {
                previous_combinations[sub_amount as usize] +=
                    previous_combinations[(sub_amount - coin) as usize];
            }
        }

        previous_combinations[amount as usize]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        // 0: 1
        // 1: 1 - 1
        // 2: 2 - 1,1; 2
        // 3: 2 - 1,1,1; 1,2
        // 4: 3 - 1,1,1,1; 1,1,2; 2,2
        // 5: 4 - 1,1,1,1,1; 1,1,1,2; 1,2,2; 5
        assert_eq!(4, Solution::change(5, vec![1, 2, 5]));
    }

    #[test]
    fn test2() {
        assert_eq!(0, Solution::change(3, vec![2]));
    }
    #[test]
    fn test3() {
        assert_eq!(1, Solution::change(10, vec![10]));
    }
}
