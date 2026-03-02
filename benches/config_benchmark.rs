use criterion::{Criterion, criterion_group, criterion_main};

use sound_themer::config::{get_config_home_dir, init_toml_config};

/// # Documentation
/// Benchmark the `get_config_home_dir()` function
fn get_config_home_dir_benchmark(c: &mut Criterion) {
    c.bench_function("Get Config Home From Env", |b| b.iter(get_config_home_dir));
}

/// # Documentation
/// Benchmark the `init_toml_config()` function
fn config_init_benchmark(c: &mut Criterion) {
    c.bench_function("Config Initialisation", |b| b.iter(init_toml_config));
}

criterion_group!(benches, get_config_home_dir_benchmark, config_init_benchmark);
criterion_main!(benches);
