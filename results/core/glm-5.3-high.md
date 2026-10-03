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


#### buffered-synchronized-writes / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 1611.7 |
| MAD | 1.400000000000091 |
| CV | 0.0014472339546251773 |
| Candidate-failed samples | 0 |

#### buffered-synchronized-writes / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 1599.4 |
| MAD | 2.900000000000091 |
| CV | 0.004313737775521652 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / cold

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 5 |
| Median | 63864445.0 |
| MAD | 146845.0 |
| CV | 0.005212601065505538 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / warm

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 5 |
| Median | 64062963.0 |
| MAD | 497406.0 |
| CV | 0.0073272637366256135 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 2 |
| Median | 1143.909330597277 |
| MAD | 2.016933367205752 |
| CV | 0.0017631933871477708 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 5564.681133648331 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 775.9364537557893 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 2 |
| Median | 1287.0010581399488 |
| MAD | 0.7003471850249525 |
| CV | 0.0005441698595315348 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 822.5070829402357 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 2 |
| Median | 1021.5362439656697 |
| MAD | 5.5979231903702384 |
| CV | 0.0054799065852414 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 421.32908259103345 |
| MAD | 2.0608917391467685 |
| CV | 0.008353772969873772 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 2 |
| Median | 1141.6114439416106 |
| MAD | 1.2844084062011234 |
| CV | 0.001125083681507678 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 5606.4557060079815 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 770.8845764947515 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 1285.5611559158428 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 825.1859522091789 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 2 |
| Median | 1004.3102519340488 |
| MAD | 3.520513113806146 |
| CV | 0.0035054039396954515 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 420.42698901345796 |
| MAD | 0.7684894747255839 |
| CV | 0.007575239175441071 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 3 |
| Median | 2885.011499 |
| MAD | 2.6997299999998177 |
| CV | 0.001213961184618075 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 3258.615708 |
| MAD | 648.8377839999998 |
| CV | 0.4354697026994488 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 3 |
| Median | 7023.794522 |
| MAD | 36.29202300000088 |
| CV | 0.005783034705488414 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 3 |
| Median | 2886.1113889999997 |
| MAD | 9.599039999999604 |
| CV | 0.0031337006330200597 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 2999.30021 |
| MAD | 398.7803659999995 |
| CV | 0.3771498563137553 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 7025.889395 |
| MAD | 15.893033999999716 |
| CV | 0.0036821636578937343 |
| Candidate-failed samples | 0 |

Run: `perf-v2-core-glm-5-3-perf-cpu-r1`
Execution backend: `hyperv`

### NVMe performance

Profile: `perf-nvme`

Outcome: `pass`; classification: `diagnostic`.


#### buffered-synchronized-writes / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 896.5 |
| MAD | 7.7000000000000455 |
| CV | 0.009596986019893788 |
| Candidate-failed samples | 0 |

#### buffered-synchronized-writes / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 902.5 |
| MAD | 0.39999999999997726 |
| CV | 0.002717528966142581 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / cold

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 5 |
| Median | 30008078.0 |
| MAD | 209663.0 |
| CV | 0.00847431326486644 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / warm

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 5 |
| Median | 30212623.0 |
| MAD | 84877.0 |
| CV | 0.0050651194524952104 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 673.5921591405712 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 5531.933446780196 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 522.9776138408405 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 733.2063096746535 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 422.25303384289083 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 444.06149846392185 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 263.8029206465514 |
| MAD | 1.2414113463732974 |
| CV | 0.006580665641700023 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 658.6508828608303 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 5670.1008198914415 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 521.7906429364851 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 734.7022371692535 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 458.04345427289235 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 441.22580901784266 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 261.8933198687576 |
| MAD | 0.5148097808061038 |
| CV | 0.008593423066792474 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 3 |
| Median | 1506.971388 |
| MAD | 0.3220529999998689 |
| CV | 0.0006434730440648522 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 4 |
| Median | 2497.688479 |
| MAD | 694.4228985 |
| CV | 0.2834024863273443 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 4 |
| Median | 2907.0222705 |
| MAD | 140.64072299999998 |
| CV | 0.062350442412229465 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 3 |
| Median | 1505.571258 |
| MAD | 0.7781070000000909 |
| CV | 0.0019892889844502324 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 4 |
| Median | 1369.6524475 |
| MAD | 83.19894700000009 |
| CV | 0.21938536402774883 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 4 |
| Median | 3099.1783649999998 |
| MAD | 49.85966200000007 |
| CV | 0.020181130155228495 |
| Candidate-failed samples | 0 |

Run: `perf-v2-core-glm-5-3-perf-nvme-r1`
Execution backend: `hyperv`

## Scope and provenance

Original profile results remain independent; the index adds a derived provisional total with missing scores imputed as zero. Each profile lists its run and, where recorded, its execution backend; different backends are not treated as identical execution conditions.

`valid` means the profile evidence is valid, not that all cases passed or release eligibility is established. `candidate-failed` is not an infrastructure error. `diagnostic` is diagnostic-only; `noneligible` cannot enter rankings. Hyper-V performance is not release- or ranking-eligible; protocol-v2 Hyper-V runs on the fixed VHDX/memory devices form their own `hyperv-fixed-vhdx` pool, counted in the provisional total and not comparable with KVM/raw-NVMe scores.

Security-review and maintainability scores are not assessed. Reference is a separate calibration baseline, not a model-generation entry.
<!-- core-results:end -->
