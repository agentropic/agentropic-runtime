use agentropic_runtime::prelude::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn spawn_single_agent(c: &mut Criterion) {
    c.bench_function("spawn_single_agent", |b| {
        b.iter(|| {
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                let runtime = Runtime::new();
                let agent_id = AgentId::new();
                runtime.spawn(black_box(agent_id), "test").await.unwrap();
            });
        });
    });
}

fn spawn_100_agents(c: &mut Criterion) {
    c.bench_function("spawn_100_agents", |b| {
        b.iter(|| {
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                let runtime = Runtime::new();
                for _ in 0..100 {
                    let agent_id = AgentId::new();
                    runtime.spawn(black_box(agent_id), "test").await.unwrap();
                }
            });
        });
    });
}

fn spawn_1000_agents(c: &mut Criterion) {
    c.bench_function("spawn_1000_agents", |b| {
        b.iter(|| {
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                let runtime = Runtime::new();
                for _ in 0..1000 {
                    let agent_id = AgentId::new();
                    runtime.spawn(black_box(agent_id), "test").await.unwrap();
                }
            });
        });
    });
}

criterion_group!(
    benches,
    spawn_single_agent,
    spawn_100_agents,
    spawn_1000_agents
);
criterion_main!(benches);
