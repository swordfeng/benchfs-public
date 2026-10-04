# Fable 5.1 — Core — High — Claude Code (`-cc`)

[中文](fable-5.1-high-cc.zh-CN.md)

This page records a second Fable 5.1 Core generation run for comparison with [Fable 5.1 under the Pi harness](fable-5.1-high.md). All eight non-performance profiles are complete: SDK, 51-case Spec, PJD, xfstests, robustness, canonical-format, crash-core and real-world.

## Run

| Field | Value |
|---|---|
| Model | `claude-fable-5-1` |
| Effort | High (`--effort high`) |
| Variant | Core |
| Agent harness | Claude Code 2.1.283, minimized (see below) |
| Generation runs | 1 (`n=1`) |
| Generation date | 2026-09-27 (UTC) |
| Evaluation | All eight non-performance profiles complete and independently validated |

**Harness difference.** This run used Claude Code instead of the Pi harness, through a Claude subscription, so it is not directly comparable with Pi-harness reports; comparing it with the Pi run of the same model is its purpose. The task surface was unchanged: the same Dev VM image, agent-visible documents, and prompt text. The harness was reduced to the `Bash`, `Read`, `Edit`, and `Write` tools, with no subagents, MCP servers, web tools, or memory. Its model traffic reached Anthropic through the environment's egress allowlist instead of the model proxy. The prompt was entered once as a `/goal` condition. Unlike Pi's goal extension, where the agent declares completion itself, Claude Code's `/goal` has a separate small model (Haiku 4.5) judge completion after each turn and passes its reason back when the condition is not yet met. Claude Code also adds its own system prompt, tool descriptions, and context reminders; with a subscription login these include the account's identity.

**Interruption.** The generation session was interrupted and resumed with `--continue` and one content-free `continue` message. The process-scope policy was changed on resumption, so the environment differs from earlier runs. The goal condition and all agent work before and after the interruption remain in one continuous transcript; the timing table below includes the interruption.

## Submission size

Only candidate-owned source under `bench/` is counted. The fixed SDK and FUSE adapter, dependencies, documentation, configuration, lockfiles, and generated artifacts are excluded.

| Component | Files containing this component | Physical lines | Code lines |
|---|---:|---:|---:|
| Implementation (Rust) | 11 | 7,106 | 6,353 |
| Agent-written tests and test support (Rust) | 5 | 867 | 810 |
| Qualification scripts | 0 | 0 | 0 |

Physical lines include blanks and comments. Code lines count each line holding at least one non-comment token: blank lines, comments, and documentation comments (Rust `///` and `//!`, Python docstrings) are excluded, while Rust attributes such as `#[derive(...)]` count as code. Tests comprise one dedicated file under `tests/` and inline `#[cfg(test)]` modules, including their attribute lines, in four implementation files (`src/bitmap.rs`, `src/btree.rs`, `src/cache.rs`, `src/format.rs`). Those four files overlap in the file counts; physical and code lines are partitioned without overlap. There are 12 distinct Rust files. These are source-size measurements, not test-coverage or code-quality scores.

<!-- source-tree:begin -->
### Directory structure

Every file under `bench/`, excluding build output. Numbers are code lines counted as above; "test" is the part of them in test code.

```text
bench/
├── src/
│   ├── bitmap.rs  (253 code, 48 test)
│   ├── btree.rs  (1,159 code, 234 test)
│   ├── cache.rs  (320 code, 50 test)
│   ├── check.rs  (473 code)
│   ├── core.rs  (1,061 code)
│   ├── format.rs  (1,084 code, 55 test)
│   ├── fs.rs  (1,912 code)
│   ├── lib.rs  (8 code)
│   ├── main.rs  (181 code)
│   ├── mkfs.rs  (75 code)
│   └── mount.rs  (214 code)
├── tests/
│   └── memfs.rs  (423 code, 423 test)
├── Cargo.toml
├── NOTES.md
└── README.md
```
<!-- source-tree:end -->

## Generation time and activity

| Metric | Value |
|---|---|
| Goal start | 2026-09-27 06:08:15.712 UTC |
| Goal completion | 2026-09-27 09:16:03.855 UTC |
| Goal active time | **2 h 59 min 12 s** |
| Elapsed time, including the interruption | 3 h 7 min 48 s |
| Harness interruption | 8 min 36 s (06:41:43.526 – 06:50:19.294 UTC) |
| Final goal status | Met (judged by the `/goal` evaluator) |
| Main-model API responses with usage | 127 |
| API error records | 0 |
| Agent tool calls | 173 |
| API time / tool execution time | 1 h 5 min 18 s / 1 h 54 min 14 s |

Active time is the elapsed time minus the interruption, measured from the last transcript record before the harness was terminated to the record written when the resumed session reported the interrupted call. There were no usage-limit pauses, API error records, or context compactions; Claude Code's session state attributes under one second of API time to retries. Besides the goal condition, the only operator input to the model was the `continue` message after the resume. API and tool times come from Claude Code's cumulative session state, which the resumed session carried over. Active time is harness-recorded wall time, not model inference time. A met goal does not imply that evaluation tests pass.

Response counts come from the archived Claude Code transcript, deduplicated by message ID because Claude Code writes one record per content block (348 assistant records for 127 responses). There is no model-proxy log for this harness. The environment's egress log shows the agent reaching only `crates.io` besides Anthropic. Connection attempts to `downloads.claude.ai` and `github.com` made by Claude Code at startup, before the goal was set and after it was met, were denied by the allowlist.

### Tool usage

| Tool | Requests | Recorded failures | Unknown outcomes |
|---|---:|---:|---:|
| `Bash` | 163 | 4 | 1 |
| `Read` | 10 | 0 | 0 |
| **Total** | **173** | **4** | **1** |

Each `tool_use` block counts as one request under its tool name. Results are matched by `tool_use_id`; `tool_result.is_error: true` counts as a recorded failure, except for the call that was running when the harness was terminated: Claude Code marks its result as an error but states that its outcome is unknown, so it is counted as unknown. Every call has a matching result. The agent wrote files through `Bash` and did not use `Edit` or `Write`. These are tool-level outcomes, not model API errors or evaluation test results.

## Token usage and API cost

Main model (`claude-fable-5-1`):

| Metric | Value |
|---|---:|
| Input tokens, excluding cache (`input_tokens`) | 3,704 |
| Cache-read input tokens (`cache_read_input_tokens`) | 49,120,546 |
| Cache-write input tokens (`cache_creation_input_tokens`) | 559,750 |
| Total input tokens, including cache | 49,684,000 |
| Output tokens, including thinking (`output_tokens`) | 340,562 |
| Thinking tokens | 135,351 |
| **Total tokens** | **50,024,562** |
| Cache hit rate (input tokens) | 98.87% |

Completion evaluator (`claude-haiku-4-5-20251001`, used by `/goal`): 3 input, 115,555 cache-write, 0 cache-read, and 345 output tokens; **115,903 tokens** in total.

| | Estimated API cost (USD) |
|---|---:|
| Main model | $40.54 |
| Completion evaluator | $0.15 |
| **Total** | **$40.69** |

Main-model tokens are summed from the transcript's per-response usage and match Claude Code's cumulative session totals exactly. The last session-state snapshot, written when the resumed session exited, reverted to the pre-interruption values; the totals use the largest cumulative snapshot. The evaluator does not appear in the transcript as responses, so its tokens come from the session totals only. Thinking tokens are a component of output and are not added again. Input totals include repeated context across requests; they are not the size of a single context or the number of unique tokens.

Cost uses the non-batch model-catalog rates for `anthropic/claude-fable-5.1` and `anthropic/claude-haiku-4.5` in [OpenRouter's Models API](https://openrouter.ai/api/v1/models), retrieved on 2026-09-27; the main-model rates are the same as those used for the Pi run. Rates below are USD per million tokens. This is a current-list-price estimate, not a charge: the run used a subscription.

| Token class (main model) | USD / million tokens | Estimated cost (USD) |
|---|---:|---:|
| Non-cached input | $10.00 | $0.037040 |
| Output | $50.00 | $17.028100 |
| Cache read | $0.25 | $12.280137 |
| Cache write, 5-minute TTL | $12.50 | $0.000000 |
| Cache write, 1-hour TTL | $20.00 | $11.195000 |

The unrounded main-model cost is $40.5402765. All 559,750 cache-write tokens are recorded as 1-hour writes (`ephemeral_1h_input_tokens`), so the 1-hour rate applies; this equals Claude Code's own recorded estimate. The Pi run's cache writes were 5-minute writes, so cache-write costs of the two runs are not directly comparable. The session totals do not split the evaluator's cache writes by TTL. At the 5-minute rate ($1.25 per million, with $1.00 input and $5.00 output) the evaluator costs $0.14617175, which equals Claude Code's recorded estimate; the 1-hour rate would give $0.232838. The total uses the 5-minute figure: $40.68644825, rounded once to **$40.69**. Token totals cover only reported usage.

Generation time, source size, tokens, activity, and cost are descriptive metadata, not correctness scores.

## Results

SDK records 54 passes; Spec records 44 passes and 7 N/A; xfstests records 187 passes, 1 expected failure, 3 failures, 3 timeouts and 473 N/A. PJD records 169 passes and 48 N/A. Robustness passed 47/47. Canonical-format completed 479 checkpoints: 431 passed and 48 failed. Its native conformance score is 95.45; checkpoint pass rate is not substituted for that score.

**Provisional total: 61.56/100; C: 97.45; R: 68.52; W: 83.33.** All non-performance inputs are measured. Performance, security review and maintainability remain unmeasured and zero-imputed under the existing scoring rule; this is not a complete six-dimension result.

<!-- core-results:begin -->

### Build and SDK checks

| Profile | Classification | Pass | Expected fail | Fail | Unexpected pass | Timeout | Skip N/A | Skip impl | Pass rate | Category macro average |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `sdk-smoke` | `diagnostic` | 54 | 0 | 0 | 0 | 0 | 0 | 0 | 100.00% | — |

`sdk-smoke` run: `cc-fable-sdk-20260927-r1`

### POSIX correctness

| Profile | Classification | Pass | Expected fail | Fail | Unexpected pass | Timeout | Skip N/A | Skip impl | Pass rate | Category macro average |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `spec-tests` | `diagnostic` | 44 | 0 | 0 | 0 | 0 | 7 | 0 | 100.00% | 100.00% |
| `pjdfstest-core` | `diagnostic` | 169 | 0 | 0 | 0 | 0 | 48 | 0 | 100.00% | 100.00% |
| `xfstests-core` | `diagnostic` | 187 | 1 | 3 | 0 | 3 | 473 | 0 | 96.91% | 94.36% |

`spec-tests` run: `cc-fable-spec-20260927-r1`

`pjdfstest-core` run: `case-correction-core-fable-5.1-cc-pjdfstest-core`

`xfstests-core` run: `cc-fable-xfs-20260927-r1`

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

Run: `cc-fable-robust-approved-r1`
Execution backend: `qemu-kvm`

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

Run: `cc-fable-canonical-approved-r1`
Execution backend: `qemu-kvm`

### Crash consistency

Profile: `crash-core`

Outcome: `candidate-failed`; classification: `valid`.

| Metric | Value |
|---|---:|
| Total | 216 |
| Pass | 80 |
| Fail | 136 |
| Timeout | 0 |
| Pass rate | 37.04% |
| Category macro average | 37.04% |

The table above summarizes whether cases meet all requirements. The table below scores each verified capability for robustness R.

| Check | Score / 100 | Requirements met | Requirements not met | Not verified (scores 0) |
|---|---:|---:|---:|---:|
| Disk structure | 100.00 | 216 | 0 | 0 |
| Data correctness | 100.00 | 216 | 0 | 0 |
| Unmount and reopen | 37.04 | 80 | 136 | 0 |

Crash score: **87.41**. Weights: disk structure 40%, data correctness 40%, unmount and reopen 20%.

Permitted safe rejections: 68. Two rounds tested: 12. Reopen tests not run after failure: 136. First round not tested: 0.

Each round includes mount, checks, and unmount. Safe rejection meets requirements but does not prove data recovery. Reopen not tested after an earlier failure does not count as a pass. See [scoring](../../docs/scoring.md) for the meaning of each check.

Run: `cc-fable-crash-approved-r2`
Execution backend: `qemu-kvm`

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

Run: `cc-fable-realworld-approved-r1`
Execution backend: `qemu-kvm`

### CPU performance

Profile: `perf-cpu`

Outcome: `pass`; classification: `diagnostic`.

Performance score: **66.27** (comparison pool `hyperv-fixed-vhdx`). Assessed budget: 100.0%. The range allowed by missing inputs is 66.27–66.27, not a confidence interval.

Components (out of 100): File I/O 55.07; Metadata 42.21; RAM 100.00; Write efficiency 99.67.
Peak RAM: 31.3 MiB. Metadata: 14 cells have valid rates, 0 candidate failures, 0 unmeasured.

Write amplification = controller write bytes / application logical write bytes in the timed window; it is not SSD-internal amplification. This ratio does not apply to metadata operations. A dash is not zero. Candidate failures score 0; unmeasured cells retain their budget and provisionally contribute 0. * marks an unassessed share.

| Workload | Cache | Samples | Rate | MAD | CV | Candidate-failed samples | Write amplification | Speed score / 100 |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| Sequential I/O | cold | 3 | 632.5 MiB/s | 0.4 | 0.1% | 0 | 1.000× | 54.80 |
| Sequential I/O | warm | 3 | 632.4 MiB/s | 0.7 | 0.2% | 0 | 1.000× | 54.89 |
| Random I/O, single thread | cold | 3 | 5,093.0 IOPS | 11.6 | 0.3% | 0 | 1.000× | 31.64 |
| Random I/O, single thread | warm | 3 | 5,088.6 IOPS | 3.9 | 0.1% | 0 | 1.000× | 31.56 |
| Random I/O, multiple threads | cold | 5 | 15,168.1 IOPS | 11.7 | 0.2% | 0 | 1.000× | 41.76 |
| Random I/O, multiple threads | warm | 5 | 15,116.7 IOPS | 15.3 | 0.2% | 0 | 1.000× | 41.69 |
| Random I/O, shared regions | cold | 5 | 10,482.9 IOPS | 53.6 | 0.6% | 0 | 1.000× | 45.62 |
| Random I/O, shared regions | warm | 5 | 10,515.0 IOPS | 14.0 | 0.4% | 0 | 1.000× | 45.65 |
| Synchronized overwrite | cold | 3 | 2,617.4 ops/s | 6.2 | 0.2% | 0 | 1.031× | 78.54 |
| Synchronized overwrite | warm | 3 | 2,623.6 ops/s | 9.9 | 0.5% | 0 | 1.031× | 78.56 |
| Create, stat and unlink | cold | 3 | 2,520.6 ops/s | 2.0 | 0.1% | 0 | — | 22.79 |
| Create, stat and unlink | warm | 3 | 2,515.0 ops/s | 5.0 | 0.2% | 0 | — | 23.41 |
| Deep and large-directory lookup | cold | 3 | 8,881.6 ops/s | 8.2 | 0.3% | 0 | — | 19.31 |
| Deep and large-directory lookup | warm | 3 | 8,923.8 ops/s | 8.0 | 0.2% | 0 | — | 19.76 |
| Same-directory rename | cold | 3 | 2,323.3 ops/s | 1.2 | 0.1% | 0 | — | 33.82 |
| Same-directory rename | warm | 3 | 2,317.1 ops/s | 1.6 | 0.2% | 0 | — | 33.44 |
| Same-directory concurrent mutation | cold | 3 | 3,943.7 ops/s | 9.8 | 0.3% | 0 | — | 31.41 |
| Same-directory concurrent mutation | warm | 3 | 3,940.6 ops/s | 5.6 | 0.2% | 0 | — | 31.91 |
| Concurrent creation in different directories | cold | 3 | 3,993.6 ops/s | 9.3 | 0.3% | 0 | — | 38.19 |
| Concurrent creation in different directories | warm | 3 | 3,991.4 ops/s | 12.1 | 0.3% | 0 | — | 38.02 |
| Parallel fsync | cold | 5 | 1,659.9 ops/s | 37.0 | 3.1% | 0 | — | 100.00 |
| Parallel fsync | warm | 3 | 1,697.4 ops/s | 13.5 | 0.8% | 0 | — | 100.00 |
| Reclamation near full capacity | cold | 4 | 574.2 ops/s | 6.1 | 1.9% | 0 | — | 58.96 |
| Reclamation near full capacity | warm | 3 | 573.5 ops/s | 1.0 | 1.4% | 0 | — | 57.80 |

Run: `perf-v2-core-fable-5-1-cc-perf-cpu-r2`
Execution backend: `hyperv`

### NVMe performance

Profile: `perf-nvme`

Outcome: `pass`; classification: `diagnostic`.

Performance score: **63.39** (comparison pool `hyperv-fixed-vhdx`). Assessed budget: 100.0%. The range allowed by missing inputs is 63.39–63.39, not a confidence interval.

Components (out of 100): File I/O 50.76; Metadata 38.72; RAM 100.00; Write efficiency 99.66.
Peak RAM: 36.2 MiB. Metadata: 14 cells have valid rates, 0 candidate failures, 0 unmeasured.

Write amplification = controller write bytes / application logical write bytes in the timed window; it is not SSD-internal amplification. This ratio does not apply to metadata operations. A dash is not zero. Candidate failures score 0; unmeasured cells retain their budget and provisionally contribute 0. * marks an unassessed share.

| Workload | Cache | Samples | Rate | MAD | CV | Candidate-failed samples | Write amplification | Speed score / 100 |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| Sequential I/O | cold | 4 | 323.8 MiB/s | 0.6 | 0.2% | 0 | 1.000× | 46.02 |
| Sequential I/O | warm | 4 | 327.4 MiB/s | 0.7 | 0.3% | 0 | 1.000× | 46.36 |
| Random I/O, single thread | cold | 3 | 3,377.3 IOPS | 18.9 | 0.7% | 0 | 1.000× | 32.72 |
| Random I/O, single thread | warm | 3 | 3,414.5 IOPS | 4.7 | 0.8% | 0 | 1.000× | 32.85 |
| Random I/O, multiple threads | cold | 3 | 8,226.8 IOPS | 2.2 | 0.1% | 0 | 1.000× | 38.01 |
| Random I/O, multiple threads | warm | 3 | 8,183.6 IOPS | 16.4 | 0.2% | 0 | 1.000× | 37.91 |
| Random I/O, shared regions | cold | 5 | 4,872.2 IOPS | 0.6 | 0.2% | 0 | 1.000× | 38.96 |
| Random I/O, shared regions | warm | 5 | 4,880.8 IOPS | 13.9 | 0.5% | 0 | 1.000× | 38.97 |
| Synchronized overwrite | cold | 3 | 1,796.7 ops/s | 1.0 | 0.3% | 0 | 1.031× | 75.53 |
| Synchronized overwrite | warm | 3 | 1,797.1 ops/s | 8.0 | 0.5% | 0 | 1.031× | 75.51 |
| Create, stat and unlink | cold | 1 | 2,213.6 ops/s | 0.0 | 0.0% | 0 | — | 19.97 |
| Create, stat and unlink | warm | 1 | 2,218.1 ops/s | 0.0 | 0.0% | 0 | — | 20.68 |
| Deep and large-directory lookup | cold | 1 | 8,849.3 ops/s | 0.0 | 0.0% | 0 | — | 19.23 |
| Deep and large-directory lookup | warm | 1 | 8,897.3 ops/s | 0.0 | 0.0% | 0 | — | 19.70 |
| Same-directory rename | cold | 1 | 2,333.3 ops/s | 0.0 | 0.0% | 0 | — | 33.92 |
| Same-directory rename | warm | 1 | 2,342.8 ops/s | 0.0 | 0.0% | 0 | — | 33.68 |
| Same-directory concurrent mutation | cold | 1 | 3,640.2 ops/s | 0.0 | 0.0% | 0 | — | 29.67 |
| Same-directory concurrent mutation | warm | 1 | 3,779.9 ops/s | 0.0 | 0.0% | 0 | — | 31.00 |
| Concurrent creation in different directories | cold | 1 | 3,309.6 ops/s | 0.0 | 0.0% | 0 | — | 34.11 |
| Concurrent creation in different directories | warm | 1 | 3,275.7 ops/s | 0.0 | 0.0% | 0 | — | 33.73 |
| Parallel fsync | cold | 1 | 566.8 ops/s | 0.0 | 0.0% | 0 | — | 90.81 |
| Parallel fsync | warm | 1 | 630.0 ops/s | 0.0 | 0.0% | 0 | — | 92.99 |
| Reclamation near full capacity | cold | 1 | 376.5 ops/s | 0.0 | 0.0% | 0 | — | 49.80 |
| Reclamation near full capacity | warm | 1 | 380.0 ops/s | 0.0 | 0.0% | 0 | — | 48.86 |

Run: `perf-v2-core-fable-5-1-cc-perf-nvme-r1`
Execution backend: `hyperv`

### Agent code review

S: **44.84** / 100; status: `complete`; purpose: `diagnostic`.

| Critical | High | Medium | Low |
|---:|---:|---:|---:|
| 1 | 0 | 16 | 3 |

Counts include confirmed, deduplicated root causes; S contributes 20% of the provisional total.

Scoring rule: `agent-review-score-v2`.

## Scope and provenance

Original profile results remain independent; the index adds a derived provisional total with missing scores imputed as zero. Each profile lists its run and, where recorded, its execution backend; different backends are not treated as identical execution conditions.

`valid` means the profile evidence is valid, not that all cases passed or release eligibility is established. `candidate-failed` is not an infrastructure error. `diagnostic` is diagnostic-only; `noneligible` cannot enter rankings. Hyper-V performance is not release- or ranking-eligible; protocol-v2 Hyper-V runs on the fixed VHDX/memory devices form their own `hyperv-fixed-vhdx` pool, counted in the provisional total and not comparable with KVM/raw-NVMe scores.

Maintainability (`maint-v3-policy`, 5% of the overall score): **87.02** / 100; components are in the README. Reference is a separate calibration baseline, not a model-generation entry.
<!-- core-results:end -->
