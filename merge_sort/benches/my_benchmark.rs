use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use merge_sort::{merge_seq, merge_par};
use rand::Rng;

fn bench_sort(c: &mut Criterion) {
    let mut group = c.benchmark_group("Merge sort");
    let mut n = 10;
    while n < 1000 {
        // random input
        //let mut v = (0..n).map(|_| rand::thread_rng().gen_range(0, 100)).collect();

        // reverse input
        let v: Vec<i32> = (0..n).rev().collect();

        group.bench_with_input(BenchmarkId::new("sequential", n), &n,
            |b, _| b.iter(|| merge_seq(v.clone())));
        group.bench_with_input(BenchmarkId::new("parallel", n), &n,
            |b, _| b.iter(|| merge_par(v.clone())));
        n = n*2;
    }
    group.finish();
}

criterion_group!(benches, bench_sort);
criterion_main!(benches);

/*
fn criterion_benchmark(c: &mut Criterion) {
    println!("data: {:?}",v);
    c.bench_function("merge_iter rand", |b| b.iter(|| merge_seq(black_box(&mut v))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
*/
