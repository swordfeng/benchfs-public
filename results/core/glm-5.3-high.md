# GLM 5.3 — Core — High

[中文](glm-5.3-high.zh-CN.md)

This page collects independent Core Eval profile results. Each profile lists its status and run provenance; unmeasured dimensions remain unmeasured.

## Run

| Field | Value |
|---|---|
| Model | `glm-5.3` |
| Effort | High |
| Variant | Core |
| Agent harness | Pi |
| Generation runs | 1 (`n=1`) |
| SDK/POSIX evaluation run | `eval-core-glm-5.3` |
| SDK/POSIX evaluation date | 2026-09-06 (UTC) |

The effective effort is High, confirmed by the operator for the proxy override. The Pi session's local thinking-level setting does not represent the provider-side effort.

## Submission size

Only candidate-owned source under `bench/` is counted. The fixed SDK and FUSE adapter, dependencies, documentation, configuration, lockfiles, and generated artifacts are excluded.

| Component | Files containing this component | Physical lines | Code lines |
|---|---:|---:|---:|
| Implementation (Rust) | 9 | 7,923 | 6,936 |
| Agent-written tests and test support (Rust) | 4 | 694 | 629 |
| Qualification scripts | 0 | 0 | 0 |

Physical lines include blanks and comments. Code lines count each line holding at least one non-comment token: blank lines, comments, and documentation comments (Rust `///` and `//!`, Python docstrings) are excluded, while Rust attributes such as `#[derive(...)]` count as code. Corrected on 2026-09-27: earlier versions of this page counted documentation comments as code and Rust attributes as comments; file and physical-line counts are unchanged. Tests comprise inline test modules and three test-only helper methods in four implementation files. File counts overlap; physical and code lines are partitioned without overlap. There are nine distinct Rust files and no dedicated test files or scripts. These are source-size measurements, not test-coverage or code-quality scores.

<!-- source-tree:begin -->
### Directory structure

Every file under `bench/`, excluding build output. Numbers are code lines counted as above; "test" is the part of them in test code.

```text
bench/
├── src/
│   ├── btree.rs  (779 code, 225 test)
│   ├── core.rs  (3,475 code, 234 test)
│   ├── device.rs  (833 code, 119 test)
│   ├── format.rs  (1,352 code, 51 test)
│   ├── lib.rs  (6 code)
│   ├── main.rs  (59 code)
│   ├── mkfs.rs  (257 code)
│   ├── mkfs_main.rs  (67 code)
│   └── validate.rs  (737 code)
├── Cargo.toml
└── NOTES.md
```
<!-- source-tree:end -->

## Generation time and activity

| Metric | Value |
|---|---|
| Goal start | 2026-09-04 16:40:57.061 UTC |
| Goal completion | 2026-09-05 18:10:08.085 UTC |
| Goal active time | **7 h 32 min 19 s** |
| Elapsed time, including gaps between active periods | 25 h 29 min 11 s |
| Final goal status | Complete (harness-recorded) |
| Proxy API requests / HTTP responses | 629 / 629 |
| Assistant response records with nonzero token usage | 613 |
| Assistant error records | 16 |
| Agent tool calls | 615 |

Active time is the final cumulative Pi goal counter, 27,139.120 seconds. The four goal identities belong to a resumed generation and inherit earlier counters; their snapshots are not summed. It is harness-accounted time, not model inference time. A completed goal does not imply that all evaluation tests passed.

Request counts come from the archived proxy log; assistant and tool-call counts come from the Pi session. The proxy records 613 HTTP 200 responses and 16 HTTP 429 responses. All 16 assistant error records report zero usage. Reported zero usage does not prove that failed requests incurred no upstream cost.

### Tool usage

| Tool | Requests | Recorded failures | Unknown outcomes |
|---|---:|---:|---:|
| `bash` | 585 | 45 | 0 |
| `edit` | 5 | 1 | 0 |
| `goal_complete` | 1 | 0 | 0 |
| `read` | 9 | 0 | 0 |
| `write` | 15 | 1 | 0 |
| **Total** | **615** | **47** | **0** |

Each archived assistant `toolCall` counts as one request under its recorded tool name, including retries. Results are matched by `toolCallId` and checked for the same tool name; only `toolResult.isError: true` counts as a recorded failure. Missing or unparseable results are unknown, not assumed successful or failed. These are tool-level outcomes, not model API errors or evaluation test results.

## Token usage and API cost

| Metric | Value |
|---|---:|
| Non-cached input tokens (estimated) | 15,248,264 |
| Cache-read input tokens (estimated) | 280,413,851 |
| Cache-write input tokens | Not estimated by this heuristic |
| Total input tokens, including cache | 295,662,115 |
| Output tokens, including thinking (`output`) | 541,837 |
| Thinking / reasoning tokens (retained-text estimate) | ≈241,702 |
| **Total tokens** | **296,203,952** |
| Cache hit rate (input tokens, estimated) | 94.84% |
| Estimated API cost (USD, prefix + five-minute heuristic) | **$62.99** |

Total input, output, and total tokens are summed from Pi assistant-message usage records and match both the archived proxy totals and the final cumulative goal token counter. All 613 usage-bearing records contain a zero reasoning field, despite retained thinking text in 304 assistant records; zero is not treated as a measurement of no reasoning. Input totals include repeated context across requests, not unique context tokens.

The reasoning estimate tokenizes 947,869 characters in 304 retained thinking blocks from 304 of the 613 usage-bearing responses, using the [published GLM 5.3 tokenizer](https://huggingface.co/zai-org/GLM-5.3/blob/aca966e4e02791568aa6a4ced368624b3d897f42/tokenizer.json) at revision `aca966e4e02791568aa6a4ced368624b3d897f42`, with `tokenizers 0.23.2`. Each block is encoded independently, without added special tokens, chat framing, or signatures; all 304 blocks round-trip back to the original text. This estimates retained text only, not complete reasoning usage or provider-billed tokens, and is not extrapolated to responses without retained text. It is not added to output, total tokens, or cost.

Pi records all 295,662,115 prompt tokens under `input`, with zero `cacheRead` and `cacheWrite`. The proxy log independently preserves only prompt, completion, and total counts, not the cache split or original usage payloads. The table uses reconstructed message prefixes plus an operator-specified five-minute idle-expiry rule; these are estimates, not provider cache measurements.

Reconstruct each successful request's preceding message history from the session parent chain. Compare canonical message blocks by content, retaining user text, assistant text and signed thinking, tool calls and results; convert persisted custom messages to user messages. Exclude errored/aborted assistant messages, usage/timestamp metadata, tool-result details, and goal-state bookkeeping. This follows the relevant [Pi message conversion](https://github.com/earendil-works/pi/blob/bfb004d4418ff05c6f909eaaab856cbe75c1fde0/packages/coding-agent/src/core/messages.ts) and [history filtering](https://github.com/earendil-works/pi/blob/bfb004d4418ff05c6f909eaaab856cbe75c1fde0/packages/ai/src/api/transform-messages.ts) rules. All 612 adjacent successful-request comparisons retain the entire previous reconstructed history as an exact message-level prefix. No compaction, branching, images, or unresolved tool calls requiring synthetic results occur in this run.

The first successful request is cold. For each later successful request, measure idle time on the host proxy clock, from the preceding successful response's usage-log timestamp to the current request's start. An interval strictly greater than 300 seconds makes the current input entirely non-cached; otherwise, because the full previous message prefix is retained, estimate cache reads as the preceding successful request's reported input tokens. The 16 HTTP 429 responses have no reported usage and do not refresh the assumed cache. This uses request-start time, not the end of the current response.

The 613 successful input counts match the proxy sequence exactly and grow from 2,792 to 702,541 tokens. The idle rule expires 24 times: 25 cold requests including the first, and 588 warm requests. It removes 14,545,723 cache-read tokens from the no-expiry estimate, leaving 280,413,851 estimated cache-read tokens and 15,248,264 non-cached tokens, a 94.84% estimated hit rate. The prefix check itself causes no reduction in this run; the five-minute rule accounts for the entire change.

This is a reconstructed-message estimate, not a token-by-token comparison of final provider requests. Historical system prompts, tool definitions, final post-hook payloads, and provider rendering are not retained; treating the preceding reported input count as the reusable token prefix assumes those unobserved components remain stable and introduce no earlier prefix changes. Five minutes is an operator-defined idle TTL, not a verified GLM provider policy. Actual cache routing, eviction, minimum cacheable lengths, and separate cache writes remain unmeasured.

Conditional pricing uses the non-batch model-catalog rates for `z-ai/glm-5.3` in [OpenRouter's Models API](https://openrouter.ai/api/v1/models), retrieved on 2026-09-06. [GLM endpoint pricing](https://openrouter.ai/api/v1/models/z-ai/glm-5.3/endpoints) lists multiple provider prices; cheaper endpoints are not substituted for the catalog basis. These are current-list-price assumptions, not evidence of the run-date price, actual endpoint routing, or a provider invoice; session cost fields are not used.

| Token class | USD / million tokens | Estimated cost (USD) |
|---|---:|---:|
| Non-cached input (estimated) | $1.40 | $21.347570 |
| Output | $4.40 | $2.384083 |
| Cache read (estimated) | $0.14 | $39.257939 |

[OpenRouter's Z.AI caching rules](https://openrouter.ai/docs/guides/best-practices/prompt-caching#zai) list no separate write/storage fee at the retrieval date, and the catalog lists no long-context override. Using the estimated cache split, the unrounded cost is `(15,248,264 × 1.40 + 280,413,851 × 0.14 + 541,837 × 4.40) / 1,000,000 = $62.98959154`, rounded once to **$62.99**. This is an operator-defined prefix-and-expiry estimate, not a recovered bill. Unreported usage on failed requests remains unknown.

Generation time, source size, tokens, activity, and cost are descriptive metadata, not correctness scores.

## Results

<!-- core-results:begin -->

### POSIX correctness

| Profile | Classification | Pass | Expected fail | Fail | Unexpected pass | Timeout | Skip N/A | Skip impl | Pass rate | Category macro average |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `spec-tests` | `valid` | 44 | 0 | 0 | 0 | 0 | 7 | 0 | 100.00% | 100.00% |
| `pjdfstest-core` | `diagnostic` | 169 | 0 | 0 | 0 | 0 | 48 | 0 | 100.00% | 100.00% |
| `xfstests-core` | `valid` | 182 | 1 | 5 | 0 | 6 | 473 | 0 | 94.33% | 89.17% |

`spec-tests` run: `eval-core-glm-5.3`

`pjdfstest-core` run: `case-correction-core-glm-5.3-pjdfstest-core`

`xfstests-core` run: `eval-core-glm-5.3`

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

Run: `eval-submission-robustness-v2-core-glm-5-3-r3`

### On-disk format conformance

Profile: `canonical-format`

Outcome: `candidate-failed`; classification: `diagnostic`.

| Metric | Value |
|---|---:|
| Total | 479 |
| Pass | 417 |
| Fail | 62 |
| Timeout | 0 |
| Pass rate | 87.06% |
| Category macro average | 94.01% |
| Conformance score (0–100) | 94.05 |
| Full conformance | no |

Run: `core-canonical-glm-oracle3-r5`
Execution backend: `hyperv`

### Crash consistency

Profile: `crash-core`

Outcome: `candidate-failed`; classification: `diagnostic`.

| Metric | Value |
|---|---:|
| Total | 216 |
| Pass | 215 |
| Fail | 1 |
| Timeout | 0 |
| Pass rate | 99.54% |
| Category macro average | 99.54% |

Run: `case-correction-core-glm-5.3-crash-core`

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
| Category macro average | 83.33% |

Run: `core-realworld-glm-20260925-r1`
Execution backend: `hyperv`

### CPU performance

Profile: `perf-cpu`

Outcome: `pass`; classification: `diagnostic`.

Performance score: **43.58** (comparison pool `hyperv-fixed-vhdx`).

| Family | Cache | Primary metric | Samples | Median | MAD | CV | Candidate-failed samples |
|---|---|---|---:|---:|---:|---:|---:|
| buffered-synchronized-writes | cold | operations_per_second | 3 | 1,611.7 | 1.4 | 0.1% | 0 |
| buffered-synchronized-writes | warm | operations_per_second | 3 | 1,599.4 | 2.9 | 0.4% | 0 |
| bulk-sequential-io | cold | MiB/s | 5 | 60.9 | 0.1 | 0.5% | 0 |
| bulk-sequential-io | warm | MiB/s | 5 | 61.1 | 0.5 | 0.7% | 0 |
| metadata-concurrency | cold | operations_per_second | 2 | 1,143.9 | 2.0 | 0.2% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 5,564.7 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 775.9 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 2 | 1,287.0 | 0.7 | 0.1% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 822.5 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 2 | 1,021.5 | 5.6 | 0.5% | 0 |
| metadata-concurrency | cold | operations_per_second | 3 | 421.3 | 2.1 | 0.8% | 0 |
| metadata-concurrency | warm | operations_per_second | 2 | 1,141.6 | 1.3 | 0.1% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 5,606.5 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 770.9 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 1,285.6 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 825.2 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 2 | 1,004.3 | 3.5 | 0.4% | 0 |
| metadata-concurrency | warm | operations_per_second | 3 | 420.4 | 0.8 | 0.8% | 0 |
| small-random-io | cold | iops | 3 | 2,885.0 | 2.7 | 0.1% | 0 |
| small-random-io | cold | iops | 5 | 3,258.6 | 648.8 | 43.5% | 0 |
| small-random-io | cold | iops | 3 | 7,023.8 | 36.3 | 0.6% | 0 |
| small-random-io | warm | iops | 3 | 2,886.1 | 9.6 | 0.3% | 0 |
| small-random-io | warm | iops | 5 | 2,999.3 | 398.8 | 37.7% | 0 |
| small-random-io | warm | iops | 5 | 7,025.9 | 15.9 | 0.4% | 0 |

Run: `perf-v2-core-glm-5-3-perf-cpu-r1`
Execution backend: `hyperv`

### NVMe performance

Profile: `perf-nvme`

Outcome: `pass`; classification: `diagnostic`.

Performance score: **41.70** (comparison pool `hyperv-fixed-vhdx`).

| Family | Cache | Primary metric | Samples | Median | MAD | CV | Candidate-failed samples |
|---|---|---|---:|---:|---:|---:|---:|
| buffered-synchronized-writes | cold | operations_per_second | 3 | 896.5 | 7.7 | 1.0% | 0 |
| buffered-synchronized-writes | warm | operations_per_second | 3 | 902.5 | 0.4 | 0.3% | 0 |
| bulk-sequential-io | cold | MiB/s | 5 | 28.6 | 0.2 | 0.8% | 0 |
| bulk-sequential-io | warm | MiB/s | 5 | 28.8 | 0.1 | 0.5% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 673.6 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 5,531.9 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 523.0 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 733.2 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 422.3 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 444.1 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 3 | 263.8 | 1.2 | 0.7% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 658.7 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 5,670.1 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 521.8 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 734.7 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 458.0 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 441.2 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 3 | 261.9 | 0.5 | 0.9% | 0 |
| small-random-io | cold | iops | 3 | 1,507.0 | 0.3 | 0.1% | 0 |
| small-random-io | cold | iops | 4 | 2,497.7 | 694.4 | 28.3% | 0 |
| small-random-io | cold | iops | 4 | 2,907.0 | 140.6 | 6.2% | 0 |
| small-random-io | warm | iops | 3 | 1,505.6 | 0.8 | 0.2% | 0 |
| small-random-io | warm | iops | 4 | 1,369.7 | 83.2 | 21.9% | 0 |
| small-random-io | warm | iops | 4 | 3,099.2 | 49.9 | 2.0% | 0 |

Run: `perf-v2-core-glm-5-3-perf-nvme-r1`
Execution backend: `hyperv`

### Agent code review

S: **27.47** / 100; status: `complete`; purpose: `diagnostic`.

| Critical | High | Medium | Low |
|---:|---:|---:|---:|
| 1 | 6 | 25 | 9 |

Counts include confirmed, deduplicated root causes; S contributes 20% of the provisional total.

Scoring rule: `agent-review-score-v2`.

## Scope and provenance

Original profile results remain independent; the index adds a derived provisional total with missing scores imputed as zero. Each profile lists its run and, where recorded, its execution backend; different backends are not treated as identical execution conditions.

`valid` means the profile evidence is valid, not that all cases passed or release eligibility is established. `candidate-failed` is not an infrastructure error. `diagnostic` is diagnostic-only; `noneligible` cannot enter rankings. Hyper-V performance is not release- or ranking-eligible; protocol-v2 Hyper-V runs on the fixed VHDX/memory devices form their own `hyperv-fixed-vhdx` pool, counted in the provisional total and not comparable with KVM/raw-NVMe scores.

Maintainability (`maint-v3-policy`, 5% of the overall score): **82.30** / 100; components are in the README. Reference is a separate calibration baseline, not a model-generation entry.
<!-- core-results:end -->
