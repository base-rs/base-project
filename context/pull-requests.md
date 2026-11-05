---
metadata:
  type: skill
description: Prepare commits and GitHub pull requests for Base Rust projects. Use when writing commit messages, creating or updating a pull request, or deciding what to include in the description and release notes.
---

# Pull Requests

Use these conventions when preparing commits or pull requests for Base Rust projects.

## Design requirements

Use `base-design-review` to assess the changes against applicable `design/*.md` before opening or updating a pull request. Review requirements at both the base and proposed revisions, including added, changed, and removed documents. Use `base-design-author` when the work introduces, changes, or removes a user requirement; keep the document change with the implementation.

Include deliberate requirement changes and unresolved design findings or missing validation evidence in the pull request description when relevant. Re-review affected requirements after subsequent changes. Design review complements the general adversarial review below; coordinate their findings and reuse applicable evidence.

## Adversarial review

When sub-agents are available, use a sub-agent that did not implement the change for an independent adversarial review before opening the pull request. Keep the reviewer focused on inspecting the changes and reporting findings.

- Give the reviewer the user's intent, relevant repository guidance, the diff, and the exact commit to review.
- Ask it to challenge assumptions and identify concrete correctness, compatibility, testing, and documentation gaps. Findings should explain the affected behavior and supporting evidence.
- Evaluate each finding against the intended behavior. Fix substantive issues and run the relevant checks; explain with evidence when a concern does not apply.
- Iterate locally through review and fixes until all non-trivial issues are resolved. Have the reviewer confirm the final commit before opening the pull request. If the changes are revised later, re-review the affected changes before merging.

Scale the review to the change's complexity and risk. A small documentation change needs a focused check of clarity and factual accuracy; changes to public APIs or runtime behavior need deeper scrutiny of semantics and regressions. Complete the required tests and configured GitHub approvals alongside the adversarial review.

## Titles and commits

- Pull request titles must use Markdown, be complete sentences, and end with a full stop.
- Commit subjects must use Markdown and end with a full stop. Release commits also carry a `Release: TAG` Git trailer as specified by the releasing skill; its tag value has no trailing period.
- The first line of a commit message must focus on what changed.
- Most commit messages should be a single line.
- Keep relevant context in the code itself, such as comments, rather than using the commit message as a side channel for important details.
- Do not include agent links, attribution footers, generated-by annotations, or similar metadata in commit messages.

## Pull request description

The description becomes the message of the squash-merged commit on `main`, as the GitHub repository guide configures. Write it for someone reading the history later: a brief summary, then the problem and the solution, in a few short paragraphs. Never leave it empty; the merged commit would have only a title.

Keep the description brief and put the long-form material in the linked issue: the investigation, alternatives considered, reproduction steps, measurements, and discussion. An issue can be commented on and revised as understanding changes; a merged commit message cannot. Open or update the issue before the pull request when the change needs that context, and link it from the description. When there is no issue, add that material as comments on the pull request rather than expanding the description. Once the pull request is open, record review findings, measurements, and follow-up discussion in comments too; edit the description only when the summary of the change itself is no longer accurate. Implementation details belong in the code and its comments; mention them in the description only when a reviewer needs them to follow the change. Include screenshots for visual changes.

Use this structure, replacing the guidance with project-specific content:

```markdown
Briefly summarize the change in 1–3 sentences.

Describe the problem and the solution in a few short paragraphs. Link the
issue that holds the detailed context. Include screenshots for visual changes.
```

Only a release pull request ends with a Git trailer (`Release: TAG`, written by the `cargo:release:*` tasks); do not end other descriptions with a `Key: value` line.

Do not add a `Types of Changes` section. Issue types classify GitHub issues; do not assign an issue type to a pull request or include issue type metadata in its description. If a related issue needs classification, set the type on that issue.

## Testing

Changes should include suitable test coverage. Aim for complete coverage of the behavior being changed or introduced. If a change directly affects downstream crates, add or update downstream integration coverage when useful.

Do not list passing test commands or verification steps in the pull request description unless they explain an unusual risk, limitation, or manual validation requirement.

## Release notes

For user-visible changes, add a brief entry to `releases.md` following the release notes guidance provided by `bake-releases`.

## Issue types

Issue types apply to GitHub issues. Do not try to set an issue type when creating or updating a pull request. For a linked issue, use its issue type:

- Use `Bug` for defect fixes and regressions.
- Use `Feature` for new user-facing capabilities.
- Use `Task` for maintenance, refactoring, documentation, tests, release work, and internal improvements.
