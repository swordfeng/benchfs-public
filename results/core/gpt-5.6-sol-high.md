# GPT 5.6 Sol — Core — High

[中文](gpt-5.6-sol-high.zh-CN.md)

This page collects independent Core Eval profile results. Each profile lists its status and run provenance; unmeasured dimensions remain unmeasured.

## Run

| Field | Value |
|---|---|
| Model | `gpt-5.6-sol` |
| Effort | High |
| Variant | Core |
| Agent harness | Pi |
| Generation runs | 1 (`n=1`) |
| Submission archive | `core-gpt-5.6-sol-v2` |
| SDK/POSIX evaluation run | `eval-core-gpt-5-6-sol-v2-r1` |
| Initial SDK/POSIX evaluation date | 2026-09-02 (UTC) |
| SDK/POSIX results updated | 2026-09-05 (UTC) |

The effective effort is High, confirmed by the operator for the proxy override. The Pi session's local thinking-level setting does not represent the provider-side effort.

## Submission size

Only candidate-owned source under `bench/` is counted. The fixed SDK and FUSE adapter, dependencies, documentation, configuration, lockfiles, and generated artifacts are excluded.

| Component | Files containing this component | Physical lines | Code lines |
|---|---:|---:|---:|
| Implementation (Rust) | 4 | 3,311 | 3,236 |
| Agent-written tests (Rust) | 0 | 0 | 0 |
| Qualification scripts | 0 | 0 | 0 |

Physical lines include blanks and comments. Code lines count each line holding at least one non-comment token: blank lines, comments, and documentation comments (Rust `///` and `//!`, Python docstrings) are excluded, while Rust attributes such as `#[derive(...)]` count as code. Corrected on 2026-09-27: earlier versions of this page counted documentation comments as code and Rust attributes as comments; file and physical-line counts are unchanged. There are no dedicated test files, inline test modules, or scripts in the archived `bench/` tree. This does not imply that the agent ran no tests during development. These are source-size measurements, not test-coverage or code-quality scores.

<!-- source-tree:begin -->
### Directory structure

Every file under `bench/`, excluding build output. Numbers are code lines counted as above; "test" is the part of them in test code.

```text
bench/
├── src/
│   ├── format.rs  (486 code)
│   ├── fs.rs  (2,687 code)
│   ├── main.rs  (45 code)
│   └── mkfs.rs  (18 code)
└── Cargo.toml
```
<!-- source-tree:end -->

## Generation time and activity

| Metric | Value |
|---|---|
| Goal start | 2026-08-31 16:50:48.032 UTC |
| Goal completion | 2026-08-31 19:33:45.460 UTC |
| Goal active time | **2 h 42 min 57 s** |
| Elapsed time | 2 h 42 min 57 s |
| Final goal status | Complete (harness-recorded) |
| Proxy API requests / HTTP responses | 197 / 197 |
| Assistant response records with nonzero token usage | 195 |
| Assistant error records | 1 |
| Agent tool calls | 234 |

Active time is the final cumulative Pi goal counter, 9,777.428 seconds, from one goal. Active time is harness-accounted time, not model inference time. A completed goal does not imply that all evaluation tests passed.

Request counts come from the archived proxy log; assistant and tool-call counts come from the Pi session. The session contains 196 assistant records, while the proxy records 197 requests. These are separate recorded counts, not interchangeable measures or a guarantee that every upstream retry or failed request was billed or fully accounted for.

### Tool usage

| Tool | Requests | Recorded failures | Unknown outcomes |
|---|---:|---:|---:|
| `bash` | 84 | 7 | 0 |
| `edit` | 78 | 11 | 1 |
| `goal_complete` | 1 | 0 | 0 |
| `read` | 67 | 0 | 0 |
| `write` | 4 | 0 | 0 |
| **Total** | **234** | **18** | **1** |

Each archived assistant `toolCall` counts as one request under its recorded tool name, including retries. Results are matched by `toolCallId` and checked for the same tool name; only `toolResult.isError: true` counts as a recorded failure. Missing or unparseable results are unknown, not assumed successful or failed. These are tool-level outcomes, not model API errors or evaluation test results.

One `edit` request has no matching archived result, so 18 is the observed failure count, not a complete failure total.

## Token usage and API cost

| Metric | Value |
|---|---:|
| Input tokens, excluding cache (`input`) | 323,794 |
| Cache-read input tokens (`cache_read`) | 30,650,368 |
| Cache-write input tokens (`cache_write`) | 0 |
| Total input tokens, including cache | 30,974,162 |
| Output tokens, including thinking (`output`) | 87,220 |
| Thinking / reasoning tokens | 26,681 |
| **Total tokens** | **31,061,382** |
| Cache hit rate (input tokens) | 98.95% |
| Estimated API cost (USD) | **$7.81** |

Token counts are summed from Pi assistant-message usage records and match both the archived proxy token totals and the final cumulative goal token counter. Reasoning usage is explicitly recorded on all 195 responses with nonzero usage; it is a component of output and is not added again. Input totals include repeated context across requests; they are not the size of a single context or the number of unique tokens.

Cost uses the non-batch model-catalog rates for `openai/gpt-5.6-sol` in [OpenRouter's Models API](https://openrouter.ai/api/v1/models), corroborated by the standard OpenAI endpoint in [Sol endpoint pricing](https://openrouter.ai/api/v1/models/openai/gpt-5.6-sol/endpoints), retrieved on 2026-09-05. Rates below are USD per million tokens. This is a current-list-price estimate, not evidence of the run-date price, actual endpoint routing, or a provider invoice; session cost fields are not used.

| Token class | USD / million tokens | Estimated cost (USD) |
|---|---:|---:|
| Input, assumed to populate the cache | $2.50 | $0.809485 |
| Output | $10.00 | $0.872200 |
| Cache read | $0.20 | $6.130074 |

[OpenRouter's OpenAI caching rules](https://openrouter.ai/docs/guides/best-practices/prompt-caching#openai) charge automatic cache writes on the GPT-5.6 family at 1.25 times ordinary input pricing. The session records zero separate cache writes but does not establish how many non-cached input tokens populated the cache. For this estimate, all 323,794 input tokens are conservatively priced as cache writes at $2.50, replacing rather than adding to the ordinary $2.00 prompt charge. This is a billing assumption, not a recovered cache-write token count; the recorded usage table is unchanged. Pricing all input at the ordinary prompt rate instead would yield $7.6498616, or $7.65.

Price tiers are checked per response using total input, including cache reads and writes. All 195 usage-bearing responses are below the 272,000-token long-context threshold; the maximum is 215,066. Therefore no long-context premium is applied. The unrounded cache-write-assumption total is $7.8117586, rounded once to **$7.81**. Reasoning is already included in output. The token totals cover reported usage only; unreported usage on failed requests remains unknown.

Generation time, source size, tokens, activity, and cost are descriptive metadata, not correctness scores.

## Results

<!-- core-results:begin -->

### POSIX correctness

| Profile | Classification | Pass | Expected fail | Fail | Unexpected pass | Timeout | Skip N/A | Skip impl | Pass rate | Category macro average |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `spec-tests` | `valid` | 44 | 0 | 0 | 0 | 0 | 7 | 0 | 100.00% | 100.00% |
| `pjdfstest-core` | `diagnostic` | 169 | 0 | 0 | 0 | 0 | 48 | 0 | 100.00% | 100.00% |
| `xfstests-core` | `valid` | 183 | 1 | 4 | 0 | 6 | 473 | 0 | 94.85% | 89.62% |

`spec-tests` run: `eval-core-gpt-5-6-sol-v2-r1`

`pjdfstest-core` run: `case-correction-core-gpt-5.6-sol-v2-pjdfstest-core`

`xfstests-core` run: `eval-core-gpt-5-6-sol-v2-r1`

### Robustness

Profile: `robustness`

Outcome: `candidate-failed`; classification: `valid`.

| Metric | Value |
|---|---:|
| Total | 47 |
| Pass | 40 |
| Fail | 7 |
| Timeout | 0 |
| Pass rate | 85.11% |
| Category macro average | 78.75% |

Timeouts count as failures; N/A is excluded; the macro average weights categories equally.

Run: `eval-submission-robustness-v2-core-gpt-5-6-sol-v2-r1`

### On-disk format conformance

Profile: `canonical-format`

Outcome: `candidate-failed`; classification: `diagnostic`.

| Metric | Value |
|---|---:|
| Total | 479 |
| Pass | 442 |
| Fail | 37 |
| Timeout | 0 |
| Pass rate | 92.28% |
| Category macro average | 88.95% |
| Conformance score (0–100) | 92.15 |
| Full conformance | no |

Run: `core-canonical-sol-oracle3-r5`
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
| Disk structure | 99.54 | 215 | 1 | 0 |
| Data correctness | 100.00 | 216 | 0 | 0 |
| Unmount and reopen | 98.15 | 212 | 4 | 0 |

Crash score: **99.44**. Weights: disk structure 40%, data correctness 40%, unmount and reopen 20%.

Permitted safe rejections: 38. Two rounds tested: 176. Reopen tests not run after failure: 2. First round not tested: 0.

Each round includes mount, checks, and unmount. Safe rejection meets requirements but does not prove data recovery. Reopen not tested after an earlier failure does not count as a pass. See [scoring](../../docs/scoring.md) for the meaning of each check.

Run: `case-correction-core-gpt-5.6-sol-v2-crash-core`

### Real-world applications

Profile: `real-world`

Outcome: `candidate-failed`; classification: `diagnostic`.

| Metric | Value |
|---|---:|
| Total | 4 |
| Pass | 2 |
| Fail | 0 |
| Timeout | 2 |
| Pass rate | 50.00% |
| Category macro average | 33.33% |

Run: `core-realworld-sol-oracle3-r6`
Execution backend: `hyperv`

### CPU performance

Profile: `perf-cpu`

Outcome: `candidate-failed`; classification: `diagnostic`.

Performance score: **38.12** (comparison pool `hyperv-fixed-vhdx`). Assessed budget: 100.0%. The range allowed by missing inputs is 38.12–38.12, not a confidence interval.

Components (out of 100): File I/O 8.35; Metadata 29.60; RAM 100.00; Write efficiency 85.59.
Peak RAM: 427.0 MiB. Metadata: 8 cells have valid rates, 6 candidate failures, 0 unmeasured.

Write amplification = controller write bytes / application logical write bytes in the timed window; it is not SSD-internal amplification. This ratio does not apply to metadata operations. A dash is not zero. Candidate failures score 0; unmeasured cells retain their budget and provisionally contribute 0. * marks an unassessed share.

| Workload | Cache | Samples | Rate | MAD | CV | Candidate-failed samples | Write amplification | Speed score / 100 |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| Sequential I/O | cold | 5 | 28.4 MiB/s | 0.4 | 4.7% | 0 | 0.921× | 0.00 |
| Sequential I/O | warm | 5 | 28.9 MiB/s | 0.8 | 7.9% | 0 | 0.924× | 0.00 |
| Random I/O, single thread | cold | 5 | 361.9 IOPS | 38.2 | 8.5% | 0 | 0.000× | 0.00 |
| Random I/O, single thread | warm | 5 | 377.7 IOPS | 27.7 | 6.3% | 0 | 0.000× | 0.00 |
| Random I/O, multiple threads | cold | 5 | 2,662.0 IOPS | 1,506.9 | 54.6% | 0 | 0.000× | 5.89 |
| Random I/O, multiple threads | warm | 5 | 491.6 IOPS | 258.4 | 109.8% | 0 | 0.000× | 0.00 |
| Random I/O, shared regions | cold | 5 | 6,803.6 IOPS | 123.3 | 2.6% | 0 | 0.000× | 37.23 |
| Random I/O, shared regions | warm | 5 | 6,653.3 IOPS | 90.1 | 3.4% | 0 | 0.000× | 36.77 |
| Synchronized overwrite | cold | 5 | 102.5 ops/s | 3.1 | 8.0% | 0 | 3.617× | 14.24 |
| Synchronized overwrite | warm | 5 | 105.4 ops/s | 1.0 | 5.6% | 0 | 3.963× | 14.79 |
| Create, stat and unlink | cold | 3 | 2,902.4 ops/s | 5.7 | 0.2% | 0 | — | 25.85 |
| Create, stat and unlink | warm | 2 | 2,894.6 ops/s | 10.9 | 0.4% | 0 | — | 26.46 |
| Deep and large-directory lookup | cold | 0 | — | — | — | 0 | — | 0.00 |
| Deep and large-directory lookup | warm | 0 | — | — | — | 0 | — | 0.00 |
| Same-directory rename | cold | 0 | — | — | — | 0 | — | 0.00 |
| Same-directory rename | warm | 0 | — | — | — | 0 | — | 0.00 |
| Same-directory concurrent mutation | cold | 2 | 4,024.6 ops/s | 21.4 | 0.5% | 0 | — | 31.85 |
| Same-directory concurrent mutation | warm | 2 | 4,026.1 ops/s | 14.2 | 0.4% | 0 | — | 32.37 |
| Concurrent creation in different directories | cold | 0 | — | — | — | 1 | — | 0.00 |
| Concurrent creation in different directories | warm | 0 | — | — | — | 1 | — | 0.00 |
| Parallel fsync | cold | 2 | 420.8 ops/s | 24.6 | 5.9% | 0 | — | 84.34 |
| Parallel fsync | warm | 2 | 429.6 ops/s | 13.4 | 3.1% | 0 | — | 84.67 |
| Reclamation near full capacity | cold | 5 | 883.3 ops/s | 48.0 | 5.8% | 0 | — | 68.31 |
| Reclamation near full capacity | warm | 4 | 967.8 ops/s | 8.6 | 1.5% | 0 | — | 69.16 |

Run: `perf-v2-core-gpt-5-6-sol-v2-perf-cpu-r1`
Execution backend: `hyperv`

### NVMe performance

Profile: `perf-nvme`

Outcome: `candidate-failed`; classification: `diagnostic`.

Performance score: **29.86** (comparison pool `hyperv-fixed-vhdx`). Assessed budget: 97.4%. The range allowed by missing inputs is 29.86–32.48, not a confidence interval.

Components (out of 100): File I/O 2.13; Metadata 16.75*; RAM 94.04; Write efficiency 70.00.
Peak RAM: 695.2 MiB. Metadata: 6 cells have valid rates, 6 candidate failures, 2 unmeasured.

Write amplification = controller write bytes / application logical write bytes in the timed window; it is not SSD-internal amplification. This ratio does not apply to metadata operations. A dash is not zero. Candidate failures score 0; unmeasured cells retain their budget and provisionally contribute 0. * marks an unassessed share.

| Workload | Cache | Samples | Rate | MAD | CV | Candidate-failed samples | Write amplification | Speed score / 100 |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| Sequential I/O | cold | 5 | 13.3 MiB/s | 0.1 | 0.7% | 0 | 0.000× | 0.00 |
| Sequential I/O | warm | 5 | 24.9 MiB/s | 0.1 | 0.6% | 0 | 0.000× | 0.00 |
| Random I/O, single thread | cold | 5 | 89.0 IOPS | 1.3 | 1.8% | 0 | 0.000× | 0.00 |
| Random I/O, single thread | warm | 5 | 88.2 IOPS | 0.8 | 1.2% | 0 | 0.000× | 0.00 |
| Random I/O, multiple threads | cold | 5 | 296.0 IOPS | 231.4 | 114.3% | 0 | 0.000× | 0.00 |
| Random I/O, multiple threads | warm | 5 | 60.1 IOPS | 10.6 | 65.9% | 0 | 0.000× | 0.00 |
| Random I/O, shared regions | cold | 5 | 1,605.1 IOPS | 367.5 | 44.7% | 0 | 0.000× | 19.96 |
| Random I/O, shared regions | warm | 5 | 1,736.9 IOPS | 560.2 | 51.4% | 0 | 0.000× | 21.30 |
| Synchronized overwrite | cold | 5 | 20.1 ops/s | 0.3 | 2.2% | 0 | 17.145× | 0.06 |
| Synchronized overwrite | warm | 3 | 20.5 ops/s | 0.0 | 0.6% | 0 | 16.913× | 0.39 |
| Create, stat and unlink | cold | 2 | 2,888.2 ops/s | 10.0 | 0.3% | 0 | — | 25.75 |
| Create, stat and unlink | warm | 2 | 2,868.9 ops/s | 11.3 | 0.4% | 0 | — | 26.27 |
| Deep and large-directory lookup | cold | 0 | — | — | — | 0 | — | 0.00 |
| Deep and large-directory lookup | warm | 0 | — | — | — | 0 | — | 0.00 |
| Same-directory rename | cold | 0 | — | — | — | 0 | — | 0.00 |
| Same-directory rename | warm | 0 | — | — | — | 0 | — | 0.00 |
| Same-directory concurrent mutation | cold | 2 | 3,983.9 ops/s | 46.4 | 1.2% | 0 | — | 31.63 |
| Same-directory concurrent mutation | warm | 1 | 4,023.6 ops/s | 0.0 | 0.0% | 0 | — | 32.36 |
| Concurrent creation in different directories | cold | 0 | — | — | — | 1 | — | 0.00 |
| Concurrent creation in different directories | warm | 0 | — | — | — | 1 | — | 0.00 |
| Parallel fsync | cold | 0 | — | — | — | 0 | — | — |
| Parallel fsync | warm | 0 | — | — | — | 0 | — | — |
| Reclamation near full capacity | cold | 3 | 869.8 ops/s | 5.9 | 0.7% | 0 | — | 67.98 |
| Reclamation near full capacity | warm | 5 | 879.0 ops/s | 42.2 | 6.5% | 0 | — | 67.07 |

Run: `perf-v2-core-gpt-5-6-sol-v2-perf-nvme-r2`
Execution backend: `hyperv`

### Agent code review

S: **31.15** / 100; status: `complete`; purpose: `diagnostic`.

| Critical | High | Medium | Low |
|---:|---:|---:|---:|
| 2 | 5 | 13 | 1 |

Counts include confirmed, deduplicated root causes; S contributes 20% of the provisional total.

Scoring rule: `agent-review-score-v2`.

## Scope and provenance

Original profile results remain independent; the index adds a derived provisional total with missing scores imputed as zero. Each profile lists its run and, where recorded, its execution backend; different backends are not treated as identical execution conditions.

`valid` means the profile evidence is valid, not that all cases passed or release eligibility is established. `candidate-failed` is not an infrastructure error. `diagnostic` is diagnostic-only; `noneligible` cannot enter rankings. Hyper-V performance is not release- or ranking-eligible; protocol-v2 Hyper-V runs on the fixed VHDX/memory devices form their own `hyperv-fixed-vhdx` pool, counted in the provisional total and not comparable with KVM/raw-NVMe scores.

Maintainability (`maint-v3-policy`, 5% of the overall score): **75.69** / 100; components are in the README. Reference is a separate calibration baseline, not a model-generation entry.
<!-- core-results:end -->
