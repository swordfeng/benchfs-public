# Qwen 3.8 Flash Next — Core — High

[中文](qwen-3.8-flash-next-high.zh-CN.md)

This page collects independent Core Eval profile results. Each profile lists its status and run provenance; unmeasured dimensions remain unmeasured.

## Run

| Field | Value |
|---|---|
| Model | `qwen-3.8-flash-next` |
| Effort | High |
| Variant | Core |
| Agent harness | Pi |
| Generation runs | 1 (`n=1`) |
| SDK/POSIX evaluation run | `eval-core-qwen-3.8-flash-next` |
| SDK/POSIX evaluation date | 2026-09-06 (UTC) |

The effective effort is High, confirmed by the operator for the proxy override. The Pi session's local thinking-level setting does not represent the provider-side effort.

## Submission size

Only candidate-owned source under `bench/` is counted. The fixed SDK and FUSE adapter, dependencies, documentation, configuration, lockfiles, and generated artifacts are excluded.

| Component | Files containing this component | Physical lines | Code lines |
|---|---:|---:|---:|
| Implementation (Rust) | 13 | 8,740 | 7,530 |
| Agent-written tests and test support (Rust) | 3 | 829 | 763 |
| Qualification scripts | 0 | 0 | 0 |

Physical lines include blanks and comments. Code lines count each line holding at least one non-comment token: blank lines, comments, and documentation comments (Rust `///` and `//!`, Python docstrings) are excluded, while Rust attributes such as `#[derive(...)]` count as code. Corrected on 2026-09-27: earlier versions of this page counted documentation comments as code and Rust attributes as comments; file and physical-line counts are unchanged. Tests comprise one dedicated integration-test file and inline test modules in two implementation files. Those two files overlap in the component file counts; physical and code lines are partitioned without overlap. There are 14 distinct Rust files. These are source-size measurements, not test-coverage or code-quality scores.

<!-- source-tree:begin -->
### Directory structure

Every file under `bench/`, excluding build output. Numbers are code lines counted as above; "test" is the part of them in test code.

```text
bench/
├── src/
│   ├── boot.rs  (516 code)
│   ├── check.rs  (687 code)
│   ├── device.rs  (50 code)
│   ├── engine.rs  (2,392 code)
│   ├── format.rs  (1,121 code)
│   ├── formatter.rs  (214 code)
│   ├── fs.rs  (1,563 code)
│   ├── geom.rs  (402 code, 36 test)
│   ├── lib.rs  (9 code)
│   ├── main.rs  (106 code)
│   ├── mkfs.rs  (95 code)
│   ├── sync.rs  (250 code)
│   └── tree.rs  (205 code, 44 test)
├── tests/
│   └── model.rs  (683 code, 683 test)
└── Cargo.toml
```
<!-- source-tree:end -->

## Generation time and activity

| Metric | Value |
|---|---|
| Goal start | 2026-09-04 16:46:44.784 UTC |
| Last goal-state update / last session record | 2026-09-05 09:13:21.102 UTC |
| Last recorded goal active time | **12 h 0 min 54 s** |
| Elapsed time to last goal-state update, including gaps | 16 h 26 min 36 s |
| Last recorded goal status | Active; completion not harness-recorded |
| Archived proxy API requests / HTTP responses (partial) | 37 / 36 |
| Assistant response records with nonzero token usage | 519 |
| Assistant error records | 41 |
| Assistant aborted records | 3 |
| Agent tool calls | 522 |

The last cumulative Pi goal active-time counter is 43,253.875 seconds. Seven goal identities belong to a resumed generation; their snapshots are not summed. The archive ends with an active goal and does not establish a final generation-completion time. Active time is harness-accounted time, not model inference time. The separately completed Eval evaluates the archived candidate regardless of the recorded goal status.

The proxy archive covers only a late segment: its usage timestamps span 2026-09-05 08:45:19.305732–09:05:08.231703 UTC. It records 37 requests, 36 HTTP 200 responses, and only 33 usage records. Those are partial counts, not the complete generation's request volume. The Pi session contains 563 assistant records, including 519 with nonzero usage. Two malformed session lines are tool-result records, not assistant-usage records; they are excluded from parsing. No malformed assistant-usage record was found. Tool calls are counted from assistant records, not tool-result lines.

### Tool usage

| Tool | Requests | Recorded failures | Unknown outcomes |
|---|---:|---:|---:|
| `bash` | 481 | 24 | 2 |
| `edit` | 7 | 1 | 0 |
| `read` | 13 | 0 | 0 |
| `write` | 21 | 1 | 0 |
| **Total** | **522** | **26** | **2** |

Each archived assistant `toolCall` counts as one request under its recorded tool name, including retries. Results are matched by `toolCallId` and checked for the same tool name; only `toolResult.isError: true` counts as a recorded failure. Missing or unparseable results are unknown, not assumed successful or failed. These are tool-level outcomes, not model API errors or evaluation test results.

After excluding the two malformed tool-result lines, 520 results match the 522 requests. Two `bash` outcomes remain unknown; 26 is the observed failure count, not a complete failure total.

## Token usage and API cost

| Metric | Value |
|---|---:|
| Input tokens, excluding cache (`input`) | 2,582,958 |
| Cache-read input tokens (`cache_read`) | 151,479,808 |
| Cache-write input tokens (`cache_write`) | 0 |
| Total input tokens, including cache | 154,062,766 |
| Output tokens, including thinking (`output`) | 734,568 |
| Thinking / reasoning tokens | 480,599 |
| **Total tokens** | **154,797,334** |
| Cache hit rate (input tokens) | 98.32% |
| Estimated API cost (USD) | **$3.29** |

Token counts are summed from Pi assistant-message usage records, not the partial proxy summary of 14,748,770 tokens. A recorded goal-token reset prevents using the last counter alone: 89,446,529 tokens before the reset plus the final 65,350,805 counter equals 154,797,334, matching the session sum. Intermediate cumulative snapshots are not added. Reasoning usage is explicitly recorded on all 519 responses with nonzero usage; it is a component of output and is not added again. Input totals include repeated context across requests, not unique context tokens.

Cost uses the non-batch model-catalog rates for `qwen/qwen3.8-flash` in [OpenRouter's Models API](https://openrouter.ai/api/v1/models), corroborated by [Qwen endpoint pricing](https://openrouter.ai/api/v1/models/qwen/qwen3.8-flash/endpoints), retrieved on 2026-09-06. The catalog identifies this model's Hugging Face repository as `Qwen/Qwen3.8-Flash-Next`, connecting its shorter catalog name to the submission name. This is a current-list-price estimate, not evidence of the run-date price, actual endpoint routing, or a provider invoice; session cost fields are not used.

| Token class | USD / million tokens | Estimated cost (USD) |
|---|---:|---:|
| Input, assumed to populate the cache | $0.20 | $0.516592 |
| Output | $0.47 | $0.345247 |
| Cache read | $0.016 | $2.423677 |

The catalog and Alibaba endpoint list a $0.20 cache-write rate versus a $0.15 ordinary prompt rate. The session records zero separate writes but does not establish how many non-cached input tokens populated the cache. For this estimate, all 2,582,958 non-cached input tokens are conservatively priced at the listed write rate, replacing rather than adding to ordinary input charges. [OpenRouter's Alibaba caching documentation](https://openrouter.ai/docs/guides/best-practices/prompt-caching#alibaba-qwen) describes separate write pricing; model-specific catalog rates are used rather than its generic multiplier. This is a billing assumption, not a recovered cache-write count; the usage table is unchanged.

The catalog lists no long-context override for this model. The unrounded cache-write-assumption total is $3.285515488, rounded once to **$3.29**. Pricing non-cached input at the ordinary prompt rate instead yields $3.156367588, or $3.16. Unreported usage on failed or aborted requests remains unknown and is excluded.

Generation time, source size, tokens, activity, and cost are descriptive metadata, not correctness scores.

## Results

<!-- core-results:begin -->

### POSIX correctness

| Profile | Classification | Pass | Expected fail | Fail | Unexpected pass | Timeout | Skip N/A | Skip impl | Pass rate | Category macro average |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `spec-tests` | `valid` | 44 | 0 | 0 | 0 | 0 | 7 | 0 | 100.00% | 100.00% |
| `pjdfstest-core` | `diagnostic` | 169 | 0 | 0 | 0 | 0 | 48 | 0 | 100.00% | 100.00% |
| `xfstests-core` | `valid` | 167 | 1 | 24 | 0 | 2 | 473 | 0 | 86.60% | 81.52% |

`spec-tests` run: `eval-core-qwen-3.8-flash-next`

`pjdfstest-core` run: `case-correction-core-qwen-3.8-flash-next-pjdfstest-core`

`xfstests-core` run: `eval-core-qwen-3.8-flash-next`

### Robustness

Profile: `robustness`

Outcome: `candidate-failed`; classification: `valid`.

| Metric | Value |
|---|---:|
| Total | 47 |
| Pass | 32 |
| Fail | 15 |
| Timeout | 0 |
| Pass rate | 68.09% |
| Category macro average | 66.25% |

Timeouts count as failures; N/A is excluded; the macro average weights categories equally.

Run: `eval-submission-robustness-v2-core-qwen-3-8-flash-next-r1`

### On-disk format conformance

Profile: `canonical-format`

Outcome: `candidate-failed`; classification: `diagnostic`.

| Metric | Value |
|---|---:|
| Total | 479 |
| Pass | 368 |
| Fail | 111 |
| Timeout | 0 |
| Pass rate | 76.83% |
| Category macro average | 67.51% |
| Conformance score (0–100) | 75.33 |
| Full conformance | no |

Run: `core-canonical-qwen-oracle3-r5`
Execution backend: `hyperv`

### Crash consistency

Profile: `crash-core`

Outcome: `candidate-failed`; classification: `diagnostic`.

| Metric | Value |
|---|---:|
| Total | 216 |
| Pass | 150 |
| Fail | 66 |
| Timeout | 0 |
| Pass rate | 69.44% |
| Category macro average | 69.44% |

The table above summarizes whether cases meet all requirements. The table below scores each verified capability for robustness R.

| Check | Score / 100 | Requirements met | Requirements not met | Not verified (scores 0) |
|---|---:|---:|---:|---:|
| Disk structure | 69.44 | 150 | 5 | 61 |
| Data correctness | 71.76 | 155 | 1 | 60 |
| Unmount and reopen | 69.44 | 150 | 18 | 48 |

Crash score: **70.37**. Weights: disk structure 40%, data correctness 40%, unmount and reopen 20%.

Permitted safe rejections: 18. Two rounds tested: 132. Reopen tests not run after failure: 66. First round not tested: 48.

Each round includes mount, checks, and unmount. Safe rejection meets requirements but does not prove data recovery. Reopen not tested after an earlier failure does not count as a pass. See [scoring](../../docs/scoring.md) for the meaning of each check.

Run: `case-correction-core-qwen-3.8-flash-next-crash-core`

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
| Category macro average | 33.33% |

Run: `core-realworld-qwen-oracle3-r6`
Execution backend: `hyperv`

### CPU performance

Profile: `perf-cpu`

Outcome: `candidate-failed`; classification: `diagnostic`.

Performance score: **57.80** (comparison pool `hyperv-fixed-vhdx`). Assessed budget: 100.0%. The range allowed by missing inputs is 57.80–57.80, not a confidence interval.

Components (out of 100): File I/O 42.37; Metadata 31.96; RAM 100.00; Write efficiency 99.66.
Peak RAM: 82.5 MiB. Metadata: 11 cells have valid rates, 3 candidate failures, 0 unmeasured.

Write amplification = controller write bytes / application logical write bytes in the timed window; it is not SSD-internal amplification. This ratio does not apply to metadata operations. A dash is not zero. Candidate failures score 0; unmeasured cells retain their budget and provisionally contribute 0. * marks an unassessed share.

| Workload | Cache | Samples | Rate | MAD | CV | Candidate-failed samples | Write amplification | Speed score / 100 |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| Sequential I/O | cold | 5 | 244.9 MiB/s | 1.7 | 1.0% | 0 | 1.000× | 26.60 |
| Sequential I/O | warm | 5 | 244.0 MiB/s | 3.3 | 1.2% | 0 | 1.000× | 26.55 |
| Random I/O, single thread | cold | 3 | 4,580.5 IOPS | 20.3 | 0.5% | 0 | 1.000× | 29.58 |
| Random I/O, single thread | warm | 3 | 4,567.2 IOPS | 1.8 | 0.6% | 0 | 1.000× | 29.46 |
| Random I/O, multiple threads | cold | 5 | 6,368.1 IOPS | 4.5 | 0.2% | 0 | 1.000× | 23.87 |
| Random I/O, multiple threads | warm | 3 | 6,380.8 IOPS | 8.1 | 0.2% | 0 | 1.000× | 23.91 |
| Random I/O, shared regions | cold | 5 | 3,666.4 IOPS | 20.0 | 0.9% | 0 | 1.000× | 25.23 |
| Random I/O, shared regions | warm | 5 | 3,695.2 IOPS | 32.3 | 0.9% | 0 | 1.001× | 25.36 |
| Synchronized overwrite | cold | 3 | 2,623.1 ops/s | 3.5 | 0.4% | 0 | 1.031× | 78.58 |
| Synchronized overwrite | warm | 3 | 2,627.0 ops/s | 15.7 | 0.5% | 0 | 1.032× | 78.59 |
| Create, stat and unlink | cold | 0 | — | — | — | 1 | — | 0.00 |
| Create, stat and unlink | warm | 1 | 2,474.2 ops/s | 0.0 | 0.0% | 1 | — | 23.05 |
| Deep and large-directory lookup | cold | 3 | 7,261.3 ops/s | 4.3 | 0.2% | 0 | — | 14.93 |
| Deep and large-directory lookup | warm | 3 | 9,160.0 ops/s | 102.4 | 1.0% | 0 | — | 20.33 |
| Same-directory rename | cold | 3 | 1,945.8 ops/s | 0.3 | 0.1% | 0 | — | 29.97 |
| Same-directory rename | warm | 3 | 1,945.7 ops/s | 0.4 | 0.2% | 0 | — | 29.65 |
| Same-directory concurrent mutation | cold | 3 | 3,510.7 ops/s | 9.5 | 0.5% | 0 | — | 28.88 |
| Same-directory concurrent mutation | warm | 3 | 3,512.2 ops/s | 14.6 | 0.4% | 0 | — | 29.41 |
| Concurrent creation in different directories | cold | 3 | 2,881.7 ops/s | 7.1 | 0.2% | 0 | — | 31.10 |
| Concurrent creation in different directories | warm | 3 | 2,894.3 ops/s | 1.8 | 0.2% | 0 | — | 31.04 |
| Parallel fsync | cold | 3 | 969.4 ops/s | 9.1 | 1.1% | 0 | — | 100.00 |
| Parallel fsync | warm | 4 | 949.2 ops/s | 19.4 | 2.1% | 0 | — | 100.00 |
| Reclamation near full capacity | cold | 0 | — | — | — | 1 | — | 0.00 |
| Reclamation near full capacity | warm | 0 | — | — | — | 1 | — | 0.00 |

Run: `perf-v2-core-qwen-3-8-flash-next-perf-cpu-r1`
Execution backend: `hyperv`

### NVMe performance

Profile: `perf-nvme`

Outcome: `candidate-failed`; classification: `diagnostic`.

Performance score: **56.24** (comparison pool `hyperv-fixed-vhdx`). Assessed budget: 100.0%. The range allowed by missing inputs is 56.24–56.24, not a confidence interval.

Components (out of 100): File I/O 41.90; Metadata 24.43; RAM 100.00; Write efficiency 99.66.
Peak RAM: 84.1 MiB. Metadata: 10 cells have valid rates, 4 candidate failures, 0 unmeasured.

Write amplification = controller write bytes / application logical write bytes in the timed window; it is not SSD-internal amplification. This ratio does not apply to metadata operations. A dash is not zero. Candidate failures score 0; unmeasured cells retain their budget and provisionally contribute 0. * marks an unassessed share.

| Workload | Cache | Samples | Rate | MAD | CV | Candidate-failed samples | Write amplification | Speed score / 100 |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| Sequential I/O | cold | 5 | 178.2 MiB/s | 0.3 | 0.8% | 0 | 1.000× | 31.31 |
| Sequential I/O | warm | 5 | 177.3 MiB/s | 0.5 | 0.6% | 0 | 1.000× | 31.23 |
| Random I/O, single thread | cold | 3 | 2,638.2 IOPS | 17.9 | 0.6% | 0 | 1.000× | 28.49 |
| Random I/O, single thread | warm | 3 | 2,610.6 IOPS | 1.9 | 0.2% | 0 | 1.001× | 28.26 |
| Random I/O, multiple threads | cold | 5 | 3,196.1 IOPS | 8.0 | 0.3% | 0 | 1.000× | 20.95 |
| Random I/O, multiple threads | warm | 5 | 3,198.5 IOPS | 7.7 | 0.4% | 0 | 1.000× | 20.97 |
| Random I/O, shared regions | cold | 5 | 1,826.4 IOPS | 5.5 | 0.6% | 0 | 1.000× | 22.17 |
| Random I/O, shared regions | warm | 5 | 1,844.4 IOPS | 7.8 | 0.5% | 0 | 1.000× | 22.32 |
| Synchronized overwrite | cold | 3 | 1,753.9 ops/s | 6.1 | 0.6% | 0 | 1.031× | 75.12 |
| Synchronized overwrite | warm | 3 | 1,749.7 ops/s | 8.9 | 0.5% | 0 | 1.032× | 75.06 |
| Create, stat and unlink | cold | 0 | — | — | — | 1 | — | 0.00 |
| Create, stat and unlink | warm | 0 | — | — | — | 1 | — | 0.00 |
| Deep and large-directory lookup | cold | 2 | 5,624.7 ops/s | 61.0 | 1.1% | 0 | — | 9.39 |
| Deep and large-directory lookup | warm | 2 | 8,998.9 ops/s | 145.6 | 1.6% | 0 | — | 19.95 |
| Same-directory rename | cold | 1 | 1,563.2 ops/s | 0.0 | 0.0% | 0 | — | 25.22 |
| Same-directory rename | warm | 1 | 1,571.1 ops/s | 0.0 | 0.0% | 0 | — | 25.00 |
| Same-directory concurrent mutation | cold | 3 | 3,082.4 ops/s | 5.3 | 0.5% | 0 | — | 26.05 |
| Same-directory concurrent mutation | warm | 3 | 3,083.7 ops/s | 4.6 | 0.6% | 0 | — | 26.58 |
| Concurrent creation in different directories | cold | 3 | 1,978.7 ops/s | 5.2 | 0.9% | 0 | — | 22.94 |
| Concurrent creation in different directories | warm | 4 | 2,034.7 ops/s | 15.1 | 1.7% | 0 | — | 23.39 |
| Parallel fsync | cold | 4 | 378.0 ops/s | 2.1 | 1.9% | 0 | — | 82.01 |
| Parallel fsync | warm | 4 | 378.4 ops/s | 5.3 | 2.0% | 0 | — | 81.92 |
| Reclamation near full capacity | cold | 0 | — | — | — | 1 | — | 0.00 |
| Reclamation near full capacity | warm | 0 | — | — | — | 1 | — | 0.00 |

Run: `perf-v2-core-qwen-3-8-flash-next-perf-nvme-r2`
Execution backend: `hyperv`

### Agent code review

S: **16.39** / 100; status: `complete`; purpose: `diagnostic`.

| Critical | High | Medium | Low |
|---:|---:|---:|---:|
| 8 | 3 | 29 | 0 |

Counts include confirmed, deduplicated root causes; S contributes 20% of the provisional total.

Scoring rule: `agent-review-score-v2`.

## Scope and provenance

Original profile results remain independent; the index adds a derived provisional total with missing scores imputed as zero. Each profile lists its run and, where recorded, its execution backend; different backends are not treated as identical execution conditions.

`valid` means the profile evidence is valid, not that all cases passed or release eligibility is established. `candidate-failed` is not an infrastructure error. `diagnostic` is diagnostic-only; `noneligible` cannot enter rankings. Hyper-V performance is not release- or ranking-eligible; protocol-v2 Hyper-V runs on the fixed VHDX/memory devices form their own `hyperv-fixed-vhdx` pool, counted in the provisional total and not comparable with KVM/raw-NVMe scores.

Maintainability (`maint-v3-policy`, 5% of the overall score): **83.67** / 100; components are in the README. Reference is a separate calibration baseline, not a model-generation entry.
<!-- core-results:end -->
