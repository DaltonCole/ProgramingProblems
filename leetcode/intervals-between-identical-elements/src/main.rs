struct Solution;

// TODO: Clean up faster()
use std::collections::HashMap;

impl Solution {
    pub fn get_distances(arr: Vec<i32>) -> Vec<i64> {
        #[cfg(test)]
        println!("{arr:?}");
        //return Self::brute_force(&arr);
        //return Self::slow(&arr);
        return Self::faster(&arr);
    }

    fn faster(nums: &Vec<i32>) -> Vec<i64> {
        let mut result = vec![0; nums.len()];
        // num: (index, number of occurrences to this point)
        let mut map: HashMap<i32, (usize, i64)> = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            match map.get(&num) {
                // First occurrence of element:
                //   * Just insert index and 1 (first count)
                //   * No need to update result
                None => {
                    map.insert(num, (i, 1));
                }
                Some(&(map_index, count)) => {
                    let old_distance = result[map_index];
                    let distance_from_old_to_new = (i - map_index) as i64 * (count);
                    result[i] = old_distance + distance_from_old_to_new;
                    map.insert(num, (i, count + 1));
                }
            }
        }

        #[cfg(test)]
        {
            println!("{:?}", vec![0, 0, 0, 2, 4, 3, 5]);
            println!("{result:?}");
        }

        let mut result2 = vec![0; nums.len()];
        map.clear();
        for (i, &num) in nums.iter().enumerate().rev() {
            match map.get(&num) {
                // First occurrence of element:
                //   * Just insert index and 1 (first count)
                //   * No need to update result2
                None => {
                    map.insert(num, (i, 1));
                }
                Some(&(map_index, count)) => {
                    let old_distance = result2[map_index];
                    let distance_from_old_to_new = (map_index - i) as i64 * (count);
                    result2[i] = old_distance + distance_from_old_to_new;
                    map.insert(num, (i, count + 1));
                }
            }
        }

        #[cfg(test)]
        {
            println!("");
            println!("{:?}", vec![4, 2, 7, 0, 0, 1, 0]);
            println!("{result:?}");
        }

        for i in 0..result.len() {
            result[i] += result2[i];
        }

        result
    }

    // n*log(n) average case. n*n worst case
    fn slow(nums: &Vec<i32>) -> Vec<i64> {
        let mut result = vec![0; nums.len()];
        let mut pointer = Vec::with_capacity(nums.len());
        let mut map = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            match map.get(num) {
                None => pointer.push(i),
                Some(&x) => pointer.push(x),
            }
            map.insert(num, i);
        }

        #[cfg(test)]
        println!("{pointer:?}");

        for i in 0..pointer.len() {
            let mut p = i;

            #[cfg(test)]
            println!("{}: {}", i, nums[i]);

            while p != pointer[p] {
                #[cfg(test)]
                println!("{p} == {}", pointer[p]);
                p = pointer[p];

                let diff = (p as i64 - i as i64).abs();
                result[i] += diff;
                result[p] += diff;
            }
        }

        result
    }

    // n*n
    fn brute_force(nums: &Vec<i32>) -> Vec<i64> {
        let mut result = vec![0; nums.len()];

        for (i, num1) in nums.iter().enumerate() {
            let mut total = 0;
            for (j, num2) in nums.iter().enumerate() {
                if i != j && num1 == num2 {
                    total += (j as i64 - i as i64).abs()
                }
            }
            result[i] = total;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::get_distances(vec![2, 1, 3, 1, 2, 3, 3]),
            vec![4, 2, 7, 2, 4, 4, 5]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::get_distances(vec![10, 5, 10, 10]),
            vec![5, 0, 3, 4]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::get_distances(vec![]), vec![]);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::get_distances(vec![5]), vec![0]);
    }
}
