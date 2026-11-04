---
metadata:
  type: skill
description: Organize benchmark packages, measurement harnesses, diagnostic builds, and committed results in Base Rust projects. Use when adding or changing a benchmark package or harness, adding a diagnostic build for a measurement, or committing benchmark results.
---

# Benchmarks

Use this skill for where benchmark code and results live and how they stay reproducible. Follow `base-project-performance` for how to measure, compare builds, interpret results, decide what to keep, and report numbers.

## Benchmark packages

- A crate's `benches/` targets suit microbenchmarks run with `cargo bench` that need nothing beyond the crate and its development dependencies. Put a benchmark in its own package when it needs its own binaries, dependencies, or driver scripts, or measures several packages together.
- Put such a package under `benchmarks/<name>/`, with `publish = false`, and list it in the workspace members. CI then builds, lints, and tests it with the rest of the workspace.
- Name the package `<project>-benchmark-<name>`, where `<project>` is the workspace's root package name and `<name>` is the directory name. For example, `benchmarks/timers` in `base-scheduler` is `base-scheduler-benchmark-timers`. The package name then follows its directory, as other workspace packages do, and the benchmarks sort together.
- Name the directory for the subject it measures, such as `timers` or `echo`, rather than for one experiment. Add a workload for the same subject to the existing package, as a mode of its binary or as another binary, rather than adding a package.
- A benchmark depends on the packages it measures, never the reverse.

## Harness code

- Write new measurement code as targets of a benchmark package, or as `benches/` targets for microbenchmarks. Do not add Rust sources beside results to be built by hand. CI does not compile, lint, or test such sources, so they drift from the APIs they use, and their results stop being reproducible.
- Driver scripts that run binaries, interleave builds, and summarize results may stay as scripts, beside the benchmark (such as `run.py`) or with the results they produced. Pass binaries, revisions, and output paths as arguments or environment variables rather than writing host-specific paths into them.
- Make every duration, count, and rate a parameter, with defaults for real measurements, so that tests can run each workload briefly. Benchmark packages fall under the project's coverage requirement like any other package; test every workload with small parameters, following `base-project-testing`.
- Print one machine-readable record per process, such as a JSON object, that repeats every parameter alongside per-operation figures and process CPU time.
- Fail on invalid arguments and on a failed run. Never report a partial run as a measurement.

## Diagnostic builds

- When a benchmark needs internal counts from a measured package, such as rescues, retries, or fallbacks, expose them through a non-default feature of that package, documented as unstable; `diagnostics` is a good name. Forward it through a feature of the same name on the benchmark package, and include the counts in each run's record when it is enabled.
- Build the diagnostic variant separately from the timed variant, and do not take timing from it, as `base-project-performance` requires for other tools that perturb timing.

## Results

- Commit each measurement's results in a directory beside the benchmark, `benchmarks/<name>/results/<topic>/`. When a measurement spans several benchmarks, use the benchmark for the change's main subject and link to the results from the others' documents. Existing results in other layouts stay where they are.
- Give each results directory a `readme.md` that states what the results check, the details that `base-project-performance` requires for each result, each file and what produced it, the outcome, every metric outside its budget with its numbers, and what was not measured. Link it from the project's performance record.
- Keep the raw records (JSON Lines works well), the driver scripts, and any patch that defines a variant exactly as run, and generate summaries from the raw records. Copy numbers into readmes and pull requests from the generated summaries; never transcribe them from terminal output. Keep raw records small enough to review and diff, and leave large profiles and traces out of the repository.
- Past results document what ran at the time, including package and binary names, and any sources that were built by hand. Do not rewrite them when the harness changes later; point from them to the harness that replaces them instead.
