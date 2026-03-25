mod common;

use common::{LARGE, MEDIUM, SMALL, TINY};
use criterion::measurement::WallTime;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkGroup, Criterion};

trait Rope {
    fn from_str(s: &str) -> Self;
}

trait RopeBuilder {
    type Rope: Rope;

    fn new() -> Self;
    fn append(self, s: &str) -> Self;
    fn build(self) -> Self::Rope;
}

impl Rope for crop::Rope {
    #[inline]
    fn from_str(s: &str) -> Self {
        Self::from(s)
    }
}

impl Rope for jumprope::JumpRope {
    #[inline]
    fn from_str(s: &str) -> Self {
        Self::from(s)
    }
}

impl Rope for ropey::Rope {
    #[inline]
    fn from_str(s: &str) -> Self {
        Self::from_str(s)
    }
}

impl Rope for xi_rope::Rope {
    #[inline]
    fn from_str(s: &str) -> Self {
        Self::from(s)
    }
}

impl Rope for zed_rope::Rope {
    #[inline]
    fn from_str(s: &str) -> Self {
        Self::from(s)
    }
}

impl RopeBuilder for crop::RopeBuilder {
    type Rope = crop::Rope;

    #[inline]
    fn new() -> Self {
        crop::RopeBuilder::new()
    }

    #[inline]
    fn append(mut self, s: &str) -> Self {
        crop::RopeBuilder::append(&mut self, s);
        self
    }

    #[inline]
    fn build(self) -> Self::Rope {
        self.build()
    }
}

impl RopeBuilder for ropey::RopeBuilder {
    type Rope = ropey::Rope;

    #[inline]
    fn new() -> Self {
        ropey::RopeBuilder::new()
    }

    #[inline]
    fn append(mut self, s: &str) -> Self {
        ropey::RopeBuilder::append(&mut self, s);
        self
    }

    #[inline]
    fn build(self) -> Self::Rope {
        self.finish()
    }
}

type XiRopeBuilder = xi_rope::tree::TreeBuilder<xi_rope::RopeInfo>;

impl RopeBuilder for XiRopeBuilder {
    type Rope = xi_rope::Rope;

    #[inline]
    fn new() -> Self {
        XiRopeBuilder::new()
    }

    #[inline]
    fn append(mut self, s: &str) -> Self {
        self.push_str(s);
        self
    }

    #[inline]
    fn build(self) -> Self::Rope {
        self.build()
    }
}

fn bench<F: Fn(&str)>(group: &mut BenchmarkGroup<WallTime>, to_bench: F) {
    group.bench_function("tiny", |bench| bench.iter(|| to_bench(TINY)));
    group.bench_function("small", |bench| bench.iter(|| to_bench(SMALL)));
    group.bench_function("medium", |bench| bench.iter(|| to_bench(MEDIUM)));
    group.bench_function("large", |bench| bench.iter(|| to_bench(LARGE)));
}

#[inline]
fn from_str<R: Rope>(s: &str) {
    let _ = R::from_str(black_box(s));
}

#[inline]
fn rope_builder<B: RopeBuilder>(s: &str) {
    let mut b = B::new();
    for line in s.lines() {
        b = b.append(line);
    }
    let _ = b.build();
}

fn crop_from_str(c: &mut Criterion) {
    let mut group = c.benchmark_group("crop_from_str");
    bench(&mut group, from_str::<crop::Rope>);
}

fn jumprope_from_str(c: &mut Criterion) {
    let mut group = c.benchmark_group("jumprope_from_str");
    bench(&mut group, from_str::<jumprope::JumpRope>);
}

fn ropey_from_str(c: &mut Criterion) {
    let mut group = c.benchmark_group("ropey_from_str");
    bench(&mut group, from_str::<ropey::Rope>);
}

fn xi_rope_from_str(c: &mut Criterion) {
    let mut group = c.benchmark_group("xi_rope_from_str");
    bench(&mut group, from_str::<xi_rope::Rope>);
}

fn zed_rope_from_str(c: &mut Criterion) {
    let mut group = c.benchmark_group("zed_rope_from_str");
    bench(&mut group, from_str::<zed_rope::Rope>);
}

fn crop_builder(c: &mut Criterion) {
    let mut group = c.benchmark_group("crop_builder");
    bench(&mut group, rope_builder::<crop::RopeBuilder>);
}

fn ropey_builder(c: &mut Criterion) {
    let mut group = c.benchmark_group("ropey_builder");
    bench(&mut group, rope_builder::<ropey::RopeBuilder>);
}

fn xi_rope_builder(c: &mut Criterion) {
    let mut group = c.benchmark_group("xi_rope_builder");
    bench(&mut group, rope_builder::<XiRopeBuilder>);
}

criterion_group!(
    benches,
    crop_from_str,
    jumprope_from_str,
    ropey_from_str,
    xi_rope_from_str,
    zed_rope_from_str,
    crop_builder,
    ropey_builder,
    xi_rope_builder,
);

criterion_main!(benches);
