# Scoring

[中文](scoring.zh-CN.md) · [Back to results](../README.md#results)

BenchFS scores executable behavior: completed operations, correct data, and responses to abnormal conditions that meet the task requirements. The overview shows overall and individual test scores.

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
| Applications W | Application score | 10% |

The overall score retains the weights and transformation in `core-overall-v1-zero-fill`. First aggregate each dimension, then apply the utility function U below, and finally take the weighted sum.

| Dimension score x | 0 | 20 | 40 | 60 | 80 | 100 |
|---|---:|---:|---:|---:|---:|---:|
| U(x) | 0 | 45 | 68 | 83 | 93 | 100 |

U interpolates linearly between adjacent points. Current scores are calculated as:

```text
total = 0.35 × U(C) + 0.20 × U(R) + 0.10 × U(W)
```

Measured components account for 65 points. The other 35 points in the existing rule—code review (20%), performance (10%), and maintainability (5%)—are unmeasured and contribute zero. Existing scores have not been reweighted or rescaled to 100. The overview does not score implementations through subjective code commentary or convert source size into quality points.

Missing inputs contribute zero only when calculating totals; profile tables show “—” to distinguish them from measured zeroes. A partially measured dimension keeps its original denominator and is marked with `*`. Calculations retain source precision and display two decimals. Ordering uses unrounded totals.

## Development statistics

Time, token usage, cost, and source size are reported separately. They do not affect scores or break ties. Each entry represents one development run, rather than an average across runs, and identifies both the model and its coding agent.
