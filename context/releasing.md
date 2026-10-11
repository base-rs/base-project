---
type: skill
description: Prepare and follow through a reviewed release for Socketry Rust projects. Use when asked to bump a version, publish a crate, or check release readiness.
---

# Releasing

Use this skill to prepare releases for Socketry Rust projects. Read the root `agents.md`, `readme.md`, and installed agent context for any project-specific release steps.

## Prepare a release

Keep one Cargo workspace when its publishable packages share a version, release notes, and tag. Use separate repositories for packages that need independent release timing. See the Repository and package boundaries section of the Conventions guide supplied by `socketry-project`.

Use the tasks linked by the project's private `bake/` package to bump the workspace version:

```sh
cargo bake cargo:version:patch
cargo bake cargo:version:minor
cargo bake cargo:version:major
cargo bake cargo:version:bump --version X.Y.Z
```

Choose one version task. The `socketry-project` hook updates `license.md`, `releases.md`, and generated sections of `readme.md`, then normalizes those files and all Markdown under the public `context/` directory. Review the generated changes and ensure the release notes describe the actual changes.

Run the project's required tests and coverage checks using the `socketry-project-testing` skill. Commit the version and release files, then run `cargo bake cargo:release` from the clean worktree. This validates and packages the release candidate; it does not publish or tag it. Follow the [adversarial review process in the socketry-project-pull-requests skill](https://github.com/socketry/socketry-project-rust/blob/main/context/pull-requests.md#adversarial-review), iterating locally until all non-trivial issues are resolved, then open the pull request after validation and review succeed.

## Publish through GitHub Actions

Generate `.github/workflows/publish.yml` with `cargo bake cargo:setup:workflow`. The canonical template is maintained in [`bake-cargo-rust/src/publish.yml`](https://github.com/socketry/bake-cargo-rust/blob/main/src/publish.yml). Keep repository-specific changes limited to the configured branch and update this shared template when the standard workflow changes.

Require both `check` from `publish.yml` and `test-result` from `test.yml` in the branch ruleset. The aggregate test result must depend on every required test and coverage job, including matrix jobs, and run with `if: always()` so failures, cancellations, and skipped prerequisites cannot silently bypass it. Ordinary tests run only on pushes in `publish.yml`; pull requests rely on the required testing workflow.

The check job installs the Bake launcher and runs `cargo:release:detect --base "$BAKE_BEFORE" --sha "$BAKE_SHA"`. That task compares the current workspace version with the base commit, checks the matching `releases.md` heading when the version changed, and writes the `release` and `version` GitHub outputs. Release candidates then run `cargo:release` before the formatting, Clippy, and test checks.

After a merge to the configured branch, the `crates-io` environment gates the publish job. `cargo:publish:pending --version "$BAKE_VERSION"` checks which workspace packages still need publishing. GitHub OIDC authentication runs only when packages are pending. `cargo:release:publish --version "$BAKE_VERSION" --sha "$BAKE_SHA"` publishes the remaining packages, then creates and pushes the version tag and creates or updates the matching GitHub Release from `releases.md` using `cargo:releases:github:release`.

These operations are Bake tasks. The supported task-provider requirements are declared in the resolved `socketry-project` package's `Cargo.toml`; the [source manifest](https://github.com/socketry/socketry-project-rust/blob/main/Cargo.toml) shows the current project's requirements. Use `cargo metadata --locked` to locate the resolved provider's manifest and confirm the versions used by the task binary. Update the workspace lockfile deliberately when adopting newer tooling, then regenerate task links with `cargo bake --regenerate`.

After merging, check the workflow result, published package versions, version tag, and GitHub Release. If a publish workflow is still waiting for environment approval, obtain that approval through the configured reviewer process.

## Set up publishing

For a new repository, use `cargo bake cargo:setup:workflow` to generate the workflow. Put required environment reviewers in `Cargo.toml`, preview settings with `cargo bake cargo:setup:github:plan`, and apply them with `cargo bake cargo:setup:github:apply` after reviewing the plan.

For a crate's first upload, `cargo bake cargo:bootstrap PACKAGE` publishes it with an owner token and registers its trusted publisher. Keep the token out of the repository. After the initial upload, use GitHub OIDC for routine releases; enable trusted-publishing-only mode after the workflow has published successfully.

For exact arguments and behavior of these tasks, consult the [Bake Cargo Readme](https://github.com/socketry/bake-cargo-rust/blob/main/readme.md) and each task's `--help` output.
