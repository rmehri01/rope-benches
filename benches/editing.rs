mod common;

use common::{PercentRanges, LARGE, MEDIUM, SMALL, TINY};
use criterion::measurement::WallTime;
use criterion::{criterion_group, criterion_main, Bencher, BenchmarkGroup, Criterion};
use std::ops::Range;

trait Rope: Clone {
    fn from_str(s: &str) -> Self;
    fn len(&self) -> usize;
    fn insert(&mut self, at: usize, text: &str);
    fn delete(&mut self, range: Range<usize>);
    fn replace(&mut self, range: Range<usize>, text: &str);
}

impl Rope for crop::Rope {
    #[inline]
    fn from_str(s: &str) -> Self {
        Self::from(s)
    }

    #[inline]
    fn len(&self) -> usize {
        self.byte_len()
    }

    #[inline]
    fn insert(&mut self, at_byte: usize, text: &str) {
        self.insert(at_byte, text);
    }

    #[inline]
    fn delete(&mut self, byte_range: Range<usize>) {
        self.delete(byte_range);
    }

    #[inline]
    fn replace(&mut self, byte_range: Range<usize>, text: &str) {
        self.replace(byte_range, text);
    }
}

impl Rope for jumprope::JumpRope {
    #[inline]
    fn from_str(s: &str) -> Self {
        Self::from(s)
    }

    #[inline]
    fn len(&self) -> usize {
        self.len_bytes()
    }

    #[inline]
    fn insert(&mut self, at_byte: usize, text: &str) {
        self.insert(at_byte, text);
    }

    #[inline]
    fn delete(&mut self, byte_range: Range<usize>) {
        self.remove(byte_range);
    }

    #[inline]
    fn replace(&mut self, byte_range: Range<usize>, text: &str) {
        self.replace(byte_range, text);
    }
}

impl Rope for ropey::Rope {
    #[inline]
    fn from_str(s: &str) -> Self {
        Self::from_str(s)
    }

    #[inline]
    fn len(&self) -> usize {
        self.len_chars()
    }

    #[inline]
    fn insert(&mut self, at_char: usize, text: &str) {
        self.insert(at_char, text);
    }

    #[inline]
    fn delete(&mut self, char_range: Range<usize>) {
        self.remove(char_range);
    }

    #[inline]
    fn replace(&mut self, char_range: Range<usize>, text: &str) {
        let start = char_range.start;
        self.remove(char_range);
        self.insert(start, text);
    }
}

impl Rope for xi_rope::Rope {
    #[inline]
    fn from_str(s: &str) -> Self {
        Self::from(s)
    }

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn insert(&mut self, at_byte: usize, text: &str) {
        self.edit(at_byte..at_byte, text);
    }

    #[inline]
    fn delete(&mut self, byte_range: Range<usize>) {
        self.edit(byte_range, "");
    }

    #[inline]
    fn replace(&mut self, byte_range: Range<usize>, text: &str) {
        self.edit(byte_range, text);
    }
}

impl Rope for zed_rope::Rope {
    #[inline]
    fn from_str(s: &str) -> Self {
        Self::from(s)
    }

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn insert(&mut self, at_byte: usize, text: &str) {
        self.replace(at_byte..at_byte, text);
    }

    #[inline]
    fn delete(&mut self, byte_range: Range<usize>) {
        self.replace(byte_range, "");
    }

    #[inline]
    fn replace(&mut self, byte_range: Range<usize>, text: &str) {
        self.replace(byte_range, text);
    }
}

fn bench_insert<R: Rope>(group: &mut BenchmarkGroup<WallTime>, insert: &str) {
    #[inline(always)]
    fn bench<R: Rope>(bench: &mut Bencher, s: &str, insert: &str) {
        let mut r = R::from_str(s);
        let mut ranges = PercentRanges::new(r.len()).cycle();
        let mut i = 0;
        bench.iter(|| {
            let range = ranges.next().unwrap();
            let at = if i % 2 == 0 { range.start } else { range.end };
            r.insert(at, insert);
            i += 1;
        });
    }

    group.bench_function("tiny", |b| bench::<R>(b, TINY, insert));
    group.bench_function("small", |b| bench::<R>(b, SMALL, insert));
    group.bench_function("medium", |b| bench::<R>(b, MEDIUM, insert));
    group.bench_function("large", |b| bench::<R>(b, LARGE, insert));
}

fn bench_delete<R: Rope>(group: &mut BenchmarkGroup<WallTime>, delete_bytes: usize) {
    #[inline(always)]
    fn bench<R: Rope>(bench: &mut Bencher, s: &str, delete_bytes: usize) {
        let mut r = R::from_str(s);
        let mut ranges = PercentRanges::new(r.len()).cycle();
        let mut i = 0;
        let orig_len = r.len();
        bench.iter(|| {
            let range = ranges.next().unwrap();
            let start = (if i % 2 == 0 { range.start } else { range.end }).min(r.len());
            let end = (start + delete_bytes).min(r.len());
            r.delete(start..end);
            i += 1;

            if r.len() < orig_len / 4 {
                r = R::from_str(s);
            }
        });
    }

    group.bench_function("tiny", |b| bench::<R>(b, TINY, delete_bytes));
    group.bench_function("small", |b| bench::<R>(b, SMALL, delete_bytes));
    group.bench_function("medium", |b| bench::<R>(b, MEDIUM, delete_bytes));
    group.bench_function("large", |b| bench::<R>(b, LARGE, delete_bytes));
}

fn bench_replace<R: Rope>(group: &mut BenchmarkGroup<WallTime>, replace: &str) {
    #[inline(always)]
    fn bench<R: Rope>(bench: &mut Bencher, s: &str, replace: &str) {
        let mut r = R::from_str(s);
        let mut ranges = PercentRanges::new(r.len()).cycle();
        let mut i = 0;
        bench.iter(|| {
            let range = ranges.next().unwrap();
            let start = if i % 2 == 0 { range.start } else { range.end };
            let end = (start + replace.len()).min(r.len());
            r.replace(start..end, replace);
            i += 1;
        });
    }

    group.bench_function("tiny", |b| bench::<R>(b, TINY, replace));
    group.bench_function("small", |b| bench::<R>(b, SMALL, replace));
    group.bench_function("medium", |b| bench::<R>(b, MEDIUM, replace));
    group.bench_function("large", |b| bench::<R>(b, LARGE, replace));
}

fn crop_insert_char(c: &mut Criterion) {
    let mut group = c.benchmark_group("crop_insert_char");
    bench_insert::<crop::Rope>(&mut group, "a");
}

fn crop_insert_sentence(c: &mut Criterion) {
    let mut group = c.benchmark_group("crop_insert_sentence");
    bench_insert::<crop::Rope>(
        &mut group,
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
    );
}

fn crop_insert_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("crop_insert_large");
    bench_insert::<crop::Rope>(&mut group, SMALL);
}

fn crop_delete_char(c: &mut Criterion) {
    let mut group = c.benchmark_group("crop_delete_char");
    bench_delete::<crop::Rope>(&mut group, "a".len());
}

fn crop_delete_sentence(c: &mut Criterion) {
    let mut group = c.benchmark_group("crop_delete_sentence");
    bench_delete::<crop::Rope>(
        &mut group,
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit.".len(),
    );
}

fn crop_delete_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("crop_delete_large");
    bench_delete::<crop::Rope>(&mut group, SMALL.len());
}

fn crop_replace_char(c: &mut Criterion) {
    let mut group = c.benchmark_group("crop_replace_char");
    bench_replace::<crop::Rope>(&mut group, "a");
}

fn crop_replace_sentence(c: &mut Criterion) {
    let mut group = c.benchmark_group("crop_replace_sentence");
    bench_replace::<crop::Rope>(
        &mut group,
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
    );
}

fn crop_replace_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("crop_replace_large");
    bench_replace::<crop::Rope>(&mut group, SMALL);
}

fn ropey_insert_char(c: &mut Criterion) {
    let mut group = c.benchmark_group("ropey_insert_char");
    bench_insert::<ropey::Rope>(&mut group, "a");
}

fn ropey_insert_sentence(c: &mut Criterion) {
    let mut group = c.benchmark_group("ropey_insert_sentence");
    bench_insert::<ropey::Rope>(
        &mut group,
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
    );
}

fn ropey_insert_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("ropey_insert_large");
    bench_insert::<ropey::Rope>(&mut group, SMALL);
}

fn ropey_delete_char(c: &mut Criterion) {
    let mut group = c.benchmark_group("ropey_delete_char");
    bench_delete::<ropey::Rope>(&mut group, "a".len());
}

fn ropey_delete_sentence(c: &mut Criterion) {
    let mut group = c.benchmark_group("ropey_delete_sentence");
    bench_delete::<ropey::Rope>(
        &mut group,
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit.".len(),
    );
}

fn ropey_delete_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("ropey_delete_large");
    bench_delete::<ropey::Rope>(&mut group, SMALL.len());
}

fn ropey_replace_char(c: &mut Criterion) {
    let mut group = c.benchmark_group("ropey_replace_char");
    bench_replace::<ropey::Rope>(&mut group, "a");
}

fn ropey_replace_sentence(c: &mut Criterion) {
    let mut group = c.benchmark_group("ropey_replace_sentence");
    bench_replace::<ropey::Rope>(
        &mut group,
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
    );
}

fn ropey_replace_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("ropey_replace_large");
    bench_replace::<ropey::Rope>(&mut group, SMALL);
}

fn xi_rope_insert_char(c: &mut Criterion) {
    let mut group = c.benchmark_group("xi_rope_insert_char");
    bench_insert::<xi_rope::Rope>(&mut group, "a");
}

fn xi_rope_insert_sentence(c: &mut Criterion) {
    let mut group = c.benchmark_group("xi_rope_insert_sentence");
    bench_insert::<xi_rope::Rope>(
        &mut group,
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
    );
}

fn xi_rope_insert_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("xi_rope_insert_large");
    bench_insert::<xi_rope::Rope>(&mut group, SMALL);
}

fn xi_rope_delete_char(c: &mut Criterion) {
    let mut group = c.benchmark_group("xi_rope_delete_char");
    bench_delete::<xi_rope::Rope>(&mut group, "a".len());
}

fn xi_rope_delete_sentence(c: &mut Criterion) {
    let mut group = c.benchmark_group("xi_rope_delete_sentence");
    bench_delete::<xi_rope::Rope>(
        &mut group,
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit.".len(),
    );
}

fn xi_rope_delete_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("xi_rope_delete_large");
    bench_delete::<xi_rope::Rope>(&mut group, SMALL.len());
}

fn xi_rope_replace_char(c: &mut Criterion) {
    let mut group = c.benchmark_group("xi_rope_replace_char");
    bench_replace::<xi_rope::Rope>(&mut group, "a");
}

fn xi_rope_replace_sentence(c: &mut Criterion) {
    let mut group = c.benchmark_group("xi_rope_replace_sentence");
    bench_replace::<xi_rope::Rope>(
        &mut group,
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
    );
}

fn xi_rope_replace_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("xi_rope_replace_large");
    bench_replace::<xi_rope::Rope>(&mut group, SMALL);
}

fn jumprope_insert_char(c: &mut Criterion) {
    let mut group = c.benchmark_group("jumprope_insert_char");
    bench_insert::<jumprope::JumpRope>(&mut group, "a");
}

fn jumprope_insert_sentence(c: &mut Criterion) {
    let mut group = c.benchmark_group("jumprope_insert_sentence");
    bench_insert::<jumprope::JumpRope>(
        &mut group,
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
    );
}

fn jumprope_insert_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("jumprope_insert_large");
    bench_insert::<jumprope::JumpRope>(&mut group, SMALL);
}

fn jumprope_delete_char(c: &mut Criterion) {
    let mut group = c.benchmark_group("jumprope_delete_char");
    bench_delete::<jumprope::JumpRope>(&mut group, "a".len());
}

fn jumprope_delete_sentence(c: &mut Criterion) {
    let mut group = c.benchmark_group("jumprope_delete_sentence");
    bench_delete::<jumprope::JumpRope>(
        &mut group,
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit.".len(),
    );
}

fn jumprope_delete_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("jumprope_delete_large");
    bench_delete::<jumprope::JumpRope>(&mut group, SMALL.len());
}

fn jumprope_replace_char(c: &mut Criterion) {
    let mut group = c.benchmark_group("jumprope_replace_char");
    bench_replace::<jumprope::JumpRope>(&mut group, "a");
}

fn jumprope_replace_sentence(c: &mut Criterion) {
    let mut group = c.benchmark_group("jumprope_replace_sentence");
    bench_replace::<jumprope::JumpRope>(
        &mut group,
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
    );
}

fn jumprope_replace_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("jumprope_replace_large");
    bench_replace::<jumprope::JumpRope>(&mut group, SMALL);
}

fn zed_rope_insert_char(c: &mut Criterion) {
    let mut group = c.benchmark_group("zed_rope_insert_char");
    bench_insert::<zed_rope::Rope>(&mut group, "a");
}

fn zed_rope_insert_sentence(c: &mut Criterion) {
    let mut group = c.benchmark_group("zed_rope_insert_sentence");
    bench_insert::<zed_rope::Rope>(
        &mut group,
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
    );
}

fn zed_rope_insert_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("zed_rope_insert_large");
    bench_insert::<zed_rope::Rope>(&mut group, SMALL);
}

fn zed_rope_delete_char(c: &mut Criterion) {
    let mut group = c.benchmark_group("zed_rope_delete_char");
    bench_delete::<zed_rope::Rope>(&mut group, "a".len());
}

fn zed_rope_delete_sentence(c: &mut Criterion) {
    let mut group = c.benchmark_group("zed_rope_delete_sentence");
    bench_delete::<zed_rope::Rope>(
        &mut group,
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit.".len(),
    );
}

fn zed_rope_delete_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("zed_rope_delete_large");
    bench_delete::<zed_rope::Rope>(&mut group, SMALL.len());
}

fn zed_rope_replace_char(c: &mut Criterion) {
    let mut group = c.benchmark_group("zed_rope_replace_char");
    bench_replace::<zed_rope::Rope>(&mut group, "a");
}

fn zed_rope_replace_sentence(c: &mut Criterion) {
    let mut group = c.benchmark_group("zed_rope_replace_sentence");
    bench_replace::<zed_rope::Rope>(
        &mut group,
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
    );
}

fn zed_rope_replace_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("zed_rope_replace_large");
    bench_replace::<zed_rope::Rope>(&mut group, SMALL);
}

criterion_group!(
    benches,
    crop_insert_char,
    crop_insert_sentence,
    crop_insert_large,
    crop_delete_char,
    crop_delete_sentence,
    crop_delete_large,
    crop_replace_char,
    crop_replace_sentence,
    crop_replace_large,
    jumprope_insert_char,
    jumprope_insert_sentence,
    jumprope_insert_large,
    jumprope_delete_char,
    jumprope_delete_sentence,
    jumprope_delete_large,
    jumprope_replace_char,
    jumprope_replace_sentence,
    jumprope_replace_large,
    ropey_insert_char,
    ropey_insert_sentence,
    ropey_insert_large,
    ropey_delete_char,
    ropey_delete_sentence,
    ropey_delete_large,
    ropey_replace_char,
    ropey_replace_sentence,
    ropey_replace_large,
    xi_rope_insert_char,
    xi_rope_insert_sentence,
    xi_rope_insert_large,
    xi_rope_delete_char,
    xi_rope_delete_sentence,
    xi_rope_delete_large,
    xi_rope_replace_char,
    xi_rope_replace_sentence,
    xi_rope_replace_large,
    zed_rope_insert_char,
    zed_rope_insert_sentence,
    zed_rope_insert_large,
    zed_rope_delete_char,
    zed_rope_delete_sentence,
    zed_rope_delete_large,
    zed_rope_replace_char,
    zed_rope_replace_sentence,
    zed_rope_replace_large,
);

criterion_main!(benches);
