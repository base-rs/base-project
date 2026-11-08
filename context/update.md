---
metadata:
  type: skill
description: Audit and update an existing Rust repository against current Base conventions, shared Bake tasks, agent context, tests, and GitHub workflows. Use when asked to modernize or bring an existing crate up to date.
---

# Update a Base Rust Project

Use this skill to bring an existing crate repository up to the current Base baseline. For a new repository, use the `base-project-setup` skill. If an established repository lacks foundational Bake setup, use that skill for the initial setup steps.

## Read the project's guidance

- Follow the repository's `agents.md` and project-specific instructions.
- Discover and read applicable `design/*.md` before choosing changes. Use `base-design-review` to assess the plan and resulting changes against those requirements.
- Run `cargo bake agent:context:install` when shared task dependencies change or installed context may be missing or stale. Use `cargo bake agent:context:list` and `cargo bake agent:context:show` to discover guidance, then read the skills that apply to the work.
- Consult the current `base-project` context and the installed `bake-*` context for task behavior. Prefer those task guides over copying their implementation details into this skill.

## Audit the current baseline

Review the repository's Cargo manifests, source and test layout, standard root files, Bake package, agent context, and GitHub workflows. Compare them with the current guidance in `base-project` and the relevant shared task packages. Compare semantics, layout, and naming with related Base packages. Reuse patterns that fit; when this package establishes a new pattern, document its rationale and intended usage in the package context.

Check these areas:

- **Cargo and Bake setup:** package and workspace boundaries, the private `bake/` package, the shared `base-project` dependency, and generated task links.
- **Project files:** lowercase `readme.md`, `license.md`, and `releases.md`, with standard sections and generated content maintained by shared tasks.
- **License headers:** every tracked Rust file, including test fixtures, carries the header that `license:update` writes, so `cargo bake license:check` passes on a full clone. Run it to find files the next release would otherwise change.
- **Agent context:** reusable package guidance in `context/`, project-only instructions and skills in `.agents/`, and current installed context and skills.
- **Design requirements:** concrete project requirements in `design/*.md`, with scope and validation methods. Check that project instructions trigger discovery before implementation and that requirement changes accompany implementation changes. Use `base-design-author` to capture agreed requirements without inventing constraints merely to fill the directory.
- **Security records:** audit and advisory records in `security/`, and agent guidance that triggers `base-security-audit` and `base-security-advisory`. Do not create records without performing an audit.
- **Testing:** tests for behavior, the 100% region-coverage expectation, and downstream testing where public compatibility makes it useful. Use the `base-project-testing` skill and the Rust Testing Tasks guide supplied by `bake-test` for the current policy and task details.
- **GitHub workflows:** the standard test and publish workflows, including the test workflow's `license` job within the `test-result` aggregate, plus external testing when downstream projects are configured. Use the GitHub repository skill for repository settings and the `base-project-releasing` skill for publishing workflow setup. Consult the Bake Cargo Readme for task behavior, workflow generation, and trusted-publishing details.

Classify findings as required baseline changes, optional recommendations, or valid project-specific exceptions. Preserve deliberate local architecture and custom workflows when they do not conflict with an explicit organization convention.

## Apply updates with shared tasks

- Use `cargo bake --regenerate` after adding or changing task dependencies so generated task links match the Bake package.
- Use shared tasks such as `license:update`, `readme:update`, and `cargo:setup:workflow` where they apply. Review generated workflow changes before replacing project-specific configuration.
- Adopt the license check by updating the workspace lockfile so the private `bake/` package resolves a `base-project` that requires `bake-license` 0.2.9 or later, for example `cargo update -p base-project -p bake-license`, then `cargo bake --regenerate`. When `cargo bake license:check` fails, run `cargo bake license:update` in its own change, regenerate compile-fail snapshots whose line numbers moved (for trybuild, `TRYBUILD=overwrite cargo test --workspace`), run the tests, and commit the result, then add the `license` job to `test.yml` and its `test-result` aggregate as the `bake-test` canonical workflow shows. The check needs the full Git history; run `git fetch --unshallow` in a shallow clone.
- Use `cargo bake test:coverage` for the standard local coverage check. Consult `bake-test` for feature, target, and optional downstream test setup.
- Let `cargo:after_version_bump` refresh standard release files and license headers during release preparation; with `license:check` passing, it changes no sources. Follow the `base-project-releasing` skill for version bumps, validation, and the reviewed GitHub release process.
- Keep changes focused on the identified baseline gaps; do not rewrite valid project-specific choices merely to match an example layout.

Review the final diff and summarize completed baseline changes, optional recommendations, and any retained project-specific exceptions.
