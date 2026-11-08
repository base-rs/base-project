// Released under the MIT License.
// Copyright, 2026, by Samuel Williams.

//! Shared Base Rust project conventions and development tasks.
//!
//! Add this crate to a private `bake/` package to link the standard project
//! tasks and their published agent context into a repository.
use bake::{Context, Result};
use bake_agent_context as _;
use bake_cargo as _;
use bake_license as _;
use bake_markdown as _;
use bake_readme as _;
use bake_releases as _;
use bake_test as _;

mod markdown;

/// Refresh the standard project files after a Cargo version bump.
///
/// This applies the license headers of every tracked Rust file, so that each released file carries the correct license. The standard test workflow's `license:check` should already have caught any header this changes; a change here moves the line numbers that compile-fail snapshots record, so it is reported.
#[bake::task(name = "cargo:after_version_bump")]
pub fn after_version_bump(context: &mut Context, version: String) -> Result<()> {
    let tag = bake_cargo::release_tag(context, &version)?;
    context.call("license:update", &[])?;
    report_changed_sources(context);
    context.call("releases:update", &[&tag])?;
    context.call("readme:update", &[])?;
    markdown::normalize(context)?;
    Ok(())
}

/// Warn, on standard error, about the tracked Rust files with uncommitted changes, which the release tasks' clean-tree requirement attributes to `license:update`.
fn report_changed_sources(context: &Context) {
    let sources = changed_sources(context);
    if !sources.is_empty() {
        eprintln!("{}", changed_sources_warning(&sources));
    }
}

fn changed_sources(context: &Context) -> Vec<String> {
    let output = context
        .command("git")
        .args(["diff", "--name-only", "--relative", "-z", "--", "*.rs"])
        .output()
        .ok()
        .filter(|output| output.status.success());
    // Without Git the release task cannot run; the report is only advice, so it is skipped.
    let Some(output) = output else {
        return Vec::new();
    };
    output
        .stdout
        .split(|byte| *byte == 0)
        .filter(|path| !path.is_empty())
        .map(|path| String::from_utf8_lossy(path).into_owned())
        .collect()
}

fn changed_sources_warning(sources: &[String]) -> String {
    format!(
        "warning: the release updated the license headers of {} Rust file(s), which `cargo bake license:check` in pull request CI should have caught: {}. Run the tests before merging the release, and update any compile-fail snapshots, such as trybuild `.stderr` files, whose line numbers moved.",
        sources.len(),
        sources.join(", "),
    )
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
