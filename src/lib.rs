// Released under the MIT License.
// Copyright, 2026, by Samuel Williams.

//! Shared Core Rust project conventions and development tasks.
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
#[bake::task(name = "cargo:after_version_bump")]
pub fn after_version_bump(context: &mut Context, version: String) -> Result<()> {
    context.call("license:update", &[])?;
    context.call("releases:update", &[&format!("v{version}")])?;
    context.call("readme:update", &[])?;
    markdown::normalize(context)?;
    Ok(())
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
