# Test Suite Overview

[中文](test-suite.zh-CN.md)

BenchFS combines benchmark-specific semantic checks, upstream POSIX suites, robustness workloads, and variant-specific correctness, recovery, format-conformance, and performance evaluation. Individual case identifiers, case bodies, expected outputs, seeds, and hidden selection details are intentionally not published.

## Frozen aggregate counts

Counts below describe the current Core correctness manifests. “Selected” includes cases classified before model execution as not applicable. “Scored” excludes those preclassified N/A entries.

| Suite | Selected | Dev-visible | Scored | Preclassified N/A |
|---|---:|---:|---:|---:|
| BenchFS semantic checks | 51 | 33 | 44 | 7 |
| pjdfstest Core profile | 217 | 217 | 215 | 2 |
| xfstests generic profile | 667 | 31 | 184 | 483 |
| Seeded robustness workloads | 2 | 0 | 2 | 0 |

### xfstests comparison

The evaluator selects **667** upstream generic cases in total. The development VM exposes a frozen **31-case** subset, all of which belongs to the evaluator's selected set. Thus about 4.6% of the selected xfstests surface is Dev-visible. The public repository publishes only these aggregate counts—never the individual case numbers.

A selected case can be preclassified N/A because its prerequisite belongs to a filesystem or environment capability outside the benchmark profile. This classification is frozen before candidate results are observed.

## Visibility rules

- **Dev-visible:** present in the development VM and available for agent feedback.
- **Eval-hidden:** absent from the development VM and executed only after the trajectory ends.
- Hidden checks may exercise the same published capability families more broadly, but may not introduce a new undocumented feature requirement.
- Hidden test material, manifests, exact identities, and detailed failure evidence are not part of this public repository.

## Next-version status

The formal real-world and performance profiles described below are requirements for the next benchmark version. They are not retroactively added to the current pilot, and no formal result is published until their runners, environments, metrics, and oracles pass qualification and are frozen.

## Synthetic performance workloads

Performance uses two separately reported environments: a memory-backed profile for software overhead and a dedicated persistent-storage profile for end-to-end behavior. The mounted filesystem is exercised through ordinary Linux system calls and FUSE. Workload families cover bulk sequential I/O, small random I/O, buffered synchronized writes, metadata and directory activity, capacity pressure, concurrency, and applicable Variant operations.

Integrity prechecks and postchecks run outside the timed window. Each timed workload has a warm-up, fixed measurement window, cold/warm cache classification, and repeated retained samples. Exact job files, parameters, and operation sequences are frozen evaluator data and are not published here.

## Real-world workload suite

The next version has a separate, small application-level correctness suite. It reports completion, data integrity, reopen behavior, and error handling independently from synthetic performance. Exact applications, scenario steps, fixtures, individual identities, and expected outputs are not published in this repository.

## Crash testing

Crash testing is a separate evaluation path, not part of the synthetic performance or real-world workload labels. The evaluator interrupts a running filesystem at frozen points, restarts it from the resulting persistent state, and checks whether it mounts or recovers according to the selected Variant, preserves required durable state, and remains structurally valid.

Crash workload identities, interruption points, persistence selections, seeds, traces, and expected states are evaluator-only and are not published here.

## Additional dimensions

Other profiles measure format conformance, objective maintainability metrics, and post-run code review. Released results report real-world scenarios, crash testing, and synthetic performance as separate dimensions rather than combining them under “workload.”
