mod rope;

use crdt_testdata::{load_testing_data, TestPatch};
use criterion::measurement::WallTime;
use criterion::{
    criterion_group, criterion_main, BenchmarkGroup, BenchmarkId, Criterion, Throughput,
};
use rope::Rope;

const TRACES: &[&str] = &[
    "automerge-paper",
    "rustcode",
    "sveltecomponent",
    "seph-blog1",
];

fn traces(c: &mut Criterion) {
    fn bench<R: Rope>(group: &mut BenchmarkGroup<WallTime>, trace_file: &str) {
        let mut trace = load_testing_data(&format!("./traces/{trace_file}.json.gz"));

        if R::EDITS_USE_BYTE_OFFSETS {
            trace = trace.chars_to_bytes();
        }

        group.throughput(Throughput::Elements(trace.len() as u64));

        group.bench_function(BenchmarkId::new(trace_file, R::NAME), |b| {
            b.iter(|| {
                let mut rope = R::from_str(&trace.start_content);
                for txn in &trace.txns {
                    for TestPatch(pos, del, ins) in &txn.patches {
                        rope.replace(*pos..*pos + del, ins);
                    }
                }
                assert_eq!(rope.len(), trace.end_content.len());
            })
        });
    }

    for trace in TRACES {
        let mut group = c.benchmark_group("traces");

        bench::<String>(&mut group, trace);
        bench::<crop::Rope>(&mut group, trace);
        bench::<jumprope::JumpRope>(&mut group, trace);
        bench::<jumprope::JumpRopeBuf>(&mut group, trace);
        bench::<ropey::Rope>(&mut group, trace);
        bench::<xi_rope::Rope>(&mut group, trace);
        bench::<zed_rope::Rope>(&mut group, trace);
    }
}

criterion_group!(benches, traces);

criterion_main!(benches);
