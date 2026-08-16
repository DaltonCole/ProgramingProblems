struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn coin_change(mut coins: Vec<i32>, amount: i32) -> i32 {
        coins.sort_by(|a, b| b.cmp(a));
        let mut remember_coins = HashMap::new();
        remember_coins.insert(amount, 0);

        Self::dynamic(amount, &coins, &mut remember_coins);

        *remember_coins.get(&0).unwrap_or(&-1)
    }

    // Memorization solution
    fn dynamic(amount_left: i32, coins: &Vec<i32>, remember_coins: &mut HashMap<i32, i32>) {
        if amount_left < 0 {
            return;
        }

        for coin in coins {
            // Number of coins required to get to amount_left
            let starting_coins = remember_coins.get(&amount_left).unwrap();

            // See if we have previously seen this new amount of money
            match remember_coins.get(&(amount_left - coin)) {
                // If we have previously gotten to this new coin amount before
                Some(found_coin_amount) => {
                    // If we can do better using this coin, check out this approach
                    if found_coin_amount > &(starting_coins + 1) {
                        remember_coins.insert(amount_left - coin, starting_coins + 1);
                        Self::dynamic(amount_left - coin, coins, remember_coins);
                    }
                }
                // We have never seen this amount before, explore
                None => {
                    remember_coins.insert(amount_left - coin, starting_coins + 1);
                    Self::dynamic(amount_left - coin, coins, remember_coins);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::coin_change(vec![2], 3), -1);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::coin_change(vec![1], 0), 0);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::coin_change(vec![186, 419, 83, 408], 6249), 20);
    }

    #[test]
    fn test5() {
        assert_eq!(
            Solution::coin_change(vec![205, 21, 66, 115, 396, 469, 202, 442, 364], 5563),
            13
        );
    }
}
