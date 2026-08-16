struct Solution;

impl Solution {
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        //Self::slow(&stones, 0)
        Self::fast(&stones)
        //Self::short(&stones)
    }

    fn slow(stones: &[i32], total: i32) -> i32 {
        if stones.len() == 0 {
            return total;
        }

        let include = stones.last().unwrap() + total;
        let uninclude = (-stones.last().unwrap() + total).abs();

        let new_len = stones.len() - 1;
        return std::cmp::min(
            Self::slow(&stones[0..new_len], include),
            Self::slow(&stones[0..new_len], uninclude),
        );
    }

    fn short(stones: &[i32]) -> i32 {
        use std::collections::HashSet;

        let mut set = HashSet::new();

        set.insert(stones[0]);
        set.insert(-stones[0]);

        stones.iter().skip(1).for_each(|x| {
            let mut new_set = HashSet::new();
            for ele in &set {
                new_set.insert(ele + x);
                new_set.insert(ele - x);
            }
            set = new_set;
        });
        set.iter().map(|x| x.abs()).min().unwrap()
    }

    fn fast(stones: &[i32]) -> i32 {
        let total_weight: i32 = stones.iter().sum();
        let half_weight: i32 = (total_weight + 1) / 2;
        let smaller_half = Self::knapsack(half_weight as usize, &stones, &stones);
        let larger_half = total_weight - smaller_half;
        (larger_half - smaller_half).abs()
    }

    fn knapsack(total_weight: usize, weights: &[i32], values: &[i32]) -> i32 {
        let mut prev_row = vec![0; total_weight + 1];
        let mut row = vec![0; total_weight + 1];

        for (&weight, &value) in weights.iter().zip(values) {
            for col in 1..(total_weight + 1) {
                let dont_include = prev_row[col];
                let mut include = 0;

                if col >= weight as usize {
                    include = value + prev_row[col - weight as usize];
                }

                row[col] = std::cmp::max(include, dont_include);
            }
            prev_row = row;
            row = vec![0; total_weight + 1];
        }
        *prev_row.last().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(1, Solution::last_stone_weight_ii(vec![2, 7, 4, 1, 8, 1]));
    }
    #[test]
    fn test2() {
        assert_eq!(5, Solution::last_stone_weight_ii(vec![31, 26, 33, 21, 40]));
    }
    #[test]
    fn test3() {
        assert_eq!(
            0,
            Solution::last_stone_weight_ii(vec![
                89, 23, 100, 93, 82, 98, 91, 85, 33, 95, 72, 98, 63, 46, 17, 91, 92, 72, 77, 79,
                99, 96, 55, 72, 24, 98, 79, 93, 88, 92
            ])
        );
    }
}
