use std::collections::HashMap;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut mem = HashMap::new();
        Solution::memory(n, &mut mem)
    }

    fn memory(n: i32, mem: &mut HashMap<i32, i32>) -> i32 {
        match n {
            0 => 0,
            1 => 1,
            2 => 2,
            _ => match mem.get(&n) {
                Some(&x) => x,
                None => {
                    let value = Solution::memory(n - 1, mem) + Solution::memory(n - 2, mem);
                    mem.insert(n, value);
                    *(mem.get(&n).unwrap())
                }
            },
        }
    }
}
