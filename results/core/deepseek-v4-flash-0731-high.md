# DeepSeek V4 Flash 0731 — Core — High

[中文](deepseek-v4-flash-0731-high.zh-CN.md)

This page collects independent Core Eval profile results. Each profile lists its status and run provenance; unmeasured dimensions remain unmeasured.

## Run

| Field | Value |
|---|---|
| Model | `deepseek-v4-flash-0731` |
| Effort | High |
| Variant | Core |
| Agent harness | Pi |
| Generation runs | 1 (`n=1`) |
| SDK/POSIX evaluation run | `eval-core-deepseek-v4-flash-0731` |
| Initial SDK/POSIX evaluation date | 2026-09-05 (UTC) |
| SDK/POSIX results updated | 2026-09-06 (UTC) |

The effective effort is High, confirmed by the operator for the proxy override. The Pi session's local thinking-level setting does not represent the provider-side effort.

## Submission size

Only candidate-owned source under `bench/` is counted. The fixed SDK and FUSE adapter, dependencies, documentation, configuration, lockfiles, and generated artifacts are excluded.

| Component | Files containing this component | Physical lines | Code lines |
|---|---:|---:|---:|
| Implementation (Rust) | 12 | 7,144 | 6,226 |
| Agent-written tests and test support (Rust) | 2 | 643 | 572 |
| Qualification scripts | 0 | 0 | 0 |

Physical lines include blanks and comments. Code lines count each line holding at least one non-comment token: blank lines, comments, and documentation comments (Rust `///` and `//!`, Python docstrings) are excluded, while Rust attributes such as `#[derive(...)]` count as code. Corrected on 2026-09-27: earlier versions of this page counted documentation comments as code and Rust attributes as comments; file and physical-line counts are unchanged. Tests comprise one dedicated integration-test file and an explicitly documented test helper in an implementation file. File counts overlap in that one file; physical and code lines are partitioned without overlap. There are 13 distinct Rust files. These are source-size measurements, not test-coverage or code-quality scores.

<!-- source-tree:begin -->
### Directory structure

Every file under `bench/`, excluding build output. Numbers are code lines counted as above; "test" is the part of them in test code.

```text
bench/
├── src/
│   ├── fs/
│   │   ├── btree.rs  (744 code)
│   │   ├── checker.rs  (668 code, 7 test)
│   │   ├── format.rs  (780 code)
│   │   ├── format_image.rs  (106 code)
│   │   ├── inode.rs  (514 code)
│   │   ├── layout.rs  (55 code)
│   │   ├── mod.rs  (269 code)
│   │   ├── ops.rs  (2,337 code)
│   │   └── state.rs  (621 code)
│   ├── lib.rs  (1 code)
│   ├── main.rs  (47 code)
│   └── mkfs.rs  (91 code)
├── tests/
│   └── core.rs  (565 code, 565 test)
└── Cargo.toml
```
<!-- source-tree:end -->

## Generation time and activity

| Metric | Value |
|---|---|
| Goal start | 2026-09-01 16:38:42.243 UTC |
| Last goal-state update | 2026-09-02 04:58:31.350 UTC |
| Last recorded goal active time | **12 h 19 min 49 s** |
| Elapsed time to last goal-state update | 12 h 19 min 49 s |
| Last recorded goal status | Blocked; completion not harness-recorded |
| Last session record | 2026-09-02 05:01:41.760 UTC |
| Proxy API requests / HTTP responses | 476 / 476 |
| Assistant response records with nonzero token usage | 476 |
| Assistant error records | 2 |
| Agent tool calls | 482 |

The single goal's last cumulative active-time counter is 44,389.107 seconds. Three further assistant records report 835,162 tokens after that goal update; their usage is included below, but their time is not recovered by the stopped goal counter. The archived session therefore does not establish a final generation-completion time. Active time is harness-accounted time, not model inference time. The separately completed Eval evaluates the archived candidate regardless of the recorded goal status.

Request counts come from the archived proxy log; assistant and tool-call counts come from the Pi session. All 476 recorded HTTP responses have status 200. The session has 478 assistant records, including two errors without usage. These counts are not interchangeable measures or a guarantee that unreported failed requests incurred no charges.

### Tool usage

| Tool | Requests | Recorded failures | Unknown outcomes |
|---|---:|---:|---:|
| `bash` | 459 | 51 | 1 |
| `edit` | 1 | 0 | 0 |
| `h` | 1 | 1 | 0 |
| `read` | 15 | 0 | 0 |
| `write` | 6 | 1 | 0 |
| **Total** | **482** | **53** | **1** |

Each archived assistant `toolCall` counts as one request under its recorded tool name, including retries. Results are matched by `toolCallId` and checked for the same tool name; only `toolResult.isError: true` counts as a recorded failure. Missing or unparseable results are unknown, not assumed successful or failed. These are tool-level outcomes, not model API errors or evaluation test results.

One `bash` request has no matching archived result, so 53 is the observed failure count, not a complete failure total. The requested tool `h` is retained as recorded.

## Token usage and API cost

| Metric | Value |
|---|---:|
| Input tokens, excluding cache (`input`) | 11,723,837 |
| Cache-read input tokens (`cache_read`) | 150,212,352 |
| Cache-write input tokens (`cache_write`) | 0 |
| Total input tokens, including cache | 161,936,189 |
| Output tokens, including thinking (`output`) | 518,078 |
| Thinking / reasoning tokens | 257,581 |
| **Total tokens** | **162,454,267** |
| Cache hit rate (input tokens) | 92.76% |
| Estimated API cost (USD) | **$3.26** |

Token counts are summed from Pi assistant-message usage records and match the archived proxy totals. The last goal token counter is only 161,619,105; adding the 835,162 tokens reported afterward reconciles it with the session total. Reasoning usage is explicitly recorded on all 476 responses with nonzero usage; it is a component of output and is not added again. Input totals include repeated context across requests, not unique context tokens.

Cost uses the non-batch model-catalog rates for `deepseek/deepseek-v4-flash-0731` in [OpenRouter's Models API](https://openrouter.ai/api/v1/models), also listed by the Relace endpoint in [DeepSeek endpoint pricing](https://openrouter.ai/api/v1/models/deepseek/deepseek-v4-flash-0731/endpoints), retrieved on 2026-09-06. This is a current-list-price estimate, not evidence of the run-date price, actual endpoint routing, or a provider invoice; session cost fields are not used.

| Token class | USD / million tokens | Estimated cost (USD) |
|---|---:|---:|
| Non-cached input / cache misses | $0.065 | $0.762049 |
| Output | $0.18 | $0.093254 |
| Cache read | $0.016 | $2.403398 |

[OpenRouter's DeepSeek caching rules](https://openrouter.ai/docs/guides/best-practices/prompt-caching#deepseek) charge cache writes at ordinary input pricing. Non-cached input is therefore charged once, with no additional write premium. The catalog lists no long-context override for this model. The unrounded total is $3.258701077, rounded once to **$3.26**. Unreported usage on failed requests remains unknown and is excluded.

Generation time, source size, tokens, activity, and cost are descriptive metadata, not correctness scores.

## Results

<!-- core-results:begin -->

### POSIX correctness

| Profile | Classification | Pass | Expected fail | Fail | Unexpected pass | Timeout | Skip N/A | Skip impl | Pass rate | Category macro average |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `spec-tests` | `valid` | 44 | 0 | 0 | 0 | 0 | 7 | 0 | 100.00% | 100.00% |
| `pjdfstest-core` | `diagnostic` | 169 | 0 | 0 | 0 | 0 | 48 | 0 | 100.00% | 100.00% |
| `xfstests-core` | `valid` | 166 | 1 | 13 | 0 | 14 | 473 | 0 | 86.08% | 84.02% |

`spec-tests` run: `eval-core-deepseek-v4-flash-0731`

`pjdfstest-core` run: `case-correction-core-deepseek-v4-flash-0731-pjdfstest-core`

`xfstests-core` run: `eval-core-deepseek-v4-flash-0731`

### Robustness

Profile: `robustness`

Outcome: `candidate-failed`; classification: `valid`.

| Metric | Value |
|---|---:|
| Total | 47 |
| Pass | 35 |
| Fail | 9 |
| Timeout | 3 |
| Pass rate | 74.47% |
| Category macro average | 69.58% |

Timeouts count as failures; N/A is excluded; the macro average weights categories equally.

Run: `eval-submission-robustness-v2-core-deepseek-v4-flash-0731-r2`

### On-disk format conformance

Profile: `canonical-format`

Outcome: `candidate-failed`; classification: `diagnostic`.

| Metric | Value |
|---|---:|
| Total | 479 |
| Pass | 388 |
| Fail | 91 |
| Timeout | 0 |
| Pass rate | 81.00% |
| Category macro average | 73.56% |
| Conformance score (0–100) | 74.86 |
| Full conformance | no |

Run: `core-canonical-deepseek-oracle3-r5`
Execution backend: `hyperv`

### Crash consistency

Profile: `crash-core`

Outcome: `candidate-failed`; classification: `diagnostic`.

| Metric | Value |
|---|---:|
| Total | 216 |
| Pass | 206 |
| Fail | 10 |
| Timeout | 0 |
| Pass rate | 95.37% |
| Category macro average | 95.37% |

Run: `case-correction-core-deepseek-v4-flash-0731-crash-core`

### Real-world applications

Profile: `real-world`

Outcome: `candidate-failed`; classification: `diagnostic`.

| Metric | Value |
|---|---:|
| Total | 4 |
| Pass | 2 |
| Fail | 2 |
| Timeout | 0 |
| Pass rate | 50.00% |
| Category macro average | 66.67% |

Run: `core-realworld-deepseek-oracle3-r6`
Execution backend: `hyperv`

### CPU performance

Profile: `perf-cpu`

Outcome: `pass`; classification: `diagnostic`.


#### buffered-synchronized-writes / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 2 |
| Median | 1990.7 |
| MAD | 6.7999999999999545 |
| CV | 0.0034158838599487387 |
| Candidate-failed samples | 0 |

#### buffered-synchronized-writes / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 2 |
| Median | 2007.25 |
| MAD | 1.349999999999909 |
| CV | 0.0006725619628844982 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / cold

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 3 |
| Median | 36473070.0 |
| MAD | 144282.0 |
| CV | 0.006044678690869043 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / warm

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 3 |
| Median | 36386452.0 |
| MAD | 18139.0 |
| CV | 0.008296502671628205 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 1062.3583542146798 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 5302.304723440579 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 504.19149211888777 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 1242.2802279530006 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 546.4200561126744 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 1389.9621498520874 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 36.27033446732453 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 1052.2838883079573 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 6679.588135761936 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 505.6460925970641 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 1241.753410382451 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 544.1013104423156 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 1388.0538903712597 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 38.32138183959831 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 2 |
| Median | 3468.8031195 |
| MAD | 51.34486549999974 |
| CV | 0.014801896715141531 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 2 |
| Median | 6671.214491999999 |
| MAD | 110.4291790000002 |
| CV | 0.016553084769261265 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 2 |
| Median | 6973.352375 |
| MAD | 10.948934999999892 |
| CV | 0.0015701106743512142 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 2 |
| Median | 3464.6035395 |
| MAD | 50.24497549999978 |
| CV | 0.014502373771531438 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 2 |
| Median | 4971.448727 |
| MAD | 1167.609555 |
| CV | 0.23486303874737718 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 2 |
| Median | 6925.356234499999 |
| MAD | 10.347955499999898 |
| CV | 0.0014942127378877003 |
| Candidate-failed samples | 0 |

Run: `perf-v2-core-deepseek-v4-flash-0731-perf-cpu-r6`
Execution backend: `hyperv`

### NVMe performance

Profile: `perf-nvme`

Outcome: `pass`; classification: `diagnostic`.


#### buffered-synchronized-writes / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 2 |
| Median | 1227.75 |
| MAD | 8.150000000000091 |
| CV | 0.006638159234371893 |
| Candidate-failed samples | 0 |

#### buffered-synchronized-writes / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 2 |
| Median | 1221.25 |
| MAD | 3.75 |
| CV | 0.0030706243602865915 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / cold

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 2 |
| Median | 16371986.5 |
| MAD | 177759.5 |
| CV | 0.010857540103639837 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / warm

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 2 |
| Median | 16373350.5 |
| MAD | 59645.5 |
| CV | 0.0036428402360286615 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 661.4806316073033 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 5323.895168323387 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 336.4771103816801 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 752.8836716763333 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 315.3020855849321 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 556.5241269770506 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 21.232220975058862 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 664.7329421854189 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 6770.160514499312 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 332.88915850800953 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 750.2848144038722 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 305.5980058950596 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 557.4249210221611 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 22.329008405862393 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 2 |
| Median | 1922.907709 |
| MAD | 15.09848999999997 |
| CV | 0.007851905699547003 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 2 |
| Median | 2393.9783575 |
| MAD | 60.49055950000002 |
| CV | 0.025267797142146893 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 1 |
| Median | 3061.7765489999997 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 2 |
| Median | 1901.5598439999999 |
| MAD | 41.14588500000002 |
| CV | 0.021637964816005037 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 2 |
| Median | 1847.902861 |
| MAD | 6.455395000000067 |
| CV | 0.003493362739049337 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 1 |
| Median | 2974.702903 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

Run: `perf-v2-core-deepseek-v4-flash-0731-perf-nvme-r6`
Execution backend: `hyperv`

## Scope and provenance

Original profile results remain independent; the index adds a derived provisional total with missing scores imputed as zero. Each profile lists its run and, where recorded, its execution backend; different backends are not treated as identical execution conditions.

`valid` means the profile evidence is valid, not that all cases passed or release eligibility is established. `candidate-failed` is not an infrastructure error. `diagnostic` is diagnostic-only; `noneligible` cannot enter rankings. Hyper-V performance is not release- or ranking-eligible; protocol-v2 Hyper-V runs on the fixed VHDX/memory devices form their own `hyperv-fixed-vhdx` pool, counted in the provisional total and not comparable with KVM/raw-NVMe scores.

Security-review and maintainability scores are not assessed. Reference is a separate calibration baseline, not a model-generation entry.
<!-- core-results:end -->
