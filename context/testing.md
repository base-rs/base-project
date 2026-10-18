---
metadata:
  type: skill
description: Add or update tests in Core Rust projects, require 100% region coverage, and decide when downstream compatibility tests are useful. Use when asked to add tests or when behavior changes need regression coverage.
---

# Testing Core Rust Projects

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

## Use the shared testing tasks

Use the shared Bake test tasks for local verification and CI. Consult the installed `bake-test` context at `.agents/context/bake-test/testing.md` for canonical workflow files, task setup and invocation, coverage options, and downstream test configuration.
