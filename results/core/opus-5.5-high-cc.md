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

<!-- source-tree:begin -->
### Directory structure

Every file under `bench/`, excluding build output. Numbers are code lines counted as above; "test" is the part of them in test code.

```text
bench/
├── src/
│   ├── btree/
│   │   ├── mod.rs  (8 code)
│   │   ├── node.rs  (362 code)
│   │   ├── ops.rs  (554 code)
│   │   └── walk.rs  (97 code)
│   ├── format/
│   │   ├── geometry.rs  (284 code)
│   │   ├── inode.rs  (313 code)
│   │   ├── meta.rs  (130 code)
│   │   ├── mod.rs  (182 code, 29 test)
│   │   └── records.rs  (155 code)
│   ├── fs/
│   │   ├── data/
│   │   │   ├── mod.rs  (107 code)
│   │   │   ├── range.rs  (288 code)
│   │   │   ├── read.rs  (132 code)
│   │   │   └── write.rs  (328 code)
│   │   ├── handles.rs  (179 code)
│   │   ├── mod.rs  (487 code)
│   │   ├── namespace.rs  (697 code)
│   │   ├── ops.rs  (551 code)
│   │   └── xattr.rs  (285 code)
│   ├── alloc.rs  (254 code)
│   ├── bin_fsck.rs  (37 code)
│   ├── cache.rs  (253 code)
│   ├── check.rs  (427 code)
│   ├── crc.rs  (93 code, 18 test)
│   ├── dev.rs  (109 code)
│   ├── devlock.rs  (22 code)
│   ├── extmap.rs  (259 code)
│   ├── image.rs  (128 code)
│   ├── lib.rs  (14 code)
│   ├── main.rs  (89 code)
│   ├── mkfs.rs  (70 code)
│   ├── prof.rs  (49 code)
│   └── store.rs  (243 code)
├── tests/
│   ├── common/
│   │   └── mod.rs  (65 code, 65 test)
│   ├── basic.rs  (408 code, 408 test)
│   ├── btree.rs  (138 code, 138 test)
│   ├── concurrent.rs  (172 code, 172 test)
│   ├── corrupt.rs  (138 code, 138 test)
│   ├── crash.rs  (220 code, 220 test)
│   ├── faults.rs  (184 code, 184 test)
│   ├── model.rs  (713 code, 713 test)
│   └── spec.rs  (542 code, 542 test)
├── Cargo.toml
└── README.md
```
<!-- source-tree:end -->

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

Outcome: `pass`; classification: `diagnostic`.


#### buffered-synchronized-writes / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 3618.6 |
| MAD | 13.400000000000091 |
| CV | 0.003380303467982015 |
| Candidate-failed samples | 0 |

#### buffered-synchronized-writes / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 3625.3 |
| MAD | 4.400000000000091 |
| CV | 0.0017480489346214036 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / cold

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 5 |
| Median | 400895286.0 |
| MAD | 2620926.0 |
| CV | 0.00878331771992901 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / warm

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 5 |
| Median | 400121375.0 |
| MAD | 863561.0 |
| CV | 0.00443640982116473 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 2829.1222054652103 |
| MAD | 3.601067760503156 |
| CV | 0.001816196599441849 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 8755.74934263318 |
| MAD | 26.757748270829325 |
| CV | 0.004214432043626846 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 2264.031774246933 |
| MAD | 0.7171466160084492 |
| CV | 0.0009761831989363384 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 3923.6519573906435 |
| MAD | 9.680574509797225 |
| CV | 0.0035345925121552935 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 4617.596144149114 |
| MAD | 11.865968244625947 |
| CV | 0.00277690493177315 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 1648.8806466346969 |
| MAD | 2.806177142264687 |
| CV | 0.006068023530224995 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 5 |
| Median | 438.6819893216323 |
| MAD | 5.223205668311607 |
| CV | 0.05016474054499573 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 2835.271016622115 |
| MAD | 8.680825769275089 |
| CV | 0.005713893956555655 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 8927.112996556418 |
| MAD | 54.81036388480061 |
| CV | 0.005524096301938361 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 2262.3948322312085 |
| MAD | 0.5076082845889687 |
| CV | 0.00023125139984904434 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 3916.199957306097 |
| MAD | 5.458935006055071 |
| CV | 0.00198214469068188 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 4620.995104170442 |
| MAD | 7.71175917356959 |
| CV | 0.003820162944504548 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 1646.2358427298088 |
| MAD | 0.08498204557849931 |
| CV | 0.0026189418123480548 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 4 |
| Median | 447.41609930147996 |
| MAD | 4.122819089709651 |
| CV | 0.023632980916288123 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 3 |
| Median | 5662.633737 |
| MAD | 22.097789999999804 |
| CV | 0.003893438080372834 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 10987.80122 |
| MAD | 68.09319000000141 |
| CV | 0.01076635394813121 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 11604.739526 |
| MAD | 59.094091000000844 |
| CV | 0.017032623210413596 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 3 |
| Median | 5690.230976999999 |
| MAD | 8.899109999999382 |
| CV | 0.002687477289404754 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 10954.074399000001 |
| MAD | 78.8891820000008 |
| CV | 0.01125765590321533 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 11554.89673 |
| MAD | 64.6413160000011 |
| CV | 0.007075424600643035 |
| Candidate-failed samples | 0 |

Run: `perf-v2-core-opus-5-5-cc-perf-cpu-r2`
Execution backend: `hyperv`

### NVMe performance

Profile: `perf-nvme`

Outcome: `pass`; classification: `diagnostic`.


#### buffered-synchronized-writes / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 2264.0 |
| MAD | 4.0 |
| CV | 0.002204865873172623 |
| Candidate-failed samples | 0 |

#### buffered-synchronized-writes / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 2251.6741629185403 |
| MAD | 3.4237978971336815 |
| CV | 0.0018235963250297315 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / cold

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 5 |
| Median | 287751873.0 |
| MAD | 1629737.0 |
| CV | 0.012840603627522948 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / warm

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 5 |
| Median | 289862185.0 |
| MAD | 628798.0 |
| CV | 0.013529575468760015 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 2 |
| Median | 2755.1179473169786 |
| MAD | 39.850866624562286 |
| CV | 0.01446430511745253 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 8592.134308616944 |
| MAD | 5.050428554426617 |
| CV | 0.007079959435793175 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 2224.7416558769655 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 2 |
| Median | 3850.318827632992 |
| MAD | 33.986953004407724 |
| CV | 0.00882704901227658 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 2 |
| Median | 4367.694712582444 |
| MAD | 7.614860532204148 |
| CV | 0.0017434507293440811 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 452.63499123987947 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 347.31780697701737 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 2 |
| Median | 2797.736420668325 |
| MAD | 5.256001812823797 |
| CV | 0.0018786622549554685 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 8881.513176666505 |
| MAD | 49.12031763644154 |
| CV | 0.009347800283119006 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 2227.1549718240726 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 2 |
| Median | 3856.889736523633 |
| MAD | 9.433909838245654 |
| CV | 0.002445988991831747 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 4334.721651409519 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 463.4447691295564 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 344.8505167843632 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 4 |
| Median | 3252.2747719999998 |
| MAD | 9.749024999999847 |
| CV | 0.005809182134598832 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 4 |
| Median | 5058.464223 |
| MAD | 38.30277950000027 |
| CV | 0.01910468502712074 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 4 |
| Median | 3348.4352595 |
| MAD | 6.2770009999999274 |
| CV | 0.0065078852017453155 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 3 |
| Median | 3267.673232 |
| MAD | 11.198879999999917 |
| CV | 0.003377615414767482 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 4 |
| Median | 5080.2457595 |
| MAD | 45.345465500000046 |
| CV | 0.015068858719949242 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 3 |
| Median | 3296.4688180000003 |
| MAD | 17.12428200000022 |
| CV | 0.008517384874835006 |
| Candidate-failed samples | 0 |

Run: `perf-v2-core-opus-5-5-cc-perf-nvme-r2`
Execution backend: `hyperv`

## Scope and provenance

Original profile results remain independent; the index adds a derived provisional total with missing scores imputed as zero. Each profile lists its run and, where recorded, its execution backend; different backends are not treated as identical execution conditions.

`valid` means the profile evidence is valid, not that all cases passed or release eligibility is established. `candidate-failed` is not an infrastructure error. `diagnostic` is diagnostic-only; `noneligible` cannot enter rankings. Hyper-V performance is not release- or ranking-eligible; protocol-v2 Hyper-V runs on the fixed VHDX/memory devices form their own `hyperv-fixed-vhdx` pool, counted in the provisional total and not comparable with KVM/raw-NVMe scores.

Security-review and maintainability scores are not assessed. Reference is a separate calibration baseline, not a model-generation entry.
<!-- core-results:end -->
