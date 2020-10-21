use criterion::{black_box, criterion_group, criterion_main, Criterion};
use merge_sort::{merge_seq, merge_parallel};
use rand::Rng;

fn criterion_benchmark(c: &mut Criterion) {
    let mut v = (0..30).map(|_| rand::thread_rng().gen_range(0, 100)).collect();
    println!("data: {:?}",v);
    c.bench_function("merge_iter rand", |b| b.iter(|| merge_seq(black_box(&mut v))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
