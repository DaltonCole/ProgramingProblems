struct Solution;

impl Solution {
    pub fn get_good_indices(variables: Vec<Vec<i32>>, target: i32) -> Vec<i32> {
        let mut answer = Vec::new();

        for (index, &ref group) in variables.iter().enumerate() {
            let a = group[0] % 10;
            let b = group[1];
            let c = group[2];
            let d = group[3];

            let mut total = a;

            for _ in 1..b {
                total *= a;
                total %= 10;
            }
            total %= 10;

            let mut final_total = total;

            for _ in 1..c {
                final_total *= total;
                final_total %= d;
            }
            final_total %= d;

            if final_total == target {
                answer.push(index as i32);
            }
        }

        answer
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            vec![0, 2],
            Solution::get_good_indices(
                vec![vec![2, 3, 3, 10], vec![3, 3, 3, 1], vec![6, 1, 1, 4]],
                2
            )
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            Vec::<i32>::new(),
            Solution::get_good_indices(vec![vec![39, 3, 1000, 1000]], 17)
        );
    }
}
