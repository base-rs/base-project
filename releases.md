# Releases

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
