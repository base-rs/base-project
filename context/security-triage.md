---
metadata:
  type: skill
description: Handle a GitHub Security Advisory from receipt to published fix. Use when asked to triage, review, accept, close, describe, fix, merge, backport, or publish a security report for a Base project.
---

# Security Triage

Use this procedure to take a security report through to fixed releases and a published advisory. Follow the phases in order. Where a step depends on a project decision, state the decision you are assuming and confirm it with the user, who is normally the maintainer, before acting on GitHub.

## Context

- Reports arrive as GitHub Security Advisories on the project repository. GitHub may create a temporary private fork for the fix. Often no fix exists yet and you write it.
- Identify the remote that points at the canonical repository with `git remote -v`; its name varies between checkouts. Fetch it before relying on branch or tag state.
- Determine the supported release series from the project's security policy. Identify the release branch for each supported series, or confirm that releases are cut from `main` only.
- `gh api --paginate 'repos/<owner>/<repo>/security-advisories?state=triage'` lists reports awaiting a decision; omit the state filter or pass `published` to inspect earlier advisories and their descriptions. Check for duplicate reports before working on any of them.
- Nothing is pushed, merged, commented, or edited on GitHub unless the user asked for that step. Report findings in the conversation, including the commit SHAs reviewed, results per version, test counts, and anything not verified.

## Phase 1: Review the report

- Read the advisory description. If a fix pull request exists, read it and record the commit SHA you reviewed.
- Reproduce the issue locally against `main` and against each candidate release series, using checkouts of the released tags. Verify the reporter's version range instead of trusting it.
- Look for open pull requests and issues that touch the same code and note merge interactions.
- Work in temporary copies under `/tmp`, never in the working checkout. Isolate toolchain and dependency caches if the system state interferes.
- Test variants beyond the reporter's shape. A report often demonstrates the maximal exploit; check whether ordinary, well-formed input reaches the same code path, since that changes the impact statement.

### Duplicate reports

The same issue is often reported more than once, sometimes months apart. Search the open reports for the same component and mechanism before working on any of them.

- Choose one report as the canonical advisory. Normally this is the earliest, because the first reporter found it first. Prefer a later report when it is materially better: the reporter is engaged and responsive, it comes with a proposed fix and tests, or it describes the real scope accurately while the earlier one does not. A submitted advisory with an active author or a proposed fix is worth more than a low-effort report that arrived first. Both timing and quality matter; say which you chose and why.
- Close the others as duplicates with a short comment thanking the reporter and explaining that an earlier report covers the same issue and that they will be credited on the published advisory. Reporters cannot see each other's private advisories, so do not link them.
- Add every reporter to the credits of the canonical advisory. Carry over any reproduction, affected version, or impact detail from the duplicates that the canonical report lacks.
- Handle the set as one issue from then on: one fix, one description, one CVE, one severity.

## Phase 2: Decide, then accept or close

Decide whether the report is a security issue before working on a fix. Accepting creates an identifier, coordinated releases on every supported series, and a public advisory, so the bar is real attacker impact.

- **Accept** when an unauthenticated or low-privilege attacker can cause a concrete confidentiality, integrity, or availability impact in a default or common configuration, and applications could not be expected to already defend against it.
- **Close and request a pull request** when the report is a bug without attacker leverage: a correctness fix, a hardening suggestion, a documented limitation, a configuration already considered unsafe, or a cost already bounded by existing limits. Close with a short note explaining the decision and asking the reporter to open an ordinary pull request or issue. Thank them either way.
- When unsure, compare the reproduction against the project's previously accepted and closed advisories for the same component.

If accepting:

1. Accept the report in the GitHub advisory interface.
2. Copy the reporter's original description verbatim into an advisory comment so the original report is preserved.
3. Replace the description with a simplified version following the `advisory-template.md` resource installed with this skill. Match the tone and length of the project's published advisories. Keep the reporter's credit.
4. Request a CVE from GitHub immediately after accepting, so the identifier is available for release notes and the fix pull request. Fill in whatever fields the request requires, using provisional affected ranges that Phase 6 finalizes. The identifier can take time to arrive; later phases tolerate that.
5. Propose a severity. Draft a CVSS 3.1 vector with a one-line justification per metric, compare it with the project's published advisories of the same impact class, and discuss it with the user before setting it. Use the same vector as an earlier advisory with the same mechanism unless there is a reason to differ. As a reference shape, unauthenticated, low-complexity resource exhaustion that is unbounded and held for the lifetime of a request scores Availability High, a bounded or transient cost scores Availability Low, and impacts that require an uncommon configuration or high attack complexity score lower.

## Phase 3: Obtain or create the fix

- If a fix exists, review it for correctness. Check that it does not change behavior outside the vulnerable path and that existing limits, accounting, and invariants are preserved.
- If no fix exists, or the proposed one is inadequate, write one in the advisory's temporary private fork when GitHub provides one. Keep the change minimal and local to the vulnerable code path. Follow the style of previous security fixes in the same file for naming, placement of limits, and tests.
- Add regression tests that fail without the fix and pass with it. Cover the reproduction shape from the report as well as the general mechanism.
- Apply the fix to a temporary copy of `main` and confirm that the reproduction no longer triggers, the new tests fail without the fix and pass with it, the affected test file passes, and then the full test suite passes. Record the counts.
- Temporary private forks behave differently from ordinary repositories. Their pull requests target the parent repository's `main`, they cannot be forked, and GitHub does not update an existing pull request when more commits are pushed to its branch. To add a commit such as a changelog entry, push the updated history to a new branch in the private fork and open a new pull request there, then close the old one. Merge private fork pull requests from the advisory page.

## Phase 4: Merge to main

- Confirm the pull request targets the current `main` and CI checks pass.
- Ensure the change includes an entry under the `## Unreleased` heading in `releases.md`, following the `bake-releases` guidance. The entry links the identifier and states the issue in one sentence:

  ```markdown
  - [CVE-YYYY-NNNNN](https://github.com/advisories/GHSA-xxxx-xxxx-xxxx) One-sentence description of the issue.
  ```

  Use the advisory link alone if the CVE has not been assigned yet, and fill in the CVE before release. The advisory link resolves only after the advisory is published.
- Merging to a public branch discloses the fix. Keep the merge close to the releases that carry it, and if an embargo was agreed with other parties, do not merge until it ends.
- Merge the fix into `main`.

## Phase 5: Backport

- Settle the backport target list with the user. The default is every supported release series, plus any older series the maintainers still patch.
- For each target, cherry-pick or re-apply the fix onto the series release branch in a temporary copy or a dedicated branch, re-run the reproduction and the test suite on that branch, and add the release note under a new patch version heading.
- Do not assume the `main` result carries over. Older series can have different code and additional problems that need separate fixes.
- Open a pull request per series titled `Backport <subject> to <series>.`, then merge.
- Release each series with the releasing skill: bump the patch version with the `cargo bake cargo:version:patch` flow and put the `Release:` trailer on the final commit. The publish workflow and branch ruleset must cover each series branch that releases.

### Several advisories in one release

Security fixes are often batched so each supported series gets one patch release. When several advisories are in flight:

- Run Phases 1 to 3 independently for each advisory. Each keeps its own fix, description, CVE, and severity. Do not merge unrelated fixes into one pull request.
- Hold the version bump until every fix in the batch has merged to `main` and been backported to each target series. One release per series then carries all of them.
- List every advisory as a separate entry under the same version heading in `releases.md` on `main` and on each series branch.
- Set the same patched version on every advisory in the batch for a given series, then publish the advisories together once the releases are on the registry.
- If one fix is not ready, decide with the user whether to hold the release or ship without it. Do not let a half-finished fix delay a release indefinitely.

## Phase 6: Set fixed versions and publish

- Before publishing, set the affected products on the advisory: the ecosystem (Rust for crates.io packages), the package name, and one row per release series with the affected range and the patched version, for example `>= 3.2.0, < 3.2.8` patched in `3.2.8`. Include every backported series. Mark unsupported older series as affected with no patched version when they are not fixed.
- Confirm the patched versions are published to the registry and the release notes on each branch name them.
- Confirm the CVE is assigned and that the description, credits, and severity are final.
- Publish the advisory.
