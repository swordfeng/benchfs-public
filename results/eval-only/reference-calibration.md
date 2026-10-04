# Reference calibration baseline

Submission: `core-reference`

[中文](reference-calibration.zh-CN.md)

This page reports Eval observations only, without generation counts, duration, tokens or costs. Candidate and fixed-surface identities are recorded per profile.

SDK/POSIX and other profiles do not all use the same candidate source, so they are not a complete evaluation of one candidate binary; each profile lists its own run.

## Results

<!-- core-results:begin -->

### Build and SDK checks

| Profile | Classification | Pass | Expected fail | Fail | Unexpected pass | Timeout | Skip N/A | Skip impl | Pass rate | Category macro average |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `sdk-smoke` | `diagnostic` | 82 | 0 | 0 | 0 | 0 | 0 | 0 | 100.00% | — |

`sdk-smoke` run: `reference-review-sdk-20261004-r1`

### POSIX correctness

| Profile | Classification | Pass | Expected fail | Fail | Unexpected pass | Timeout | Skip N/A | Skip impl | Pass rate | Category macro average |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `spec-tests` | `diagnostic` | 44 | 0 | 0 | 0 | 0 | 7 | 0 | 100.00% | 100.00% |
| `pjdfstest-core` | `diagnostic` | 169 | 0 | 0 | 0 | 0 | 48 | 0 | 100.00% | 100.00% |
| `xfstests-core` | `diagnostic` | 190 | 1 | 2 | 0 | 1 | 473 | 0 | 98.45% | 99.42% |

`spec-tests` run: `reference-readdir-spec-20261004-r1`

`pjdfstest-core` run: `reference-readdir-pjd-20261004-r1`

`xfstests-core` run: `case-correction-core-reference-xfstests-core`

### Robustness

Profile: `robustness`

Outcome: `pass`; classification: `diagnostic`.

| Metric | Value |
|---|---:|
| Total | 47 |
| Pass | 47 |
| Fail | 0 |
| Timeout | 0 |
| Pass rate | 100.00% |
| Category macro average | 100.00% |

Timeouts count as failures; N/A is excluded; the macro average weights categories equally.

Run: `reference-review-robustness-20261004-r1`
Execution backend: `hyperv`

### On-disk format conformance

Profile: `canonical-format`

Outcome: `pass`; classification: `diagnostic`.

| Metric | Value |
|---|---:|
| Total | 479 |
| Pass | 479 |
| Fail | 0 |
| Timeout | 0 |
| Pass rate | 100.00% |
| Category macro average | 100.00% |
| Conformance score (0–100) | 100.00 |
| Full conformance | yes |

Run: `reference-readdir-canonical-20261004-r1`
Execution backend: `hyperv`

### Crash consistency

Profile: `crash-core`

Outcome: `pass`; classification: `diagnostic`.

| Metric | Value |
|---|---:|
| Total | 216 |
| Pass | 216 |
| Fail | 0 |
| Timeout | 0 |
| Pass rate | 100.00% |
| Category macro average | 100.00% |

The table above summarizes whether cases meet all requirements. The table below scores each verified capability for robustness R.

| Check | Score / 100 | Requirements met | Requirements not met | Not verified (scores 0) |
|---|---:|---:|---:|---:|
| Disk structure | 100.00 | 216 | 0 | 0 |
| Data correctness | 100.00 | 216 | 0 | 0 |
| Unmount and reopen | 100.00 | 216 | 0 | 0 |

Crash score: **100.00**. Weights: disk structure 40%, data correctness 40%, unmount and reopen 20%.

Permitted safe rejections: 204. Two rounds tested: 12. Reopen tests not run after failure: 0. First round not tested: 0.

Each round includes mount, checks, and unmount. Safe rejection meets requirements but does not prove data recovery. Reopen not tested after an earlier failure does not count as a pass. See [scoring](../../docs/scoring.md) for the meaning of each check.

Run: `reference-readdir-crash-20261004-r1`
Execution backend: `hyperv`

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

Run: `reference-review-realworld-20261004-r2`
Execution backend: `hyperv`

### CPU performance

Profile: `perf-cpu`

Outcome: `pass`; classification: `diagnostic`.

Performance score: **61.00** (comparison pool `hyperv-fixed-vhdx`). Assessed budget: 100.0%. The range allowed by missing inputs is 61.00–61.00, not a confidence interval.

Components (out of 100): File I/O 45.90; Metadata 39.64; RAM 100.00; Write efficiency 99.66.
Peak RAM: 62.1 MiB. Metadata: 14 cells have valid rates, 0 candidate failures, 0 unmeasured.

Write amplification = controller write bytes / application logical write bytes in the timed window; it is not SSD-internal amplification. This ratio does not apply to metadata operations. A dash is not zero. Candidate failures score 0; unmeasured cells retain their budget and provisionally contribute 0. * marks an unassessed share.

| Workload | Cache | Samples | Rate | MAD | CV | Candidate-failed samples | Write amplification | Speed score / 100 |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| Sequential I/O | cold | 5 | 323.5 MiB/s | 6.9 | 6.5% | 0 | 0.998× | 34.87 |
| Sequential I/O | warm | 5 | 323.0 MiB/s | 2.2 | 9.4% | 0 | 1.001× | 34.89 |
| Random I/O, single thread | cold | 3 | 4,207.9 IOPS | 23.7 | 0.5% | 0 | 0.996× | 27.93 |
| Random I/O, single thread | warm | 3 | 4,159.2 IOPS | 42.6 | 1.1% | 0 | 0.996× | 27.65 |
| Random I/O, multiple threads | cold | 5 | 7,018.3 IOPS | 20.8 | 1.1% | 0 | 0.008× | 25.87 |
| Random I/O, multiple threads | warm | 5 | 7,051.6 IOPS | 35.5 | 0.5% | 0 | 0.000× | 25.97 |
| Random I/O, shared regions | cold | 5 | 6,947.5 IOPS | 19.4 | 0.7% | 0 | 0.017× | 37.64 |
| Random I/O, shared regions | warm | 5 | 6,929.9 IOPS | 83.3 | 1.0% | 0 | 0.000× | 37.56 |
| Synchronized overwrite | cold | 3 | 2,590.8 ops/s | 22.8 | 0.9% | 0 | 1.031× | 78.34 |
| Synchronized overwrite | warm | 3 | 2,614.4 ops/s | 6.4 | 0.6% | 0 | 1.032× | 78.49 |
| Create, stat and unlink | cold | 3 | 2,686.0 ops/s | 14.2 | 0.4% | 0 | — | 24.17 |
| Create, stat and unlink | warm | 3 | 2,665.4 ops/s | 4.3 | 0.4% | 0 | — | 24.67 |
| Deep and large-directory lookup | cold | 3 | 7,144.4 ops/s | 30.5 | 0.4% | 0 | — | 14.58 |
| Deep and large-directory lookup | warm | 3 | 7,284.5 ops/s | 37.1 | 0.6% | 0 | — | 15.36 |
| Same-directory rename | cold | 3 | 1,715.2 ops/s | 15.0 | 0.8% | 0 | — | 27.23 |
| Same-directory rename | warm | 3 | 1,716.0 ops/s | 0.3 | 0.4% | 0 | — | 26.92 |
| Same-directory concurrent mutation | cold | 3 | 3,614.3 ops/s | 5.3 | 0.6% | 0 | — | 29.51 |
| Same-directory concurrent mutation | warm | 3 | 3,589.4 ops/s | 3.7 | 0.7% | 0 | — | 29.88 |
| Concurrent creation in different directories | cold | 3 | 3,506.6 ops/s | 31.1 | 1.4% | 0 | — | 35.36 |
| Concurrent creation in different directories | warm | 3 | 3,594.4 ops/s | 3.4 | 0.1% | 0 | — | 35.74 |
| Parallel fsync | cold | 3 | 1,418.9 ops/s | 4.6 | 1.2% | 0 | — | 100.00 |
| Parallel fsync | warm | 3 | 1,402.9 ops/s | 0.3 | 1.1% | 0 | — | 100.00 |
| Reclamation near full capacity | cold | 4 | 396.7 ops/s | 4.4 | 1.8% | 0 | — | 50.93 |
| Reclamation near full capacity | warm | 5 | 389.4 ops/s | 7.7 | 4.6% | 0 | — | 49.39 |

Run: `reference-fallocate-perf-cpu-20261004-r1`
Execution backend: `hyperv`

### NVMe performance

Profile: `perf-nvme`

Outcome: `pass`; classification: `diagnostic`.

Performance score: **60.24** (comparison pool `hyperv-fixed-vhdx`). Assessed budget: 100.0%. The range allowed by missing inputs is 60.24–60.24, not a confidence interval.

Components (out of 100): File I/O 45.25; Metadata 37.25; RAM 100.00; Write efficiency 99.67.
Peak RAM: 67.9 MiB. Metadata: 14 cells have valid rates, 0 candidate failures, 0 unmeasured.

Write amplification = controller write bytes / application logical write bytes in the timed window; it is not SSD-internal amplification. This ratio does not apply to metadata operations. A dash is not zero. Candidate failures score 0; unmeasured cells retain their budget and provisionally contribute 0. * marks an unassessed share.

| Workload | Cache | Samples | Rate | MAD | CV | Candidate-failed samples | Write amplification | Speed score / 100 |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| Sequential I/O | cold | 5 | 248.8 MiB/s | 3.4 | 2.0% | 0 | 0.999× | 39.53 |
| Sequential I/O | warm | 5 | 251.2 MiB/s | 0.7 | 1.5% | 0 | 1.000× | 39.82 |
| Random I/O, single thread | cold | 5 | 2,536.5 IOPS | 16.1 | 0.7% | 0 | 1.000× | 27.82 |
| Random I/O, single thread | warm | 5 | 2,533.3 IOPS | 15.1 | 0.6% | 0 | 1.000× | 27.75 |
| Random I/O, multiple threads | cold | 5 | 3,161.9 IOPS | 0.7 | 0.7% | 0 | 0.007× | 20.76 |
| Random I/O, multiple threads | warm | 5 | 3,205.4 IOPS | 25.8 | 2.1% | 0 | 0.000× | 21.01 |
| Random I/O, shared regions | cold | 5 | 3,155.0 IOPS | 15.0 | 0.7% | 0 | 0.011× | 31.52 |
| Random I/O, shared regions | warm | 5 | 3,156.1 IOPS | 8.6 | 0.5% | 0 | 0.000× | 31.51 |
| Synchronized overwrite | cold | 3 | 1,761.9 ops/s | 21.9 | 1.3% | 0 | 1.030× | 75.20 |
| Synchronized overwrite | warm | 3 | 1,761.8 ops/s | 8.1 | 0.4% | 0 | 1.032× | 75.18 |
| Create, stat and unlink | cold | 3 | 2,669.0 ops/s | 0.0 | 0.2% | 0 | — | 24.03 |
| Create, stat and unlink | warm | 3 | 2,649.6 ops/s | 6.2 | 0.3% | 0 | — | 24.54 |
| Deep and large-directory lookup | cold | 1 | 6,968.2 ops/s | 0.0 | 0.0% | 0 | — | 14.04 |
| Deep and large-directory lookup | warm | 1 | 7,331.6 ops/s | 0.0 | 0.0% | 0 | — | 15.50 |
| Same-directory rename | cold | 1 | 1,692.3 ops/s | 0.0 | 0.0% | 0 | — | 26.94 |
| Same-directory rename | warm | 1 | 1,671.1 ops/s | 0.0 | 0.0% | 0 | — | 26.34 |
| Same-directory concurrent mutation | cold | 3 | 3,593.7 ops/s | 10.6 | 0.3% | 0 | — | 29.39 |
| Same-directory concurrent mutation | warm | 3 | 3,578.2 ops/s | 7.0 | 0.3% | 0 | — | 29.81 |
| Concurrent creation in different directories | cold | 2 | 3,386.6 ops/s | 2.2 | 0.1% | 0 | — | 34.61 |
| Concurrent creation in different directories | warm | 2 | 3,361.8 ops/s | 35.3 | 1.1% | 0 | — | 34.29 |
| Parallel fsync | cold | 2 | 526.6 ops/s | 14.3 | 2.7% | 0 | — | 89.21 |
| Parallel fsync | warm | 1 | 533.4 ops/s | 0.0 | 0.0% | 0 | — | 89.37 |
| Reclamation near full capacity | cold | 5 | 314.4 ops/s | 7.5 | 3.7% | 0 | — | 45.88 |
| Reclamation near full capacity | warm | 5 | 309.2 ops/s | 10.0 | 5.5% | 0 | — | 44.38 |

Run: `reference-fallocate-perf-nvme-20261004-r1`
Execution backend: `hyperv`

### Agent code review

S: **95.24** / 100; status: `complete`; purpose: `diagnostic`.

| Critical | High | Medium | Low |
|---:|---:|---:|---:|
| 0 | 0 | 1 | 0 |

Counts include confirmed, deduplicated root causes; S contributes 20% of the provisional total.

Scoring rule: `agent-review-score-v2`.

## Scope and provenance

Original profile results remain independent; the index adds a derived provisional total with missing scores imputed as zero. Each profile lists its run and, where recorded, its execution backend; different backends are not treated as identical execution conditions.

`valid` means the profile evidence is valid, not that all cases passed or release eligibility is established. `candidate-failed` is not an infrastructure error. `diagnostic` is diagnostic-only; `noneligible` cannot enter rankings. Hyper-V performance is not release- or ranking-eligible; protocol-v2 Hyper-V runs on the fixed VHDX/memory devices form their own `hyperv-fixed-vhdx` pool, counted in the provisional total and not comparable with KVM/raw-NVMe scores.

Maintainability (`maint-v3-policy`, 5% of the overall score): **84.63** / 100; components are in the README. Reference is a separate calibration baseline, not a model-generation entry.
<!-- core-results:end -->
