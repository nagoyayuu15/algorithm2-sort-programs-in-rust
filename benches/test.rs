use std::time::Duration;

use homework::combsort;
use homework::heapsort;
use homework::mergesort;
use homework::quicksort;
use homework::shellsort;

use rand::{distributions::Uniform, Rng};
use criterion::*;

// ベンチマーク: https://zenn.dev/termoshtt/books/b4bce1b9ea5e6853cb07/viewer/criterion

fn random_vec(length: u32) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let range = Uniform::new(0, length);
    (0..length).map(|_| rng.sample(&range)).collect()
}

fn bench_template(c: &mut criterion::Criterion, group_name: &str, sort_func: fn(&mut Vec<u32>)) {
    let mut group = c.benchmark_group("sort");
    group.measurement_time(Duration::new(5, 0));
    for n in 1..6 {
        let size = 10u32.pow(n);
        group.bench_with_input(BenchmarkId::new(group_name, size), &size, |b, &size| 
            b.iter_batched(|| random_vec(size), |mut v| sort_func(&mut v), BatchSize::SmallInput)
        );
    }
    group.finish();
}

fn comb_bench(c: &mut criterion::Criterion) {
    bench_template(c, "combsort", combsort::combsort);
}

fn heap_bench(c: &mut criterion::Criterion) {
    bench_template(c, "heapsort", heapsort::heapsort);
}

fn merge_bench(c: &mut criterion::Criterion) {
    bench_template(c, "mergesort", mergesort::mergesort);
}

fn quick_bench(c: &mut criterion::Criterion) {
    bench_template(c, "quicksort", quicksort::quicksort);
}

fn shell_bench(c: &mut criterion::Criterion) {
    bench_template(c, "shellsort", shellsort::shellsort);
}

// ベンチマークグループを定義する
criterion_group!(benches, comb_bench, heap_bench, merge_bench, quick_bench, shell_bench);

// main関数を用意
criterion_main!(benches);