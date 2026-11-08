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

### v0.1.7

- Limit the pull request description to one paragraph in the `base-project-pull-requests` skill, since it becomes the squash-merged commit message: what changed, why, and the linked issue. Detailed investigation and discussion go in the issue, or in pull request comments when there is no issue.
- Rewrite the `base-project-releasing` skill around `cargo bake cargo:release:patch` (or `minor`, `major`, or `bump --version X.Y.Z`), which bumps the version, commits with the `Release: TAG` trailer, validates the candidate, and opens the release pull request in one step; `cargo:version:*` no longer exist.
- Require bake-cargo 0.7.0, which provides the `cargo:release:*` tasks and configures repositories to build squash merge commits from the pull request title and description. Describe that setting in the GitHub repository guide; it keeps the release trailer through review commits, so `gh pr merge --squash --body-file` is only needed for repositories not yet set up with `cargo:setup:github:apply`.

### v0.1.6

- Add guidance to the `base-project-testing` skill on excluding files from the coverage gate: a last resort for raw device, hardware, or platform calls, in a thin dedicated file listed with a reason.
- Require bake-test 0.3.5, which supports the per-file coverage exclusions that the `base-project-testing` skill describes.
- Add the `base-project-benchmarks` skill: private `benchmarks/<name>` packages named `<project>-benchmark-<name>`, measurement harnesses kept as tested package targets, diagnostic feature builds, and the layout of committed results. List `benchmarks/` in the repository layout guide.
- Extend the `base-project-performance` skill with a recorded baseline revision, verdict rules decided before a run, reporting large median moves and unmeasured design budget items, an exclusive lock on a shared benchmark host, and variants recorded as a base revision and a patch.

<!-- bake-readme:releases:end -->

## See Also

- [`bake`](https://github.com/base-rs/bake).
- [`bake-test`](https://github.com/base-rs/bake-test).

## Contributing

Please open an issue or pull request on [GitHub](https://github.com/base-rs/base-project).

### Agent Context

Run `cargo bake agent:context:install` to install shared context and skills. Use `cargo bake agent:context:list` and `cargo bake agent:context:show` to discover relevant guides, follow `agents.md` if present, and apply skills under `.agents/skills/`. The installer preserves repository-owned `agents.md`; it does not create or regenerate that file.
