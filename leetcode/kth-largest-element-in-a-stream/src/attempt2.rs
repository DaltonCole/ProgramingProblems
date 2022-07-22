use std::cmp::Reverse;

struct KthLargest {
    k: i32,
    nums: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {

    fn new(k: i32, mut nums: Vec<i32>) -> Self {
        // Sort vector in descending order
        nums.sort_by(|a, b| b.cmp(a));
        // Only keep k largest elements
        if nums.len() > k as usize {
            nums.drain(k as usize..);
        }
        KthLargest {
            k,
            nums,
        }
        
    }
    
    fn add(&mut self, val: i32) -> i32 {
        if self.nums.len() < self.k as usize {
            let position = self.nums.binary_search_by_key(&Reverse(val), |&num| Reverse(num)).unwrap_or_else(|e| e);
            self.nums.insert(position, val);
            return *self.nums.last().unwrap()

        }

        match val <= *self.nums.last().unwrap() {
            true => *self.nums.last().unwrap(),
            false => {
                self.nums.pop();
                let position = self.nums.binary_search_by_key(&Reverse(val), |&num| Reverse(num)).unwrap_or_else(|e| e);
                self.nums.insert(position, val);
                *self.nums.last().unwrap()
            }
            
        }
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut x = KthLargest::new(3, [4, 5, 8, 3].to_vec());
        assert_eq!(4, x.add(3));
        assert_eq!(5, x.add(5));
        assert_eq!(5, x.add(10));
        assert_eq!(8, x.add(9));
        assert_eq!(8, x.add(4));
    }

    #[test]
    fn test2() {
        let mut x = KthLargest::new(3, [5, 8].to_vec());
        assert_eq!(4, x.add(4));
        assert_eq!(5, x.add(5));
        assert_eq!(5, x.add(10));
        assert_eq!(8, x.add(9));
        assert_eq!(8, x.add(4));
    }
}
