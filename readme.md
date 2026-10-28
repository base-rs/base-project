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

In `bake/Cargo.toml`, prefix the version selected by Cargo with `>=` to make it an open-ended minimum. Keep that actual minimum in the manifest and the resolved version in `Cargo.lock`; the setup documentation does not need a copy of either version.

Then run `cargo bake --regenerate` again to link the dependency's tasks. The command keeps generated links separate from task source, so no manual import in `main.rs` is needed. Run `cargo bake --list` to see the available tasks. The open-ended minimum requirement keeps this development dependency eligible for newer releases. `Cargo.lock` records the selected version for reproducible builds; update it deliberately when adopting a newer release.

This adds the standard Cargo, release, license, Readme, Markdown, agent-context, and testing tasks, including `test` and `test:external`. It also registers a version-bump hook that updates the project's standard files.

It also includes `base-design`, which provides the `design/*.md` convention and the `base-design-author` and `base-design-review` skills. Capture specific project requirements, consider them before implementation, and validate them during pull request review. `base-design` contributes guidance and skills without adding Bake tasks.

See the [repository setup skill](context/setup.md) for the minimal Cargo configuration and the [conventions](context/conventions.md) for repository layout, documentation, and code conventions. The [Rust Repository Layout](context/layout.md) explains source organization. The testing skill sets expectations and points to `bake-test` for task and workflow details.

## Releasing

Prepare a release with `cargo bake cargo:version:patch` (or `minor`, `major`, or `bump --version X.Y.Z`), commit with a `Release: TAG` Git trailer matching the configured release tag (for this repository, `vVERSION`), then run `cargo bake cargo:release` and open a pull request. After review and merge, GitHub Actions publishes the release when the configured `crates-io` environment approves it, then creates or updates the matching GitHub Release from `releases.md`. See the shared [Releasing skill](context/releasing.md) for the standard release process.

## Releases

<!-- bake-readme:releases:start -->

See [releases.md](releases.md) for the full release history.

### v0.1.1

- Document numeric limit naming and default constant conventions.

### v0.1.0

- Publish as `base-project` in the `base-rs` organization, with package versions starting at 0.1.0.

- Require the task providers migrated to the source-backed `base-markdown` document AST, including the `bake-markdown` API with parser types from `base-markdown`.

- Include `base-design` guidance and skills for project-specific `design/*.md` requirements, with discovery before implementation and review during pull request preparation.

- Use `base-markdown` throughout the shared task stack.

- Adopt the new agent-context installer and `metadata.type: skill` declarations. Discover guidance with `agent:context:list` and `agent:context:show`; the index and separate skill tasks are no longer available.

- Require a `Release: vVERSION` Git trailer on the final release commit; version changes alone no longer trigger publication.

<!-- bake-readme:releases:end -->

## See Also

- [`bake`](https://github.com/base-rs/bake).
- [`bake-test`](https://github.com/base-rs/bake-test).

## Contributing

Please open an issue or pull request on [GitHub](https://github.com/base-rs/base-project).

### Agent Context

Run `cargo bake agent:context:install` to install shared context and skills. Use `cargo bake agent:context:list` and `cargo bake agent:context:show` to discover relevant guides, follow `agents.md` if present, and apply skills under `.agents/skills/`. The installer preserves repository-owned `agents.md`; it does not create or regenerate that file.

Before planning or changing behavior, discover and read applicable `design/*.md`. Use `base-design-review` to assess plans and changes, and `base-design-author` when user requirements are introduced, changed, or removed. Review design compliance before opening or updating a pull request.

[Agent Context guide]: https://github.com/base-rs/bake-agent-context/blob/main/context/agent-context.md
