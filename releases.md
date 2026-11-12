# Releases

## v0.1.12

- Require `bake-releases` 0.3.10, reject hand edits to `releases.md` in CI, and have the skills ask for user-facing pull request titles and `Breaking:` paragraphs instead of release-note entries. ([#88](https://github.com/base-rs/base-project/pull/88))

## v0.1.11

- Describe squash-only merging in the GitHub repository skill, and drop rebase merging from the releasing skill, which now refers to the GitHub repository skill for required checks.
- Keep title, commit, and description conventions in the pull request skill rather than repeating them in the GitHub repository skill. The pull request skill now says that titles become squash commit subjects and commit subjects are complete sentences, refers to the testing skill's 100% region coverage, and links to the Bake Releases readme for the `releases.md` format, since `bake-releases` supplies no context guide.

## v0.1.10

- Require bake-agent-context 0.4.5, bake-cargo 0.9.3, bake-license 0.2.11, bake-markdown 0.6.2, bake-readme 0.3.2, bake-releases 0.3.9, and bake-test 0.3.9, which require `bake` 0.20 or later instead of an open requirement. bake-cargo 0.9.3 also installs Bake launcher 0.20.2 in generated `publish.yml` and `release-artifacts.yml` workflows.
- Require `bake` 0.20.2, the version the task binary already resolved. It was held at 0.20.0 because, in a workspace declaring Rust 1.87, the task crates' open `bake` requirements resolved `bake` 0.19.2 alongside 0.20.2 and the `bake/` package failed to compile; with every task crate now requiring `bake` 0.20 or later, Cargo resolves a single `bake` 0.20.2.

## v0.1.9

- Require bake-agent-context 0.4.4, bake-cargo 0.9.2, bake-license 0.2.10, bake-markdown 0.6.1, bake-readme 0.3.1, bake-releases 0.3.8, bake-test 0.3.8, base-design 0.1.1, and base-security 0.2.1. The Bake task crates' releases declare Rust 1.88, and the edition 2024 resolver prefers versions compatible with a workspace's `rust-version`, so repositories declaring Rust 1.87 kept resolving their older releases; `base-design` and `base-security` declare 1.87 and are raised to match. A repository's `bake/` package already needed Rust 1.88 to build, since Bake 0.20 uses let-chains; the canonical `minimum-rust` job skips it as a private package. Keep the `bake` minimum at 0.20.0: in a Rust 1.87 workspace, requiring bake 0.20.2 makes the task crates' open `bake` requirements resolve bake 0.19.2 alongside it, and the task package fails to compile.

## v0.1.8

- Report on standard error when the release hook's `license:update` changes a Rust file, naming the files, since a new header moves the line numbers in compile-fail snapshots such as trybuild `.stderr` files. The hook still applies headers.
- Require bake-license 0.2.9, which provides `license:check`, and bake-test 0.3.7, whose canonical `test.yml` runs it in a `license` job within the `test-result` aggregate. Describe the check, and applying headers to every source, including test fixtures, from the start, in the setup, update, testing, pull request, GitHub repository, and releasing skills and the conventions; the hook's descriptions now say that it updates license headers.
- Describe in the testing skill why tests must not depend on wall-clock upper bounds on shared CI runners, and how to test timing deterministically with a controlled clock or ordering, leaving real-time properties to benchmarks or explicitly opted-in tests.
- Describe in the Package Organization guide how a foundation shared by several families, such as `base-message`, has no family, and record `base-message`'s scope.
- Describe in the testing skill how to compile `context/` guide and readme examples as doctests under `cfg(doctest)`, keeping guides out of the published API documentation.
- Add a Package Organization context guide, which explains the `base-http*`, `base-web-*`, and `bake-*` crate families, how to choose a crate's family, and that family members never depend on their umbrella or framework crate, and link it from the setup skill where crates are named.
- Collect benchmark packages with `base-benchmark` and the `bake-benchmark` tasks in the `base-project-benchmarks` skill, instead of new driver scripts, and build workload records with `base-benchmark`.
- Describe release artifacts in the `base-project-releasing` skill: attach them with Bake Cargo's `cargo:setup:artifacts` workflow, build the immutable release commit checked to be on `main`, keep signing secrets in a protected environment and give them to one step as files, verify fail-closed, and rely on `main`, the immutable commit, and verification rather than self-approved deployments.
- Protect release tags against updates as well as deletion in the GitHub repository guide, and protect signing environments like `crates-io`.

## v0.1.7

- Limit the pull request description to one paragraph in the `base-project-pull-requests` skill, since it becomes the squash-merged commit message: what changed, why, and the linked issue. Detailed investigation and discussion go in the issue, or in pull request comments when there is no issue.
- Rewrite the `base-project-releasing` skill around `cargo bake cargo:release:patch` (or `minor`, `major`, or `bump --version X.Y.Z`), which bumps the version, commits with the `Release: TAG` trailer, validates the candidate, and opens the release pull request in one step; `cargo:version:*` no longer exist.
- Require bake-cargo 0.7.0, which provides the `cargo:release:*` tasks and configures repositories to build squash merge commits from the pull request title and description. Describe that setting in the GitHub repository guide; it keeps the release trailer through review commits, so `gh pr merge --squash --body-file` is only needed for repositories not yet set up with `cargo:setup:github:apply`.

## v0.1.6

- Add guidance to the `base-project-testing` skill on excluding files from the coverage gate: a last resort for raw device, hardware, or platform calls, in a thin dedicated file listed with a reason.
- Require bake-test 0.3.5, which supports the per-file coverage exclusions that the `base-project-testing` skill describes.
- Add the `base-project-benchmarks` skill: private `benchmarks/<name>` packages named `<project>-benchmark-<name>`, measurement harnesses kept as tested package targets, diagnostic feature builds, and the layout of committed results. List `benchmarks/` in the repository layout guide.
- Extend the `base-project-performance` skill with a recorded baseline revision, verdict rules decided before a run, reporting large median moves and unmeasured design budget items, an exclusive lock on a shared benchmark host, and variants recorded as a base revision and a patch.

## v0.1.5

- Require giving the maintainers team access to a repository before running `cargo:setup:github:apply`, since GitHub silently drops environment reviewers without access, and show how to verify the `crates-io` environment's reviewers afterwards.
- Require Bake Cargo 0.6.1, whose `cargo:setup:github:apply` fails when a reviewer team cannot access the repository and verifies the saved environment reviewers.

## v0.1.4

- Include `base-security` guidance and skills for `security/` audit and advisory records, and enable private vulnerability reporting in the GitHub repository setup.

## v0.1.3

- Require no pull request approvals by default, so maintainers merge their own pull requests once checks pass, and document when a project should still require approvals. Require fork pull request approval for first-time contributors, and restrict publishing to `main` with self-review allowed.
- Add the `performance` skill describing how to measure, isolate, accept, and report performance optimizations.

## v0.1.2

- Document borrowed inputs, meaningful types, standard traits, and public API documentation conventions.
- Document ownership, macro, and constructor naming conventions.
- Document conversion and getter naming, owned inputs, readable control flow, and lint exceptions.
- Establish Rust 1.87 as the default minimum supported version, with guidance for declaring, verifying, and updating it.

## v0.1.1

- Document numeric limit naming and default constant conventions.

## v0.1.0

- Publish as `base-project` in the `base-rs` organization, with package versions starting at 0.1.0.

- Require the task providers migrated to the source-backed `base-markdown` document AST, including the `bake-markdown` API with parser types from `base-markdown`.

- Include `base-design` guidance and skills for project-specific `design/*.md` requirements, with discovery before implementation and review during pull request preparation.

- Use `base-markdown` throughout the shared task stack.

- Adopt the new agent-context installer and `metadata.type: skill` declarations. Discover guidance with `agent:context:list` and `agent:context:show`; the index and separate skill tasks are no longer available.

- Require a `Release: vVERSION` Git trailer on the final release commit; version changes alone no longer trigger publication.
