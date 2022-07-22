use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct MedianFinder {
    large_num_min_heap: BinaryHeap<Reverse<i32>>,
    // Max heap keeps the larger half of the numbers
    // Should always be the same size as min_heap or larger
    small_num_max_heap: BinaryHeap<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    fn new() -> Self {
        MedianFinder {
            large_num_min_heap: BinaryHeap::new(),
            small_num_max_heap: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        // If min heap is empty, just put it in max heap
        if self.small_num_max_heap.len() == 0 {
            self.large_num_min_heap.push(Reverse(num));
        } else {
            // Determine which heap num belongs in
            if *self.small_num_max_heap.peek().unwrap() > num {
                self.small_num_max_heap.push(num);
            } else {
                self.large_num_min_heap.push(Reverse(num));
            }
        }

        // --- Make sure max heap is the same size or 1 larger than min heap --- //
        // Small is larger than large
        if self.small_num_max_heap.len() > self.large_num_min_heap.len() {
            self.large_num_min_heap
                .push(Reverse(self.small_num_max_heap.pop().unwrap()));
        }
        // Large is two larger than small
        if self.small_num_max_heap.len() + 2 == self.large_num_min_heap.len() {
            self.small_num_max_heap
                .push(self.large_num_min_heap.pop().unwrap().0);
        }
    }

    fn find_median(&self) -> f64 {
        // Large size is larger than small
        if self.large_num_min_heap.len() == self.small_num_max_heap.len() + 1 {
            self.large_num_min_heap.peek().unwrap().0 as f64
        } else {
            (self.large_num_min_heap.peek().unwrap().0 as f64
                + *self.small_num_max_heap.peek().unwrap() as f64)
                / 2.0
        }
    }

    #[allow(dead_code)]
    fn debug(&self) {
        println!(
            "{} {}",
            self.small_num_max_heap.peek().unwrap_or(&-999),
            self.large_num_min_heap.peek().unwrap_or(&Reverse(-999)).0
        );
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_eq;

    #[test]
    fn test1() {
        let mut med = MedianFinder::new();
        med.add_num(1);
        med.add_num(2);
        assert!(approx_eq!(f64, 1.5, med.find_median(), epsilon = 0.000001));
        med.add_num(3);
        assert!(approx_eq!(f64, 2.0, med.find_median(), epsilon = 0.000001));
    }

    #[test]
    fn test2() {
        let mut med = MedianFinder::new();
        med.add_num(-1);
        assert!(approx_eq!(f64, -1.0, med.find_median(), epsilon = 0.000001));
        med.add_num(-2);
        assert!(approx_eq!(f64, -1.5, med.find_median(), epsilon = 0.000001));
        med.add_num(-3);
        assert!(approx_eq!(f64, -2.0, med.find_median(), epsilon = 0.000001));
        med.add_num(-4);
        assert!(approx_eq!(f64, -2.5, med.find_median(), epsilon = 0.000001));
        med.add_num(-5);
        assert!(approx_eq!(f64, -3.0, med.find_median(), epsilon = 0.000001));
    }

    #[test]
    fn test3() {
        let mut med = MedianFinder::new();
        med.add_num(1);
        assert!(approx_eq!(f64, 1.0, med.find_median(), epsilon = 0.000001));
        med.add_num(2);
        assert!(approx_eq!(f64, 1.5, med.find_median(), epsilon = 0.000001));
        med.add_num(3);
        assert!(approx_eq!(f64, 2.0, med.find_median(), epsilon = 0.000001));
        med.add_num(4);
        assert!(approx_eq!(f64, 2.5, med.find_median(), epsilon = 0.000001));
        med.add_num(5);
        assert!(approx_eq!(f64, 3.0, med.find_median(), epsilon = 0.000001));
    }
}
