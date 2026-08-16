struct Solution;

impl Solution {
    pub fn coin_change(mut coins: Vec<i32>, amount: i32) -> i32 {
        coins.sort_by(|a, b| b.cmp(a));
        let mut remember_coins = vec![std::i32::MAX; amount as usize + 1];

        remember_coins[amount as usize] = 0;

        Self::dynamic(amount, &coins, &mut remember_coins);

        if remember_coins[0] == std::i32::MAX {
            -1
        } else {
            remember_coins[0]
        }
    }

    // Memorization solution
    fn dynamic(amount_left: i32, coins: &Vec<i32>, remember_coins: &mut Vec<i32>) {
        for coin in coins {
            if amount_left - coin < 0 {
                continue;
            }

            // Number of coins required to get to amount_left
            let starting_coins = remember_coins[amount_left as usize];

            let found_coin_amount = remember_coins[(amount_left - coin) as usize];

            // If we have previously gotten to this new coin amount before
            if found_coin_amount != std::i32::MAX {
                // If we can do better using this coin, check out this approach
                if found_coin_amount > starting_coins + 1 {
                    remember_coins[(amount_left - coin) as usize] = starting_coins + 1;
                    Self::dynamic(amount_left - coin, coins, remember_coins);
                }
            }
            // We have never seen this amount before, explore
            else {
                remember_coins[(amount_left - coin) as usize] = starting_coins + 1;
                Self::dynamic(amount_left - coin, coins, remember_coins);
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

    #[test]
    fn test6() {
        assert_eq!(Solution::coin_change(vec![1, 2147483647], 2), 2);
    }
}
