// Released under the MIT License.
// Copyright, 2026, by Samuel Williams.

use bake::{Arguments, Parameter, Registry, Result, Task, Value};

fn project() -> tempfile::TempDir {
    let directory = tempfile::tempdir().unwrap();
    std::fs::write(
        directory.path().join("Cargo.toml"),
        "[package]\nname = 'fixture'\nversion = '0.1.0'\n",
    )
    .unwrap();
    directory
}

#[derive(Default)]
struct Calls {
    values: Vec<(String, Option<String>)>,
    markdown_paths: Vec<std::path::PathBuf>,
    fail_at: Option<String>,
}

fn record(context: &mut bake::Context, name: &str, value: Option<String>) -> Result<Value> {
    let calls = context.get_mut::<Calls>().expect("the test records calls");
    calls.values.push((name.to_owned(), value));

    if calls.fail_at.as_deref() == Some(name) {
        return Err(bake::Error::new("injected task failure"));
    }

    Ok(Value::Null)
}

fn update_license(context: &mut bake::Context, _: &Arguments) -> Result<Value> {
    record(context, "license:update", None)
}

fn update_releases(context: &mut bake::Context, arguments: &Arguments) -> Result<Value> {
    record(
        context,
        "releases:update",
        Some(arguments.required::<String>("version")?),
    )
}

fn update_readme(context: &mut bake::Context, _: &Arguments) -> Result<Value> {
    record(context, "readme:update", None)
}

fn normalize_markdown(context: &mut bake::Context, arguments: &Arguments) -> Result<Value> {
    let paths = arguments.repeated::<std::path::PathBuf>("paths")?;
    record(context, "markdown:normalize", None)?;
    context.get_mut::<Calls>().unwrap().markdown_paths = paths;
    Ok(Value::Null)
}

fn registry() -> Registry {
    let mut registry = Registry::discover().unwrap();
    registry
        .replace(
            "license:update",
            Task::new("license:update", "", vec![], update_license),
        )
        .unwrap();
    registry
        .replace(
            "releases:update",
            Task::new(
                "releases:update",
                "",
                vec![Parameter::new::<String>("version")],
                update_releases,
            ),
        )
        .unwrap();
    registry
        .replace(
            "readme:update",
            Task::new("readme:update", "", vec![], update_readme),
        )
        .unwrap();
    registry
        .replace(
            "markdown:normalize",
            Task::new(
                "markdown:normalize",
                "",
                vec![Parameter::new::<std::path::PathBuf>("paths").variadic()],
                normalize_markdown,
            ),
        )
        .unwrap();

    registry
}

#[test]
fn exports_markdown_normalization_with_hyphen_markers() {
    let directory = project();
    let path = directory.path().join("readme.md");
    std::fs::write(&path, "* first\n* second\n").unwrap();

    let registry = Registry::discover().unwrap();
    let mut context = registry.context(directory.path());
    context.call("markdown:normalize", &["readme.md"]).unwrap();

    assert_eq!(
        std::fs::read_to_string(path).unwrap(),
        "- first\n- second\n"
    );
}

#[test]
fn refreshes_project_files_in_order_for_the_requested_version() {
    let registry = registry();
    let directory = project();
    let mut context = registry.context(directory.path());
    context.insert(Calls::default());

    super::after_version_bump(&mut context, "0.3.0".to_owned()).unwrap();

    assert_eq!(
        context.get::<Calls>().unwrap().values,
        [
            ("license:update".to_owned(), None),
            ("releases:update".to_owned(), Some("v0.3.0".to_owned())),
            ("readme:update".to_owned(), None),
            ("markdown:normalize".to_owned(), None),
        ]
    );
}

#[test]
fn propagates_license_update_failure() {
    let registry = registry();
    let directory = project();
    let mut context = registry.context(directory.path());
    context.insert(Calls {
        fail_at: Some("license:update".to_owned()),
        ..Calls::default()
    });

    let error = super::after_version_bump(&mut context, "0.3.0".to_owned()).unwrap_err();

    assert_eq!(error.to_string(), "license:update: injected task failure");
    assert_eq!(
        context.get::<Calls>().unwrap().values,
        [("license:update".to_owned(), None)]
    );
}

#[test]
fn propagates_release_update_failure() {
    let registry = registry();
    let directory = project();
    let mut context = registry.context(directory.path());
    context.insert(Calls {
        fail_at: Some("releases:update".to_owned()),
        ..Calls::default()
    });

    let error = super::after_version_bump(&mut context, "0.3.0".to_owned()).unwrap_err();

    assert_eq!(error.to_string(), "releases:update: injected task failure");
    assert_eq!(
        context.get::<Calls>().unwrap().values,
        [
            ("license:update".to_owned(), None),
            ("releases:update".to_owned(), Some("v0.3.0".to_owned())),
        ]
    );
}

#[test]
fn propagates_readme_update_failure() {
    let registry = registry();
    let directory = project();
    let mut context = registry.context(directory.path());
    context.insert(Calls {
        fail_at: Some("readme:update".to_owned()),
        ..Calls::default()
    });

    let error = super::after_version_bump(&mut context, "0.3.0".to_owned()).unwrap_err();

    assert_eq!(error.to_string(), "readme:update: injected task failure");
    assert_eq!(
        context.get::<Calls>().unwrap().values,
        [
            ("license:update".to_owned(), None),
            ("releases:update".to_owned(), Some("v0.3.0".to_owned())),
            ("readme:update".to_owned(), None),
        ]
    );
}

#[test]
fn propagates_markdown_normalization_failure() {
    let registry = registry();
    let directory = project();
    let mut context = registry.context(directory.path());
    context.insert(Calls {
        fail_at: Some("markdown:normalize".to_owned()),
        ..Calls::default()
    });

    let error = super::after_version_bump(&mut context, "0.3.0".to_owned()).unwrap_err();

    assert_eq!(
        error.to_string(),
        "markdown:normalize: injected task failure"
    );
    assert_eq!(
        context.get::<Calls>().unwrap().values,
        [
            ("license:update".to_owned(), None),
            ("releases:update".to_owned(), Some("v0.3.0".to_owned())),
            ("readme:update".to_owned(), None),
            ("markdown:normalize".to_owned(), None),
        ]
    );
}

#[test]
fn passes_standard_and_recursive_context_paths_to_the_normalizer() {
    let directory = project();
    std::fs::create_dir_all(directory.path().join("context/nested")).unwrap();
    std::fs::write(directory.path().join("context/guide.md"), "Guide.\n").unwrap();
    std::fs::write(directory.path().join("context/nested/skill.md"), "Skill.\n").unwrap();

    let registry = registry();
    let mut context = registry.context(directory.path());
    context.insert(Calls::default());

    super::after_version_bump(&mut context, "0.3.0".to_owned()).unwrap();

    assert_eq!(
        context.get::<Calls>().unwrap().markdown_paths,
        [
            std::path::PathBuf::from("context/guide.md"),
            std::path::PathBuf::from("context/nested/skill.md"),
            std::path::PathBuf::from("license.md"),
            std::path::PathBuf::from("readme.md"),
            std::path::PathBuf::from("releases.md"),
        ]
    );
}

#[test]
fn rejects_a_missing_release_version_argument() {
    let registry = registry();
    let directory = project();
    let mut context = registry.context(directory.path());
    context.insert(Calls::default());

    let error = update_releases(&mut context, &Arguments::default()).unwrap_err();

    assert_eq!(error.to_string(), "missing argument \"version\"");
    assert!(context.get::<Calls>().unwrap().values.is_empty());
}

#[test]
fn registers_the_project_version_bump_hook() {
    let registry = Registry::discover().unwrap();
    assert!(
        registry
            .tasks()
            .any(|task| task.name() == "cargo:after_version_bump")
    );
}

#[test]
fn uses_the_configured_release_tag_for_notes() {
    let directory = project();
    std::fs::write(
        directory.path().join("Cargo.toml"),
        "[workspace.metadata.bake.release]\ntag-prefix = 'base-v'\n",
    )
    .unwrap();
    let registry = registry();
    let mut context = registry.context(directory.path());
    context.insert(Calls::default());
    super::after_version_bump(&mut context, "0.1.0".to_owned()).unwrap();
    assert_eq!(
        context.get::<Calls>().unwrap().values[1],
        ("releases:update".to_owned(), Some("base-v0.1.0".to_owned()))
    );
}

#[test]
fn rejects_an_invalid_release_tag_before_updating_project_files() {
    let directory = project();
    let registry = registry();
    let mut context = registry.context(directory.path());
    context.insert(Calls::default());
    assert!(super::after_version_bump(&mut context, "invalid".to_owned()).is_err());
    assert!(context.get::<Calls>().unwrap().values.is_empty());
}

fn git(root: &std::path::Path, arguments: &[&str]) {
    let output = std::process::Command::new("git")
        .current_dir(root)
        .args([
            "-c",
            "user.name=Fixture Author",
            "-c",
            "user.email=fixture@example.com",
            "-c",
            "commit.gpgsign=false",
        ])
        .args(arguments)
        .env("GIT_AUTHOR_DATE", "2026-01-02T00:00:00+00:00")
        .env("GIT_COMMITTER_DATE", "2026-01-02T00:00:00+00:00")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git {arguments:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

/// A committed project whose compile-fail fixture has no license header, as base-web-content's had before its first release.
fn repository() -> tempfile::TempDir {
    let directory = project();
    let root = directory.path();
    std::fs::create_dir_all(root.join("src")).unwrap();
    std::fs::write(root.join("src/lib.rs"), "pub fn example() {}\n").unwrap();
    std::fs::create_dir_all(root.join("tests/ui")).unwrap();
    std::fs::write(
        root.join("tests/ui/fails.rs"),
        "fn main() {\n    compile_error!(\"line 2\");\n}\n",
    )
    .unwrap();
    std::fs::write(
        root.join("releases.md"),
        "# Releases\n\n## Unreleased\n\n- Add the example.\n",
    )
    .unwrap();
    std::fs::write(root.join("readme.md"), "# Fixture\n\nAn example.\n").unwrap();
    git(root, &["init", "--quiet"]);
    git(root, &["add", "--all"]);
    git(root, &["commit", "--quiet", "-m", "Add the fixture."]);
    directory
}

#[test]
fn lists_rust_files_with_uncommitted_changes() {
    let directory = repository();
    let root = directory.path();
    let context = Registry::new().context(root);
    assert!(super::changed_sources(&context).is_empty());

    std::fs::write(root.join("tests/ui/fails.rs"), "// Header.\nfn main() {}\n").unwrap();
    std::fs::write(root.join("releases.md"), "# Releases\n").unwrap();
    assert_eq!(super::changed_sources(&context), ["tests/ui/fails.rs"]);

    let outside = tempfile::tempdir().unwrap();
    assert!(super::changed_sources(&Registry::new().context(outside.path())).is_empty());
}

#[test]
fn warns_about_changed_sources_with_the_check_and_snapshot_fix() {
    assert_eq!(
        super::changed_sources_warning(&["src/lib.rs".to_owned(), "tests/ui/fails.rs".to_owned()]),
        "warning: the release updated the license headers of 2 Rust file(s), which `cargo bake license:check` in pull request CI should have caught: src/lib.rs, tests/ui/fails.rs. Run the tests before merging the release, and update any compile-fail snapshots, such as trybuild `.stderr` files, whose line numbers moved."
    );
}

#[test]
fn reports_sources_changed_during_the_hook_and_continues() {
    let directory = repository();
    let registry = registry();
    let mut context = registry.context(directory.path());
    context.insert(Calls::default());
    // The recording `license:update` does not write; change a source as the real one would.
    std::fs::write(
        directory.path().join("tests/ui/fails.rs"),
        "// Released under the MIT License.\nfn main() {}\n",
    )
    .unwrap();

    super::after_version_bump(&mut context, "0.3.0".to_owned()).unwrap();

    assert_eq!(context.get::<Calls>().unwrap().values.len(), 4);
}

#[test]
fn license_check_fails_before_a_release_would_add_headers_and_passes_after_it() {
    let directory = repository();
    let root = directory.path();
    // A second author makes `license.md` use a Markdown line break, which normalization rewrites unless already normalized.
    std::fs::write(
        root.join("src/lib.rs"),
        "pub fn example() -> u8 {\n    1\n}\n",
    )
    .unwrap();
    git(
        root,
        &[
            "-c",
            "user.name=Second Author",
            "commit",
            "--quiet",
            "--all",
            "-m",
            "Change the example.",
        ],
    );
    let registry = Registry::discover().unwrap();
    let mut context = registry.context(root);

    // Pull request CI: the fixture without a header fails the check, which names it.
    let error = context.call("license:check", &[]).unwrap_err().to_string();
    assert!(error.contains("tests/ui/fails.rs"), "{error}");
    assert!(error.contains("cargo bake license:update"), "{error}");

    // The release hook applies the headers and normalizes the Markdown it wrote.
    super::after_version_bump(&mut context, "0.1.0".to_owned()).unwrap();
    assert!(
        std::fs::read_to_string(root.join("tests/ui/fails.rs"))
            .unwrap()
            .starts_with(
                "// Released under the MIT License.\n// Copyright, 2026, by Fixture Author.\n\n"
            )
    );
    assert!(
        std::fs::read_to_string(root.join("license.md"))
            .unwrap()
            .contains(
                "Copyright, 2026, by Fixture Author.\\\nCopyright, 2026, by Second Author.\n"
            )
    );
    git(root, &["commit", "--quiet", "--all", "-m", "Release."]);

    // The released files pass the check, including the normalized `license.md`.
    let result = context.call("license:check", &[]).unwrap();
    assert_eq!(
        result.as_str(),
        Some("License file and Rust source headers are up to date")
    );
}
