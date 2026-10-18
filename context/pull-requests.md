---
type: skill
description: Prepare commits and GitHub pull requests for Core Rust projects. Use when writing commit messages, creating or updating a pull request, or deciding what to include in the description and release notes.
---

# Pull Requests

Use these conventions when preparing commits or pull requests for Core Rust projects.

## Adversarial review

When sub-agents are available, use a sub-agent that did not implement the change for an independent adversarial review before opening the pull request. Keep the reviewer focused on inspecting the changes and reporting findings.

- Give the reviewer the user's intent, relevant repository guidance, the diff, and the exact commit to review.
- Ask it to challenge assumptions and identify concrete correctness, compatibility, testing, and documentation gaps. Findings should explain the affected behavior and supporting evidence.
- Evaluate each finding against the intended behavior. Fix substantive issues and run the relevant checks; explain with evidence when a concern does not apply.
- Iterate locally through review and fixes until all non-trivial issues are resolved. Have the reviewer confirm the final commit before opening the pull request. If the changes are revised later, re-review the affected changes before merging.

Scale the review to the change's complexity and risk. A small documentation change needs a focused check of clarity and factual accuracy; changes to public APIs or runtime behavior need deeper scrutiny of semantics and regressions. Complete the required tests and configured GitHub approvals alongside the adversarial review.

## Titles and commits

- Pull request titles must use Markdown, be complete sentences, and end with a full stop.
- Commit subjects must use Markdown and end with a full stop. Release commits also carry a `Release: vVERSION` Git trailer as specified by the releasing skill; its tag value has no trailing period.
- The first line of a commit message must focus on what changed.
- Most commit messages should be a single line.
- Keep relevant context in the code itself, such as comments, rather than using the commit message as a side channel for important details.
- Do not include agent links, attribution footers, generated-by annotations, or similar metadata in commit messages.

## Pull request description

Start with a brief summary, followed by a detailed description of the problem and solution. Include implementation details that help reviewers understand the change, link relevant issues when applicable, and include screenshots for visual changes.

Use this structure, replacing the guidance with project-specific content:

```markdown
Briefly summarize the change in 1–3 sentences.

Describe the problem, context, and solution. Include implementation details
that help reviewers understand the change. Link relevant issues if applicable.
Include screenshots for visual changes.
```

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
