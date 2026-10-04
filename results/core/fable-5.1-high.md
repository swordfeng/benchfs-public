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

The table above summarizes whether cases meet all requirements. The table below scores each verified capability for robustness R.

| Check | Score / 100 | Requirements met | Requirements not met | Not verified (scores 0) |
|---|---:|---:|---:|---:|
| Disk structure | 100.00 | 216 | 0 | 0 |
| Data correctness | 100.00 | 216 | 0 | 0 |
| Unmount and reopen | 36.57 | 79 | 137 | 0 |

Crash score: **87.31**. Weights: disk structure 40%, data correctness 40%, unmount and reopen 20%.

Permitted safe rejections: 66. Two rounds tested: 13. Reopen tests not run after failure: 137. First round not tested: 0.

Each round includes mount, checks, and unmount. Safe rejection meets requirements but does not prove data recovery. Reopen not tested after an earlier failure does not count as a pass. See [scoring](../../docs/scoring.md) for the meaning of each check.

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

Performance score: **62.89** (comparison pool `hyperv-fixed-vhdx`). Assessed budget: 100.0%. The range allowed by missing inputs is 62.89–62.89, not a confidence interval.

Components (out of 100): File I/O 50.47; Metadata 36.80; RAM 100.00; Write efficiency 99.55.
Peak RAM: 30.3 MiB. Metadata: 12 cells have valid rates, 2 candidate failures, 0 unmeasured.

Write amplification = controller write bytes / application logical write bytes in the timed window; it is not SSD-internal amplification. This ratio does not apply to metadata operations. A dash is not zero. Candidate failures score 0; unmeasured cells retain their budget and provisionally contribute 0. * marks an unassessed share.

| Workload | Cache | Samples | Rate | MAD | CV | Candidate-failed samples | Write amplification | Speed score / 100 |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| Sequential I/O | cold | 3 | 407.0 MiB/s | 0.0 | 0.5% | 0 | 1.000× | 41.70 |
| Sequential I/O | warm | 5 | 404.1 MiB/s | 1.6 | 0.5% | 0 | 1.000× | 41.56 |
| Random I/O, single thread | cold | 3 | 5,101.4 IOPS | 41.6 | 0.7% | 0 | 1.000× | 31.68 |
| Random I/O, single thread | warm | 3 | 5,082.9 IOPS | 26.7 | 0.5% | 0 | 1.000× | 31.54 |
| Random I/O, multiple threads | cold | 3 | 15,498.3 IOPS | 97.0 | 0.7% | 0 | 1.000× | 42.20 |
| Random I/O, multiple threads | warm | 5 | 15,466.2 IOPS | 38.7 | 0.4% | 0 | 1.000× | 42.16 |
| Random I/O, shared regions | cold | 3 | 7,492.6 IOPS | 12.7 | 0.2% | 0 | 1.032× | 39.10 |
| Random I/O, shared regions | warm | 5 | 7,507.2 IOPS | 26.9 | 0.5% | 0 | 1.035× | 39.11 |
| Synchronized overwrite | cold | 3 | 2,604.1 ops/s | 10.7 | 0.6% | 0 | 1.031× | 78.44 |
| Synchronized overwrite | warm | 3 | 2,603.0 ops/s | 4.9 | 0.2% | 0 | 1.031× | 78.41 |
| Create, stat and unlink | cold | 0 | — | — | — | 1 | — | 0.00 |
| Create, stat and unlink | warm | 0 | — | — | — | 1 | — | 0.00 |
| Deep and large-directory lookup | cold | 3 | 8,731.9 ops/s | 42.8 | 0.4% | 0 | — | 18.94 |
| Deep and large-directory lookup | warm | 3 | 8,906.5 ops/s | 20.5 | 0.5% | 0 | — | 19.72 |
| Same-directory rename | cold | 3 | 2,222.5 ops/s | 4.3 | 0.3% | 0 | — | 32.86 |
| Same-directory rename | warm | 3 | 2,186.5 ops/s | 6.1 | 0.4% | 0 | — | 32.18 |
| Same-directory concurrent mutation | cold | 3 | 3,882.1 ops/s | 25.8 | 1.1% | 0 | — | 31.06 |
| Same-directory concurrent mutation | warm | 3 | 3,920.4 ops/s | 22.8 | 0.7% | 0 | — | 31.80 |
| Concurrent creation in different directories | cold | 3 | 3,959.1 ops/s | 13.4 | 1.5% | 0 | — | 38.00 |
| Concurrent creation in different directories | warm | 3 | 3,950.3 ops/s | 11.8 | 1.0% | 0 | — | 37.79 |
| Parallel fsync | cold | 3 | 1,256.9 ops/s | 17.3 | 1.4% | 0 | — | 100.00 |
| Parallel fsync | warm | 3 | 1,238.0 ops/s | 3.0 | 0.9% | 0 | — | 100.00 |
| Reclamation near full capacity | cold | 5 | 439.6 ops/s | 7.6 | 2.5% | 0 | — | 53.16 |
| Reclamation near full capacity | warm | 3 | 434.5 ops/s | 1.7 | 1.5% | 0 | — | 51.77 |

Run: `perf-v2-core-fable-5-1-perf-cpu-r2`
Execution backend: `hyperv`

### NVMe performance

Profile: `perf-nvme`

Outcome: `candidate-failed`; classification: `diagnostic`.

Performance score: **62.38** (comparison pool `hyperv-fixed-vhdx`). Assessed budget: 100.0%. The range allowed by missing inputs is 62.38–62.38, not a confidence interval.

Components (out of 100): File I/O 49.92; Metadata 35.56; RAM 100.00; Write efficiency 99.50.
Peak RAM: 40.1 MiB. Metadata: 13 cells have valid rates, 1 candidate failures, 0 unmeasured.

Write amplification = controller write bytes / application logical write bytes in the timed window; it is not SSD-internal amplification. This ratio does not apply to metadata operations. A dash is not zero. Candidate failures score 0; unmeasured cells retain their budget and provisionally contribute 0. * marks an unassessed share.

| Workload | Cache | Samples | Rate | MAD | CV | Candidate-failed samples | Write amplification | Speed score / 100 |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| Sequential I/O | cold | 3 | 331.5 MiB/s | 0.0 | 0.6% | 0 | 1.000× | 46.60 |
| Sequential I/O | warm | 3 | 331.1 MiB/s | 0.9 | 0.4% | 0 | 1.000× | 46.64 |
| Random I/O, single thread | cold | 3 | 3,410.8 IOPS | 16.3 | 0.5% | 0 | 1.000× | 32.89 |
| Random I/O, single thread | warm | 3 | 3,401.8 IOPS | 3.0 | 0.2% | 0 | 1.000× | 32.78 |
| Random I/O, multiple threads | cold | 5 | 8,219.1 IOPS | 30.3 | 0.3% | 0 | 1.000× | 37.99 |
| Random I/O, multiple threads | warm | 5 | 8,198.5 IOPS | 4.8 | 0.2% | 0 | 1.000× | 37.94 |
| Random I/O, shared regions | cold | 3 | 3,204.9 IOPS | 13.1 | 1.2% | 0 | 1.071× | 31.79 |
| Random I/O, shared regions | warm | 3 | 3,246.9 IOPS | 12.9 | 0.9% | 0 | 1.026× | 32.00 |
| Synchronized overwrite | cold | 3 | 1,701.9 ops/s | 1.8 | 0.1% | 0 | 1.031× | 74.62 |
| Synchronized overwrite | warm | 3 | 1,699.8 ops/s | 8.4 | 0.6% | 0 | 1.031× | 74.57 |
| Create, stat and unlink | cold | 1 | 2,278.3 ops/s | 0.0 | 0.0% | 1 | — | 20.60 |
| Create, stat and unlink | warm | 0 | — | — | — | 1 | — | 0.00 |
| Deep and large-directory lookup | cold | 3 | 8,526.2 ops/s | 4.0 | 0.2% | 0 | — | 18.42 |
| Deep and large-directory lookup | warm | 3 | 8,888.6 ops/s | 16.2 | 0.3% | 0 | — | 19.68 |
| Same-directory rename | cold | 3 | 2,167.5 ops/s | 6.7 | 0.4% | 0 | — | 32.32 |
| Same-directory rename | warm | 3 | 2,110.4 ops/s | 0.9 | 0.2% | 0 | — | 31.41 |
| Same-directory concurrent mutation | cold | 3 | 3,931.3 ops/s | 0.6 | 0.3% | 0 | — | 31.34 |
| Same-directory concurrent mutation | warm | 3 | 3,915.6 ops/s | 1.8 | 0.1% | 0 | — | 31.77 |
| Concurrent creation in different directories | cold | 3 | 3,189.7 ops/s | 0.6 | 0.2% | 0 | — | 33.30 |
| Concurrent creation in different directories | warm | 3 | 3,228.7 ops/s | 2.0 | 0.1% | 0 | — | 33.41 |
| Parallel fsync | cold | 3 | 408.1 ops/s | 4.5 | 1.0% | 0 | — | 83.67 |
| Parallel fsync | warm | 3 | 407.6 ops/s | 2.7 | 1.0% | 0 | — | 83.53 |
| Reclamation near full capacity | cold | 3 | 425.6 ops/s | 2.3 | 0.6% | 0 | — | 52.46 |
| Reclamation near full capacity | warm | 3 | 422.9 ops/s | 0.2 | 0.1% | 0 | — | 51.18 |

Run: `perf-v2-core-fable-5-1-perf-nvme-r2`
Execution backend: `hyperv`

### Agent code review

S: **38.31** / 100; status: `complete`; purpose: `diagnostic`.

| Critical | High | Medium | Low |
|---:|---:|---:|---:|
| 0 | 6 | 13 | 6 |

Counts include confirmed, deduplicated root causes; S contributes 20% of the provisional total.

Scoring rule: `agent-review-score-v2`.

## Scope and provenance

Original profile results remain independent; the index adds a derived provisional total with missing scores imputed as zero. Each profile lists its run and, where recorded, its execution backend; different backends are not treated as identical execution conditions.

`valid` means the profile evidence is valid, not that all cases passed or release eligibility is established. `candidate-failed` is not an infrastructure error. `diagnostic` is diagnostic-only; `noneligible` cannot enter rankings. Hyper-V performance is not release- or ranking-eligible; protocol-v2 Hyper-V runs on the fixed VHDX/memory devices form their own `hyperv-fixed-vhdx` pool, counted in the provisional total and not comparable with KVM/raw-NVMe scores.

Maintainability (`maint-v3-policy`, 5% of the overall score): **90.76** / 100; components are in the README. Reference is a separate calibration baseline, not a model-generation entry.
<!-- core-results:end -->
