use criterion::{Criterion, criterion_group, criterion_main};

use sound_themer::config::init_toml_config;

/// # Documentation
/// Benchmark the `init_toml_config()` function
fn config_init_benchmark(c: &mut Criterion) {
    c.bench_function("Config Initialisation", |b| b.iter(init_toml_config));
}

criterion_group!(benches, config_init_benchmark);
criterion_main!(benches);
