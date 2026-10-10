use criterion::{
	Criterion,
	criterion_group,
	criterion_main
};

use core::hint::black_box;

criterion_group!(arithmetic, benchmark);
criterion_main!(arithmetic);

fn benchmark(c: &mut Criterion) {
	use num_traits::FromPrimitive as _;
	use num_traits::ToPrimitive as _;
	use spectrum::{FixedPointValue, With};

	let mut group: criterion::BenchmarkGroup<_> = c.benchmark_group("arithmetic");

	let lhs: FixedPointValue<_> = FixedPointValue::<With<2, u64>>::from_u64(20_00).unwrap();
	let rhs: FixedPointValue<_> = FixedPointValue::<With<2, u64>>::from_u64(20_00).unwrap();

	group.bench_function("reference_add_f66", |b| {
		b.iter(|| black_box(20.0) + black_box(20.0))
	});

	group.bench_function("reference_sub_f64", |b| {
		b.iter(|| black_box(20.0) - black_box(20.0))
	});

	group.bench_function("reference_mul_f64", |b| {
		b.iter(|| black_box(20.0) * black_box(20.0))
	});

	group.bench_function("reference_div_f64", |b| {
		b.iter(|| black_box(20.0) / black_box(20.0))
	});

	group.bench_function("add", |b| {
		b.iter(|| black_box(lhs) + black_box(rhs))
	});

	group.bench_function("sub", |b| {
		b.iter(|| black_box(lhs) - black_box(rhs))
	});

	group.bench_function("mul", |b| {
		b.iter(|| black_box(lhs) * black_box(rhs))
	});

	group.bench_function("div", |b| {
		b.iter(|| black_box(lhs) / black_box(rhs))
	});

	group.bench_function("to_u8", |b| b.iter(|| black_box(lhs).to_u8()));
	group.bench_function("to_u16", |b| b.iter(|| black_box(lhs).to_u16()));
	group.bench_function("to_u32", |b| b.iter(|| black_box(lhs).to_u32()));
	group.bench_function("to_u64", |b| b.iter(|| black_box(lhs).to_u64()));
	group.bench_function("to_u128", |b| b.iter(|| black_box(lhs).to_u128()));
	group.bench_function("to_usize", |b| b.iter(|| black_box(lhs).to_usize()));
	group.bench_function("to_i8", |b| b.iter(|| black_box(lhs).to_i8()));
	group.bench_function("to_i16", |b| b.iter(|| black_box(lhs).to_i16()));
	group.bench_function("to_i32", |b| b.iter(|| black_box(lhs).to_i32()));
	group.bench_function("to_i64", |b| b.iter(|| black_box(lhs).to_i64()));
	group.bench_function("to_i128", |b| b.iter(|| black_box(lhs).to_i128()));
	group.bench_function("to_isize", |b| b.iter(|| black_box(lhs).to_isize()));
	group.bench_function("to_f32", |b| b.iter(|| black_box(lhs).to_f64()));
	group.bench_function("to_f64", |b| b.iter(|| black_box(lhs).to_f64()));

	group.finish();
}
