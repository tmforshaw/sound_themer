use std::{iter, sync::LazyLock};

use clap::Parser;
use criterion::{BatchSize, Criterion, criterion_group, criterion_main};

use sound_themer::{
    cli::{Cli, evaluate_cli},
    config::{TOMLConfig, init_toml_config},
    theme::reset_selected_theme,
};

/// # Documentation
/// Call `evaluate_cli()` with the given arguments parsed by `Cli::parse_from(args)`
fn evaluate_cli_benchmark_inner<S: AsRef<str>>(c: &mut Criterion, args: &[S]) {
    let args = iter::once("sound_themer")
        .chain(args.iter().map(AsRef::as_ref))
        .collect::<Vec<_>>();

    c.bench_function(
        // Generate the name for this using the arguments
        format!("Evaluate CLI: \"{}\"", args.join(" ")).as_str(),
        |b| {
            b.iter_batched(
                || {
                    {
                        // Force the initialisation of TOMLConfig on each iteration
                        let config: LazyLock<TOMLConfig> = LazyLock::new(init_toml_config);
                        #[allow(clippy::explicit_auto_deref)]
                        let _cfg_ref: &TOMLConfig = &*config;
                    }

                    // Reset the selected theme to config default
                    reset_selected_theme();
                },
                |_| {
                    // Add the package name before the arguments, and parse it as a Cli object
                    let cli = Cli::parse_from(args.iter());

                    // Evaluate the dummy CLI and execute the commands
                    evaluate_cli(&cli, std::io::sink())
                },
                BatchSize::SmallInput,
            )
        },
    );
}

/// # Documentation
/// Call `evaluate_cli()` with "play --duration 0 complete" as the args
fn evaluate_cli_play_duration_zero_benchmark(c: &mut Criterion) {
    evaluate_cli_benchmark_inner(c, &["play", "--duration", "0", "complete"])
}

/// # Documentation
/// Call `evaluate_cli()` with "--theme deepin play --duration 0 complete" as the args
fn evaluate_cli_play_duration_zero_and_theme_benchmark(c: &mut Criterion) {
    evaluate_cli_benchmark_inner(c, &["--theme", "deepin", "play", "--duration", "0", "complete"])
}

/// # Documentation
/// Call `evaluate_cli()` with "list" as the args
fn evaluate_cli_list_benchmark(c: &mut Criterion) {
    evaluate_cli_benchmark_inner(c, &["list"])
}

criterion_group!(
    benches,
    evaluate_cli_play_duration_zero_benchmark,
    evaluate_cli_play_duration_zero_and_theme_benchmark,
    evaluate_cli_list_benchmark
);
criterion_main!(benches);
