# Releases

## Unreleased

- Add the `security-triage` skill describing how to review, fix, accept or close, backport, and publish a GitHub Security Advisory.

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
