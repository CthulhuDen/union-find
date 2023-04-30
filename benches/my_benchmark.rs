use std::fs::File;
use std::io::{BufRead, BufReader};

use criterion::{criterion_group, criterion_main, BatchSize, Criterion, Throughput};

use union_find::{CompressedWeightedQuickUnionUF, CthlUF, QuickFindUF, QuickUnionUF, UF, WeightedQuickUnionUF};

#[derive(Clone)]
struct TestCase {
    num_vertices: usize,
    edges: Vec<(u32, u32)>,
    expected_lines: usize,
}

impl TestCase {
    fn from_file(path: &str, expected_lines: usize) -> std::io::Result<Self> {
        let mut reader = BufReader::new(File::open(path)?).lines();
        let num_vertices = reader.next().unwrap()?.parse().unwrap();

        let mut edges = vec![];
        for line in reader {
            let line = line?;
            let mut pieces = line.split_ascii_whitespace();
            edges.push((
                pieces.next().unwrap().parse().unwrap(),
                pieces.next().unwrap().parse().unwrap(),
            ));
        }

        Ok(Self {
            num_vertices,
            edges,
            expected_lines,
        })
    }
}

fn run_with_uf<U: UF, F: FnOnce(usize) -> U>(case: TestCase, f: F) {
    let mut u = f(case.num_vertices);
    let mut counter = 0;
    for edge in case.edges {
        if u.connect(edge.0, edge.1) {
            counter += 1;
        }
    }

    if counter != case.expected_lines {
        panic!(
            "Wrong number of output lines, {} expected and {} produced",
            case.expected_lines, counter
        )
    }
}

fn run_fast_adhoc(case: TestCase) {
    let counter = union_find::count_useful_edges(case.num_vertices, case.edges);
    if counter != case.expected_lines {
        panic!(
            "Wrong number of output lines, {} expected and {} produced",
            case.expected_lines, counter
        )
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let input: TestCase =
        TestCase::from_file("mediumUF.txt", 622).unwrap();

    let mut group = c.benchmark_group("medium");
    group.throughput(Throughput::Elements(900));
    group.bench_function("quick_find", |b| {
        b.iter_batched(
            || input.clone(),
            |case| run_with_uf(case, QuickFindUF::new),
            BatchSize::SmallInput,
        )
    });
    group.bench_function("quick_union", |b| {
        b.iter_batched(
            || input.clone(),
            |case| run_with_uf(case, QuickUnionUF::new),
            BatchSize::SmallInput,
        )
    });
    group.bench_function("weighted", |b| {
        b.iter_batched(
            || input.clone(),
            |case| run_with_uf(case, WeightedQuickUnionUF::new),
            BatchSize::SmallInput,
        )
    });
    group.bench_function("weighted_compressed", |b| {
        b.iter_batched(
            || input.clone(),
            |case| run_with_uf(case, CompressedWeightedQuickUnionUF::new),
            BatchSize::SmallInput,
        )
    });
    group.bench_function("cth", |b| {
        b.iter_batched(
            || input.clone(),
            |case| run_with_uf(case, CthlUF::new),
            BatchSize::SmallInput,
        )
    });
    group.bench_function("wc_adhoc", |b| {
        b.iter_batched(
            || input.clone(),
            run_fast_adhoc,
            BatchSize::SmallInput,
        )
    });
    group.finish();

    let input: TestCase =
        TestCase::from_file("largeUF.txt", 999994).unwrap();

    let mut group = c.benchmark_group("large");
    group.throughput(Throughput::Elements(2_000_000));
    group.bench_function("weighted", |b| {
        b.iter_batched(
            || input.clone(),
            |case| run_with_uf(case, WeightedQuickUnionUF::new),
            BatchSize::LargeInput,
        )
    });
    group.bench_function("weighted_compressed", |b| {
        b.iter_batched(
            || input.clone(),
            |case| run_with_uf(case, CompressedWeightedQuickUnionUF::new),
            BatchSize::LargeInput,
        )
    });
    group.bench_function("cth", |b| {
        b.iter_batched(
            || input.clone(),
            |case| run_with_uf(case, CthlUF::new),
            BatchSize::LargeInput,
        )
    });
    group.bench_function("wc_adhoc", |b| {
        b.iter_batched(
            || input.clone(),
            run_fast_adhoc,
            BatchSize::LargeInput,
        )
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
