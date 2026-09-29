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

No publishable evaluation result yet (`not-run`), not a measured zero; the index provisionally imputes zero.

### NVMe performance

Profile: `perf-nvme`

No publishable evaluation result yet (`not-run`), not a measured zero; the index provisionally imputes zero.

## Scope and provenance

[JSON](../data/core/qwen-3.8-flash-next-high.json)

Original profile results remain independent; the index adds a derived provisional total with missing scores imputed as zero. JSON retains per-profile run, manifest, report, candidate, and fixed-surface identities, plus the execution backend where recorded; different backends are not treated as identical execution conditions.

`valid` means the profile evidence is valid, not that all cases passed or release eligibility is established. `candidate-failed` is not an infrastructure error. `diagnostic` is diagnostic-only; `noneligible` cannot enter rankings. Hyper-V performance is always diagnostic.

Security-review and maintainability scores are not assessed. Reference is a separate calibration baseline, not a model-generation entry.
<!-- core-results:end -->
