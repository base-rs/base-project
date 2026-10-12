# Conventions

Use these conventions for Rust repositories in the Socketry organization. A project's local `.agents/` files and package-specific documentation can add requirements for that repository.

Choose the simplest structure that makes the behavior, state, and responsibilities easy to follow. Use this principle to guide tradeoffs when applying these conventions.

## Repository and package boundaries

- Use a separate repository when packages need independent versions, release notes, or tags.
- Keep packages in one workspace when they are intentionally released together: use one shared version, one root `releases.md`, and one `vVERSION` tag.
- Name crates for their public purpose. Use the `socketry-` prefix where needed to identify Socketry packages in the flat crates.io namespace; keep Rust module paths semantic and concise.
- Use Cargo's standard `src/`, `tests/`, and `examples/` directories. Use integration tests for public behavior across crate boundaries.

## Crate and module paths

Treat the Cargo crate name as the root namespace. Do not repeat it with a same-named top-level module. Keep modules for meaningful domain concepts, and re-export the main public API from the crate root when deeper modules only organize its implementation. For example, callers can import the agent-context API from the root:

```rust
use bake_agent_context::{Installer, install_skills, list_skills};
```

Keep public paths independent of redundant namespace layers such as `bake_agent_context::context`. Public modules remain useful when they name a real part of the API: a crate named `protocol_http` can expose `protocol_http::headers::AcceptHeader`, where `headers` identifies the domain concept within the crate.

## Source naming

- Use `snake_case` for modules, source files, functions, methods, and variables. Use `UpperCamelCase` for structs, enums, traits, and type parameters.
- Treat acronyms and initialisms as single words in `UpperCamelCase`, following the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/naming.html#casing-conforms-to-rfc-430-c-case): write `HtmlRenderer`, `HttpClient`, `UrlParser`, and `FileIo`. Use lowercase acronyms in `snake_case`, including filenames such as `html_renderer.rs`, `http_client.rs`, and `file_io.rs`. This keeps Socketry's Rust APIs consistent with the Rust ecosystem; names required by external APIs retain their original spelling.
- Make public type and trait names fully descriptive; include the kind of thing being named instead of relying on the module path to supply it. For example, use `io_stream::BufferedStream` rather than `io_stream::Buffered`.
- Prefer clear, consistent names. Avoid abbreviations unless they are an established domain initialism or required by an external API.
- Give each primary public struct, enum, or trait its own source file, named after the item using lowercase `snake_case`. Keep small, closely related helper types alongside it when that makes the code easier to understand.

## Consistency across packages

Before introducing a new semantic, layout, or naming pattern, check how related Socketry packages handle the same concern. Reuse an established pattern when it fits the package's purpose. When a package establishes or changes a pattern, document the rationale and intended usage in that package's context so later work has a reference. Preserve differences that reflect real domain needs; consistency should make related packages easier to understand, not flatten their APIs into one shape.

## Construction and operations

Keep `new` and `Default` limited to establishing supplied state, creating simple owned values, and cheap normalization. Name substantive work explicitly with operations such as `load`, `parse`, `discover`, `prepare`, `start`, or `install`. A `load(path)` operation can read a file and delegate to `parse`. Choose phases that give callers a useful boundary, and return `Result` for expected failures.

Keep derived values with the type that owns their inputs. Choose explicit preparation or computation on demand according to cost and lifecycle. Make filesystem access, dependency discovery, process execution, and runtime startup visible in the calling code.

## Inputs and API contracts

Use direct function parameters for required inputs. Introduce options or configuration types when they clarify a real set of choices for callers.

Work directly with the expected data structure. Supply simple defaults for absent optional values, and keep any necessary shape validation at the parsing boundary. Accessors should expose the stored value, and defaults should represent expected absence. Keep malformed data distinguishable from missing input.

Keep predicates focused on the question they answer. For extensible metadata, recognize the values the consumer understands and use the format's defined fallback for unfamiliar values. This lets existing consumers continue processing data when providers introduce new types.

Use the format's canonical names. Add aliases and accepted forms when a concrete requirement calls for them, keeping the vocabulary small and consistent.

## Responsibility and readable flow

Give each domain rule and file format one owner. Generic parsing belongs in a shared document or protocol module; renderers and installers consume its interpretation. Use the parser library's syntax tree and source locations for syntax it already supports. Keep provider metadata, installation ownership, and command presentation as separate concerns.

Keep task adapters small: translate arguments, call a library operation, and format the result. Put the workflow in the library so programmatic callers and commands use the same behavior.

Make a multi-step function read as a sequence of domain operations. Introduce each logical phase with a short comment explaining the behavior it implements. Keep related calculations, assignments, and branches together under that comment, and separate phases with blank lines. A comment introduces the following code with a trailing colon.

Use documentation comments to explain the overall operation and its contract. Internal comments explain what each phase accomplishes, how it contributes to the operation, and any relevant ordering constraints, ownership, or reasoning. For example, this simplified method within a scheduler implementation groups timing measurements and accumulation:

```rust
/// Record the busy and idle time for this event loop iteration.
fn record_load(&mut self, start: Instant) {
    // Measure the elapsed time and the portion spent waiting for events:
    let elapsed = start.elapsed();
    let idle = self.selector.idle_duration();
    let busy = elapsed - idle;

    // Accumulate both durations for subsequent load reporting:
    self.busy_time += busy;
    self.idle_time += idle;
}
```

A function implementing a single step can usually be explained by its documentation comment. For example, within a buffered body:

```rust
/// Append a chunk to the buffered body.
fn write(&mut self, chunk: Vec<u8>) {
    self.chunks.push(chunk);
}
```

Choose comment granularity by behavior: explain the purpose of a group of statements in domain terms. Add an internal comment to a small function when it explains a detail beyond the function's contract. Keep the normal flow visible alongside the failure handling needed to understand its guarantees.

Keep a cohesive sequence together when its shared state and ordering are easiest to follow locally. Extract helpers when they separate a distinct responsibility or name an independently meaningful operation. Give intermediate values meaningful names, expand dense expressions, and keep helpers private until callers need the contract. Use structs for related state, modules for related operations, and traits for capabilities that concrete consumers need.

## Effects and failure boundaries

Validate data as part of loading or parsing it, and use that validated representation when applying changes. Prefer straightforward operations with explicit effects and simple error handling.

A sequence such as `Foo::load(path)?.write(path)?` is appropriate for regenerable local state. For generated files that can be recreated by rerunning the operation, prefer a direct `std::fs::write`. Completed steps can remain in place when a later step fails; report the failure so the caller can rerun the operation. Keep cleanup near the effect it protects.

Distinguish missing optional input from malformed or unreadable input, and report the relevant source or destination.

Match error handling and recovery to the operation's actual requirements and keep them easy to follow. Use focused tests for observable success and failure behavior.

## Alignment across implementations

When Rust and Ruby implement the same feature, align domain concepts, responsibilities, data flow, and observable behavior. Keep a short mapping of corresponding components, and document intentional differences caused by language semantics or package resolution. Use idiomatic Rust ownership, data types, modules, and functions to express the shared design.

Keep portable formats and behavioral rules in a shared specification. Test each implementation against the agreed behavior. Use shared fixtures where they make a common format or rule easier to verify. Compare exact bytes when formatting or preservation is part of the contract; otherwise compare meaning.

Before a substantial reorganization, record the current responsibilities, the problem with their boundaries, the proposed data flow, and an incremental implementation sequence in a package design guide. Label proposed APIs and behavior clearly, then update the guide as implementation progresses. Promote reusable lessons into shared conventions and keep domain-specific decisions in the package's context.

## Source and documentation

- Keep authored repository-root Markdown files to lowercase `readme.md`, `license.md`, and `releases.md`.
- Start `license.md` with `# MIT License`.
- Keep `readme.md` human-focused: explain the project, its motivation when useful, how to use it, recent releases, and how to contribute. Follow the `Readme Structure` guide supplied by `bake-readme`.
- Put reusable, package-specific agent guidance in `context/`; keep repository-only instructions and project-owned skills in `.agents/`. Treat the root `agents.md` as repository-owned guidance.
- Refer to skills by their installed names and context guides by their package and title when crossing between skills and context. The installer places them in separate directories, so relative links between them do not survive installation. Keep Markdown links within context files installed together or to stable external documentation.
- Avoid duplicating Rust-wide guidance from `bake-agent-context`; add project context for the architecture and decisions that are specific to the crate.
- Follow the Agent Context section in `readme.md` to install and discover shared context and skills. The `bake-agent-context` guide explains the installer's behavior.

## Dependency versions in documentation

- Prefer installation commands such as `cargo add socketry-markdown` or `cargo add --manifest-path bake/Cargo.toml socketry-project`. Let Cargo select the dependency and write its manifest requirement. See [Cargo's `cargo add` documentation](https://doc.rust-lang.org/cargo/commands/cargo-add.html).
- Treat `Cargo.toml` as the authority for supported dependency requirements and the workspace's root `Cargo.lock` as its resolved selection, following [Cargo's manifest and lockfile guidance](https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html). Link to the relevant manifest; `cargo metadata --locked` identifies resolved package versions and manifest paths. Use the resolved provider's manifest when checking installed tooling.
- Describe required capabilities, APIs, and task names in prose. When a numeric minimum explains a compatibility boundary, keep it in one provider-owned guide with the reason and a link to the supporting release or API documentation. Other guides should refer to that source.
- Keep explicit versions where they convey necessary information: migration instructions, historical release notes, minimum supported Rust versions, or reproducible examples tied to a particular release. Preserve historical references when newer releases appear.
- Use manifest snippets when explaining dependency requirements or features, and validate them against the supported API. Choose ranges according to the package's compatibility policy. The open-ended minimum for private `socketry-project` tooling follows its setup guidance; runtime libraries use requirements appropriate to their supported APIs.
- When a version must appear in several current documents, generate those references from authoritative metadata as part of the documentation tasks.

## Development and releases

- Keep development tasks in a private `bake/` package. Depend on `socketry-project` there so task tooling does not become a runtime dependency of the published library.
- Use the `cargo:after_version_bump` hook registered by `socketry-project` to update `license.md`, `releases.md`, and the generated sections of `readme.md`, then normalize those files and all Markdown under the public `context/` directory.
- Follow the `socketry-project-pull-requests` skill, including independent adversarial review and local iteration until all non-trivial issues are resolved.
- Follow the `socketry-project-releasing` skill to prepare and publish a release. It links to Bake Cargo task documentation for workflow setup, trusted publishing, reviewers, and tag creation.
