# Test suites

[中文](test-suite.zh-CN.md) · [Methods and result tables](../README.md#methods)

BenchFS combines dedicated checks, upstream filesystem suites, and application tests. Each group has its own results, presented alongside its method in the overview.

| Test group | What it checks | Score |
|---|---|---|
| BenchFS semantic tests | Filesystem behavior required by the task | Category average |
| pjdfstest | POSIX file-operation semantics at the syscall level | Category average |
| xfstests | Operation sequences, data integrity, and complex usage | Category average |
| On-disk format | Compliance with the required storage format | Native conformance score |
| Robustness | Behavior under stress and abnormal conditions | Category average |
| Crash consistency | State handling, durability, and structural integrity after interruption | Category average |
| Applications | Task completion, correct data, and behavior after reopening | Category average |
| SDK smoke | Build and basic checks | Unscored |

Applicable counts, pass rates, and scores come from each implementation's actual test records; see the [profile results](../README.md#methods). Performance has not been measured.

## Development feedback

Agents can run a subset of feedback tests in their development VMs, including BenchFS semantic checks, pjdfstest, and a subset of xfstests. The full evaluation runs after submission. Hidden tests exercise the stated task capabilities without adding unstated feature requirements.

## Public material

This repository describes test goals, methods, and aggregate results without releasing individual case identifiers, operation sequences, input fixtures, or expected outputs. The [VM materials catalog](../vm-agent/README.md) lists the materials supplied to agents.
