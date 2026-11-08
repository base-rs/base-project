---
metadata:
  type: skill
description: Add or update tests in Base Rust projects, require 100% region coverage, and decide when downstream compatibility tests are useful. Use when asked to add tests, when behavior changes need regression coverage, or when fixing a flaky or timing-dependent test.
---

# Testing Base Rust Projects

Use this skill whenever a change adds or changes behavior that should be verified by tests.

## Expectations

- Add tests that exercise the changed behavior and relevant regression cases.
- Require 100% region coverage for supported target and feature configurations. Use LLVM's region report to find executable paths that tests have not run.
- Treat uncovered code as an opportunity to review semantics. Check that its behavior is correct, add tests for meaningful behavior, and fix defects rather than writing tests that codify accidental behavior. Remove dead code or make invariants explicit when a region cannot represent supported behavior; do not manufacture impossible internal states just to reach it.
- Run coverage on each supported target or configuration that compiles distinct platform-specific code.
- Consider external compatibility tests when a change could affect downstream crates that use this project's public API. Select the relevant consumers; downstream testing is not required for every change.

Follow the repository's test layout guidance when choosing between unit and integration tests.

## Difficult-to-trigger I/O failures

Use real temporary directories for normal filesystem behavior. When an important error path depends on an operating-system failure that is difficult to trigger reliably, a private `#[cfg(test)]` shim can make that failure deterministic:

- Keep production builds delegated directly to the standard library.
- In tests, wrap only the needed operation and delegate to the real filesystem except for a specifically injected failure.
- Scope injected failures to the operation and path, and isolate them from parallel tests.
- Assert observable behavior, such as the returned error and whether changes were rolled back or cleaned up.

Filesystem error kinds and path resolution vary by operating system. Inject failures for portable error-path tests; use platform-gated tests when the OS-specific behavior itself is what the test needs to verify.

Keep the shim small and local to the code under test. Do not build a broad fake filesystem or add production abstractions solely to satisfy coverage. If many call sites need fault injection, reconsider the design and introduce a shared abstraction only when it also makes the production code clearer.

## Wall-clock time

Shared CI runners, such as those of GitHub Actions, are heavily oversubscribed, so any thread can stall for hundreds of milliseconds or more. A test that measures elapsed wall-clock time, or expects something to happen within a wall-clock bound, such as sleeping for 50 ms and expecting completion within 100 ms, will fail there intermittently. Widening the bound is not a fix.

- Make such tests deterministic. Control the clock with an injectable clock or a manual time source that the test advances, or assert on ordering, events, counts, or causality rather than elapsed time. When production code has no clock abstraction, use a private `#[cfg(test)]` time source, as for I/O shims, rather than adding one solely for tests.
- A lower bound that the code guarantees, such as a timer not firing before its deadline, is fine, because slowness cannot break it. Measure it with the same monotonic clock the code uses, such as `Instant` rather than `SystemTime`, from a reading taken before the code takes its own, and allow for coarse clocks and rounding to ticks.
- An upper bound that encodes how fast the code should be is not fine. A generous watchdog that only turns a hang into a failure is: make it seconds long and never tune it to the expected speed.
- Measure properties that genuinely need real time, such as latency or throughput, in a benchmark, following `base-project-benchmarks`, and judge the results by the verdict rules and on a host that meets the requirements in `base-project-performance`. CI builds and tests benchmark packages with small parameters, so nothing CI runs may assert a timing threshold.
- A real-time test that must stay a test needs an opt-in that CI never turns on. Mark it `#[ignore = "…"]` with a reason saying how and where to run it, such as `cargo test -- --ignored NAME` on an idle host; the shared test and coverage tasks never run ignored tests. Put it in a file that coverage excludes, under `tests/`, `examples/`, or `benches/`, or named `tests.rs`, `*_tests.rs`, or `*-tests.rs`, since an ignored test elsewhere under `src/` leaves its own body uncovered. Do not gate it on a Cargo feature, which coverage enables with `--all-features`, or on an environment variable check that silently passes and leaves the guarded branch uncovered.
- Code that only such benchmarks or opted-in tests reach still needs coverage from deterministic tests.

## Excluding files from coverage

Excluding a file from the coverage gate is a last resort, only for raw device, hardware, or platform calls that cannot run in CI, such as an OpenXR runtime, a window system's event loop, or audio hardware. Try constructed inputs, software drivers, or simulated devices first.

- Keep logic out of excluded files. Move only the untestable calls into a dedicated file and keep it thin; decisions, state, and conversions stay in covered modules.
- List each file in its package's `Cargo.toml` under `[[package.metadata.bake.test.coverage.exclude]]`, with a `reason` explaining why it cannot run in CI. Exclude files, not crates. Do not use `#[coverage(off)]`, which is unstable, or `cfg(coverage)`, which changes what is built.
- Reviewers should push back on new entries and on excluded files that grow, and prefer changes that shrink the list.

## Test guide and readme examples

Compile every Rust example in `context/` guides and `readme.md` as a doctest, so documentation cannot drift from the API. Include each file in the crate under `cfg(doctest)`, which tests its examples without adding the text to the published API documentation:

```rust
#[cfg(doctest)]
#[doc = include_str!("../context/getting-started.md")]
struct GettingStartedGuide;
```

Keep the crate-level documentation an overview with a short example, and link to the guides rather than including them with `#![doc = include_str!(...)]`. Include only files inside the package directory: a guide outside it is missing from the published package, so its doctests would fail there. Test such a guide from the package that contains it.

## Use the shared testing tasks

Use the shared Bake test tasks for local verification and CI. Consult the installed `bake-test` context at `.agents/context/bake-test/testing.md` for canonical workflow files, task setup and invocation, coverage options, and downstream test configuration.
