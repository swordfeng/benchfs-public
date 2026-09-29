# GPT 5.6 Luna: Eval-only evidence

Submission: `core-gpt-5.6-luna`

[中文](gpt-5.6-luna-eval-only.zh-CN.md)

This page reports Eval observations only, without generation counts, duration, tokens or costs. Candidate and fixed-surface identities are recorded per profile; performance is unmeasured.

Missing generation provenance does not suppress completed Eval observations; unavailable results are not measured zeroes, although the index provisionally imputes zero.

## Results

<!-- core-results:begin -->

### Build and SDK checks

| Profile | Classification | Pass | Expected fail | Fail | Unexpected pass | Timeout | Skip N/A | Skip impl | Pass rate | Category macro average |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `sdk-smoke` | `diagnostic` | 44 | 0 | 0 | 0 | 0 | 0 | 0 | 100.00% | — |

`sdk-smoke` run: `core-sdk-luna-20260925-r1`

### POSIX correctness

| Profile | Classification | Pass | Expected fail | Fail | Unexpected pass | Timeout | Skip N/A | Skip impl | Pass rate | Category macro average |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `spec-tests` | `not-run` | — | — | — | — | — | — | — | — | — |
| `pjdfstest-core` | `not-run` | — | — | — | — | — | — | — | — | — |
| `xfstests-core` | `not-run` | — | — | — | — | — | — | — | — | — |

### Robustness

Profile: `robustness`

Outcome: `candidate-failed`; classification: `valid`.

| Metric | Value |
|---|---:|
| Total | 47 |
| Pass | 0 |
| Fail | 47 |
| Timeout | 0 |
| Pass rate | 0.00% |
| Category macro average | 0.00% |

Timeouts count as failures; N/A is excluded; the macro average weights categories equally.

Run: `eval-submission-robustness-v2-core-gpt-5-6-luna-r1`

### On-disk format conformance

Profile: `canonical-format`

Outcome: `candidate-failed`; classification: `diagnostic`.

| Metric | Value |
|---|---:|
| Total | 479 |
| Pass | 0 |
| Fail | 479 |
| Timeout | 0 |
| Pass rate | 0.00% |
| Category macro average | 0.00% |
| Conformance score (0–100) | 0.00 |
| Full conformance | no |

Run: `core-canonical-luna-oracle3-r6`
Execution backend: `hyperv`

### Crash consistency

Profile: `crash-core`

Outcome: `candidate-failed`; classification: `diagnostic`.

| Metric | Value |
|---|---:|
| Total | 216 |
| Pass | 0 |
| Fail | 216 |
| Timeout | 0 |
| Pass rate | 0.00% |
| Category macro average | 0.00% |

Run: `core-crash-luna-20260925-r1`
Execution backend: `hyperv`

### Real-world applications

Profile: `real-world`

Outcome: `candidate-failed`; classification: `diagnostic`.

| Metric | Value |
|---|---:|
| Total | 4 |
| Pass | 0 |
| Fail | 4 |
| Timeout | 0 |
| Pass rate | 0.00% |
| Category macro average | 0.00% |

Run: `core-realworld-luna-20260925-r1`
Execution backend: `hyperv`

### CPU performance

Profile: `perf-cpu`

No publishable evaluation result yet (`not-run`), not a measured zero; the index provisionally imputes zero.

### NVMe performance

Profile: `perf-nvme`

No publishable evaluation result yet (`not-run`), not a measured zero; the index provisionally imputes zero.

## Scope and provenance

[JSON](../data/core/gpt-5.6-luna-eval-only.json)

Original profile results remain independent; the index adds a derived provisional total with missing scores imputed as zero. JSON retains per-profile run, manifest, report, candidate, and fixed-surface identities, plus the execution backend where recorded; different backends are not treated as identical execution conditions.

`valid` means the profile evidence is valid, not that all cases passed or release eligibility is established. `candidate-failed` is not an infrastructure error. `diagnostic` is diagnostic-only; `noneligible` cannot enter rankings. Hyper-V performance is always diagnostic.

Security-review and maintainability scores are not assessed. Reference is a separate calibration baseline, not a model-generation entry.
<!-- core-results:end -->
