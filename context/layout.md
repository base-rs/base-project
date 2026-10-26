# Rust Repository Layout

Use Cargo's standard structure so contributors can find package code, tests, examples, and project documentation quickly. Add directories as the project needs them; a small crate does not need every directory shown here.

```text
project/
├── Cargo.toml
├── src/
│   └── lib.rs
├── tests/
├── examples/
├── context/
├── design/
├── bake/
├── .agents/
├── .github/
├── readme.md
├── license.md
└── releases.md
```

## Cargo package and source modules

The root `Cargo.toml` defines the package and, when needed, the workspace. Put library code under `src/`, integration tests under `tests/`, and runnable examples under `examples/`. See [Conventions](conventions.md) for package and workspace boundaries, and the `base-project-setup` skill for the private Bake package.

## Source modules and files

Organize source by subsystem so the module tree is visible. For a module with children, use the modern file-plus-directory layout: a same-named source file and directory. For example, `src/parser.rs` defines `parser` and declares children whose files live under `src/parser/`:

```text
src/
├── lib.rs
├── parser.rs
└── parser/
    ├── block_parser.rs
    └── inline_parser.rs
```

Declare each source module in its parent. Keep module and directory names aligned; avoid `mod.rs` for new modules. Follow the [Base Rust naming conventions](conventions.md#source-naming) for Rust items and source files.

Cargo package metadata belongs in `Cargo.toml`. Follow the `base-project-releasing` skill for the release workflow and consult Bake Cargo task documentation for package inclusion details.

## Root files

The example project tree shows the standard root documentation files. See [Conventions](conventions.md#source-and-documentation) for their naming and content requirements.

Keep current project-specific requirements in `design/*.md`. Follow `base-design-author` for document scope, validation, and lifecycle, and `base-design-review` to assess plans and changes against the requirements. Create this directory when the project has concrete requirements to capture.

## Test layout

Prefer to mirror the source organization in tests. Keep small unit test suites within the module they exercise, using an inline `#[cfg(test)] mod tests`. When a suite grows, move it into that module's directory; for example, `src/parser/inline_parser.rs` can declare tests from `src/parser/inline_parser/tests.rs`. This keeps tests able to access private implementation details without exposing them as public API.

Put integration tests under `tests/` to check the public crate interface. A multi-file suite can be grouped by subsystem, with `main.rs` as the test target root and sibling files as test modules:

```text
tests/
└── parser/
    ├── main.rs
    ├── block_parser.rs
    └── inline_parser.rs
```

Declare child modules from `main.rs`; Cargo discovers the target root and the declared modules organize the suite. Integration tests exercise the public crate API. See the [Cargo integration test layout](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#integration-tests).

Put runnable examples under `examples/`. Store project tool configuration in a clearly named configuration file or the relevant Cargo metadata; avoid adding a configuration directory without a concrete tool that uses it.

Follow the `base-project-testing` skill for testing expectations and consult the installed `bake-test` context for task and workflow details. Use the `base-project-setup` skill for workflow setup.
