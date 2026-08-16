struct Solution;

impl Solution {
    pub fn get_winner(mut arr: Vec<i32>, k: i32) -> i32 {
        if k as usize > arr.len() {
            return *arr.iter().max().unwrap();
        }
        Self::fast(&mut arr, k as usize)
        //Self::slow(&mut arr, k)
    }

    fn fast(arr: &mut Vec<i32>, k: usize) -> i32 {
        let mut current_index = 0;
        let mut next_index = 1;

        while current_index == 0 && next_index - current_index <= k && next_index < arr.len() {
            if arr[next_index] > arr[current_index] {
                current_index = next_index;
            }
            next_index += 1;
        }

        if current_index != 0 {
            while next_index - current_index < k && next_index < arr.len() {
                if arr[next_index] > arr[current_index] {
                    current_index = next_index;
                }
                next_index += 1;
            }
        }

        arr[current_index]
    }

    fn slow(arr: &mut Vec<i32>, k: i32) -> i32 {
        let mut wins = 0;

        while wins < k {
            if arr[0] < arr[1] {
                wins = 1;
                let tmp = arr.remove(0);
                arr.push(tmp);
            } else {
                wins += 1;
                let tmp = arr.remove(1);
                arr.push(tmp);
            }
        }
        arr[0]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(5, Solution::get_winner(vec![2, 1, 3, 5, 4, 6, 7], 2));
    }

    #[test]
    fn test2() {
        assert_eq!(3, Solution::get_winner(vec![3, 2, 1], 10));
    }
}
