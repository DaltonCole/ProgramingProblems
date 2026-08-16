struct Solution;

const KNOWN_PRIMES: [usize; 25] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
];

//const BOUND: usize = 100_001;
const BOUND: usize = 100_001;

struct Primes {
    primes: Vec<usize>,
}

impl Primes {
    pub fn new() -> Primes {
        Primes {
            primes: vec![0; BOUND],
        }
    }

    pub fn update_prime(&mut self, num: usize) {
        let mut prime_index = 0;
        while prime_index < KNOWN_PRIMES.len() {
            if num % KNOWN_PRIMES[prime_index] == 0 {
                self.primes[num] |= 1 << prime_index;
            }

            prime_index += 1;
        }
    }

    pub fn prime(&mut self, num: usize) -> usize {
        if self.primes[num] == 0 {
            self.update_prime(num);
        }

        self.primes[num]
    }
}

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    pub fn new() -> UnionFind {
        let mut parent = vec![0; BOUND];

        for i in 0..BOUND {
            parent[i] = i;
        }

        UnionFind { parent }
    }

    /// Finds the representative node for this cluster
    pub fn find(&self, x: usize) -> usize {
        if self.parent[x] == x {
            return x;
        }

        self.find(self.parent[x])
    }

    /// Finds the representative node for this cluster and updates all nodes to other cluster
    fn update(&mut self, x: usize, set: usize) {
        if self.parent[x] != x {
            self.update(self.parent[x], set);
        }

        self.parent[x] = set;
    }

    /// Joins two clusters
    pub fn union(&mut self, x: usize, y: usize) {
        // Find x's representative
        let x_rep = self.find(x);

        // If not in the same cluster, update
        if x_rep != self.parent[y] {
            self.update(y, x_rep);
        }
    }

    pub fn one_cluster(&self, nums: &Vec<i32>) -> bool {
        for i in 1..nums.len() {
            if self.find(nums[i - 1] as usize) != self.find(nums[i] as usize) {
                return false;
            }
        }

        true
    }

    pub fn print(&self, nums: &Vec<i32>) {
        for &num in nums {
            println!("{}", self.parent[num as usize]);
        }
    }
}

impl Solution {
    pub fn can_traverse_all_pairs(nums: Vec<i32>) -> bool {
        let ones = nums.iter().filter(|&x| *x == 1).count();
        if ones > 1 || ones == 1 && nums.len() > 1 {
            return false;
        }

        let mut union_find = UnionFind::new();
        let mut primes = Primes::new();

        for num1 in 0..nums.len() {
            for num2 in (num1 + 1)..nums.len() {
                if primes.prime(nums[num1] as usize) & primes.prime(nums[num2] as usize) != 0 {
                    union_find.union(nums[num1] as usize, nums[num2] as usize);
                }
            }
        }

        dbg!(union_find.print(&nums));

        union_find.one_cluster(&nums)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
    #[test]
    fn test_primes() {
        assert_eq!(1, PRIMES[2]);
        assert_eq!(2, PRIMES[3]);
        assert_eq!(1, PRIMES[4]);
        assert_eq!(4, PRIMES[5]);
        assert_eq!(3, PRIMES[6]);
        assert_eq!(1 << 24 | 1, PRIMES[97 * 2]);
        assert_eq!(1 << 24 | 3, PRIMES[97 * 2 * 3]);
    }
    */

    #[test]
    fn test1() {
        assert_eq!(true, Solution::can_traverse_all_pairs(vec![2, 3, 6]));
    }

    #[test]
    fn test2() {
        assert_eq!(false, Solution::can_traverse_all_pairs(vec![3, 9, 5]));
    }

    #[test]
    fn test3() {
        assert_eq!(true, Solution::can_traverse_all_pairs(vec![4, 3, 12, 8]));
    }

    #[test]
    fn test4() {
        assert_eq!(
            true,
            Solution::can_traverse_all_pairs(vec![15, 15, 30, 7, 45, 30, 21])
        );
    }

    #[test]
    fn test5() {
        assert_eq!(false, Solution::can_traverse_all_pairs(vec![1, 1]));
    }

    #[test]
    fn test6() {
        assert_eq!(true, Solution::can_traverse_all_pairs(vec![1]));
    }

    #[test]
    fn test7() {
        assert_eq!(false, Solution::can_traverse_all_pairs(vec![1, 3]));
    }

    #[test]
    fn test8() {
        assert_eq!(true, Solution::can_traverse_all_pairs(vec![10007, 20014]));
    }
}
