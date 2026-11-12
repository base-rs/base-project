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

Enable Issues, Discussions, Pull Requests, Sponsorships, and repository preservation. Disable Projects and Wiki unless the project has a concrete use for them. Enable private vulnerability reporting so reporters can submit vulnerabilities without opening public issues; `base-security-advisory` describes how reports are handled.

Use GitHub issue types to classify issues consistently:

- `Bug` for defects and regressions.
- `Feature` for new user-facing capabilities.
- `Task` for maintenance, refactoring, documentation, tests, and release work.

Prefer organization or repository labels that already exist. Add labels only when the project needs a reusable classification not covered by issue types or existing labels. Do not repeat issue type information in issue text when GitHub already records it as metadata. Issue types apply to issues; do not assign one to a pull request or include issue type information in its text.

## Pull requests and commits

- Allow squash merging only; disable merge commits and rebase merging, so each pull request lands on `main` as one commit carrying its title and description.
- Build squash merge commits from the pull request title and description (`PR_TITLE` and `PR_BODY`), not from the concatenated commit messages, so the merged commit reads as the description was written and a release pull request keeps its `Release: TAG` trailer through review commits. `cargo:setup:github:apply` sets this.
- Suggest updating pull request branches, allow auto-merge, and delete merged head branches automatically.
- Require contributors to sign off on commits made through GitHub's web interface.
- Allow comments on individual commits.

Use the `base-project-pull-requests` skill for the full title, commit, description, testing, and release note conventions.

## Branch protection

Protect `main` with a ruleset that requires pull requests, blocks force pushes, and requires the publishing workflow's `check` job and the testing workflow's stable `test-result` job. The latter runs with `if: always()` and succeeds only when every required job in its `needs` succeeds, including matrix jobs, such as the canonical workflow's test and coverage, minimum Rust version, and `license` jobs, so failed, cancelled, or skipped prerequisites cannot bypass it, and adding a job there requires it without changing the ruleset. Keep experimental or diagnostic jobs outside this gate.

Require no approvals by default. Maintainers, meaning the organization's maintainers team or anyone with write access, and agents working under their accounts discuss and decide changes together, so a separate approval adds nothing. Maintainers merge their own pull requests once the required checks pass, without bypassing the rules. External contributors face a higher bar: only maintainers can merge, so a maintainer's merge is the sign-off.

Keep the bypass list empty or limited to the admin role, restricted to pull request merges, as an emergency exception rather than the normal way to merge or release. If merges routinely need the bypass, fix the rules instead.

Require one or more approvals when a separate reviewer adds real independence, for example when a project has many maintainers who work independently, maintainers from different organizations, or a policy that requires a second person to review each change.

## Contributor workflows

Require maintainer approval before running workflows for first-time contributors' fork pull requests (`first_time_contributors`). Keep a stricter existing policy that requires approval for all external contributors.

## Releases

Keep releases deliberate but simple. Restrict the `crates-io` environment to deployments from `main`, require the maintainers team as reviewers, and allow self-review so a maintainer approves their own release with one click rather than a bypass. GitHub's protected-branches environment option does not recognize branches protected only by rulesets, so name the release branch in a custom deployment branch policy.

Protect release tags with a ruleset that blocks deleting and updating them, with no bypass, so a tag always names the commit that was published and release artifacts can be built from it. Creation stays open because the publish workflow pushes the tag with its `GITHUB_TOKEN`. Removing a mistaken tag needs an administrator to disable the ruleset deliberately. Protect an environment holding signing secrets for release artifacts like `crates-io`.

`cargo bake cargo:setup:github:apply` from Bake Cargo applies these branch, tag, environment, and contributor settings; pass `--approvals N` to require approvals, and `--environment NAME` to configure a signing environment.

Give the maintainers team access to the repository, for example with the `maintain` role, **before** running `cargo:setup:github:apply`. GitHub silently drops environment reviewers who cannot access the repository: the request succeeds, but the `crates-io` environment is left with nobody able to approve a release. A new repository does not give the team access automatically. Grant it deliberately:

```sh
gh api --method PUT orgs/base-rs/teams/maintainers/repos/base-rs/PROJECT -f permission=maintain
```

After applying, verify that the environment lists the team as a required reviewer:

```sh
gh api repos/base-rs/PROJECT/environments/crates-io --jq '.protection_rules[] | select(.type == "required_reviewers") | .reviewers[] | "\(.type) \(.reviewer.slug // .reviewer.login)"'
```

The output should include `Team maintainers`. If it does not, grant the access and run `cargo:setup:github:apply` again.

## Apply settings safely

Use the GitHub CLI for repository changes. Inspect the target first and always use its full name in commands:

```sh
gh repo view base-rs/PROJECT
```

When maintaining an existing repository, inspect its current settings and make the smallest change that achieves the intended result. Preserve project-specific settings. Do not rename, archive, transfer, or delete a repository without explicit approval, and do not disable issues, pull requests, or required checks without approval.

The `base-project-releasing` skill describes the standard Cargo release process and links to Bake Cargo task documentation for branch rulesets, crates.io environment reviewers, and trusted publishing.
