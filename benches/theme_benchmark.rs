use criterion::{Criterion, criterion_group, criterion_main};

use sound_themer::theme::init_selected_theme;

/// # Documentation
/// Benchmark the `init_selected_theme()` function
fn init_select_theme_benchmark(c: &mut Criterion) {
    c.bench_function("Theme Initialisation", |b| b.iter(init_selected_theme));
}

criterion_group!(benches, init_select_theme_benchmark);
criterion_main!(benches);
