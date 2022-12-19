struct Solution;

impl Solution {
    pub fn coin_change(mut coins: Vec<i32>, mut amount: i32) -> i32 {
        let mut num_coins = 0;

        coins.sort();

        for coin in coins.iter().rev() {
            let mult = amount / coin;

            #[cfg(test)]
            println!("{mult}");

            num_coins += mult;
            amount -= coin * mult;
        }

        match amount {
            0 => num_coins,
            _ => -1,
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
}
