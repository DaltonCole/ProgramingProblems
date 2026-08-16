struct Solution;

impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        let mut count = 0;
        g.sort();
        s.sort();

        for i in 0..s.len() {
            if s[i] >= g[count] {
                count += 1;
                if count >= g.len() {
                    break;
                }
            }
        }
        count as i32
    }

    pub fn find_content_children2(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        let mut count = 0;

        g.sort_by_key(|x| std::cmp::Reverse(*x));
        s.sort_by_key(|x| std::cmp::Reverse(*x));

        while !g.is_empty() && !s.is_empty() {
            println!("g: {}, s: {}", g.last().unwrap(), s.last().unwrap());
            if s.last().unwrap() >= g.last().unwrap() {
                count += 1;
                s.pop();
                g.pop();
            } else {
                s.pop();
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            1,
            Solution::find_content_children(vec![1, 2, 3], vec![1, 1])
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            2,
            Solution::find_content_children(vec![1, 2], vec![1, 2, 3])
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            2,
            Solution::find_content_children(vec![10, 9, 8, 7], vec![5, 6, 7, 8])
        );
    }
}
