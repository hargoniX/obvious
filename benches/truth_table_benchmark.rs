use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use criterion::BenchmarkId;

use obvious::bruteforce::BruteforceTruthTableBuilder;
use obvious::bruteforce::ParallelBruteforceTruthTableBuilder;

fn n_table_var_linear(n: usize) -> Vec<bool> {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let variables: Vec<&str> = alphabet.split("").take(n).collect();
    let table = BruteforceTruthTableBuilder::build(&variables, |vars| {
        let mut statement = vars[0].implies(&vars[1]);
        for variable in vars[2..].iter() {
            statement = statement.implies(&variable)
        }
        statement
    }).unwrap();
    table.table.values().copied().collect()
}

fn n_table_var_parallel(n: usize) -> Vec<bool> {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let variables: Vec<&str> = alphabet.split("").take(n).collect();
    let table = ParallelBruteforceTruthTableBuilder::build(&variables, |vars| {
        let mut statement = vars[0].implies(&vars[1]);
        for variable in vars[2..].iter() {
            statement = statement.implies(&variable)
        }
        statement
    }).unwrap();
    table.table.values().copied().collect()
}

pub fn n_implications_truth_table(c: &mut Criterion) {
    let mut group = c.benchmark_group("n_implications_truth_table");
    for size in 2..18 {
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("Linear truth table", size), &size, |b, &size| b.iter(|| n_table_var_linear(size)));
        group.bench_with_input(BenchmarkId::new("Parallel truth table", size), &size, |b, &size| b.iter(|| n_table_var_parallel(size)));
    }

    // linear is too slow at this point
    for size in 18..20 {
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("Parallel truth table", size), &size, |b, &size| b.iter(|| n_table_var_parallel(size)));
    }
    group.finish();
}

criterion_group!(benches, n_implications_truth_table);
criterion_main!(benches);
