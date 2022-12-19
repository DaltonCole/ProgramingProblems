use criterion::{black_box, criterion_group, criterion_main, Criterion};

use n_queens::n_queens::Solution;

fn n_queen(c: &mut Criterion) {
    for i in 1..10 {
        c.bench_function(&format!("{}-Queen", i), |n| {
            n.iter(|| Solution::solve_n_queens(black_box(i)))
        });
    }
}

criterion_group!(benches, n_queen);
criterion_main!(benches);
