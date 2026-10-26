---
metadata:
  type: skill
description: Create or maintain GitHub repository metadata, collaboration features, merge settings, and main-branch protections for Base projects. Use when setting up a new repository or auditing its existing settings.
---

# GitHub Repository Setup

Use these defaults when creating a Base repository. Preserve deliberate project-specific settings when maintaining an existing repository.

## Repository metadata

- Use the canonical `base-rs` organization and project name.
- Write a short, accurate repository description.
- Set the homepage to the documentation site when one exists.
- Add focused topics for discovery; avoid repeating words already in the repository name.
- Use `main` as the default branch.

## Collaboration features

Enable Issues, Discussions, Pull Requests, Sponsorships, and repository preservation. Disable Projects and Wiki unless the project has a concrete use for them.

Use GitHub issue types to classify issues consistently:

- `Bug` for defects and regressions.
- `Feature` for new user-facing capabilities.
- `Task` for maintenance, refactoring, documentation, tests, and release work.

Prefer organization or repository labels that already exist. Add labels only when the project needs a reusable classification not covered by issue types or existing labels. Do not repeat issue type information in issue text when GitHub already records it as metadata. Issue types apply to issues; do not assign one to a pull request or include issue type information in its text.

## Pull requests and commits

- Disable merge commits; allow squash and rebase merging.
- Suggest updating pull request branches, allow auto-merge, and delete merged head branches automatically.
- Require contributors to sign off on commits made through GitHub's web interface.
- Allow comments on individual commits.
- Use Markdown, complete sentences, and a final period for pull request titles and commit messages.
- Keep most commit messages to one line. Start with what changed.
- Describe pull requests with a short summary followed by the problem and solution. Do not add a separate change type section or assign an issue type to a pull request.

Use the `base-project-pull-requests` skill for the full title, commit, description, testing, and release note conventions.

## Branch protection

Protect `main` and require pull requests with at least one approval. Allow administrators to bypass these rules for maintenance. Require the publishing workflow's `check` job and the testing workflow's stable `test-result` job. The latter succeeds only when all required test and coverage jobs succeed. Keep experimental or diagnostic jobs outside this gate.

## Apply settings safely

Use the GitHub CLI for repository changes. Inspect the target first and always use its full name in commands:

```sh
gh repo view base-rs/PROJECT
```

When maintaining an existing repository, inspect its current settings and make the smallest change that achieves the intended result. Preserve project-specific settings. Do not rename, archive, transfer, or delete a repository without explicit approval, and do not disable issues, pull requests, or required checks without approval.

The `base-project-releasing` skill describes the standard Cargo release process and links to Bake Cargo task documentation for branch rulesets, crates.io environment reviewers, and trusted publishing.
