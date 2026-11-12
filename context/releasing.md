---
metadata:
  type: skill
description: Prepare and follow through a reviewed release for Base Rust projects. Use when asked to bump a version, publish a crate, or check release readiness.
---

# Releasing

Use this skill to prepare releases for Base Rust projects. Read the root `agents.md`, `readme.md`, and installed agent context for any project-specific release steps.

## Prepare a release

Keep one Cargo workspace when its publishable packages share a version, release notes, and tag. Use separate repositories for packages that need independent release timing. See the Repository and package boundaries section of the Base Rust Conventions guide supplied by `base-project`.

Start the release with one task from a clean checkout of `main` with no uncommitted tracked changes:

```sh
cargo bake cargo:release:patch
cargo bake cargo:release:minor
cargo bake cargo:release:major
cargo bake cargo:release:bump --version X.Y.Z
```

Choose the task matching the change. It creates a `release-TAG` branch, bumps the workspace version, and runs the `base-project` hook, which updates `license.md`, the license headers of every tracked Rust file, and generated sections of `readme.md`, generates the new version's section of `releases.md` from Git history, then normalizes those files and all Markdown under the public `context/` directory. Applying headers at release guarantees that every published file carries the correct license. The test workflow's `license` job runs `license:check` on every pull request and push to `main`, so headers should already be correct and the release commit should change no sources. If the hook does change a Rust file, it prints a warning naming the files: a new header moves line numbers, so run the tests and update compile-fail snapshots, such as trybuild `.stderr` files, on the release branch before merging. It commits the result as `Bump patch version.` (or minor, major, or `Bump version to X.Y.Z.`) with exactly one `Release: TAG` Git trailer, validates and packages the committed candidate with `cargo:release`, pushes the branch, and opens the pull request with the release notes followed by the same trailer as its description. It does not tag or publish. Pass `--branch NAME` to release from another branch.

The tag is the shared Cargo version with the configured prefix, which defaults to `v`, producing tags such as `v0.1.0`. A project can override it with `tag-prefix` in `[workspace.metadata.bake.release]`, or `[package.metadata.bake.release]` for a single package without a workspace; see Bake Cargo's release-tag configuration for allowed prefixes. The same full tag identifies the release-notes heading, the trailer, and the pull request description.

Before starting, preview the notes with `cargo bake releases:notes Unreleased`. There is nothing to edit, because the lines come from the commit subjects since the previous release tag, so a poor line is fixed by writing clearer titles from then on; the task refuses to start when there is nothing to release. Also make sure the test workflow passes on `main`, including its `license` job: a squash merge records one author at the merge date, so a pull request can pass its own license check and still leave headers on `main` that the release would change. Fix any it reports with `cargo bake license:update` in a pull request before releasing. The task refuses a tree with uncommitted tracked changes and leaves the release branch in place when validation fails after the commit, so a failed release can be inspected and fixed on that branch. A release made from a repository that is not set up as described below needs the trailer preserved by hand when merging.

Review the release pull request as any other change: check the generated commit, run the project's required tests and coverage checks using the `base-project-testing` skill when they differ from the required workflow checks, and follow the [adversarial review process in the base-project-pull-requests skill](https://github.com/base-rs/base-project/blob/main/context/pull-requests.md#adversarial-review) for anything beyond the generated version and release-note changes. When a review fix is needed, amend the release commit or give the fix-up commit the same trailer, so the `check` job validates the candidate before merge; the merged commit's message comes from the pull request description, so a fix-up without the trailer still releases but without that validation.

## Publish through GitHub Actions

Generate `.github/workflows/publish.yml` with `cargo bake cargo:setup:workflow`. The canonical template is maintained in [`bake-cargo/src/publish.yml`](https://github.com/base-rs/bake-cargo/blob/main/src/publish.yml). Keep repository-specific changes limited to the configured branch and update this shared template when the standard workflow changes.

The branch ruleset requires `check` from `publish.yml` and `test-result` from `test.yml`, as the `base-project-github-repository` skill describes. Ordinary tests run only on pushes in `publish.yml`; pull requests rely on the required testing workflow.

The check job installs the Bake launcher and runs `cargo:release:detect --base "$BAKE_BEFORE" --sha "$BAKE_SHA"`. That task reads the `Release: TAG` Git trailer from the specified commit and validates it against the checked-out workspace version, release notes, base package identities, and existing tag. The release notes must be the ones generated from the target branch's history, so a release pull request whose notes were edited, or that was prepared before another change was merged, fails until it is prepared again from the updated branch. The workflow supplies the PR head SHA for trailer inspection while testing the merge candidate; pushes inspect the actual branch tip. Missing trailers mean no publication. Duplicate, empty, or mismatched release trailers fail validation. Versions prepared in an earlier commit may be released, and renamed packages start their own version history. The task writes the `release` and `version` GitHub outputs. Release candidates then run `cargo:release` before the formatting, Clippy, and test checks.

The trailer must end the merged commit's message. Repositories set up with `cargo:setup:github:apply` build squash commits from the pull request title and description, so a release pull request whose description ends with the trailer merges correctly from the GitHub UI or `gh pr merge --squash`, whatever review commits were added. Do not edit the trailer out of the description before merging. In a repository without that setting, GitHub concatenates the commit messages and a review commit buries the trailer; pass a body ending with the trailer using `gh pr merge --squash --body-file`, or run `cargo bake cargo:setup:github:apply` first. Check the merged commit's trailer before approving publication.

After a merge to the configured branch, the `crates-io` environment gates the publish job. `cargo:publish:pending --version "$BAKE_VERSION"` checks which workspace packages still need publishing. GitHub OIDC authentication runs only when packages are pending. `cargo:release:publish --version "$BAKE_VERSION" --sha "$BAKE_SHA"` publishes the remaining packages, then creates and pushes the version tag and creates or updates the matching GitHub Release from `releases.md` using `cargo:releases:github:release`.

These operations are Bake tasks. The supported task-provider requirements are declared in the resolved `base-project` package's `Cargo.toml`; the [source manifest](https://github.com/base-rs/base-project/blob/main/Cargo.toml) shows the current project's requirements. Use `cargo metadata --locked` to locate the resolved provider's manifest and confirm the versions used by the task binary. Update the workspace lockfile deliberately when adopting newer tooling, then regenerate task links with `cargo bake --regenerate`.

After merging, check the workflow result, published package versions, version tag, and GitHub Release. If a publish workflow is still waiting for environment approval, obtain that approval through the configured reviewer process.

## Release artifacts

Attach files other than crates, such as application packages, with Bake Cargo's release artifacts workflow (Bake Cargo 0.8.0 or later) rather than a workflow of the project's own. Generate it with `cargo bake cargo:setup:artifacts --environment NAME --secrets SECRET`, repeating `--secrets` as needed, and supply only executable `scripts/release-artifacts/build`, `sign`, and `verify` scripts; the workflow supplies the release checks. Follow the same rules when writing those scripts or reviewing any release workflow:

- Build the immutable release commit by its SHA, checked to be on `main` with a matching `Release:` trailer and version, including for manual runs. Never build from a tag or branch name, which may name another commit by the time the job runs.
- Keep signing secrets in a GitHub environment restricted to `main`, configured with `cargo bake cargo:setup:github:apply --environment NAME`. Give them to one step only, as `0600` files under `$RUNNER_TEMP` removed in an `if: always()` step, never to a build, where every build script and dependency could read them. Pass passwords to tools through files or environment variables, not command-line arguments.
- Verify fail-closed: capture a tool's output before testing it (`output=$(tool ...)`, then `grep` the variable) so a failing tool fails the check, print why a check failed, and compare signing certificates with a digest committed to the repository.
- Maintainers and their agents may approve their own deployments, so environment approval is not independent review. The protection rests on the protected `main` branch, the immutable release commit and tag, and verification.

## Set up publishing

For a new repository, use `cargo bake cargo:setup:workflow` to generate the workflow. Put required environment reviewers in `Cargo.toml`, preview settings with `cargo bake cargo:setup:github:plan`, and apply them with `cargo bake cargo:setup:github:apply` after reviewing the plan.

For a crate's first upload, `cargo bake cargo:bootstrap PACKAGE` publishes it with an owner token and registers its trusted publisher. Keep the token out of the repository. After the initial upload, use GitHub OIDC for routine releases; enable trusted-publishing-only mode after the workflow has published successfully.

For exact arguments and behavior of these tasks, consult the [Bake Cargo Readme](https://github.com/base-rs/bake-cargo/blob/main/readme.md) and each task's `--help` output.
