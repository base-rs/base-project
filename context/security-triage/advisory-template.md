# Advisory Description Template

Use `##` headings. Keep each section short and concrete. Code excerpts are welcome in Details. Replace the reporter's description with this structure after preserving the original verbatim in an advisory comment.

```markdown
## Summary

One or two paragraphs. Name the component (type, function, or module), what the attacker supplies, and what happens. End with the consequence in one sentence, for example "This results in a denial of service condition for applications that accept this input format."

## Details

How the vulnerable code behaves, with a short code excerpt when it helps. Explain why existing limits or protections do not apply. If the mechanism differs between versions, say which versions are affected and why. Distinguish the issue from any similar previously published advisory.

## Impact

Who is affected: default configuration, or only when an optional component is enabled. What an attacker can achieve and whether authentication is required. What is not required, such as malformed input, large payloads, or user interaction.

## Mitigation

- Update to a patched version, which <what the fix does>.
- <Configuration or deployment workaround, if any.>
- <Mitigation at an application boundary, if any.>
```

A good Summary names the component, the attacker-controlled input, the mechanism in one clause, and the consequence. A good Impact states the scope of affected deployments, the attacker's preconditions, and what the attacker gains. Mitigation always starts with the update line.
