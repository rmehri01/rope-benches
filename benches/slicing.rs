use std::ops::Range;

mod common;

use common::{PercentRanges, LARGE, MEDIUM, SMALL, TINY};
use criterion::measurement::WallTime;
use criterion::{criterion_group, criterion_main, BatchSize, Bencher, BenchmarkGroup, Criterion};

trait Rope: Sized {
    type RopeSlice<'a>
    where
        Self: 'a;

    fn from_str(s: &str) -> Self;
    fn len(&self) -> usize;
    fn slice(&self, range: Range<usize>) -> Self::RopeSlice<'_>;
    fn line_len(&self) -> usize;
    fn line_slice(&self, _range: Range<usize>) -> Self::RopeSlice<'_>;

    fn from_slice(_s: Self::RopeSlice<'_>) -> Self {
        unimplemented!();
    }
}

impl Rope for crop::Rope {
    type RopeSlice<'a> = crop::RopeSlice<'a>;

    #[inline]
    fn from_str(s: &str) -> Self {
        Self::from(s)
    }

    #[inline]
    fn len(&self) -> usize {
        self.byte_len()
    }

    #[inline]
    fn slice(&self, range: Range<usize>) -> Self::RopeSlice<'_> {
        self.byte_slice(range)
    }

    #[inline]
    fn line_len(&self) -> usize {
        self.line_len()
    }

    #[inline]
    fn line_slice(&self, range: Range<usize>) -> Self::RopeSlice<'_> {
        self.line_slice(range)
    }

    #[inline]
    fn from_slice(s: Self::RopeSlice<'_>) -> Self {
        Self::from(s)
    }
}

impl Rope for ropey::Rope {
    type RopeSlice<'a> = ropey::RopeSlice<'a>;

    #[inline]
    fn from_str(s: &str) -> Self {
        Self::from_str(s)
    }

    #[inline]
    fn len(&self) -> usize {
        self.len_bytes()
    }

    #[inline]
    fn slice(&self, range: Range<usize>) -> Self::RopeSlice<'_> {
        self.byte_slice(range)
    }

    #[inline]
    fn line_len(&self) -> usize {
        self.len_lines()
    }

    #[inline]
    fn line_slice(&self, range: Range<usize>) -> Self::RopeSlice<'_> {
        let start = self.line_to_byte(range.start);
        let end = self.line_to_byte(range.end);
        self.byte_slice(start..end)
    }

    #[inline]
    fn from_slice(s: Self::RopeSlice<'_>) -> Self {
        Self::from(s)
    }
}

impl Rope for xi_rope::Rope {
    type RopeSlice<'a> = xi_rope::Rope;

    #[inline]
    fn from_str(s: &str) -> Self {
        Self::from(s)
    }

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn slice(&self, range: Range<usize>) -> Self::RopeSlice<'_> {
        self.slice(range)
    }

    #[inline]
    fn line_len(&self) -> usize {
        self.line_of_offset(self.len())
    }

    #[inline]
    fn line_slice(&self, range: Range<usize>) -> Self::RopeSlice<'_> {
        let start = self.offset_of_line(range.start);
        let end = self.offset_of_line(range.end);
        self.slice(start..end)
    }
}

impl Rope for zed_rope::Rope {
    type RopeSlice<'a> = zed_rope::Rope;

    #[inline]
    fn from_str(s: &str) -> Self {
        Self::from(s)
    }

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn slice(&self, range: Range<usize>) -> Self::RopeSlice<'_> {
        self.slice(range)
    }

    #[inline]
    fn line_len(&self) -> usize {
        self.max_point().row as usize
    }

    #[inline]
    fn line_slice(&self, range: Range<usize>) -> Self::RopeSlice<'_> {
        let start = self.point_to_offset(zed_rope::Point::new(range.start as u32, 0));
        let end = self.point_to_offset(zed_rope::Point::new(range.end as u32, 0));
        self.slice(start..end)
    }
}

fn byte_slice<R: Rope>(group: &mut BenchmarkGroup<WallTime>) {
    #[inline]
    fn bench<R: Rope>(bench: &mut Bencher, s: &str) {
        let r = R::from_str(s);
        let mut ranges = PercentRanges::new(r.len()).cycle();
        let setup = || ranges.next().unwrap();
        let routine = |range| r.slice(range);
        bench.iter_batched(setup, routine, BatchSize::SmallInput);
    }

    group.bench_function("tiny", |b| bench::<R>(b, TINY));
    group.bench_function("small", |b| bench::<R>(b, SMALL));
    group.bench_function("medium", |b| bench::<R>(b, MEDIUM));
    group.bench_function("large", |b| bench::<R>(b, LARGE));
}

fn line_slice<R: Rope>(group: &mut BenchmarkGroup<WallTime>) {
    #[inline(always)]
    fn bench<R: Rope>(bench: &mut Bencher, s: &str) {
        let r = R::from_str(s);
        let mut ranges = PercentRanges::new(r.line_len()).cycle();
        let setup = || ranges.next().unwrap();
        let routine = |range| r.line_slice(range);
        bench.iter_batched(setup, routine, BatchSize::SmallInput);
    }

    group.bench_function("tiny", |b| bench::<R>(b, TINY));
    group.bench_function("small", |b| bench::<R>(b, SMALL));
    group.bench_function("medium", |b| bench::<R>(b, MEDIUM));
    group.bench_function("large", |b| bench::<R>(b, LARGE));
}

fn rope_from_slice<R: Rope>(group: &mut BenchmarkGroup<WallTime>) {
    #[inline(always)]
    fn bench<R: Rope>(bench: &mut Bencher, s: &str) {
        let r = R::from_str(s);
        let mut ranges = PercentRanges::new(r.len()).cycle();
        let setup = || {
            let range = ranges.next().unwrap();
            r.slice(range)
        };
        let routine = R::from_slice;
        bench.iter_batched(setup, routine, BatchSize::SmallInput);
    }

    group.bench_function("tiny", |b| bench::<R>(b, TINY));
    group.bench_function("small", |b| bench::<R>(b, SMALL));
    group.bench_function("medium", |b| bench::<R>(b, MEDIUM));
    group.bench_function("large", |b| bench::<R>(b, LARGE));
}

fn crop_byte_slice(c: &mut Criterion) {
    let mut group = c.benchmark_group("crop_byte_slice");
    byte_slice::<crop::Rope>(&mut group);
}

fn crop_line_slice(c: &mut Criterion) {
    let mut group = c.benchmark_group("crop_line_slice");
    line_slice::<crop::Rope>(&mut group);
}

fn crop_from_slice(c: &mut Criterion) {
    let mut group = c.benchmark_group("crop_from_slice");
    rope_from_slice::<crop::Rope>(&mut group);
}

fn ropey_byte_slice(c: &mut Criterion) {
    let mut group = c.benchmark_group("ropey_byte_slice");
    byte_slice::<ropey::Rope>(&mut group);
}

fn ropey_line_slice(c: &mut Criterion) {
    let mut group = c.benchmark_group("ropey_line_slice");
    line_slice::<ropey::Rope>(&mut group);
}

fn ropey_from_slice(c: &mut Criterion) {
    let mut group = c.benchmark_group("ropey_from_slice");
    rope_from_slice::<ropey::Rope>(&mut group);
}

fn xi_rope_byte_slice(c: &mut Criterion) {
    let mut group = c.benchmark_group("xi_rope_byte_slice");
    byte_slice::<xi_rope::Rope>(&mut group);
}

fn xi_rope_line_slice(c: &mut Criterion) {
    let mut group = c.benchmark_group("xi_rope_line_slice");
    line_slice::<xi_rope::Rope>(&mut group);
}

fn zed_rope_byte_slice(c: &mut Criterion) {
    let mut group = c.benchmark_group("zed_rope_byte_slice");
    byte_slice::<zed_rope::Rope>(&mut group);
}

fn zed_rope_line_slice(c: &mut Criterion) {
    let mut group = c.benchmark_group("zed_rope_line_slice");
    line_slice::<zed_rope::Rope>(&mut group);
}

criterion_group!(
    benches,
    crop_byte_slice,
    crop_line_slice,
    crop_from_slice,
    ropey_byte_slice,
    ropey_line_slice,
    ropey_from_slice,
    xi_rope_byte_slice,
    xi_rope_line_slice,
    zed_rope_byte_slice,
    zed_rope_line_slice,
);

criterion_main!(benches);
