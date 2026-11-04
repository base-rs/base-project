---
metadata:
  type: skill
description: Organize benchmark packages, measurement harnesses, and committed results in Base Rust projects. Use when adding or changing a benchmark, building a harness or diagnostic for a measurement, comparing builds against a performance budget, or committing benchmark results.
---

# Benchmarks

Use this skill for how benchmark code and results are organized. Follow `base-project-performance` for how to measure, interpret results, decide what to keep, and report numbers.

## Benchmark packages

- Put each benchmark in a private workspace package under `benchmarks/<name>/`, with `publish = false`, and list it in the workspace members. CI then builds, lints, and tests it with the rest of the workspace.
- Name the package `<project>-benchmark-<name>`, where `<name>` is the directory name. For example, `benchmarks/timers` in `base-scheduler` is `base-scheduler-benchmark-timers`. The package name then follows its directory, as other workspace packages do, and the benchmarks sort together.
- Name the directory for the subject it measures, such as `timers` or `echo`, rather than for one experiment. Add a workload for the same subject to the existing package, as a mode of its binary or as another binary, rather than adding a package.
- A benchmark depends on the packages it measures, never the reverse. When a benchmark needs internal counts from a measured package, expose them through a documented, non-default feature of that package; see [Diagnostic builds](#diagnostic-builds).

## Harness code

- Write measurement code as targets of a benchmark package. Do not keep Rust sources beside results to be built by hand. CI does not compile, lint, or test such sources, so they drift from the APIs they use, and their results stop being reproducible.
- Driver scripts that run binaries, interleave builds, and summarize results may stay as scripts, beside the benchmark (such as `run.py`) or with the results they produced. Pass binaries, revisions, and output paths as arguments or environment variables rather than writing host-specific paths into them.
- Make every duration, count, and rate a parameter, with defaults for real measurements, so that tests can run each workload briefly. Benchmark packages fall under the project's coverage requirement like any other package; test every workload with small parameters, following `base-project-testing`.
- Print one machine-readable record per run, such as a JSON object, that repeats every parameter alongside per-operation figures and process CPU time. Generate summaries from these records; never transcribe numbers by hand.
- Fail on invalid arguments and on a failed run. Never report a partial run as a measurement.

## Diagnostic builds

- Count internal events, such as rescues, retries, or fallbacks, through a feature of the measured package, conventionally named `diagnostics`. Forward it through a feature of the same name on the benchmark package, and include the counts in each run's record when it is enabled.
- Build the diagnostic variant separately from the timed variant, and do not take timing from it. `base-project-performance` applies the same rule to other tools that perturb timing.

## Comparing builds

- Measure a change against a recorded baseline revision, normally the head of the target branch, built with the same harness and flags. When a change modifies the harness, measure the baseline with the new harness too.
- Build each variant from a frozen tree into its own target directory, name its binaries after the variant, and record each variant's revision.
- On a shared benchmark host, hold an exclusive lock for every run. Do not build or run tests on the host while a measurement is running.
- Before running, define the verdict rule for each metric: a percentage budget, or the baseline's minimum to maximum range for noisy metrics. If a metric passes its rule but its median moves substantially, report the move with its spread instead of only calling it noise.
- When the run checks a design budget, measure every item the design lists, and name any item that was not measured.

## Results

- Commit results beside the benchmark that produced them, in `benchmarks/<name>/results/<topic>/`. Put a run that spans several benchmarks under the package whose driver ran it.
- Give each results directory a `readme.md` that states:
  - what the results check, with the baseline and candidate revisions;
  - the host, toolchain, CPU placement, and procedure, as `base-project-performance` lists them;
  - each file, and what produced it;
  - the outcome, every metric outside its budget with numbers, and what was not measured.
- Keep the raw records (JSON Lines works well), the summaries generated from them, and the driver scripts exactly as they were run. Do not rewrite past records when the harness changes later: they document what ran at the time, including package and binary names.
- Keep large profiles and traces out of the repository, as `base-project-performance` describes.
