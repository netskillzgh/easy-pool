use std::sync::Arc;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use easy_pool::{Clear, PoolArrayQueue, PoolMutex, PoolSegQueue};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

struct Test {}

impl Default for Test {
    fn default() -> Self {
        Test {}
    }
}

impl Clear for Test {
    fn clear(&mut self) {}
}

fn mutex(pool: Arc<PoolMutex<Vec<u8>>>) {
    (0..1024).into_par_iter().for_each(|_| {
        let pool = pool.clone();
        let vec = pool.create_with(|| Vec::with_capacity(1024));
        drop(vec);
    });

    (0..1024).into_par_iter().for_each(|_| {
        let pool = pool.clone();
        let vec = pool.create_with(|| Vec::with_capacity(1024));
        drop(vec);
    });
}

fn seg_queue(pool: Arc<PoolSegQueue<Vec<u8>>>) {
    (0..1024).into_par_iter().for_each(|_| {
        let pool = pool.clone();
        let vec = pool.create_with(|| Vec::with_capacity(1024));
        drop(vec);
    });

    (0..1024).into_par_iter().for_each(|_| {
        let pool = pool.clone();
        let vec = pool.create_with(|| Vec::with_capacity(1024));
        drop(vec);
    });
}

fn array_queue(pool: Arc<PoolArrayQueue<Vec<u8>>>) {
    (0..1024).into_par_iter().for_each(|_| {
        let pool = pool.clone();
        let vec = pool.create_with(|| Vec::with_capacity(1024));
        drop(vec);
    });

    (0..1024).into_par_iter().for_each(|_| {
        let pool = pool.clone();
        let vec = pool.create_with(|| Vec::with_capacity(1024));
        drop(vec);
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    let pool = Arc::new(PoolMutex::with_config(1024, 1024));
    c.bench_function("mutex", |b| b.iter(|| mutex(black_box(pool.clone()))));

    let pool = Arc::new(PoolSegQueue::new(1024));
    c.bench_function("seg_queue", |b| {
        b.iter(|| seg_queue(black_box(pool.clone())))
    });

    let pool = Arc::new(PoolArrayQueue::new(1024));
    c.bench_function("array_queue", |b| {
        b.iter(|| array_queue(black_box(pool.clone())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
