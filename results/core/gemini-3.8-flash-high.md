# Gemini 3.8 Flash — Core — High

[中文](gemini-3.8-flash-high.zh-CN.md)

This page collects independent Core Eval profile results. Each profile lists its status and run provenance; unmeasured dimensions remain unmeasured.

## Run

| Field | Value |
|---|---|
| Model | `gemini-3.8-flash` |
| Effort | High |
| Variant | Core |
| Agent harness | Pi |
| Generation runs | 1 (`n=1`) |
| SDK/POSIX evaluation run | `eval-core-gemini-3.8-flash` |
| Initial SDK/POSIX evaluation date | 2026-09-05 (UTC) |
| SDK/POSIX results updated | 2026-09-06 (UTC) |

The effective effort is High, confirmed by the operator for the proxy override. The Pi session's local thinking-level setting does not represent the provider-side effort.

## Submission size

Only candidate-owned source under `bench/` is counted. The fixed SDK and FUSE adapter, dependencies, documentation, configuration, lockfiles, and generated artifacts are excluded.

| Component | Files containing this component | Physical lines | Code lines |
|---|---:|---:|---:|
| Implementation (Rust) | 11 | 5,244 | 4,417 |
| Agent-written tests (Rust) | 0 | 0 | 0 |
| Qualification scripts | 0 | 0 | 0 |

Physical lines include blanks and comments. Code lines count each line holding at least one non-comment token: blank lines, comments, and documentation comments (Rust `///` and `//!`, Python docstrings) are excluded, while Rust attributes such as `#[derive(...)]` count as code. Corrected on 2026-09-27: earlier versions of this page counted documentation comments as code and Rust attributes as comments; file and physical-line counts are unchanged. The archived `bench/` tree has 11 distinct Rust files and no dedicated tests, inline test modules, or scripts. This does not imply that the agent ran no tests during development. These are source-size measurements, not test-coverage or code-quality scores.

<!-- source-tree:begin -->
### Directory structure

Every file under `bench/`, excluding build output. Numbers are code lines counted as above; "test" is the part of them in test code.

```text
bench/
├── src/
│   ├── btree.rs  (318 code)
│   ├── btree_mgr.rs  (334 code)
│   ├── directory.rs  (150 code)
│   ├── extent.rs  (458 code)
│   ├── format.rs  (477 code)
│   ├── fs.rs  (1,372 code)
│   ├── main.rs  (85 code)
│   ├── mkfs.rs  (283 code)
│   ├── storage.rs  (427 code)
│   ├── superblock.rs  (268 code)
│   └── xattr.rs  (245 code)
└── Cargo.toml
```
<!-- source-tree:end -->

## Generation time and activity

| Metric | Value |
|---|---|
| Goal start | 2026-09-05 06:40:31.117 UTC |
| Goal completion | 2026-09-05 10:06:00.977 UTC |
| Goal active time | **3 h 5 min 17 s** |
| Elapsed time, including gaps between active periods | 3 h 25 min 30 s |
| Final goal status | Complete (harness-recorded) |
| Proxy API requests / HTTP responses | 652 / 652 |
| Assistant response records with nonzero token usage | 650 |
| Assistant error records | 8 |
| Agent tool calls | 642 |

Active time is the final cumulative Pi goal counter, 11,116.962 seconds. The three goal identities belong to a resumed generation and inherit earlier counters; their snapshots are not summed. It is harness-accounted time, not model inference time. A completed goal does not imply that all evaluation tests passed.

Request counts come from the archived proxy log; assistant and tool-call counts come from the Pi session. The proxy records 650 HTTP 200 responses and two HTTP 402 responses; the session contains 652 assistant records. Of eight assistant error records, six still contain reported usage and are included in the token totals. Errors and zero-usage responses are not equivalent categories.

### Tool usage

| Tool | Requests | Recorded failures | Unknown outcomes |
|---|---:|---:|---:|
| `bash` | 190 | 34 | 0 |
| `edit` | 57 | 5 | 0 |
| `goal_complete` | 1 | 0 | 0 |
| `read` | 383 | 5 | 0 |
| `write` | 11 | 0 | 0 |
| **Total** | **642** | **44** | **0** |

Each archived assistant `toolCall` counts as one request under its recorded tool name, including retries. Results are matched by `toolCallId` and checked for the same tool name; only `toolResult.isError: true` counts as a recorded failure. Missing or unparseable results are unknown, not assumed successful or failed. These are tool-level outcomes, not model API errors or evaluation test results.

## Token usage and API cost

| Metric | Value |
|---|---:|
| Input tokens, excluding cache (`input`) | 16,383,967 |
| Cache-read input tokens (`cache_read`) | 244,400,079 |
| Cache-write input tokens (`cache_write`) | 0 |
| Total input tokens, including cache | 260,784,046 |
| Output tokens, including thinking (`output`) | 267,908 |
| Thinking / reasoning tokens (retained-text heuristic) | ≈4,000 |
| **Total tokens** | **261,051,954** |
| Cache hit rate (input tokens) | 93.72% |
| Estimated API cost (USD) | **$31.62** |

Token counts are summed from all Pi assistant-message usage records, including error records that contain usage, and match both the archived proxy totals and the final cumulative goal token counter. No separate reasoning-token field is present; this is not a claim of zero reasoning. Reasoning is not added again to the recorded output or total. Input totals include repeated context across requests, not unique context tokens.

The retained-text estimate covers 16,108 characters in five thinking blocks from five of the 650 usage-bearing responses. Applying [Google's rough four-characters-per-token guidance](https://ai.google.dev/gemini-api/docs/tokens) gives `16,108 / 4 = 4,027`, reported as **approximately 4,000 tokens** rather than a precise tokenizer count. This is a local character-based heuristic, not a run of Gemini's tokenizer or a provider-billed count. [Google distinguishes thought summaries from internal reasoning](https://ai.google.dev/gemini-api/docs/thinking#thought-summaries); retained summaries cannot reconstruct complete internal thought usage. The estimate is not extrapolated to other responses or added to output, total tokens, or cost. No retained text was sent to an external token-counting API.

Cost uses the non-batch model-catalog rates for `google/gemini-3.8-flash` in [OpenRouter's Models API](https://openrouter.ai/api/v1/models), corroborated by the standard Google AI Studio endpoint in [Gemini endpoint pricing](https://openrouter.ai/api/v1/models/google/gemini-3.8-flash/endpoints), retrieved on 2026-09-06. Flex and priority endpoints are not used as the pricing basis. This is a current-list-price estimate, not evidence of the run-date price, actual endpoint routing, or a provider invoice; session cost fields are not used.

| Token class | USD / million tokens | Estimated cost (USD) |
|---|---:|---:|
| Non-cached input | $0.75 | $12.287975 |
| Output, including reasoning already in output | $3.75 | $1.004655 |
| Cache read | $0.075 | $18.330006 |

[OpenRouter's Google caching rules](https://openrouter.ai/docs/guides/best-practices/prompt-caching#google-gemini) distinguish implicit caching, which has no write/storage fee, from explicit cache storage. The estimate uses reported input/output/cache reads and assumes no separately billable explicit storage; the archive reports zero cache writes but does not independently establish the storage billing mode. The catalog's cache-write/storage rate is about $0.0416667 per million tokens for the five-minute storage charge. As a sensitivity calculation, charging that once for every non-cached input token would add about $0.6826653, yielding about $32.31. That is not a recovered cache-write count or a measured charge.

The catalog lists no long-context override for this model. The reported-usage total, excluding any unrecorded explicit-storage charges, is $31.622636175, rounded once to **$31.62**. The separate catalog reasoning rate is not added to output a second time. Unreported usage on failed requests remains unknown and is excluded.

Generation time, source size, tokens, activity, and cost are descriptive metadata, not correctness scores.

## Results

<!-- core-results:begin -->

### POSIX correctness

| Profile | Classification | Pass | Expected fail | Fail | Unexpected pass | Timeout | Skip N/A | Skip impl | Pass rate | Category macro average |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `spec-tests` | `valid` | 44 | 0 | 0 | 0 | 0 | 7 | 0 | 100.00% | 100.00% |
| `pjdfstest-core` | `diagnostic` | 169 | 0 | 0 | 0 | 0 | 48 | 0 | 100.00% | 100.00% |
| `xfstests-core` | `valid` | 174 | 1 | 4 | 0 | 15 | 473 | 0 | 90.21% | 84.61% |

`spec-tests` run: `eval-core-gemini-3.8-flash`

`pjdfstest-core` run: `case-correction-core-gemini-3.8-flash-pjdfstest-core`

`xfstests-core` run: `eval-core-gemini-3.8-flash`

### Robustness

Profile: `robustness`

Outcome: `candidate-failed`; classification: `valid`.

| Metric | Value |
|---|---:|
| Total | 47 |
| Pass | 43 |
| Fail | 4 |
| Timeout | 0 |
| Pass rate | 91.49% |
| Category macro average | 89.58% |

Timeouts count as failures; N/A is excluded; the macro average weights categories equally.

Run: `eval-submission-robustness-v2-core-gemini-3-8-flash-r1`

### On-disk format conformance

Profile: `canonical-format`

Outcome: `candidate-failed`; classification: `diagnostic`.

| Metric | Value |
|---|---:|
| Total | 479 |
| Pass | 419 |
| Fail | 60 |
| Timeout | 0 |
| Pass rate | 87.47% |
| Category macro average | 71.79% |
| Conformance score (0–100) | 77.66 |
| Full conformance | no |

Run: `core-canonical-gemini-oracle3-r6`
Execution backend: `hyperv`

### Crash consistency

Profile: `crash-core`

Outcome: `candidate-failed`; classification: `diagnostic`.

| Metric | Value |
|---|---:|
| Total | 216 |
| Pass | 146 |
| Fail | 70 |
| Timeout | 0 |
| Pass rate | 67.59% |
| Category macro average | 67.59% |

The table above summarizes whether cases meet all requirements. The table below scores each verified capability for robustness R.

| Check | Score / 100 | Requirements met | Requirements not met | Not verified (scores 0) |
|---|---:|---:|---:|---:|
| Disk structure | 67.59 | 146 | 44 | 26 |
| Data correctness | 87.96 | 190 | 0 | 26 |
| Unmount and reopen | 67.59 | 146 | 70 | 0 |

Crash score: **75.74**. Weights: disk structure 40%, data correctness 40%, unmount and reopen 20%.

Permitted safe rejections: 23. Two rounds tested: 123. Reopen tests not run after failure: 70. First round not tested: 0.

Each round includes mount, checks, and unmount. Safe rejection meets requirements but does not prove data recovery. Reopen not tested after an earlier failure does not count as a pass. See [scoring](../../docs/scoring.md) for the meaning of each check.

Run: `case-correction-core-gemini-3.8-flash-crash-core`

### Real-world applications

Profile: `real-world`

Outcome: `candidate-failed`; classification: `diagnostic`.

| Metric | Value |
|---|---:|
| Total | 4 |
| Pass | 2 |
| Fail | 1 |
| Timeout | 1 |
| Pass rate | 50.00% |
| Category macro average | 50.00% |

Run: `core-realworld-gemini-oracle3-r6`
Execution backend: `hyperv`

### CPU performance

Profile: `perf-cpu`

Outcome: `pass`; classification: `diagnostic`.

Performance score: **47.10** (comparison pool `hyperv-fixed-vhdx`).

| Family | Cache | Primary metric | Samples | Median | MAD | CV | Candidate-failed samples |
|---|---|---|---:|---:|---:|---:|---:|
| buffered-synchronized-writes | cold | operations_per_second | 3 | 2,635.4 | 2.3 | 0.3% | 0 |
| buffered-synchronized-writes | warm | operations_per_second | 3 | 2,623.5 | 4.9 | 0.5% | 0 |
| bulk-sequential-io | cold | MiB/s | 4 | 34.8 | 0.3 | 0.9% | 0 |
| bulk-sequential-io | warm | MiB/s | 4 | 34.9 | 0.2 | 0.8% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 1,275.0 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 7,964.6 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 875.3 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 1,401.1 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 905.6 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 2 | 1,080.2 | 20.2 | 1.9% | 0 |
| metadata-concurrency | cold | operations_per_second | 4 | 64.7 | 1.0 | 2.1% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 1,274.2 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 8,185.5 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 871.0 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 1,396.6 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 906.3 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 1,080.0 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 4 | 63.1 | 0.8 | 1.9% | 0 |
| small-random-io | cold | iops | 3 | 4,574.7 | 3.0 | 0.1% | 0 |
| small-random-io | cold | iops | 4 | 6,187.4 | 14.6 | 0.3% | 0 |
| small-random-io | cold | iops | 3 | 2,846.8 | 15.2 | 1.8% | 0 |
| small-random-io | warm | iops | 3 | 4,573.3 | 0.1 | 0.2% | 0 |
| small-random-io | warm | iops | 4 | 6,174.6 | 14.0 | 0.2% | 0 |
| small-random-io | warm | iops | 3 | 2,941.4 | 4.7 | 0.3% | 0 |

Run: `perf-v2-core-gemini-3-8-flash-perf-cpu-r1`
Execution backend: `hyperv`

### NVMe performance

Profile: `perf-nvme`

Outcome: `pass`; classification: `diagnostic`.

Performance score: **46.02** (comparison pool `hyperv-fixed-vhdx`).

| Family | Cache | Primary metric | Samples | Median | MAD | CV | Candidate-failed samples |
|---|---|---|---:|---:|---:|---:|---:|
| buffered-synchronized-writes | cold | operations_per_second | 2 | 1,804.8 | 5.1 | 0.3% | 0 |
| buffered-synchronized-writes | warm | operations_per_second | 2 | 1,789.3 | 42.1 | 2.4% | 0 |
| bulk-sequential-io | cold | MiB/s | 2 | 15.7 | 0.1 | 0.8% | 0 |
| bulk-sequential-io | warm | MiB/s | 2 | 15.7 | 0.0 | 0.2% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 565.0 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 6,454.8 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 382.3 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 563.9 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 354.3 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 354.1 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 3 | 28.9 | 0.1 | 0.8% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 565.4 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 260,338.7 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 377.5 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 565.1 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 356.5 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 358.6 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 3 | 28.4 | 0.0 | 1.4% | 0 |
| small-random-io | cold | iops | 2 | 2,653.1 | 25.8 | 1.0% | 0 |
| small-random-io | cold | iops | 2 | 3,309.0 | 7.9 | 0.2% | 0 |
| small-random-io | cold | iops | 2 | 1,609.5 | 8.0 | 0.5% | 0 |
| small-random-io | warm | iops | 2 | 2,666.8 | 9.4 | 0.4% | 0 |
| small-random-io | warm | iops | 2 | 3,293.4 | 1.2 | 0.0% | 0 |
| small-random-io | warm | iops | 2 | 1,607.3 | 10.4 | 0.6% | 0 |

Run: `perf-v2-core-gemini-3-8-flash-perf-nvme-r5`
Execution backend: `hyperv`

### Agent code review

S: **17.86** / 100; status: `complete`; purpose: `diagnostic`.

| Critical | High | Medium | Low |
|---:|---:|---:|---:|
| 8 | 1 | 23 | 10 |

Counts include confirmed, deduplicated root causes; S contributes 20% of the provisional total.

Scoring rule: `agent-review-score-v2`.

## Scope and provenance

Original profile results remain independent; the index adds a derived provisional total with missing scores imputed as zero. Each profile lists its run and, where recorded, its execution backend; different backends are not treated as identical execution conditions.

`valid` means the profile evidence is valid, not that all cases passed or release eligibility is established. `candidate-failed` is not an infrastructure error. `diagnostic` is diagnostic-only; `noneligible` cannot enter rankings. Hyper-V performance is not release- or ranking-eligible; protocol-v2 Hyper-V runs on the fixed VHDX/memory devices form their own `hyperv-fixed-vhdx` pool, counted in the provisional total and not comparable with KVM/raw-NVMe scores.

Maintainability (`maint-v3-policy`, 5% of the overall score): **68.11** / 100; components are in the README. Reference is a separate calibration baseline, not a model-generation entry.
<!-- core-results:end -->
