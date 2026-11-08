# License Headers

## Requirement

The `cargo:after_version_bump` release hook must keep applying `license:update`, so that every released file carries the license header and `license.md` that the repository's authorship requires.

The standard project setup must make that step a no-op for Rust sources in an up-to-date repository: the canonical test workflow, required through the `test-result` aggregate, must fail whenever `license:update` would change `license.md` or the header of any tracked Rust file, including test fixtures, in the commits under review, and the failure must name each file and the command that fixes it. It runs on each pull request's head commit and on each push to `main`; the push run catches what a squash merge changes, such as authors and dates. Output that the hook writes and then normalizes must pass that check, so a release does not make the next pull request fail.

When the hook does change a Rust file, it must still complete the release preparation, and it must report the changed files.

## Rationale

Adding a header to a source moves its line numbers. In base-web-content's first release, the hook added headers to its trybuild compile-fail fixtures, which shifted the compiler output that their `.stderr` snapshots record, and the failure was found only on the release pull request. Headers are applied at release deliberately, to guarantee the published license; the check moves the discovery to the pull request that introduced the file.

## Scope

The hook in `src/lib.rs`; `license:check` and `license:update` from `bake-license`, which the hook and the check share; the canonical `test.yml` in `bake-test`'s testing guide; and the setup, update, testing, pull request, GitHub repository, and releasing skills that describe them.

## Validation

- `src/tests.rs` runs the real hook on a repository whose fixture lacks a header: `license:check` fails before it, naming the fixture and `cargo bake license:update`, and passes on the committed result, with more than one author in `license.md`.
- Tests show that the hook completes when a Rust file has changed, that the changed files are listed, and the warning's text; the warning is written to standard error, which the tests do not capture.
- Inspect the canonical `test.yml` for a `license` job, within `test-result`'s `needs`, that runs `license:check` on the commits under review with full history, on pull requests and on pushes to `main`.
