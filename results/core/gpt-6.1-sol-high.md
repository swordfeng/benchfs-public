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

No publishable evaluation result yet (`not-run`), not a measured zero; the index provisionally imputes zero.

### NVMe performance

Profile: `perf-nvme`

No publishable evaluation result yet (`not-run`), not a measured zero; the index provisionally imputes zero.

## Scope and provenance

Original profile results remain independent; the index adds a derived provisional total with missing scores imputed as zero. Each profile lists its run and, where recorded, its execution backend; different backends are not treated as identical execution conditions.

`valid` means the profile evidence is valid, not that all cases passed or release eligibility is established. `candidate-failed` is not an infrastructure error. `diagnostic` is diagnostic-only; `noneligible` cannot enter rankings. Hyper-V performance is always diagnostic.

Security-review and maintainability scores are not assessed. Reference is a separate calibration baseline, not a model-generation entry.
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
