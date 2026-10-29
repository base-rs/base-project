---
metadata:
  type: skill
description: Measure, isolate, and land performance optimizations in Base Rust projects. Use when writing or running benchmarks, investigating a performance gap or regression against a reference implementation, profiling a hot path, deciding whether an optimization is worth keeping, or reporting performance results in records, release notes, and pull requests.
---

# Performance Optimization

Use this skill when a change aims to make a Base project faster or cheaper, or when you report performance numbers.

## Set the goal and the comparison

- State the workload and the reference you are comparing against, such as Tokio for echo or Hyper for HTTP. Describe the workload concretely: protocol, transport, request and response sizes, and concurrency.
- Read the applicable `design/*.md` requirements before changing code, using `base-design-review`. When a performance target or reference comparison becomes a standing constraint, capture it with `base-design-author`.
- Compare like with like. Make the implementations perform the same protocol work, with the same syscalls per operation, comparable allocation behavior, and the same connection reuse. Measure equivalent behavior before you attribute a difference to the code.
- Keep connection-only or otherwise reduced controls separate from the full path. Subtracting one from the other does not identify the cost of any single component.

## Control the environment

- Run benchmarks on a dedicated, otherwise idle Linux host, not on a laptop or a shared CI runner.
- Pin the server and the client to separate physical cores, so they never share a core through hyperthreading. Leave any hyperthread sibling of a measured core idle unless the workload deliberately uses it.
- Build each variant from a frozen source tree and lockfile, into its own target directory. A new working-tree build is a new experiment, even when it uses the same command.
- Keep diagnostic builds and runs separate from measurement runs. Allocation counters, `strace`, and similar tools perturb timing, so never take throughput or cycles from them.

## Measure

Report per-operation figures, so that runs of different lengths can be compared:

- **User instructions per operation** (`instructions:u`). This is the primary metric. It is nearly deterministic between runs, so small reductions are visible and repeatable. It still moves with the number of syscalls, wakeups, and retries, so read it together with those counts.
- **Cycles and L1 data-cache misses per operation** (`cycles:u`, `L1-dcache-load-misses:u`). These show whether fewer instructions also means less time, and expose memory-layout effects.
- **User and system CPU time per operation**, from `getrusage` for both the client and the server. System time shows kernel work that user-space counters cannot see.
- **Voluntary and involuntary context switches per operation**, for both the client and the server. They show differences in how often each side sleeps and wakes.
- **Throughput** in operations per second. It varies more than the other metrics.

Run the variants interleaved, A B C A B C and so on, rather than one block per variant, so drift affects all of them equally. Use at least 10 rounds, and report the median together with the minimum and maximum. Do not report a single run, or a mean without its spread. Run the baseline against itself to establish the noise floor before interpreting small cycle or throughput differences; identical builds can differ by several percent in throughput.

If you are not permitted to attach `perf` to a running process, launch the process under `perf stat` or `perf record` instead.

## Isolate the cost

- Rule out the cheap explanations first: syscalls per operation (`strace -c`), TCP segments and acknowledgments (`nstat`), page faults, allocations per operation, and context switches. A gap that survives these checks is in the code or in the scheduling pattern.
- Sample a profile of each implementation, compare the time per symbol, and group the symbols by component. Look for the components whose cost differs between implementations, not just for the hottest symbol. Samples taken during a syscall or interrupt are attributed to the instruction after it returns, so do not read cost at those addresses literally.
- Write an isolated benchmark for a suspect component, without I/O, and measure its cost per operation directly. Improve the component there, then confirm the gain in the full workload.
- Change one thing per variant, and measure each against the same frozen baseline. Build combined variants only after each change has been measured on its own.
- When user time is already lower than the reference's but throughput is still behind, look at system time and context switches. The remaining cost may be in how and when work reaches the kernel, which user-space optimization cannot remove.

## Decide what to keep

- Count a repeatable reduction in instructions as a real gain, even when throughput in a synthetic benchmark barely moves. Fewer instructions usually means lower CPU cost, and the saving applies to every caller.
- Do not make ergonomics worse for a gain that does not materialize. A change that makes an API harder to use, or a contract harder to satisfy, needs a clear and repeatable gain in cycles or throughput, measured as above. Adding an API that is easy to use, such as a capacity hint, does not need that higher bar.
- Prefer removing work, state, synchronization, and allocation over adding caches, fast paths, or special cases. Additional caching or specialized paths must justify their total complexity with repeatable gains, including the invariants that future changes must maintain.
- Preserve observable behavior by default, such as ordering, cancellation, ownership, notification, and error reporting. If an optimization does change it, review it as a behavior change and document it in the release notes and the relevant `design/*.md`.
- A fast path must not leave the general path's equivalent branch unreachable. Remove the duplicate branch, so coverage stays complete without exclusions, and add tests for every invariant that the optimization introduces. Follow `base-project-testing`; tests for those invariants should fail rather than hang when the invariant breaks.

## Record and report

- Keep the project's performance records in a document such as `performance.md` at the project root or beside the benchmark, and link it from the readme. Commit summaries and small raw samples beside it, so results can be compared later; keep large profiles out of the repository.
- Record every attempt, including rejected ones. For each, give the numbers and the reason it was rejected, so it is not tried again without new evidence. Record ideas found to be unsafe, together with the reason.
- For each result, state the host, CPU model, kernel, toolchain, build profile and flags, CPU placement, workload, exact commands, frozen revisions, and number of rounds. Give medians with their ranges.
- In release notes and pull request descriptions, give before and after figures for the metric that actually moved, usually instructions per operation, with the workload and concurrency, and the reference implementation's figure where relevant. Do not claim a throughput gain that is within run-to-run variation.
- Open one pull request per repository, following `base-project-pull-requests`. When a downstream record depends on unreleased upstream changes, add it in the pull request that updates those dependencies, so its evidence can be reproduced from the repository.
