struct Solution;

impl Solution {
    pub fn coin_change(mut coins: Vec<i32>, amount: i32) -> i32 {
        coins.sort_by(|a, b| b.cmp(a));
        match Self::dynamic(&coins[..], amount) {
            None => -1,
            Some(x) => x,
        }
    }

    pub fn dynamic(coins: &[i32], amount: i32) -> Option<i32> {
        if coins.len() == 0 {
            return None;
        }

        let mut num_coins = 0;
        let mut running_amount = amount;

        for coin in coins {
            let mult = running_amount / coin;

            num_coins += mult;
            running_amount -= coin * mult;
        }

        let answer = match Self::dynamic(&coins[1..], amount) {
            None => match running_amount {
                0 => Some(num_coins),
                _ => None,
            },
            Some(x) => match running_amount {
                0 => Some(num_coins.min(x)),
                _ => Some(x),
            },
        };
        #[cfg(test)]
        println!("{coins:?}: {num_coins}, {answer:?}");

        return answer;
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
