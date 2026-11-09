# `base-project`

Shared conventions and development tasks for Rust projects in the `base-rs` organization.

## Motivation

Rust projects should share a small set of clear conventions and a reliable release process without repeating task wiring in every repository.

## Usage

If `socketry-cargo-bake` is installed, run `cargo uninstall socketry-cargo-bake` first. Both packages provide the `cargo-bake` binary.

Install the launcher and create the private task package:

```sh
cargo install bake --locked
cargo bake --regenerate
```

Let Cargo select the current dependency version:

```sh
cargo add --manifest-path bake/Cargo.toml base-project
```

In `bake/Cargo.toml`, prefix the version selected by Cargo with `>=` to make it an open-ended minimum. Then run `cargo bake --regenerate` again to link the dependency's tasks. The command keeps generated links separate from task source, so no manual import in `main.rs` is needed. Run `cargo bake --list` to see the available tasks.

This adds the standard Cargo, release, license, Readme, Markdown, agent-context, and testing tasks, including `test` and `test:external`. It also registers a version-bump hook that updates the project's standard files and the license headers of its Rust files; `license:check` verifies in pull request CI that the hook would not change `license.md` or any Rust file's header.

It also includes `base-design`, which provides the `design/*.md` convention and the `base-design-author` and `base-design-review` skills. Capture specific project requirements, consider them before implementation, and validate them during pull request review. `base-design` contributes guidance and skills without adding Bake tasks.

It also includes `base-security`, which provides the `security/` audit and advisory record formats and the `base-security-audit` and `base-security-advisory` skills. Record which files were reviewed for security issues at which revision, so later audits review only what changed, and take vulnerabilities from private report to published advisory. Like `base-design`, it contributes guidance and skills without adding Bake tasks.

See the [repository setup skill](context/setup.md) for the minimal Cargo configuration and the [conventions](context/conventions.md) for repository layout, documentation, and code conventions. The [Rust Repository Layout](context/layout.md) explains source organization. The testing skill sets expectations and points to `bake-test` for task and workflow details. The [performance skill](context/performance.md) describes how to measure optimizations, decide which to keep, and report their results, and the [benchmarks skill](context/benchmarks.md) how to organize benchmark packages, harnesses, and committed results.

## Releasing

Prepare a release with `cargo bake cargo:release:patch` (or `minor`, `major`, or `bump --version X.Y.Z`) from a clean checkout of `main`; it bumps the version, commits with the `Release: vVERSION` trailer, validates the candidate, and opens the pull request. After review and merge, GitHub Actions publishes the release when the configured `crates-io` environment approves it, then creates or updates the matching GitHub Release from `releases.md`. Follow the shared [Releasing skill](https://github.com/base-rs/base-project/blob/main/context/releasing.md) for the standard process.

## Releases

<!-- bake-readme:releases:start -->

See [releases.md](releases.md) for the full release history.

### v0.1.10

- Require bake-agent-context 0.4.5, bake-cargo 0.9.3, bake-license 0.2.11, bake-markdown 0.6.2, bake-readme 0.3.2, bake-releases 0.3.9, and bake-test 0.3.9, which require `bake` 0.20 or later instead of an open requirement. bake-cargo 0.9.3 also installs Bake launcher 0.20.2 in generated `publish.yml` and `release-artifacts.yml` workflows.
- Require `bake` 0.20.2, the version the task binary already resolved. It was held at 0.20.0 because, in a workspace declaring Rust 1.87, the task crates' open `bake` requirements resolved `bake` 0.19.2 alongside 0.20.2 and the `bake/` package failed to compile; with every task crate now requiring `bake` 0.20 or later, Cargo resolves a single `bake` 0.20.2.

### v0.1.9

- Require bake-agent-context 0.4.4, bake-cargo 0.9.2, bake-license 0.2.10, bake-markdown 0.6.1, bake-readme 0.3.1, bake-releases 0.3.8, bake-test 0.3.8, base-design 0.1.1, and base-security 0.2.1. The Bake task crates' releases declare Rust 1.88, and the edition 2024 resolver prefers versions compatible with a workspace's `rust-version`, so repositories declaring Rust 1.87 kept resolving their older releases; `base-design` and `base-security` declare 1.87 and are raised to match. A repository's `bake/` package already needed Rust 1.88 to build, since Bake 0.20 uses let-chains; the canonical `minimum-rust` job skips it as a private package. Keep the `bake` minimum at 0.20.0: in a Rust 1.87 workspace, requiring bake 0.20.2 makes the task crates' open `bake` requirements resolve bake 0.19.2 alongside it, and the task package fails to compile.

### v0.1.8

- Report on standard error when the release hook's `license:update` changes a Rust file, naming the files, since a new header moves the line numbers in compile-fail snapshots such as trybuild `.stderr` files. The hook still applies headers.
- Require bake-license 0.2.9, which provides `license:check`, and bake-test 0.3.7, whose canonical `test.yml` runs it in a `license` job within the `test-result` aggregate. Describe the check, and applying headers to every source, including test fixtures, from the start, in the setup, update, testing, pull request, GitHub repository, and releasing skills and the conventions; the hook's descriptions now say that it updates license headers.
- Describe in the testing skill why tests must not depend on wall-clock upper bounds on shared CI runners, and how to test timing deterministically with a controlled clock or ordering, leaving real-time properties to benchmarks or explicitly opted-in tests.
- Describe in the Package Organization guide how a foundation shared by several families, such as `base-message`, has no family, and record `base-message`'s scope.
- Describe in the testing skill how to compile `context/` guide and readme examples as doctests under `cfg(doctest)`, keeping guides out of the published API documentation.
- Add a Package Organization context guide, which explains the `base-http*`, `base-web-*`, and `bake-*` crate families, how to choose a crate's family, and that family members never depend on their umbrella or framework crate, and link it from the setup skill where crates are named.
- Collect benchmark packages with `base-benchmark` and the `bake-benchmark` tasks in the `base-project-benchmarks` skill, instead of new driver scripts, and build workload records with `base-benchmark`.
- Describe release artifacts in the `base-project-releasing` skill: attach them with Bake Cargo's `cargo:setup:artifacts` workflow, build the immutable release commit checked to be on `main`, keep signing secrets in a protected environment and give them to one step as files, verify fail-closed, and rely on `main`, the immutable commit, and verification rather than self-approved deployments.
- Protect release tags against updates as well as deletion in the GitHub repository guide, and protect signing environments like `crates-io`.

<!-- bake-readme:releases:end -->

## See Also

- [`bake`](https://github.com/base-rs/bake).
- [`bake-test`](https://github.com/base-rs/bake-test).

## Contributing

Please open an issue or pull request on [GitHub](https://github.com/base-rs/base-project).

### Agent Context

Run `cargo bake agent:context:install` to install shared context and skills. Use `cargo bake agent:context:list` and `cargo bake agent:context:show` to discover relevant guides, follow `agents.md` if present, and apply skills under `.agents/skills/`. The installer preserves repository-owned `agents.md`; it does not create or regenerate that file.
