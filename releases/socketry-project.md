# Releases

## v0.4.0

- Document comment formatting: avoid fixed-width hard wrapping and use Markdown where helpful.

## v0.3.13

- Use named references between skills and agent context so guidance remains usable after installation.

## v0.3.11

- Document how to maintain dependency versions in installation examples and compatibility guidance, using manifests and lockfiles as authoritative sources.
- Replace duplicated release-tooling version lists with guidance for checking the resolved provider's manifest.

## v0.3.10

- Add independent adversarial review guidance, with local iteration until all non-trivial issues are resolved before opening a pull request.
- Clarify that the private Bake package shares the workspace's root lockfile.

## v0.3.9

- Follow Rust acronym casing in type and trait names, using `HtmlRenderer`, `HttpClient`, `UrlParser`, and `FileIo`.

## v0.3.8

- Let Cargo select setup dependency versions while retaining open-ended minimum requirements.
- Require current agent-context and readme providers so installed guidance preserves repository instructions and shared conventions.

## v0.3.7

- Require a stable aggregate test and coverage result alongside publishing checks.
- Clarify merge gates and repository-owned agent guidance in the shared conventions.

## v0.3.6

- Normalize standard project Markdown files after version bumps.
- Fix the version bump hook's invocation of the Markdown normalizer.
- Include `bake-markdown` 0.3.0 in the shared task set so version bumps use hyphen markers for unordered lists.

## v0.3.5

- Require `bake-test-rust` 0.3.0 or newer for LLVM region coverage and update the shared testing guidance to match.

## v0.3.4

- Require Bake 0.18.0 and Bake Cargo 0.4.0 for shared task registration.
- Document a test-only shim pattern for deterministic coverage of difficult I/O failures.

## v0.3.3

- Point the project update skill at the shared Releasing skill and Bake Cargo task documentation.

## v0.3.2

- Add a shared Releasing skill and remove links to duplicated Cargo publishing guidance.
- Update the setup instructions to use `socketry-project` 0.3.

## v0.3.1

- Add a testing skill that sets the 100% line-coverage expectation and directs agents to review uncovered code for semantic defects and use `bake-test-rust` for canonical workflows and task details.
- Document how to maintain consistency across Socketry packages and record new semantic, layout, and naming patterns in the package that establishes them.

## v0.3.0

- Add an update skill for auditing and modernizing existing Socketry Rust projects.

## v0.2.7

- Clarify how context installation uses `.agents/context/index.md` and local Git exclusions without changing a repository-owned `agents.md`.
- Express shared task dependencies as minimum versions so compatible updates can be selected by the host project's lockfile.
- Require 100% line coverage for workspace targets in GitHub Actions.
- Use the shared `bake-test-rust` coverage task in the standard workflow.
- Cover Bake task execution and the version-bump hook's task ordering.

## v0.2.6

- Support `bake-agent-context` 0.2 while retaining compatibility with 0.1.
- Keep the generated context index under `.agents/context/` without modifying the repository owner's `agents.md`.

## v0.2.5

- Distribute repository setup, GitHub setup, and pull request guidance as installable skills.
- Provide shared Agent Context guidance through `bake-agent-context`.

## v0.2.4

- Clarify crate namespaces, descriptive type names, module layout, and test paths.

## v0.2.3

- Rename the project guides to shorter context filenames and titles.
- Clarify source, test, naming, and coverage conventions.

## v0.2.2

- Use Bake Cargo's GitHub Release automation in the standard publishing workflow.

## v0.2.1

- Document setup and task linking with `cargo bake --regenerate`.

## v0.2.0

- Move the shared project tasks to `bake` 0.17 and include standard Rust test tasks.
- Document standard local, downstream, and publishing workflows.

## v0.1.1

- Add a reusable Pull Requests guide to the packaged agent context.

## v0.1.0

- Establish shared conventions and standard Bake tasks for Socketry Rust projects.
- Document project layout, GitHub setup, and agent context installation.
