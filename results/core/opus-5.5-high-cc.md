# Opus 5.5 — Core — High — Claude Code (`-cc`)

[中文](opus-5.5-high-cc.zh-CN.md)

This page records the Opus 5.5 Core generation run and eight independently verified non-performance profile results. The separate run and binary identities are retained below. Performance, code review and maintainability remain unmeasured.

## Run

| Field | Value |
|---|---|
| Model | `claude-opus-5-5` |
| Effort | High (`--effort high`) |
| Variant | Core |
| Agent harness | Claude Code 2.1.283, minimized (see below) |
| Generation runs | 1 (`n=1`) |
| Generation date | 2026-09-27 (UTC) |
| Evaluation | Eight non-performance profile results independently verified |

**Harness difference.** This run used Claude Code instead of the Pi harness used by the other Core reports, because the model was accessed through a Claude subscription. Results are therefore not directly comparable with Pi-harness reports. The task surface was unchanged: the same Dev VM image, agent-visible documents, and prompt text. The harness was reduced to the `Bash`, `Read`, `Edit`, and `Write` tools, with no subagents, MCP servers, web tools, or memory. Its model traffic reached Anthropic through the environment's egress allowlist instead of the model proxy. The prompt was entered once as a `/goal` condition. Unlike Pi's goal extension, where the agent declares completion itself, Claude Code's `/goal` has a separate small model (Haiku 4.5) judge completion after each turn and passes its reason back when the condition is not yet met. Claude Code also adds its own system prompt, tool descriptions, and context reminders; with a subscription login these include the account's identity.

## Submission size

Only candidate-owned source under `bench/` is counted. The fixed SDK and FUSE adapter, dependencies, documentation, configuration, lockfiles, and generated artifacts are excluded.

| Component | Files containing this component | Physical lines | Code lines |
|---|---:|---:|---:|
| Implementation (Rust) | 32 | 8,318 | 7,139 |
| Agent-written tests and test support (Rust) | 11 | 2,792 | 2,627 |
| Qualification scripts | 0 | 0 | 0 |

Physical lines include blanks and comments. Code lines count each line holding at least one non-comment token: blank lines, comments, and documentation comments (Rust `///` and `//!`, Python docstrings) are excluded, while Rust attributes such as `#[derive(...)]` count as code. Tests comprise nine dedicated files under `tests/` (one of them a shared helper module) and inline `#[cfg(test)]` modules, including their attribute lines, in two implementation files (`src/crc.rs`, `src/format/mod.rs`). Those two files overlap in the file counts; physical and code lines are partitioned without overlap. There are 41 distinct Rust files. These are source-size measurements, not test-coverage or code-quality scores.

## Generation time and activity

| Metric | Value |
|---|---|
| Goal start | 2026-09-27 01:01:54.452 UTC |
| Goal completion | 2026-09-27 04:09:03.143 UTC |
| Goal active time | **3 h 7 min 9 s** |
| Elapsed time, including gaps between active periods | 3 h 7 min 9 s |
| Final goal status | Met (judged by the `/goal` evaluator) |
| Main-model API responses with usage | 249 |
| Completion-evaluator verdicts | 1 |
| API error records | 0 |
| Agent tool calls | 256 |
| API time / tool execution time | 1 h 10 min 14 s / 1 h 56 min 51 s |

The goal ran as one continuous turn: Claude Code's recorded turn duration is 11,228.721 seconds, matching the interval between setting the goal and the evaluator's verdict. There were no usage-limit pauses, API error records, or context compactions; Claude Code's session state attributes 1.1 seconds of API time to retries. The only operator input to the model was the `/goal` condition. API and tool times come from Claude Code's cumulative session state. Active time is harness-recorded wall time, not model inference time. A met goal does not imply that evaluation tests pass.

Response counts come from the archived Claude Code transcript, deduplicated by message ID because Claude Code writes one record per content block (576 assistant records for 249 responses). There is no model-proxy log for this harness. The environment's egress log shows the agent reaching only `crates.io` besides Anthropic. Two connection attempts at Claude Code startup, to `downloads.claude.ai` and `github.com`, were denied by the allowlist before the goal was set.

### Tool usage

| Tool | Requests | Recorded failures | Unknown outcomes |
|---|---:|---:|---:|
| `Bash` | 229 | 7 | 0 |
| `Edit` | 1 | 0 | 0 |
| `Read` | 7 | 0 | 0 |
| `Write` | 19 | 0 | 0 |
| **Total** | **256** | **7** | **0** |

Each `tool_use` block counts as one request under its tool name. Results are matched by `tool_use_id`; only `tool_result.is_error: true` counts as a recorded failure. Every call has a matching result. These are tool-level outcomes, not model API errors or evaluation test results.

## Token usage and API cost

Main model (`claude-opus-5-5`):

| Metric | Value |
|---|---:|
| Input tokens, excluding cache (`input_tokens`) | 498 |
| Cache-read input tokens (`cache_read_input_tokens`) | 119,028,042 |
| Cache-write input tokens (`cache_creation_input_tokens`) | 733,087 |
| Total input tokens, including cache | 119,761,627 |
| Output tokens, including thinking (`output_tokens`) | 472,273 |
| Thinking tokens | 211,358 |
| **Total tokens** | **120,233,900** |
| Cache hit rate (input tokens) | 99.39% |

Completion evaluator (`claude-haiku-4-5-20251001`, used by `/goal`): 3 input, 131,770 cache-write, 0 cache-read, and 211 output tokens; **131,984 tokens** in total.

| | Estimated API cost (USD) |
|---|---:|
| Main model | $39.12 |
| Completion evaluator | $0.17 |
| **Total** | **$39.28** |

Main-model tokens are summed from the transcript's per-response usage and match Claude Code's cumulative session totals exactly. The evaluator does not appear in the transcript as responses, so its tokens come from the session totals only. Thinking tokens are a component of output and are not added again. Input totals include repeated context across requests; they are not the size of a single context or the number of unique tokens.

Cost uses the non-batch model-catalog rates for `anthropic/claude-opus-5.5` and `anthropic/claude-haiku-4.5` in [OpenRouter's Models API](https://openrouter.ai/api/v1/models), retrieved on 2026-09-27. Rates below are USD per million tokens. This is a current-list-price estimate, not a charge: the run used a subscription.

| Token class (main model) | USD / million tokens | Estimated cost (USD) |
|---|---:|---:|
| Non-cached input | $4.00 | $0.001992 |
| Output | $20.00 | $9.445460 |
| Cache read | $0.20 | $23.805608 |
| Cache write, 5-minute TTL | $5.00 | $0.000000 |
| Cache write, 1-hour TTL | $8.00 | $5.864696 |

The unrounded main-model cost is $39.1177564. All 733,087 cache-write tokens are recorded as 1-hour writes (`ephemeral_1h_input_tokens`), so the 1-hour rate applies; this equals Claude Code's own recorded estimate. The session totals do not split the evaluator's cache writes by TTL. At the 5-minute rate ($1.25 per million, with $1.00 input and $5.00 output) the evaluator costs $0.1657705, which equals Claude Code's recorded estimate; the 1-hour rate would give $0.2645980. The total uses the 5-minute figure: $39.2835269, rounded once to **$39.28**. Token totals cover only reported usage.

Generation time, source size, tokens, activity, and cost are descriptive metadata, not correctness scores.

## Results

The eight historical profiles used identical candidate-binary, Cargo.lock and fixed-surface hashes.

xfstests records 188 passes, 1 expected failure, 3 failures, 2 timeouts and 473 N/A. Canonical-format records 479 checkpoint passes. Crash-core records 216 passes.

Complete environment-audit coverage is not established for the historical SDK, Spec, PJD, xfstests and robustness profiles.

<!-- core-results:begin -->

### Build and SDK checks

| Profile | Classification | Pass | Expected fail | Fail | Unexpected pass | Timeout | Skip N/A | Skip impl | Pass rate | Category macro average |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `sdk-smoke` | `diagnostic` | 59 | 0 | 0 | 0 | 0 | 0 | 0 | 100.00% | — |

`sdk-smoke` run: `cc-opus-sdk-20260927-r1`

### POSIX correctness

| Profile | Classification | Pass | Expected fail | Fail | Unexpected pass | Timeout | Skip N/A | Skip impl | Pass rate | Category macro average |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `spec-tests` | `diagnostic` | 44 | 0 | 0 | 0 | 0 | 7 | 0 | 100.00% | 100.00% |
| `pjdfstest-core` | `diagnostic` | 169 | 0 | 0 | 0 | 0 | 48 | 0 | 100.00% | 100.00% |
| `xfstests-core` | `diagnostic` | 188 | 1 | 3 | 0 | 2 | 473 | 0 | 97.42% | 98.90% |

`spec-tests` run: `cc-opus-spec-20260927-r1`

`pjdfstest-core` run: `case-correction-core-opus-5.5-cc-pjdfstest-core`

`xfstests-core` run: `cc-opus-xfs-20260927-r1`

### Robustness

Profile: `robustness`

Outcome: `pass`; classification: `diagnostic`.

| Metric | Value |
|---|---:|
| Total | 47 |
| Pass | 47 |
| Fail | 0 |
| Timeout | 0 |
| Pass rate | 100.00% |
| Category macro average | 100.00% |

Timeouts count as failures; N/A is excluded; the macro average weights categories equally.

Run: `cc-opus-robust-20260927-r1`
Execution backend: `hyperv`

### On-disk format conformance

Profile: `canonical-format`

Outcome: `pass`; classification: `diagnostic`.

| Metric | Value |
|---|---:|
| Total | 479 |
| Pass | 479 |
| Fail | 0 |
| Timeout | 0 |
| Pass rate | 100.00% |
| Category macro average | 100.00% |
| Conformance score (0–100) | 100.00 |
| Full conformance | yes |

Run: `cc-opus-canonical-20260927-r1`
Execution backend: `hyperv`

### Crash consistency

Profile: `crash-core`

Outcome: `pass`; classification: `diagnostic`.

| Metric | Value |
|---|---:|
| Total | 216 |
| Pass | 216 |
| Fail | 0 |
| Timeout | 0 |
| Pass rate | 100.00% |
| Category macro average | 100.00% |

Run: `cc-opus-crash-oracle6-r2`
Execution backend: `hyperv`

### Real-world applications

Profile: `real-world`

Outcome: `pass`; classification: `diagnostic`.

| Metric | Value |
|---|---:|
| Total | 4 |
| Pass | 4 |
| Fail | 0 |
| Timeout | 0 |
| Pass rate | 100.00% |
| Category macro average | 100.00% |

Run: `cc-opus-realworld-20260927-r1`
Execution backend: `hyperv`

### CPU performance

Profile: `perf-cpu`

No publishable evaluation result yet (`not-run`), not a measured zero; the index provisionally imputes zero.

### NVMe performance

Profile: `perf-nvme`

No publishable evaluation result yet (`not-run`), not a measured zero; the index provisionally imputes zero.

## Scope and provenance

[JSON](../data/core/opus-5.5-high-cc.json)

Original profile results remain independent; the index adds a derived provisional total with missing scores imputed as zero. JSON retains per-profile run, manifest, report, candidate, and fixed-surface identities, plus the execution backend where recorded; different backends are not treated as identical execution conditions.

`valid` means the profile evidence is valid, not that all cases passed or release eligibility is established. `candidate-failed` is not an infrastructure error. `diagnostic` is diagnostic-only; `noneligible` cannot enter rankings. Hyper-V performance is always diagnostic.

Security-review and maintainability scores are not assessed. Reference is a separate calibration baseline, not a model-generation entry.
<!-- core-results:end -->
