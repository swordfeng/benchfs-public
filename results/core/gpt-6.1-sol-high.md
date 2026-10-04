# GPT 6.1 Sol · Pi — Core results

[中文](gpt-6.1-sol-high.zh-CN.md) · [Back to results](../../README.md#results)

GPT 6.1 Sol used Pi to complete the Core development task. This report includes evaluation results, development time, token usage, and estimated cost.

## Run configuration

| Field | Value |
|---|---|
| Model | `gpt-6.1-sol` |
| Coding agent | Pi 0.84.3 |
| Reasoning effort | High, set by the model proxy configuration |
| Development runs | 1 |
| Submission ID | `core-gpt-6.1-sol` |
| Development date | 2026-09-30 (UTC) |

See the [experimental environment](../../docs/environment.md) for development resources and tools. Evaluation run IDs and execution backends appear alongside each profile below.

## Results

<!-- core-results:begin -->

### Build and SDK checks

| Profile | Classification | Pass | Expected fail | Fail | Unexpected pass | Timeout | Skip N/A | Skip impl | Pass rate | Category macro average |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `sdk-smoke` | `valid` | 54 | 0 | 0 | 0 | 0 | 0 | 0 | 100.00% | — |

`sdk-smoke` run: `eval-core-gpt-6-1-sol-sdk-smoke`

### POSIX correctness

| Profile | Classification | Pass | Expected fail | Fail | Unexpected pass | Timeout | Skip N/A | Skip impl | Pass rate | Category macro average |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `spec-tests` | `valid` | 44 | 0 | 0 | 0 | 0 | 7 | 0 | 100.00% | 100.00% |
| `pjdfstest-core` | `valid` | 169 | 0 | 0 | 0 | 0 | 48 | 0 | 100.00% | 100.00% |
| `xfstests-core` | `valid` | 186 | 1 | 3 | 0 | 4 | 473 | 0 | 96.39% | 89.81% |

`spec-tests` run: `eval-core-gpt-6-1-sol-spec-tests`

`pjdfstest-core` run: `eval-core-gpt-6-1-sol-pjdfstest-core`

`xfstests-core` run: `eval-core-gpt-6-1-sol-xfstests-core`

### Robustness

Profile: `robustness`

Outcome: `pass`; classification: `valid`.

| Metric | Value |
|---|---:|
| Total | 47 |
| Pass | 47 |
| Fail | 0 |
| Timeout | 0 |
| Pass rate | 100.00% |
| Category macro average | 100.00% |

Timeouts count as failures; N/A is excluded; the macro average weights categories equally.

Run: `eval-core-gpt-6-1-sol-robustness`
Execution backend: `qemu-kvm`

### On-disk format conformance

Profile: `canonical-format`

Outcome: `pass`; classification: `valid`.

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

Run: `eval-core-gpt-6-1-sol-canonical-format`
Execution backend: `qemu-kvm`

### Crash consistency

Profile: `crash-core`

Outcome: `pass`; classification: `valid`.

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

Run: `eval-core-gpt-6-1-sol-crash-core-r2`
Execution backend: `qemu-kvm`

### Real-world applications

Profile: `real-world`

Outcome: `candidate-failed`; classification: `valid`.

| Metric | Value |
|---|---:|
| Total | 4 |
| Pass | 3 |
| Fail | 1 |
| Timeout | 0 |
| Pass rate | 75.00% |
| Category macro average | 83.33% |

Run: `eval-core-gpt-6-1-sol-real-world`
Execution backend: `qemu-kvm`

### CPU performance

Profile: `perf-cpu`

Outcome: `pass`; classification: `diagnostic`.

Performance score: **57.17** (comparison pool `hyperv-fixed-vhdx`).

| Family | Cache | Primary metric | Samples | Median | MAD | CV | Candidate-failed samples |
|---|---|---|---:|---:|---:|---:|---:|
| buffered-synchronized-writes | cold | operations_per_second | 3 | 2,589.9 | 12.3 | 1.1% | 0 |
| buffered-synchronized-writes | warm | operations_per_second | 3 | 2,575.1 | 0.7 | 0.5% | 0 |
| bulk-sequential-io | cold | MiB/s | 5 | 328.3 | 1.3 | 0.5% | 0 |
| bulk-sequential-io | warm | MiB/s | 5 | 325.2 | 1.1 | 1.4% | 0 |
| metadata-concurrency | cold | operations_per_second | 3 | 2,487.9 | 6.8 | 0.4% | 0 |
| metadata-concurrency | cold | operations_per_second | 3 | 7,475.2 | 3.1 | 0.3% | 0 |
| metadata-concurrency | cold | operations_per_second | 3 | 1,741.3 | 14.2 | 0.8% | 0 |
| metadata-concurrency | cold | operations_per_second | 3 | 3,434.6 | 5.3 | 0.3% | 0 |
| metadata-concurrency | cold | operations_per_second | 3 | 2,628.7 | 0.2 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 3 | 1,280.2 | 2.5 | 0.4% | 0 |
| metadata-concurrency | cold | operations_per_second | 4 | 428.7 | 2.3 | 2.1% | 0 |
| metadata-concurrency | warm | operations_per_second | 3 | 2,490.1 | 6.5 | 0.4% | 0 |
| metadata-concurrency | warm | operations_per_second | 3 | 7,541.2 | 3.9 | 0.6% | 0 |
| metadata-concurrency | warm | operations_per_second | 3 | 1,734.6 | 3.6 | 0.3% | 0 |
| metadata-concurrency | warm | operations_per_second | 3 | 3,432.2 | 9.1 | 0.4% | 0 |
| metadata-concurrency | warm | operations_per_second | 3 | 2,618.9 | 4.8 | 0.2% | 0 |
| metadata-concurrency | warm | operations_per_second | 3 | 1,284.5 | 8.7 | 0.7% | 0 |
| metadata-concurrency | warm | operations_per_second | 3 | 432.7 | 2.4 | 1.2% | 0 |
| small-random-io | cold | iops | 3 | 4,594.4 | 41.8 | 1.0% | 0 |
| small-random-io | cold | iops | 5 | 7,248.6 | 39.2 | 1.6% | 0 |
| small-random-io | cold | iops | 3 | 7,290.2 | 17.7 | 0.4% | 0 |
| small-random-io | warm | iops | 3 | 4,593.1 | 38.9 | 0.8% | 0 |
| small-random-io | warm | iops | 5 | 7,204.1 | 57.6 | 1.5% | 0 |
| small-random-io | warm | iops | 3 | 7,274.4 | 58.4 | 0.8% | 0 |

Run: `perf-v2-core-gpt-6-1-sol-perf-cpu-r4`
Execution backend: `hyperv`

### NVMe performance

Profile: `perf-nvme`

Outcome: `pass`; classification: `diagnostic`.

Performance score: **56.91** (comparison pool `hyperv-fixed-vhdx`).

| Family | Cache | Primary metric | Samples | Median | MAD | CV | Candidate-failed samples |
|---|---|---|---:|---:|---:|---:|---:|
| buffered-synchronized-writes | cold | operations_per_second | 3 | 1,772.7 | 6.3 | 0.4% | 0 |
| buffered-synchronized-writes | warm | operations_per_second | 3 | 1,761.4 | 1.8 | 0.7% | 0 |
| bulk-sequential-io | cold | MiB/s | 5 | 271.4 | 0.5 | 0.3% | 0 |
| bulk-sequential-io | warm | MiB/s | 5 | 272.9 | 1.6 | 0.8% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 2,219.3 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 7,579.4 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 1,718.7 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 3,049.1 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 1,918.1 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 498.8 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 400.4 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 2,214.4 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 7,670.1 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 1,735.2 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 3,034.1 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 1,910.6 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 487.2 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 402.4 | 0.0 | 0.0% | 0 |
| small-random-io | cold | iops | 3 | 2,664.7 | 3.6 | 0.2% | 0 |
| small-random-io | cold | iops | 5 | 3,231.6 | 24.1 | 1.5% | 0 |
| small-random-io | cold | iops | 5 | 3,172.1 | 33.8 | 1.3% | 0 |
| small-random-io | warm | iops | 3 | 2,668.5 | 13.7 | 0.5% | 0 |
| small-random-io | warm | iops | 5 | 3,379.5 | 44.6 | 2.4% | 0 |
| small-random-io | warm | iops | 5 | 3,126.7 | 46.2 | 1.7% | 0 |

Run: `perf-v2-core-gpt-6-1-sol-perf-nvme-r7`
Execution backend: `hyperv`

### Agent code review

S: **86.96** / 100; status: `complete`; purpose: `diagnostic`.

| Critical | High | Medium | Low |
|---:|---:|---:|---:|
| 0 | 1 | 0 | 0 |

Counts include confirmed, deduplicated root causes; S contributes 20% of the provisional total.

Scoring rule: `agent-review-score-v2`.

## Scope and provenance

Original profile results remain independent; the index adds a derived provisional total with missing scores imputed as zero. Each profile lists its run and, where recorded, its execution backend; different backends are not treated as identical execution conditions.

`valid` means the profile evidence is valid, not that all cases passed or release eligibility is established. `candidate-failed` is not an infrastructure error. `diagnostic` is diagnostic-only; `noneligible` cannot enter rankings. Hyper-V performance is not release- or ranking-eligible; protocol-v2 Hyper-V runs on the fixed VHDX/memory devices form their own `hyperv-fixed-vhdx` pool, counted in the provisional total and not comparable with KVM/raw-NVMe scores.

Maintainability (`maint-v3-policy`, 5% of the overall score): **83.32** / 100; components are in the README. Reference is a separate calibration baseline, not a model-generation entry.
<!-- core-results:end -->

## Development time

| Metric | Record |
|---|---|
| Goal start | 2026-09-30 08:04:55.609 UTC |
| Goal completion | 2026-09-30 19:45:06.327 UTC |
| Active time | **7 h 14 min 12 s** |
| Elapsed time | 11 h 40 min 11 s |
| Goal status | Complete (agent-recorded) |

Active time is Pi's cumulative goal counter, 26,052.273 seconds. It is recorded separately from elapsed time and does not measure model inference time.

## Token usage and cost

| Metric | Count |
|---|---:|
| Uncached input tokens | 2,000,258 |
| Cache-read tokens | 29,043,712 |
| Separately recorded cache-write tokens | 0 |
| Total input tokens | 31,043,970 |
| Output tokens, including reasoning | 184,515 |
| Of which: reasoning tokens | 112,947 |
| Total tokens | 31,228,485 |
| Estimated cost (USD) | **$12.87** |

Usage is summed from 172 responses with token records and agrees with the archived proxy totals. Reasoning is included in output and is not charged twice. Input includes context repeated across requests.

The estimate uses [OpenAI's GPT 6.1 Sol prices](https://developers.openai.com/api/docs/models/gpt-6.1-sol), checked on 2026-09-30. Standard rates per million tokens are $2 for input, $0.10 for cache reads, $2.50 for cache writes, and $10 for output. Requests with total input above 272,000 tokens use twice the input and cache rates and 1.5 times the output rate, applied to the whole request.

| Request tier | Responses | Uncached input | Cache reads | Output, including reasoning |
|---|---:|---:|---:|---:|
| Standard | 150 | 1,078,628 | 23,389,312 | 135,009 |
| Long context | 22 | 921,630 | 5,654,400 | 49,506 |

The records do not separately identify cache writes. Consistent with the earlier Sol report, the estimate prices all uncached input at the cache-write rate, replacing the ordinary input charge rather than adding to it. The resulting $12.8672112 rounds to **$12.87**. Pricing that input at the ordinary rate instead gives $11.41. This is a recorded-usage estimate at published prices, not an invoice.

## Source size

Only implementation, tests, and scripts under the submitted `bench/` directory are counted. The fixed SDK, FUSE adapter, dependencies, documentation, configuration, and generated files are excluded.

| Component | Files containing this component | Physical lines | Code lines |
|---|---:|---:|---:|
| Rust implementation | 10 | 4,032 | 3,960 |
| Agent-written Rust tests | 4 | 1,113 | 1,100 |
| Scripts | 1 | 95 | 82 |

Physical lines include blanks and comments; code lines exclude blanks, comments, and documentation comments. Inline tests and implementation can share files, so file counts are not additive; there are 13 distinct Rust files. Source size, time, and cost are descriptive and do not affect scores.
