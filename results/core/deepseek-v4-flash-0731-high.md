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

No publishable evaluation result yet (`not-run`), not a measured zero; the index provisionally imputes zero.

### NVMe performance

Profile: `perf-nvme`

No publishable evaluation result yet (`not-run`), not a measured zero; the index provisionally imputes zero.

## Scope and provenance

Original profile results remain independent; the index adds a derived provisional total with missing scores imputed as zero. Each profile lists its run and, where recorded, its execution backend; different backends are not treated as identical execution conditions.

`valid` means the profile evidence is valid, not that all cases passed or release eligibility is established. `candidate-failed` is not an infrastructure error. `diagnostic` is diagnostic-only; `noneligible` cannot enter rankings. Hyper-V performance is always diagnostic.

Security-review and maintainability scores are not assessed. Reference is a separate calibration baseline, not a model-generation entry.
<!-- core-results:end -->
