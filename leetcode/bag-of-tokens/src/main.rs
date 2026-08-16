struct Solution;

impl Solution {
    pub fn bag_of_tokens_score(mut tokens: Vec<i32>, mut power: i32) -> i32 {
        if tokens.is_empty() {
            return 0;
        }

        let mut score = 0;
        let mut max_score = 0;

        tokens.sort();

        let mut front: usize = 0;
        let mut back = tokens.len() - 1;

        while front <= back {
            // Face-up
            if power >= tokens[front] {
                power -= tokens[front];
                score += 1;
                front += 1;
                max_score = std::cmp::max(score, max_score);
            }
            // Face-down
            else if score >= 1 {
                power += tokens[back];
                score -= 1;
                back -= 1;
            } else {
                break;
            }
        }

        max_score
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test1() {
        assert_eq!(0, Solution::bag_of_tokens_score(vec![100], 50));
    }
    #[test]
    fn test2() {
        assert_eq!(1, Solution::bag_of_tokens_score(vec![200, 100], 150));
    }
    #[test]
    fn test3() {
        assert_eq!(
            2,
            Solution::bag_of_tokens_score(vec![100, 200, 300, 400], 200)
        );
    }
}
