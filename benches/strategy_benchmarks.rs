use criterion::{black_box, criterion_group, criterion_main, Criterion};
use quanta_engine::{StrategyManager, StrategyStatus};

fn benchmark_add_strategy(c: &mut Criterion) {
    let mut manager = StrategyManager::new();
    c.bench_function("add strategy", |b| {
        b.iter(|| {
            manager.add_strategy(
                black_box("test_strategy".to_string()),
                black_box("Test Strategy".to_string()),
                black_box("user1".to_string()),
            )
        })
    });
}

fn benchmark_list_active_strategies(c: &mut Criterion) {
    let mut manager = StrategyManager::new();
    for i in 0..1000 {
        manager.add_strategy(
            format!("strategy_{}", i),
            format!("Strategy {}", i),
            "user1".to_string(),
        ).unwrap();
    }
    c.bench_function("list active strategies", |b| {
        b.iter(|| black_box(manager.list_active_strategies()))
    });
}

criterion_group!(benches, benchmark_add_strategy, benchmark_list_active_strategies);
criterion_main!(benches);