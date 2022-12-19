use criterion::{black_box, criterion_group, criterion_main, Criterion};

use sum4::sum4::Solution;

/*
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
*/

fn criterion_benchmark(c: &mut Criterion) {
    //c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
    c.bench_function("Test0", |b| {
        b.iter(|| Solution::four_sum(black_box(vec![1, 0, -1, 0, -2, 2]), 100))
    });
    c.bench_function("Test1", |b| {
        b.iter(|| Solution::four_sum(black_box(vec![1, 0, -1, 0, -2, 2]), 0))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

/*
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test0() {
        answer.sort();
        assert_eq!(answer, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test1() {
        let mut answer = Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0);
        answer.sort();
        assert_eq!(
            answer,
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]
        );
    }

    #[test]
    fn test2() {
        let mut answer = Solution::four_sum(vec![2, 2, 2, 2, 2], 8);
        answer.sort();
        assert_eq!(answer, vec![vec![2, 2, 2, 2]]);
    }

    #[test]
    fn test3() {
        let mut answer = Solution::four_sum(vec![], 0);
        answer.sort();
        assert_eq!(answer, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test4() {
        let mut answer = Solution::four_sum(
            vec![1000000000, 1000000000, 1000000000, 1000000000],
            -294967296,
        );
        answer.sort();
        assert_eq!(answer, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test5() {
        let mut answer = Solution::four_sum(vec![-2, -1, -1, 1, 1, 2, 2], 0);
        answer.sort();
        assert_eq!(answer, vec![vec![-2, -1, 1, 2], vec![-1, -1, 1, 1]]);
    }

    #[test]
    fn test6() {
        let mut answer = Solution::four_sum(vec![-5, -2, -4, -2, -5, -4, 0, 0], -13);
        answer.sort();
        assert_eq!(answer, vec![vec![-5, -4, -4, 0], vec![-5, -4, -2, -2]]);
    }
}
*/
