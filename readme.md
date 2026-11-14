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

### v0.1.13

- Check pushes to `main` for hand edits to `releases.md`, against the tip before the push, so one that passed its pull request's check still fails. ([#90](https://github.com/base-rs/base-project/pull/90))

### v0.1.12

- Require `bake-releases` 0.3.10, reject hand edits to `releases.md` in CI, and have the skills ask for user-facing pull request titles and `Breaking:` paragraphs instead of release-note entries. ([#88](https://github.com/base-rs/base-project/pull/88))

### v0.1.11

- Describe squash-only merging in the GitHub repository skill, and drop rebase merging from the releasing skill, which now refers to the GitHub repository skill for required checks.
- Keep title, commit, and description conventions in the pull request skill rather than repeating them in the GitHub repository skill. The pull request skill now says that titles become squash commit subjects and commit subjects are complete sentences, refers to the testing skill's 100% region coverage, and links to the Bake Releases readme for the `releases.md` format, since `bake-releases` supplies no context guide.

<!-- bake-readme:releases:end -->

## See Also

- [`bake`](https://github.com/base-rs/bake).
- [`bake-test`](https://github.com/base-rs/bake-test).

## Contributing

Please open an issue or pull request on [GitHub](https://github.com/base-rs/base-project).

### Agent Context

Run `cargo bake agent:context:install` to install shared context and skills. Use `cargo bake agent:context:list` and `cargo bake agent:context:show` to discover relevant guides, follow `agents.md` if present, and apply skills under `.agents/skills/`. The installer preserves repository-owned `agents.md`; it does not create or regenerate that file.
