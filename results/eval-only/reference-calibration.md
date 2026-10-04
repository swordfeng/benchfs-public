# Reference calibration baseline

Submission: `core-reference`

[中文](reference-calibration.zh-CN.md)

This page reports Eval observations only, without generation counts, duration, tokens or costs. Candidate and fixed-surface identities are recorded per profile; performance is unmeasured.

SDK/POSIX and other profiles do not all use the same candidate source, so they are not a complete evaluation of one candidate binary; each profile lists its own run.

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

The table above summarizes whether cases meet all requirements. The table below scores each verified capability for robustness R.

| Check | Score / 100 | Requirements met | Requirements not met | Not verified (scores 0) |
|---|---:|---:|---:|---:|
| Disk structure | 99.07 | 214 | 2 | 0 |
| Data correctness | 100.00 | 216 | 0 | 0 |
| Unmount and reopen | 98.15 | 212 | 4 | 0 |

Crash score: **99.26**. Weights: disk structure 40%, data correctness 40%, unmount and reopen 20%.

Permitted safe rejections: 39. Two rounds tested: 175. Reopen tests not run after failure: 2. First round not tested: 0.

Each round includes mount, checks, and unmount. Safe rejection meets requirements but does not prove data recovery. Reopen not tested after an earlier failure does not count as a pass. See [scoring](../../docs/scoring.md) for the meaning of each check.

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

### Agent code review

Status: `incomplete`; S has no complete result, shown as — and imputed as zero only in the provisional total.

Scoring rule: `agent-review-score-v2`.

## Scope and provenance

Original profile results remain independent; the index adds a derived provisional total with missing scores imputed as zero. Each profile lists its run and, where recorded, its execution backend; different backends are not treated as identical execution conditions.

`valid` means the profile evidence is valid, not that all cases passed or release eligibility is established. `candidate-failed` is not an infrastructure error. `diagnostic` is diagnostic-only; `noneligible` cannot enter rankings. Hyper-V performance is not release- or ranking-eligible; protocol-v2 Hyper-V runs on the fixed VHDX/memory devices form their own `hyperv-fixed-vhdx` pool, counted in the provisional total and not comparable with KVM/raw-NVMe scores.

Maintainability (`maint-v3-policy`, 5% of the overall score): **67.39** / 100; components are in the README. Reference is a separate calibration baseline, not a model-generation entry.
<!-- core-results:end -->
