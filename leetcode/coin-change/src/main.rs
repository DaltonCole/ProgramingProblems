struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let max = amount + 1;
        let mut remember_coins = vec![max; amount as usize + 1];
        remember_coins[0] = 0;

        for partial_amount in 1..=amount {
            for &coin in coins.iter() {
                if coin <= partial_amount {
                    remember_coins[partial_amount as usize] = std::cmp::min(
                        remember_coins[partial_amount as usize],
                        remember_coins[(partial_amount - coin) as usize] + 1,
                    );
                }
            }
        }

        if remember_coins[amount as usize] >= max {
            -1
        } else {
            remember_coins[amount as usize]
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
