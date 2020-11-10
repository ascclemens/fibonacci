use criterion::{black_box, criterion_group, criterion_main, Criterion};

use fibonacci::Fibonacci;

// FIXME(Krey): Process fib on system limit
fn criterion_benchmark(c: &mut Criterion) -> () {
	for &n in [1,2,5,10,25,50,75].iter() {
		c.bench_function(&format!("Fibonacci {}", n),|b| b.iter(|| Fibonacci::<usize>::default().nth(black_box(n)).unwrap()));
	}
}

criterion_group! {
	name = benches;
	config = Criterion::default().sample_size(500);
	targets = criterion_benchmark
}

criterion_main!(benches);