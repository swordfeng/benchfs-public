# Kimi K3 — Core — High

[中文](kimi-k3-high.zh-CN.md)

This page collects independent Core Eval profile results. Each profile lists its status and run provenance; unmeasured dimensions remain unmeasured.

## Run

| Field | Value |
|---|---|
| Model | `kimi-k3` |
| Effort | High |
| Variant | Core |
| Agent harness | Pi |
| Generation runs | 1 (`n=1`) |
| SDK/POSIX evaluation run | `eval-core-kimi-k3` |
| Initial SDK/POSIX evaluation date | 2026-09-04 (UTC) |
| SDK/POSIX results updated | 2026-09-05 (UTC) |

The effective effort is High, enforced by the proxy override. The Pi session's local thinking-level setting does not represent the provider-side effort.

## Submission size

Only candidate-owned source under `bench/` is counted. The fixed SDK and FUSE adapter, dependencies, documentation, configuration, lockfiles, and generated artifacts are excluded.

| Component | Files containing this component | Physical lines | Code lines |
|---|---:|---:|---:|
| Implementation (Rust) | 10 | 7,580 | 6,707 |
| Agent-written tests (Rust) | 5: 4 dedicated + 1 shared | 376 | 332 |
| Qualification script (Shell) | 1 | 43 | 32 |

Physical lines include blanks and comments. Code lines count each line holding at least one non-comment token: blank lines, comments, and documentation comments (Rust `///` and `//!`, Python docstrings) are excluded, while Rust attributes such as `#[derive(...)]` count as code. Corrected on 2026-09-27: earlier versions of this page counted documentation comments as code and Rust attributes as comments; file and physical-line counts are unchanged. The inline test module, including its test-only attribute, is counted as test code rather than implementation code. One Rust file contains both components, so file counts overlap; line counts do not. There are 14 distinct Rust files. These are source-size measurements, not test-coverage or code-quality scores.

<!-- source-tree:begin -->
### Directory structure

Every file under `bench/`, excluding build output. Numbers are code lines counted as above; "test" is the part of them in test code.

```text
bench/
├── src/
│   ├── alloc.rs  (460 code)
│   ├── btree.rs  (608 code)
│   ├── cache.rs  (188 code)
│   ├── format.rs  (1,255 code, 25 test)
│   ├── formatter.rs  (225 code)
│   ├── fs.rs  (3,471 code)
│   ├── lib.rs  (7 code)
│   ├── main.rs  (112 code)
│   ├── mkfs.rs  (112 code)
│   └── validate.rs  (294 code)
├── tests/
│   ├── btree_test.rs  (54 code, 54 test)
│   ├── nsmodel.rs  (113 code, 113 test)
│   ├── repro.rs  (93 code, 93 test)
│   └── validate.rs  (47 code, 47 test)
├── Cargo.toml
├── NOTES.md
└── qualify.sh  (32 code)
```
<!-- source-tree:end -->

## Generation time and activity

| Metric | Value |
|---|---|
| Goal start | 2026-09-02 17:18:34.870 UTC |
| Goal completion | 2026-09-03 06:05:53.540 UTC |
| Goal active time | **6 h 52 min 8 s** |
| Elapsed time, including the gap between active periods | 12 h 47 min 19 s |
| Final goal status | Complete (harness-recorded) |
| Proxy API requests / HTTP responses | 537 / 537 |
| Assistant response records with nonzero token usage | 535 |
| Assistant error records | 2 |
| Agent tool calls | 548 |

Active time is the final cumulative Pi goal counter, 24,727.690 seconds. The resumed goal inherited the earlier counter; the two goal snapshots are not summed. Active time is harness-accounted time, not model inference time. A completed goal does not imply that all evaluation tests passed.

Request counts come from the archived proxy log; assistant and tool-call counts come from the Pi session. They describe recorded activity, not a guarantee that every upstream retry or failed request was billed or fully accounted for.

### Tool usage

| Tool | Requests | Recorded failures | Unknown outcomes |
|---|---:|---:|---:|
| `bash` | 450 | 26 | 0 |
| `edit` | 32 | 1 | 0 |
| `goal_complete` | 1 | 0 | 0 |
| `read` | 46 | 0 | 0 |
| `write` | 19 | 0 | 0 |
| **Total** | **548** | **27** | **0** |

Each archived assistant `toolCall` counts as one request under its recorded tool name, including retries. Results are matched by `toolCallId` and checked for the same tool name; only `toolResult.isError: true` counts as a recorded failure. Missing or unparseable results are unknown, not assumed successful or failed. These are tool-level outcomes, not model API errors or evaluation test results.

## Token usage and API cost

| Metric | Value |
|---|---:|
| Input tokens, excluding cache (`input`) | 5,491,719 |
| Cache-read input tokens (`cache_read`) | 222,793,117 |
| Cache-write input tokens (`cache_write`) | 0 |
| Total input tokens, including cache | 228,284,836 |
| Output tokens, including thinking (`output`) | 478,897 |
| Thinking / reasoning tokens (retained-text estimate) | ≈43,834 |
| **Total tokens** | **228,763,733** |
| Cache hit rate (input tokens) | 97.59% |
| Estimated API cost (USD) | **$90.50** |

Token counts are summed from Pi assistant-message usage records and match the final cumulative goal token counter. The proxy token summary contains no usage records, so its zero token totals are not used. Input totals include repeated context across requests; they are not the size of a single context or the number of unique tokens.

The provider usage records do not contain a separate reasoning-token count. The estimate tokenizes 183,427 characters in 28 retained thinking blocks from 26 assistant records: 25 of the 535 usage-bearing responses, plus one error record with retained thinking but no reported usage. It uses the [published Kimi K3 tokenizer](https://huggingface.co/moonshotai/Kimi-K3/blob/f831ab66814297da540d832a5235f8e904f29d06/tokenization_kimi.py) and vocabulary at revision `f831ab66814297da540d832a5235f8e904f29d06`, with `tiktoken 0.11.0`. Each block is encoded independently as ordinary text, excluding chat framing and signatures. This measures retained thinking text only, not the complete run's reasoning usage or a provider-billed token count; it is not extrapolated to other responses. Including retained text from the error record does not recover its billed usage. The estimate is not added to output, total tokens, or cost. Unreported usage on failed requests remains unknown.

Cost is repriced using the non-batch model-catalog rates for `moonshotai/kimi-k3` in [OpenRouter's Models API](https://openrouter.ai/api/v1/models), retrieved on 2026-09-05. Rates below are USD per million tokens. This is a current-list-price estimate, not evidence of the run-date price, actual endpoint routing, or a provider invoice; session cost fields are not used.

| Token class | USD / million tokens | Estimated cost (USD) |
|---|---:|---:|
| Non-cached input / cache misses | $3.00 | $16.475157 |
| Output | $15.00 | $7.183455 |
| Cache read | $0.30 | $66.837935 |

The unrounded total is $90.4965471, rounded once to **$90.50**. [OpenRouter's caching documentation](https://openrouter.ai/docs/guides/best-practices/prompt-caching#moonshot-ai) lists no separate Moonshot cache-write charge, and the K3 catalog has no cache-write premium. Non-cached input is charged once at the prompt rate even if it populates a cache; a zero cache-write field does not justify adding a surcharge.

The earlier $77.14 estimate used the operator-supplied $2.55 / $12.75 / $0.256 rates, consistent with the lower-priced Makora listing in [K3 endpoint pricing](https://openrouter.ai/api/v1/models/moonshotai/kimi-k3/endpoints). The revised value uses the same model-catalog pricing basis as the other reports, not a claim that this run used a different endpoint. Unreported usage on failed requests is excluded.

Generation time, source size, tokens, activity, and cost are descriptive metadata, not correctness scores.

## Results

<!-- core-results:begin -->

### POSIX correctness

| Profile | Classification | Pass | Expected fail | Fail | Unexpected pass | Timeout | Skip N/A | Skip impl | Pass rate | Category macro average |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `spec-tests` | `valid` | 44 | 0 | 0 | 0 | 0 | 7 | 0 | 100.00% | 100.00% |
| `pjdfstest-core` | `diagnostic` | 169 | 0 | 0 | 0 | 0 | 48 | 0 | 100.00% | 100.00% |
| `xfstests-core` | `valid` | 185 | 1 | 3 | 0 | 5 | 473 | 0 | 95.88% | 89.75% |

`spec-tests` run: `eval-core-kimi-k3`

`pjdfstest-core` run: `case-correction-core-kimi-k3-pjdfstest-core`

`xfstests-core` run: `eval-core-kimi-k3`

### Robustness

Profile: `robustness`

Outcome: `candidate-failed`; classification: `valid`.

| Metric | Value |
|---|---:|
| Total | 47 |
| Pass | 46 |
| Fail | 1 |
| Timeout | 0 |
| Pass rate | 97.87% |
| Category macro average | 97.50% |

Timeouts count as failures; N/A is excluded; the macro average weights categories equally.

Run: `eval-submission-robustness-v2-core-kimi-k3-r2`

### On-disk format conformance

Profile: `canonical-format`

Outcome: `candidate-failed`; classification: `diagnostic`.

| Metric | Value |
|---|---:|
| Total | 479 |
| Pass | 410 |
| Fail | 69 |
| Timeout | 0 |
| Pass rate | 85.59% |
| Category macro average | 83.27% |
| Conformance score (0–100) | 84.17 |
| Full conformance | no |

Run: `core-canonical-kimi-oracle3-r5`
Execution backend: `hyperv`

### Crash consistency

Profile: `crash-core`

Outcome: `candidate-failed`; classification: `diagnostic`.

| Metric | Value |
|---|---:|
| Total | 216 |
| Pass | 205 |
| Fail | 11 |
| Timeout | 0 |
| Pass rate | 94.91% |
| Category macro average | 94.91% |

The table above summarizes whether cases meet all requirements. The table below scores each verified capability for robustness R.

| Check | Score / 100 | Requirements met | Requirements not met | Not verified (scores 0) |
|---|---:|---:|---:|---:|
| Disk structure | 97.22 | 210 | 5 | 1 |
| Data correctness | 99.54 | 215 | 0 | 1 |
| Unmount and reopen | 94.91 | 205 | 11 | 0 |

Crash score: **97.69**. Weights: disk structure 40%, data correctness 40%, unmount and reopen 20%.

Permitted safe rejections: 61. Two rounds tested: 144. Reopen tests not run after failure: 11. First round not tested: 0.

Each round includes mount, checks, and unmount. Safe rejection meets requirements but does not prove data recovery. Reopen not tested after an earlier failure does not count as a pass. See [scoring](../../docs/scoring.md) for the meaning of each check.

Run: `case-correction-core-kimi-k3-crash-core`

### Real-world applications

Profile: `real-world`

Outcome: `candidate-failed`; classification: `diagnostic`.

| Metric | Value |
|---|---:|
| Total | 4 |
| Pass | 3 |
| Fail | 1 |
| Timeout | 0 |
| Pass rate | 75.00% |
| Category macro average | 66.67% |

Run: `core-realworld-kimi-20260925-r1`
Execution backend: `hyperv`

### CPU performance

Profile: `perf-cpu`

Outcome: `candidate-failed`; classification: `diagnostic`.

Performance score: **53.30** (comparison pool `hyperv-fixed-vhdx`). Assessed budget: 100.0%. The range allowed by missing inputs is 53.30–53.30, not a confidence interval.

Components (out of 100): File I/O 35.96; Metadata 32.17; RAM 94.13; Write efficiency 99.66.
Peak RAM: 692.4 MiB. Metadata: 12 cells have valid rates, 2 candidate failures, 0 unmeasured.

Write amplification = controller write bytes / application logical write bytes in the timed window; it is not SSD-internal amplification. This ratio does not apply to metadata operations. A dash is not zero. Candidate failures score 0; unmeasured cells retain their budget and provisionally contribute 0. * marks an unassessed share.

| Workload | Cache | Samples | Rate | MAD | CV | Candidate-failed samples | Write amplification | Speed score / 100 |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| Sequential I/O | cold | 5 | 56.2 MiB/s | 0.2 | 0.8% | 0 | 1.000× | 0.00 |
| Sequential I/O | warm | 5 | 105.4 MiB/s | 1.0 | 0.8% | 0 | 1.000× | 1.57 |
| Random I/O, single thread | cold | 5 | 4,286.4 IOPS | 16.7 | 0.7% | 0 | 1.000× | 28.29 |
| Random I/O, single thread | warm | 5 | 5,116.4 IOPS | 15.2 | 0.3% | 0 | 1.000× | 31.66 |
| Random I/O, multiple threads | cold | 5 | 6,674.8 IOPS | 22.0 | 2.6% | 0 | 1.000× | 24.84 |
| Random I/O, multiple threads | warm | 5 | 6,167.5 IOPS | 41.8 | 1.8% | 0 | 1.000× | 23.21 |
| Random I/O, shared regions | cold | 5 | 6,503.4 IOPS | 86.2 | 1.4% | 0 | 1.002× | 36.35 |
| Random I/O, shared regions | warm | 5 | 10,551.0 IOPS | 122.5 | 1.0% | 0 | 1.000× | 45.72 |
| Synchronized overwrite | cold | 3 | 2,464.0 ops/s | 10.1 | 0.5% | 0 | 1.031× | 77.34 |
| Synchronized overwrite | warm | 3 | 2,483.9 ops/s | 4.1 | 0.9% | 0 | 1.031× | 77.48 |
| Create, stat and unlink | cold | 3 | 2,301.3 ops/s | 4.4 | 1.4% | 0 | — | 20.81 |
| Create, stat and unlink | warm | 3 | 2,286.8 ops/s | 3.6 | 0.3% | 0 | — | 21.34 |
| Deep and large-directory lookup | cold | 1 | 7,618.6 ops/s | 0.0 | 0.0% | 0 | — | 15.97 |
| Deep and large-directory lookup | warm | 1 | 7,871.5 ops/s | 0.0 | 0.0% | 0 | — | 17.04 |
| Same-directory rename | cold | 1 | 1,743.3 ops/s | 0.0 | 0.0% | 0 | — | 27.59 |
| Same-directory rename | warm | 1 | 1,737.8 ops/s | 0.0 | 0.0% | 0 | — | 27.19 |
| Same-directory concurrent mutation | cold | 3 | 3,037.5 ops/s | 2.7 | 0.1% | 0 | — | 25.74 |
| Same-directory concurrent mutation | warm | 3 | 3,047.7 ops/s | 5.0 | 0.2% | 0 | — | 26.33 |
| Concurrent creation in different directories | cold | 0 | — | — | — | 1 | — | 0.00 |
| Concurrent creation in different directories | warm | 0 | — | — | — | 1 | — | 0.00 |
| Parallel fsync | cold | 3 | 1,317.4 ops/s | 17.9 | 1.7% | 0 | — | 100.00 |
| Parallel fsync | warm | 3 | 1,343.4 ops/s | 13.1 | 2.0% | 0 | — | 100.00 |
| Reclamation near full capacity | cold | 4 | 224.3 ops/s | 2.0 | 2.1% | 0 | — | 38.55 |
| Reclamation near full capacity | warm | 4 | 231.3 ops/s | 3.4 | 2.2% | 0 | — | 38.08 |

Run: `perf-v2-k3-no-preallocation-cpu-r4`
Execution backend: `hyperv`

### NVMe performance

Profile: `perf-nvme`

Outcome: `candidate-failed`; classification: `diagnostic`.

Performance score: **53.00** (comparison pool `hyperv-fixed-vhdx`). Assessed budget: 100.0%. The range allowed by missing inputs is 53.00–53.00, not a confidence interval.

Components (out of 100): File I/O 34.12; Metadata 29.26; RAM 100.00; Write efficiency 99.66.
Peak RAM: 399.8 MiB. Metadata: 12 cells have valid rates, 2 candidate failures, 0 unmeasured.

Write amplification = controller write bytes / application logical write bytes in the timed window; it is not SSD-internal amplification. This ratio does not apply to metadata operations. A dash is not zero. Candidate failures score 0; unmeasured cells retain their budget and provisionally contribute 0. * marks an unassessed share.

| Workload | Cache | Samples | Rate | MAD | CV | Candidate-failed samples | Write amplification | Speed score / 100 |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| Sequential I/O | cold | 5 | 27.8 MiB/s | 0.0 | 0.6% | 0 | 1.000× | 0.00 |
| Sequential I/O | warm | 5 | 51.6 MiB/s | 0.4 | 0.7% | 0 | 1.000× | 0.79 |
| Random I/O, single thread | cold | 3 | 2,522.8 IOPS | 8.3 | 0.5% | 0 | 1.000× | 27.73 |
| Random I/O, single thread | warm | 5 | 3,350.3 IOPS | 2.9 | 6.4% | 0 | 1.000× | 32.52 |
| Random I/O, multiple threads | cold | 5 | 3,145.2 IOPS | 20.6 | 2.3% | 0 | 1.000× | 20.67 |
| Random I/O, multiple threads | warm | 5 | 3,352.2 IOPS | 35.4 | 1.3% | 0 | 1.000× | 21.81 |
| Random I/O, shared regions | cold | 5 | 2,925.0 IOPS | 73.4 | 4.1% | 0 | 1.002× | 30.23 |
| Random I/O, shared regions | warm | 5 | 4,815.4 IOPS | 107.2 | 5.0% | 0 | 1.000× | 38.74 |
| Synchronized overwrite | cold | 3 | 1,701.4 ops/s | 14.5 | 0.7% | 0 | 1.031× | 74.61 |
| Synchronized overwrite | warm | 3 | 1,718.4 ops/s | 5.9 | 0.3% | 0 | 1.031× | 74.76 |
| Create, stat and unlink | cold | 2 | 2,073.7 ops/s | 3.0 | 0.1% | 0 | — | 18.55 |
| Create, stat and unlink | warm | 2 | 2,057.1 ops/s | 10.9 | 0.5% | 0 | — | 19.04 |
| Deep and large-directory lookup | cold | 1 | 7,361.0 ops/s | 0.0 | 0.0% | 0 | — | 15.23 |
| Deep and large-directory lookup | warm | 1 | 7,751.8 ops/s | 0.0 | 0.0% | 0 | — | 16.71 |
| Same-directory rename | cold | 1 | 1,714.6 ops/s | 0.0 | 0.0% | 0 | — | 27.23 |
| Same-directory rename | warm | 1 | 1,719.4 ops/s | 0.0 | 0.0% | 0 | — | 26.96 |
| Same-directory concurrent mutation | cold | 2 | 2,752.5 ops/s | 5.8 | 0.2% | 0 | — | 23.60 |
| Same-directory concurrent mutation | warm | 2 | 2,745.6 ops/s | 11.6 | 0.4% | 0 | — | 24.06 |
| Concurrent creation in different directories | cold | 0 | — | — | — | 1 | — | 0.00 |
| Concurrent creation in different directories | warm | 0 | — | — | — | 1 | — | 0.00 |
| Parallel fsync | cold | 1 | 522.2 ops/s | 0.0 | 0.0% | 0 | — | 89.02 |
| Parallel fsync | warm | 1 | 513.2 ops/s | 0.0 | 0.0% | 0 | — | 88.53 |
| Reclamation near full capacity | cold | 4 | 197.1 ops/s | 2.5 | 2.0% | 0 | — | 35.74 |
| Reclamation near full capacity | warm | 5 | 194.3 ops/s | 4.2 | 2.9% | 0 | — | 34.29 |

Run: `perf-v2-k3-hdd-template-nvme-r2`
Execution backend: `hyperv`

### Agent code review

S: **26.88** / 100; status: `complete`; purpose: `diagnostic`.

| Critical | High | Medium | Low |
|---:|---:|---:|---:|
| 4 | 3 | 13 | 2 |

Counts include confirmed, deduplicated root causes; S contributes 20% of the provisional total.

Scoring rule: `agent-review-score-v2`.

## Scope and provenance

Original profile results remain independent; the index adds a derived provisional total with missing scores imputed as zero. Each profile lists its run and, where recorded, its execution backend; different backends are not treated as identical execution conditions.

`valid` means the profile evidence is valid, not that all cases passed or release eligibility is established. `candidate-failed` is not an infrastructure error. `diagnostic` is diagnostic-only; `noneligible` cannot enter rankings. Hyper-V performance is not release- or ranking-eligible; protocol-v2 Hyper-V runs on the fixed VHDX/memory devices form their own `hyperv-fixed-vhdx` pool, counted in the provisional total and not comparable with KVM/raw-NVMe scores.

Maintainability (`maint-v3-policy`, 5% of the overall score): **84.78** / 100; components are in the README. Reference is a separate calibration baseline, not a model-generation entry.
<!-- core-results:end -->
