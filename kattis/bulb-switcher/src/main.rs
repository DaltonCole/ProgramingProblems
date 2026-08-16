struct Solution;

impl Solution {
    pub fn bulb_switch(n: i32) -> i32 {
        //return Solution::slow(n as usize);
        //return Solution::fast(n);
        return Solution::fastest(n);
    }

    fn slow(n: usize) -> i32 {
        let mut lights = vec![false; n];

        for i in 1..=n {
            for j in i..=n {
                if j % i == 0 {
                    lights[j - 1] = !lights[j - 1]
                }
            }
        }

        lights.iter().filter(|&x| *x).count().try_into().unwrap()
    }

    fn fast(n: i32) -> i32 {
        let mut total = 0;
        let mut running = 0;
        let mut i = 1;

        while running < n {
            total += 1;
            i += 2;
            running += i;
        }
        total
    }

    fn fastest(n: i32) -> i32 {
        (n as f64).sqrt() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests() {
        let answers = [
            0, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5,
        ];
        // 0: 0
        // 1-3, 3: 1
        // 4-8, 5: 2
        // 9-15, 7: 3

        for (i, answer) in answers.iter().enumerate() {
            println!("n: {i}");
            assert_eq!(*answer, Solution::bulb_switch(i as i32));
        }
    }
}
