use criterion::{black_box, criterion_group, criterion_main, Criterion};

extern crate brilliance;
use brilliance::*;

//fn boundingbox_intersects(c: &mut Criterion) {
//	let bb = BoundingBox::new(Pointf(0., 0., 0.), Pointf(1., 2., 3.));
//	let r = Ray::new(Point(1., 1., 1.), Vector(1., 0., 0.));
//
//	c.bench_function("boundingbox_intersects", |b| {
//		b.iter(|| black_box(&bb).intersects(black_box(&r)))
//	});
//}

fn boundingbox4_intersects(c: &mut Criterion) {
	let bba = BoundingBox::new(Pointf(0., 0., 0.), Pointf(1., 2., 3.));
	let bbb = BoundingBox::new(Pointf(0., 0., 0.), Pointf(1., 2., 3.));
	let bb4 = BoundingBox4::new([&bba, &bbb, &bba, &bbb]);

	let r = Ray::new(Point(1., 1., 1.), Vector(1., 0., 0.));

	c.bench_function("boundingbox4_intersects", |b| {
		b.iter(|| black_box(&bb4).intersects_slow(black_box(&r), INF32))
	});
}

fn boundingbox4_intersects_fast(c: &mut Criterion) {
	let bba = BoundingBox::new(Pointf(0., 0., 0.), Pointf(1., 2., 3.));
	let bbb = BoundingBox::new(Pointf(0., 0., 0.), Pointf(1., 2., 3.));
	let bb4 = BoundingBox4::new([&bba, &bbb, &bba, &bbb]);

	let r = Ray::new(Point(1., 1., 1.), Vector(1., 0., 0.));
	let start = Vectorf::from(r.start);
	let invdir = Vectorf::from(r.dir).inv();

	c.bench_function("boundingbox4_intersects_fast", |b| {
		b.iter(|| black_box(&bb4).intersects_fast(black_box(start), invdir, INF32))
	});
}

// fn triangle_intersects_hit(c: &mut Criterion) {
// 	let t = Triangle::new(Point(1., 2., -1.), Point(3., 2., -1.), Point(3., 4., -1.));
//
// 	let r = Ray(Point(2., 3., 0.), -Vector::EZ);
//
// 	c.bench_function("triangle_intersects_hit", |b| {
// 		b.iter(|| black_box(&t).intersects(black_box(&r)))
// 	});
// }
//
// fn triangle_intersects_early_miss(c: &mut Criterion) {
// 	let t = Triangle::new(Point(1., 2., -1.), Point(3., 2., -1.), Point(3., 4., -1.));
//
// 	let r = Ray(Point(2., 3., 0.), Vector::EZ);
//
// 	c.bench_function("triangle_intersects_early_miss", |b| {
// 		b.iter(|| black_box(&t).intersects(black_box(&r)))
// 	});
// }
//
// fn triangle4_intersects_early_miss(c: &mut Criterion) {
// 	let t = Triangle::new(Point(1., 2., -1.), Point(3., 2., -1.), Point(3., 4., -1.));
//
// 	let t4 = Triangle4::new([&t, &t, &t, &t]);
// 	let r = Ray(Point(2., 3., 0.), Vector::EZ);
//
// 	c.bench_function("triangle4_intersects_early_miss", |b| {
// 		b.iter(|| black_box(&t4).intersects(black_box(&r)))
// 	});
// }
//
// fn triangle4_intersects_hit(c: &mut Criterion) {
// 	let t = Triangle::new(Point(1., 2., -1.), Point(3., 2., -1.), Point(3., 4., -1.));
//
// 	let t4 = Triangle4::new([&t, &t, &t, &t]);
// 	let r = Ray(Point(2., 3., 0.), -Vector::EZ);
//
// 	c.bench_function("triangle4_intersects_hit", |b| {
// 		b.iter(|| black_box(&t4).intersects(black_box(&r)))
// 	});
// }

criterion_group!(benches, boundingbox4_intersects, boundingbox4_intersects_fast);
criterion_main!(benches);
