# Scoring

[中文](scoring.zh-CN.md) · [Back to results](../README.md#results)

BenchFS measures executable behavior, confirmed code-review defects, and maintainability. The overview shows the combined score and its individual dimensions.

## Pass rates and profile scores

```text
pass rate = (pass + expected fail) / applicable tests
applicable tests = pass + expected fail + fail + unexpected pass + timeout + missing required functionality
```

N/A tests are excluded. Timeouts and missing required functionality count as failures. When a test requires an operation to be rejected, an expected failure counts as meeting that requirement.

Semantic tests, pjdfstest, xfstests, robustness, crash consistency, and applications use an **equal-weight category average**: calculate each category's pass rate, average the categories with applicable tests, and multiply by 100. Categories with more tests do not receive more weight simply because of their size.

**The format profile uses its native conformance score.** Its check pass rate is supplementary and does not replace that score. Build and SDK smoke checks are unscored.

## Overall score

| Dimension | Calculation | Overall weight |
|---|---|---:|
| Correctness C | Mean of semantic, pjdfstest, xfstests, and format scores | 35% |
| Robustness R | Mean of robustness and crash-consistency scores | 20% |
| Code review S | Diminishing curve of confirmed, deduplicated defect burden (below) | 20% |
| Applications W | Application score | 10% |
| Performance P | Mean of the CPU and NVMe performance scores | 10% |
| Maintainability M | Static `maint-v2` score of the submission's own source (see the README) | 5% |

The provisional overall score uses `core-overall-v2-zero-fill`. It retains the existing weights and utility transformation and includes the available diagnostic code-review scores. First aggregate each dimension, then apply the utility function U below, and finally take the weighted sum.

| Dimension score x | 0 | 20 | 40 | 60 | 80 | 100 |
|---|---:|---:|---:|---:|---:|---:|
| U(x) | 0 | 45 | 68 | 83 | 93 | 100 |

U interpolates linearly between adjacent points. Current scores are calculated as:

```text
total = 0.35 × U(C) + 0.20 × U(R) + 0.20 × U(S) + 0.10 × U(W) + 0.10 × U(P) + 0.05 × U(M)
```

All six dimensions together account for 100 points. Diagnostic review scores contribute to this provisional summary; their inclusion does not establish release eligibility. Weights are not redistributed when a result is missing, and source size is not converted into quality points.

Missing inputs contribute zero only when calculating totals; profile tables show “—” to distinguish them from measured zeroes. A partially measured dimension keeps its original denominator and is marked with `*`. Calculations retain source precision and display two decimals. Ordering uses unrounded totals.

## Agent code review

Agents independently review each implementation, consolidate duplicate root causes, and discuss findings and severity before confirming the result. Only confirmed, deduplicated defects contribute to S. Rejected or unproven allegations are excluded.

```text
burden = 40 × critical + 15 × high + 5 × medium + low
S = 10000 / (100 + burden)
```

This is `agent-review-score-v2`: burden 0 gives 100, burden 100 gives 50, and burden 500 gives 16.67. Additional defects always lower the score, with diminishing deductions. A complete review with no confirmed defects scores 100; it is not proof of correctness. An incomplete or unperformed review has no S score, not a zero-defect result.

The README and model reports publish completion status, severity counts and diagnostic scores. [Aggregate JSON](../results/agent-review.json) also includes opaque evidence hashes. Detailed findings and agent conversations remain private.

## Development statistics

Time, token usage, cost, and source size are reported separately. They do not affect scores or break ties. Each entry represents one development run, rather than an average across runs, and identifies both the model and its coding agent.
