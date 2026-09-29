# Reference calibration baseline

Submission: `core-reference`

[中文](reference-calibration.zh-CN.md)

This page reports Eval observations only, without generation counts, duration, tokens or costs. Candidate and fixed-surface identities are recorded per profile; performance is unmeasured.

SDK/POSIX and other profiles do not all use the same candidate source, so they are not a complete evaluation of one candidate binary; see JSON for exact identities.

## Results

<!-- core-results:begin -->

### Build and SDK checks

| Profile | Classification | Pass | Expected fail | Fail | Unexpected pass | Timeout | Skip N/A | Skip impl | Pass rate | Category macro average |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `sdk-smoke` | `valid` | 44 | 0 | 0 | 0 | 0 | 0 | 0 | 100.00% | — |

`sdk-smoke` run: `eval-core-reference`

### POSIX correctness

| Profile | Classification | Pass | Expected fail | Fail | Unexpected pass | Timeout | Skip N/A | Skip impl | Pass rate | Category macro average |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `spec-tests` | `valid` | 44 | 0 | 0 | 0 | 0 | 7 | 0 | 100.00% | 100.00% |
| `pjdfstest-core` | `diagnostic` | 169 | 0 | 0 | 0 | 0 | 48 | 0 | 100.00% | 100.00% |
| `xfstests-core` | `valid` | 193 | 0 | 2 | 0 | 0 | 472 | 0 | 98.97% | 99.48% |

`spec-tests` run: `eval-core-reference`

`pjdfstest-core` run: `case-correction-core-reference-pjdfstest-core`

`xfstests-core` run: `eval-core-reference`

### Robustness

Profile: `robustness`

Outcome: `candidate-failed`; classification: `diagnostic`.

| Metric | Value |
|---|---:|
| Total | 47 |
| Pass | 46 |
| Fail | 1 |
| Timeout | 0 |
| Pass rate | 97.87% |
| Category macro average | 97.50% |

Timeouts count as failures; N/A is excluded; the macro average weights categories equally.

Run: `core-robustness-reference-20260925-r1`
Execution backend: `hyperv`

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

Run: `core-canonical-reference-oracle3-r4`
Execution backend: `hyperv`

### Crash consistency

Profile: `crash-core`

Outcome: `candidate-failed`; classification: `diagnostic`.

| Metric | Value |
|---|---:|
| Total | 216 |
| Pass | 212 |
| Fail | 4 |
| Timeout | 0 |
| Pass rate | 98.15% |
| Category macro average | 98.15% |

Run: `case-correction-core-reference-crash-core`

### Real-world applications

Profile: `real-world`

Outcome: `pass`; classification: `diagnostic`.

| Metric | Value |
|---|---:|
| Total | 4 |
| Pass | 4 |
| Fail | 0 |
| Timeout | 0 |
| Pass rate | 100.00% |
| Category macro average | 100.00% |

Run: `core-realworld-reference-oracle3-r6`
Execution backend: `hyperv`

### CPU performance

Profile: `perf-cpu`

No publishable evaluation result yet (`not-run`), not a measured zero; the index provisionally imputes zero.

### NVMe performance

Profile: `perf-nvme`

No publishable evaluation result yet (`not-run`), not a measured zero; the index provisionally imputes zero.

## Scope and provenance

[JSON](../data/core/reference-calibration.json)

Original profile results remain independent; the index adds a derived provisional total with missing scores imputed as zero. JSON retains per-profile run, manifest, report, candidate, and fixed-surface identities, plus the execution backend where recorded; different backends are not treated as identical execution conditions.

`valid` means the profile evidence is valid, not that all cases passed or release eligibility is established. `candidate-failed` is not an infrastructure error. `diagnostic` is diagnostic-only; `noneligible` cannot enter rankings. Hyper-V performance is always diagnostic.

Security-review and maintainability scores are not assessed. Reference is a separate calibration baseline, not a model-generation entry.
<!-- core-results:end -->
