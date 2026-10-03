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

Outcome: `pass`; classification: `diagnostic`.


#### buffered-synchronized-writes / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 2589.9 |
| MAD | 12.300000000000182 |
| CV | 0.010634977592942763 |
| Candidate-failed samples | 0 |

#### buffered-synchronized-writes / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 2575.1 |
| MAD | 0.6999999999998181 |
| CV | 0.005009175776058064 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / cold

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 5 |
| Median | 344266330.0 |
| MAD | 1343721.0 |
| CV | 0.004567858205404987 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / warm

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 5 |
| Median | 341036094.0 |
| MAD | 1189710.0 |
| CV | 0.014480246901825276 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 2487.936957251538 |
| MAD | 6.824729674292485 |
| CV | 0.004449545859151276 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 7475.16981178434 |
| MAD | 3.1392913399140525 |
| CV | 0.002599565134307536 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 1741.3359903548012 |
| MAD | 14.213507644051106 |
| CV | 0.007916727349025047 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 3434.6284469153957 |
| MAD | 5.29287290630964 |
| CV | 0.0034689536489952394 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 2628.693659772316 |
| MAD | 0.19285512903661584 |
| CV | 8.216933484033831e-05 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 1280.172633225453 |
| MAD | 2.5024326215852852 |
| CV | 0.0036888234443918696 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 4 |
| Median | 428.65068790029625 |
| MAD | 2.250445796413601 |
| CV | 0.020676974519239177 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 2490.0855496630143 |
| MAD | 6.533544822847944 |
| CV | 0.004027740717027429 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 7541.235595387099 |
| MAD | 3.9051664299859112 |
| CV | 0.0063323238423675324 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 1734.6490644058342 |
| MAD | 3.6090870499558605 |
| CV | 0.0027039560228499654 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 3432.1666903519776 |
| MAD | 9.125102063893337 |
| CV | 0.004319153530145256 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 2618.9141105267854 |
| MAD | 4.768780328462981 |
| CV | 0.0015159235272029568 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 1284.540433212674 |
| MAD | 8.652845708976201 |
| CV | 0.00651471508561826 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 432.6613765640889 |
| MAD | 2.366026464568961 |
| CV | 0.012499628419648615 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 3 |
| Median | 4594.440556 |
| MAD | 41.795820000000276 |
| CV | 0.009837651237396538 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 7248.550289999999 |
| MAD | 39.17122699999891 |
| CV | 0.01609814472232201 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 3 |
| Median | 7290.170943 |
| MAD | 17.698049999999967 |
| CV | 0.00418346223358922 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 3 |
| Median | 4593.140686000001 |
| MAD | 38.89611100000002 |
| CV | 0.0075593074838785764 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 7204.070981999999 |
| MAD | 57.57129000000077 |
| CV | 0.01450394288933547 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 3 |
| Median | 7274.389714 |
| MAD | 58.41395000000011 |
| CV | 0.007644529229799184 |
| Candidate-failed samples | 0 |

Run: `perf-v2-core-gpt-6-1-sol-perf-cpu-r4`
Execution backend: `hyperv`

### NVMe performance

Profile: `perf-nvme`

Outcome: `pass`; classification: `diagnostic`.


#### buffered-synchronized-writes / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 1772.7 |
| MAD | 6.2999999999999545 |
| CV | 0.00363696834126647 |
| Candidate-failed samples | 0 |

#### buffered-synchronized-writes / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 1761.4 |
| MAD | 1.800000000000182 |
| CV | 0.006654794394648604 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / cold

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 5 |
| Median | 284546142.0 |
| MAD | 566714.0 |
| CV | 0.0034392725064478573 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / warm

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 5 |
| Median | 286132523.0 |
| MAD | 1691342.0 |
| CV | 0.008107017532512925 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 2219.3449838175734 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 7579.417969536214 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 1718.655883839244 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 3049.111745094359 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 1918.076050003212 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 498.7744451629674 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 400.37479885670575 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 2214.449216364136 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 7670.12375092865 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 1735.1704840784162 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 3034.103712731025 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 1910.5668652252132 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 487.1631681580132 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 402.44571627724565 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 3 |
| Median | 2664.7335270000003 |
| MAD | 3.5996399999994537 |
| CV | 0.002418763063490954 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 3231.553569 |
| MAD | 24.09518100000014 |
| CV | 0.015097262488616118 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 3172.065047 |
| MAD | 33.79366100000016 |
| CV | 0.01296737490477681 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 3 |
| Median | 2668.533147 |
| MAD | 13.698629999999866 |
| CV | 0.005450133010804131 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 3379.5399829999997 |
| MAD | 44.640452999999525 |
| CV | 0.024289937223036035 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 3126.7366309999998 |
| MAD | 46.19419700000026 |
| CV | 0.017480171952505463 |
| Candidate-failed samples | 0 |

Run: `perf-v2-core-gpt-6-1-sol-perf-nvme-r7`
Execution backend: `hyperv`

## Scope and provenance

Original profile results remain independent; the index adds a derived provisional total with missing scores imputed as zero. Each profile lists its run and, where recorded, its execution backend; different backends are not treated as identical execution conditions.

`valid` means the profile evidence is valid, not that all cases passed or release eligibility is established. `candidate-failed` is not an infrastructure error. `diagnostic` is diagnostic-only; `noneligible` cannot enter rankings. Hyper-V performance is not release- or ranking-eligible; protocol-v2 Hyper-V runs on the fixed VHDX/memory devices form their own `hyperv-fixed-vhdx` pool, counted in the provisional total and not comparable with KVM/raw-NVMe scores.

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
