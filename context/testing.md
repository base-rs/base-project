---
metadata:
  type: skill
description: Add or update tests in Base Rust projects, require 100% region coverage, and decide when downstream compatibility tests are useful. Use when asked to add tests or when behavior changes need regression coverage.
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

Shared CI runners, such as GitHub Actions', are heavily oversubscribed, so any thread can stall for hundreds of milliseconds. A test that measures elapsed wall-clock time, or expects something to happen within a wall-clock bound, such as sleeping for 50 ms and expecting completion within 100 ms, will fail there intermittently. Widening the bound is not a fix.

- Make such tests deterministic. Control the clock with an injectable clock or a manual time source that the test advances, or assert on ordering, events, counts, or causality rather than elapsed time.
- A lower bound that the code guarantees, such as a timer not firing before its deadline, is fine, because slowness cannot break it. An upper bound on elapsed real time is not.
- Check properties that genuinely need real time, such as latency or throughput, in a benchmark, following `base-project-benchmarks`, or in a test that must be opted into explicitly, such as an `#[ignore]` test, so it does not run on shared CI by default. Document how and where to run it.
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
