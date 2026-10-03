# Fable 5.1 — Core — High

[中文](fable-5.1-high.zh-CN.md)

This page collects independent Core Eval profile results. Each profile lists its status and run provenance; unmeasured dimensions remain unmeasured.

## Run

| Field | Value |
|---|---|
| Model | `fable-5.1` |
| Effort | High |
| Variant | Core |
| Agent harness | Pi |
| Generation runs | 1 (`n=1`) |
| SDK/POSIX evaluation run | `eval-core-fable-5.1` |
| Initial SDK/POSIX evaluation date | 2026-09-04 (UTC) |
| SDK/POSIX results updated | 2026-09-05 (UTC) |

The effective effort is High, confirmed by the operator for the proxy override. The Pi session's local thinking-level setting does not represent the provider-side effort.

## Submission size

Only candidate-owned source under `bench/` is counted. The fixed SDK and FUSE adapter, dependencies, documentation, configuration, lockfiles, and generated artifacts are excluded.

| Component | Files containing this component | Physical lines | Code lines |
|---|---:|---:|---:|
| Implementation (Rust) | 13 | 5,826 | 5,175 |
| Agent-written tests (Rust) | 2 | 395 | 347 |
| Qualification scripts | 0 | 0 | 0 |

Physical lines include blanks and comments. Code lines count each line holding at least one non-comment token: blank lines, comments, and documentation comments (Rust `///` and `//!`, Python docstrings) are excluded, while Rust attributes such as `#[derive(...)]` count as code. Corrected on 2026-09-27: earlier versions of this page counted documentation comments as code and Rust attributes as comments; file and physical-line counts are unchanged. Tests are in dedicated files; there are no inline test modules or overlapping file counts. There are 15 distinct Rust files. These are source-size measurements, not test-coverage or code-quality scores.

<!-- source-tree:begin -->
### Directory structure

Every file under `bench/`, excluding build output. Numbers are code lines counted as above; "test" is the part of them in test code.

```text
bench/
├── src/
│   ├── btree.rs  (527 code)
│   ├── checker.rs  (468 code)
│   ├── core.rs  (600 code)
│   ├── data.rs  (500 code)
│   ├── format.rs  (904 code)
│   ├── fs.rs  (923 code)
│   ├── fs_impl.rs  (304 code)
│   ├── lib.rs  (11 code)
│   ├── main.rs  (81 code)
│   ├── mkfs.rs  (68 code)
│   ├── mkfs_impl.rs  (86 code)
│   ├── node.rs  (245 code)
│   └── store.rs  (458 code)
├── tests/
│   ├── btree.rs  (182 code, 182 test)
│   └── lifecycle.rs  (165 code, 165 test)
├── Cargo.toml
└── NOTES.md
```
<!-- source-tree:end -->

## Generation time and activity

| Metric | Value |
|---|---|
| Goal start | 2026-09-02 17:12:00.551 UTC |
| Goal completion | 2026-09-03 06:00:55.969 UTC |
| Goal active time | **5 h 16 min 10 s** |
| Elapsed time, including gaps between active periods | 12 h 48 min 55 s |
| Final goal status | Complete (harness-recorded) |
| Proxy API requests / HTTP responses | 275 / 275 |
| Assistant response records with nonzero token usage | 273 |
| Assistant error records | 3 |
| Agent tool calls | 272 |

Active time is the final cumulative Pi goal counter, 18,970.246 seconds. The resumed goals inherited the earlier counters; the four goal snapshots are not summed. Active time is harness-accounted time, not model inference time. A completed goal does not imply that all evaluation tests passed.

Request counts come from the archived proxy log; assistant and tool-call counts come from the Pi session. The session contains 276 assistant records, while the proxy records 275 requests. These are separate recorded counts, not interchangeable measures or a guarantee that every upstream retry or failed request was billed or fully accounted for.

### Tool usage

| Tool | Requests | Recorded failures | Unknown outcomes |
|---|---:|---:|---:|
| `bash` | 245 | 18 | 0 |
| `edit` | 1 | 0 | 0 |
| `goal_complete` | 1 | 0 | 0 |
| `read` | 13 | 0 | 0 |
| `write` | 12 | 2 | 0 |
| **Total** | **272** | **20** | **0** |

Each archived assistant `toolCall` counts as one request under its recorded tool name, including retries. Results are matched by `toolCallId` and checked for the same tool name; only `toolResult.isError: true` counts as a recorded failure. Missing or unparseable results are unknown, not assumed successful or failed. These are tool-level outcomes, not model API errors or evaluation test results.

## Token usage and API cost

| Metric | Value |
|---|---:|
| Input tokens, excluding cache (`input`) | 556 |
| Cache-read input tokens (`cache_read`) | 97,277,903 |
| Cache-write input tokens (`cache_write`) | 2,652,910 |
| Total input tokens, including cache | 99,931,369 |
| Output tokens, including thinking (`output`) | 298,429 |
| Thinking / reasoning tokens | 52,185 |
| **Total tokens** | **100,229,798** |
| Cache hit rate (input tokens) | 97.34% |
| Estimated API cost (USD) | **$72.41** |

Token counts are summed from Pi assistant-message usage records and match both the archived proxy token totals and the final cumulative goal token counter. Reasoning usage is explicitly recorded on all 273 responses with nonzero usage; it is a component of output and is not added again. Input totals include repeated context across requests; they are not the size of a single context or the number of unique tokens.

Cost uses the non-batch model-catalog rates for `anthropic/claude-fable-5.1` in [OpenRouter's Models API](https://openrouter.ai/api/v1/models), corroborated by [Fable endpoint pricing](https://openrouter.ai/api/v1/models/anthropic/claude-fable-5.1/endpoints), retrieved on 2026-09-05. Rates below are USD per million tokens. This is a current-list-price estimate, not evidence of the run-date price, actual endpoint routing, or a provider invoice; session cost fields are not used.

| Token class | USD / million tokens | Estimated cost (USD) |
|---|---:|---:|
| Non-cached input | $10.00 | $0.005560 |
| Output | $50.00 | $14.921450 |
| Cache read | $0.25 | $24.319476 |
| Cache write, 5-minute TTL | $12.50 | $33.161375 |
| Cache write, 1-hour TTL | $20.00 | $0.000000 |

The unrounded total is $72.40786075, rounded once to **$72.41**. [OpenRouter's cache TTL rules](https://openrouter.ai/docs/guides/best-practices/prompt-caching#cache-ttl-options) distinguish 5-minute and 1-hour writes. The session records 2,652,910 cache-write tokens and zero in `cacheWrite1h`, so the writes use the 5-minute rate. The separate 556 input tokens use the ordinary prompt rate; they are not also charged as cache writes. Reasoning is already included in output. The token totals cover reported usage only; unreported usage on failed requests remains unknown.

Generation time, source size, tokens, activity, and cost are descriptive metadata, not correctness scores.

## Results

<!-- core-results:begin -->

### POSIX correctness

| Profile | Classification | Pass | Expected fail | Fail | Unexpected pass | Timeout | Skip N/A | Skip impl | Pass rate | Category macro average |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `spec-tests` | `valid` | 44 | 0 | 0 | 0 | 0 | 7 | 0 | 100.00% | 100.00% |
| `pjdfstest-core` | `diagnostic` | 168 | 0 | 1 | 0 | 0 | 48 | 0 | 99.41% | 99.26% |
| `xfstests-core` | `valid` | 182 | 1 | 7 | 0 | 4 | 473 | 0 | 94.33% | 87.35% |

`spec-tests` run: `eval-core-fable-5.1`

`pjdfstest-core` run: `case-correction-core-fable-5.1-pjdfstest-core`

`xfstests-core` run: `eval-core-fable-5.1`

### Robustness

Profile: `robustness`

Outcome: `candidate-failed`; classification: `valid`.

| Metric | Value |
|---|---:|
| Total | 47 |
| Pass | 44 |
| Fail | 3 |
| Timeout | 0 |
| Pass rate | 93.62% |
| Category macro average | 90.28% |

Timeouts count as failures; N/A is excluded; the macro average weights categories equally.

Run: `eval-submission-robustness-v2-core-fable-5-1-r1`

### On-disk format conformance

Profile: `canonical-format`

Outcome: `candidate-failed`; classification: `diagnostic`.

| Metric | Value |
|---|---:|
| Total | 479 |
| Pass | 431 |
| Fail | 48 |
| Timeout | 0 |
| Pass rate | 89.98% |
| Category macro average | 95.45% |
| Conformance score (0–100) | 95.45 |
| Full conformance | no |

Run: `core-canonical-fable-oracle3-r6`
Execution backend: `hyperv`

### Crash consistency

Profile: `crash-core`

Outcome: `candidate-failed`; classification: `diagnostic`.

| Metric | Value |
|---|---:|
| Total | 216 |
| Pass | 79 |
| Fail | 137 |
| Timeout | 0 |
| Pass rate | 36.57% |
| Category macro average | 36.57% |

Run: `case-correction-core-fable-5.1-crash-core`

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

Run: `core-realworld-fable-20260925-r1`
Execution backend: `qemu-kvm`

### CPU performance

Profile: `perf-cpu`

Outcome: `candidate-failed`; classification: `diagnostic`.


#### buffered-synchronized-writes / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 2604.1 |
| MAD | 10.699999999999818 |
| CV | 0.005544940864413025 |
| Candidate-failed samples | 0 |

#### buffered-synchronized-writes / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 2603.0 |
| MAD | 4.900000000000091 |
| CV | 0.0016960475685975733 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / cold

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 3 |
| Median | 426789903.0 |
| MAD | 42639.0 |
| CV | 0.0045491493444834085 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / warm

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 5 |
| Median | 423707306.0 |
| MAD | 1636192.0 |
| CV | 0.005414205341020102 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 1 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 8731.876347755837 |
| MAD | 42.80242202532099 |
| CV | 0.004171042564132865 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 2222.5456156976306 |
| MAD | 4.269259544247234 |
| CV | 0.0028250471111577542 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 3882.1163341842034 |
| MAD | 25.75845811342333 |
| CV | 0.011228451486673854 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 3959.141207176754 |
| MAD | 13.381353221802783 |
| CV | 0.014925662396406492 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 1256.873360242264 |
| MAD | 17.300699886166512 |
| CV | 0.014073585064573643 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 5 |
| Median | 439.57029366372603 |
| MAD | 7.554022894129787 |
| CV | 0.024873508967358907 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 1 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 8906.527815171068 |
| MAD | 20.542832541939788 |
| CV | 0.004566672940778784 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 2186.4903166253957 |
| MAD | 6.054658668955199 |
| CV | 0.003945295887678986 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 3920.447329740368 |
| MAD | 22.846875588481907 |
| CV | 0.007356784016524879 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 3950.2503027713783 |
| MAD | 11.760135002751667 |
| CV | 0.010232360056205977 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 1237.9792188831311 |
| MAD | 2.97066411858259 |
| CV | 0.008851062201650885 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 434.48110399678876 |
| MAD | 1.6971980069167216 |
| CV | 0.01498283752961236 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 3 |
| Median | 5101.389861 |
| MAD | 41.59584099999938 |
| CV | 0.006881579986230277 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 3 |
| Median | 15498.250175000001 |
| MAD | 96.99030100000164 |
| CV | 0.006684622235973873 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 3 |
| Median | 7492.6476170000005 |
| MAD | 12.701807999998891 |
| CV | 0.002139004184445192 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 3 |
| Median | 5082.89171 |
| MAD | 26.697331999999733 |
| CV | 0.004565354145084272 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 15466.153385 |
| MAD | 38.69613000000027 |
| CV | 0.0035443039065416434 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 7507.19856 |
| MAD | 26.94035900000017 |
| CV | 0.004536439459584785 |
| Candidate-failed samples | 0 |

Run: `perf-v2-core-fable-5-1-perf-cpu-r2`
Execution backend: `hyperv`

### NVMe performance

Profile: `perf-nvme`

Outcome: `candidate-failed`; classification: `diagnostic`.


#### buffered-synchronized-writes / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 1701.9 |
| MAD | 1.800000000000182 |
| CV | 0.001111878086950771 |
| Candidate-failed samples | 0 |

#### buffered-synchronized-writes / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 1699.8 |
| MAD | 8.399999999999864 |
| CV | 0.00565725313385949 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / cold

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 3 |
| Median | 347586220.0 |
| MAD | 17324.0 |
| CV | 0.0061982418166546165 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / warm

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 3 |
| Median | 347149510.0 |
| MAD | 944336.0 |
| CV | 0.00442817332926593 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 2278.268173240629 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 1 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 8526.196989214313 |
| MAD | 4.008857708508003 |
| CV | 0.002299346630218897 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 2167.4585991430054 |
| MAD | 6.671336338622041 |
| CV | 0.004224948194244578 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 3931.2778598982986 |
| MAD | 0.5873387707424627 |
| CV | 0.0031274499527529373 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 3189.694866262211 |
| MAD | 0.5819843215467699 |
| CV | 0.001653218014914798 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 408.0611035446279 |
| MAD | 4.530315799566381 |
| CV | 0.010367491987695984 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 425.5955550119281 |
| MAD | 2.3190185759776796 |
| CV | 0.005656856403828658 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 1 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 8888.633991309647 |
| MAD | 16.22675267379418 |
| CV | 0.0027969225304387826 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 2110.374966785494 |
| MAD | 0.8987903834122335 |
| CV | 0.0023950314316306134 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 3915.5909318221716 |
| MAD | 1.764826834444193 |
| CV | 0.00149497205914042 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 3228.73933505313 |
| MAD | 2.030068972322624 |
| CV | 0.0010105787910827117 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 407.64042731628905 |
| MAD | 2.698423893328993 |
| CV | 0.010000652199274589 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 422.88934833780456 |
| MAD | 0.18548418197627825 |
| CV | 0.0014353188556722927 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 3 |
| Median | 3410.7589239999998 |
| MAD | 16.298369999999522 |
| CV | 0.004685581925382182 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 8219.078092 |
| MAD | 30.28470400000151 |
| CV | 0.0033149286192976388 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 3 |
| Median | 3204.9179529999997 |
| MAD | 13.057225000000017 |
| CV | 0.011965150344786415 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 3 |
| Median | 3401.7598239999998 |
| MAD | 2.999699999999848 |
| CV | 0.0022844427601716646 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 8198.470215000001 |
| MAD | 4.809456999999384 |
| CV | 0.002263035688683878 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 3 |
| Median | 3246.85055 |
| MAD | 12.94553199999973 |
| CV | 0.009167986547735632 |
| Candidate-failed samples | 0 |

Run: `perf-v2-core-fable-5-1-perf-nvme-r2`
Execution backend: `hyperv`

## Scope and provenance

Original profile results remain independent; the index adds a derived provisional total with missing scores imputed as zero. Each profile lists its run and, where recorded, its execution backend; different backends are not treated as identical execution conditions.

`valid` means the profile evidence is valid, not that all cases passed or release eligibility is established. `candidate-failed` is not an infrastructure error. `diagnostic` is diagnostic-only; `noneligible` cannot enter rankings. Hyper-V performance is not release- or ranking-eligible; protocol-v2 Hyper-V runs on the fixed VHDX/memory devices form their own `hyperv-fixed-vhdx` pool, counted in the provisional total and not comparable with KVM/raw-NVMe scores.

Security-review and maintainability scores are not assessed. Reference is a separate calibration baseline, not a model-generation entry.
<!-- core-results:end -->
