# Releases

See [socketry-project release history](releases/socketry-project.md) for releases before the rename.

## v0.2.0

- Use `core-markdown` throughout the shared task stack.
- Adopt the new agent-context installer and `metadata.type: skill` declarations. Discover guidance with `agent:context:list` and `agent:context:show`; the index and separate skill tasks are no longer available.

## v0.1.0

- Publish as `core-project` in the `core-rs` organization, with version numbering restarted at `0.1.0`.
- Require a `Release: vVERSION` Git trailer on the final release commit; version changes alone no longer trigger publication.
