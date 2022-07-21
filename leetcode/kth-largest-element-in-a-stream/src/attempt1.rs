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
        nums.sort();
        KthLargest {
            k,
            nums,
        }
        
    }
    
    fn add(&mut self, val: i32) -> i32 {
        let position = self.nums.binary_search(&val).unwrap_or_else(|e| e);
        self.nums.insert(position, val);

        self.nums[self.nums.len() - self.k as usize]
        
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
}
