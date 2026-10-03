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


#### buffered-synchronized-writes / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 5 |
| Median | 102.5 |
| MAD | 3.0894410558944116 |
| CV | 0.07998257876308118 |
| Candidate-failed samples | 0 |

#### buffered-synchronized-writes / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 5 |
| Median | 105.35767734211315 |
| MAD | 1.0423226578868565 |
| CV | 0.05630207933460487 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / cold

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 5 |
| Median | 29805573.0 |
| MAD | 422727.0 |
| CV | 0.047043897921454234 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / warm

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 5 |
| Median | 30284194.0 |
| MAD | 874613.0 |
| CV | 0.07850364039066114 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 2902.3578312508257 |
| MAD | 5.698238086249148 |
| CV | 0.002378216532555285 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 2 |
| Median | 4024.5800539465363 |
| MAD | 21.36359462198311 |
| CV | 0.00530827920817075 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 1 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 2 |
| Median | 420.7652807481952 |
| MAD | 24.62848695912018 |
| CV | 0.058532602583859505 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 5 |
| Median | 883.2723980530152 |
| MAD | 47.99959109309259 |
| CV | 0.05765300601775447 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 2 |
| Median | 2894.564593796541 |
| MAD | 10.941880495739724 |
| CV | 0.0037801472868111883 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 2 |
| Median | 4026.0529035827967 |
| MAD | 14.16806665512263 |
| CV | 0.0035190959966061113 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 1 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 2 |
| Median | 429.5526760904163 |
| MAD | 13.374037792442522 |
| CV | 0.031134802637400934 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 4 |
| Median | 967.7824769758114 |
| MAD | 8.603357347726956 |
| CV | 0.014693734248041535 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 361.86381400000005 |
| MAD | 38.156181999999944 |
| CV | 0.08535991637977407 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 2661.9558690000003 |
| MAD | 1506.9086380000003 |
| CV | 0.5455865970703212 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 6803.617643 |
| MAD | 123.33169700000053 |
| CV | 0.026481254245043866 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 377.744329 |
| MAD | 27.715125 |
| CV | 0.06289218313440963 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 491.618511 |
| MAD | 258.439032 |
| CV | 1.0975531627162665 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 6653.330602999999 |
| MAD | 90.08855900000071 |
| CV | 0.033795974738737376 |
| Candidate-failed samples | 0 |

Run: `perf-v2-core-gpt-5-6-sol-v2-perf-cpu-r1`
Execution backend: `hyperv`

### NVMe performance

Profile: `perf-nvme`

Outcome: `candidate-failed`; classification: `diagnostic`.


#### buffered-synchronized-writes / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 5 |
| Median | 20.073903924897632 |
| MAD | 0.29565999322247194 |
| CV | 0.021689791994687194 |
| Candidate-failed samples | 0 |

#### buffered-synchronized-writes / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 20.471340123826643 |
| MAD | 0.004089360791812879 |
| CV | 0.006370344563300322 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / cold

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 5 |
| Median | 13924394.0 |
| MAD | 67137.0 |
| CV | 0.0066181278557804315 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / warm

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 5 |
| Median | 26148319.0 |
| MAD | 93491.0 |
| CV | 0.005841812291712969 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 2 |
| Median | 2888.2346682987054 |
| MAD | 9.992155689531955 |
| CV | 0.0034596065891757213 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 2 |
| Median | 3983.8589060051218 |
| MAD | 46.40702325133816 |
| CV | 0.011648761752427007 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 1 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 869.8073587939201 |
| MAD | 5.856284789961023 |
| CV | 0.0068073847989696306 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 2 |
| Median | 2868.946205382835 |
| MAD | 11.31432183182369 |
| CV | 0.003943720454080071 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 4023.635189800146 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 1 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 5 |
| Median | 879.0187915067572 |
| MAD | 42.22329150605492 |
| CV | 0.06514855939623476 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 89.01307299999999 |
| MAD | 1.275193999999999 |
| CV | 0.01813248376229172 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 295.972685 |
| MAD | 231.434131 |
| CV | 1.142901048333523 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 1605.095315 |
| MAD | 367.45504099999994 |
| CV | 0.44711327641288473 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 88.169341 |
| MAD | 0.8479300000000052 |
| CV | 0.012198910781949629 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 60.096154 |
| MAD | 10.619426000000004 |
| CV | 0.6586409374516524 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 1736.866802 |
| MAD | 560.2481899999998 |
| CV | 0.5143026553289984 |
| Candidate-failed samples | 0 |

Run: `perf-v2-core-gpt-5-6-sol-v2-perf-nvme-r2`
Execution backend: `hyperv`

## Scope and provenance

Original profile results remain independent; the index adds a derived provisional total with missing scores imputed as zero. Each profile lists its run and, where recorded, its execution backend; different backends are not treated as identical execution conditions.

`valid` means the profile evidence is valid, not that all cases passed or release eligibility is established. `candidate-failed` is not an infrastructure error. `diagnostic` is diagnostic-only; `noneligible` cannot enter rankings. Hyper-V performance is not release- or ranking-eligible; protocol-v2 Hyper-V runs on the fixed VHDX/memory devices form their own `hyperv-fixed-vhdx` pool, counted in the provisional total and not comparable with KVM/raw-NVMe scores.

Security-review and maintainability scores are not assessed. Reference is a separate calibration baseline, not a model-generation entry.
<!-- core-results:end -->
