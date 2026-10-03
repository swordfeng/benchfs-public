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

<!-- source-tree:begin -->
### Directory structure

Every file under `bench/`, excluding build output. Numbers are code lines counted as above; "test" is the part of them in test code.

```text
bench/
├── src/
│   ├── boot.rs  (516 code)
│   ├── check.rs  (687 code)
│   ├── device.rs  (50 code)
│   ├── engine.rs  (2,392 code)
│   ├── format.rs  (1,121 code)
│   ├── formatter.rs  (214 code)
│   ├── fs.rs  (1,563 code)
│   ├── geom.rs  (402 code, 36 test)
│   ├── lib.rs  (9 code)
│   ├── main.rs  (106 code)
│   ├── mkfs.rs  (95 code)
│   ├── sync.rs  (250 code)
│   └── tree.rs  (205 code, 44 test)
├── tests/
│   └── model.rs  (683 code, 683 test)
└── Cargo.toml
```
<!-- source-tree:end -->

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

Outcome: `candidate-failed`; classification: `diagnostic`.


#### buffered-synchronized-writes / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 2623.1 |
| MAD | 3.5 |
| CV | 0.0038296602109486725 |
| Candidate-failed samples | 0 |

#### buffered-synchronized-writes / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 2627.0 |
| MAD | 15.699999999999818 |
| CV | 0.005223871115614208 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / cold

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 5 |
| Median | 256759891.0 |
| MAD | 1821799.0 |
| CV | 0.00967714648259494 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / warm

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 5 |
| Median | 255867852.0 |
| MAD | 3460217.0 |
| CV | 0.011520183664419073 |
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
| Samples | 3 |
| Median | 7261.253785160714 |
| MAD | 4.334627924403321 |
| CV | 0.0016098708866112883 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 1945.8441498139596 |
| MAD | 0.31278463428634495 |
| CV | 0.0006829047653155413 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 3510.744909489339 |
| MAD | 9.543989893158141 |
| CV | 0.005307142918034422 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 2881.6983300635025 |
| MAD | 7.1115937838799255 |
| CV | 0.002112476632459412 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 969.4169886750602 |
| MAD | 9.099791492905183 |
| CV | 0.010508747658213812 |
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

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 2474.1842007123396 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 1 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 9159.990461357516 |
| MAD | 102.3804656858756 |
| CV | 0.009524797547762722 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 1945.6525040025074 |
| MAD | 0.3919849237743165 |
| CV | 0.0019809038082609946 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 3512.2201807684373 |
| MAD | 14.64117428732743 |
| CV | 0.00397017899394688 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 2894.288209076976 |
| MAD | 1.8032640895926306 |
| CV | 0.0015674369873700147 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 4 |
| Median | 949.2117107327944 |
| MAD | 19.389191641671687 |
| CV | 0.02121033996494972 |
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

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 3 |
| Median | 4580.541945 |
| MAD | 20.29797099999996 |
| CV | 0.004904649545389843 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 6368.126375 |
| MAD | 4.529601000000184 |
| CV | 0.002383826120485539 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 3666.35488 |
| MAD | 19.997580000000198 |
| CV | 0.009143192586905562 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 3 |
| Median | 4567.243275 |
| MAD | 1.7998199999992721 |
| CV | 0.005789959672370116 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 3 |
| Median | 6380.792315 |
| MAD | 8.100069000000076 |
| CV | 0.0016398441659448031 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 3695.182827 |
| MAD | 32.26441399999976 |
| CV | 0.009065043863902982 |
| Candidate-failed samples | 0 |

Run: `perf-v2-core-qwen-3-8-flash-next-perf-cpu-r1`
Execution backend: `hyperv`

### NVMe performance

Profile: `perf-nvme`

Outcome: `candidate-failed`; classification: `diagnostic`.


#### buffered-synchronized-writes / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 1753.9 |
| MAD | 6.099999999999909 |
| CV | 0.006188169637909992 |
| Candidate-failed samples | 0 |

#### buffered-synchronized-writes / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 1749.7 |
| MAD | 8.900000000000091 |
| CV | 0.005431399511383816 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / cold

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 5 |
| Median | 186895724.0 |
| MAD | 295713.0 |
| CV | 0.0076641044374444744 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / warm

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 5 |
| Median | 185933734.0 |
| MAD | 496286.0 |
| CV | 0.006241130052462012 |
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
| Median | 5624.69819836661 |
| MAD | 61.04869434908869 |
| CV | 0.010853683557069246 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 1563.2213860014583 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 3082.436401034986 |
| MAD | 5.277461414965273 |
| CV | 0.005137672059711584 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 1978.7436683808255 |
| MAD | 5.15603419681679 |
| CV | 0.009474723876224521 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 4 |
| Median | 377.9818415106846 |
| MAD | 2.0916140254201423 |
| CV | 0.01946596568778179 |
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
| Median | 8998.880486234775 |
| MAD | 145.61156722741634 |
| CV | 0.01618107579605624 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 1571.0876178204726 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 3083.6696387103166 |
| MAD | 4.640189569812264 |
| CV | 0.00588542771971937 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 4 |
| Median | 2034.6569377560522 |
| MAD | 15.117206409853452 |
| CV | 0.017008273159385492 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 4 |
| Median | 378.41843921349187 |
| MAD | 5.331669035983964 |
| CV | 0.020443354658874525 |
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

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 3 |
| Median | 2638.2361769999998 |
| MAD | 17.89820999999938 |
| CV | 0.006070307311699357 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 3196.132872 |
| MAD | 8.024691999999959 |
| CV | 0.0026502658397615005 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 1826.410484 |
| MAD | 5.51629099999991 |
| CV | 0.006332941067614796 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 3 |
| Median | 2610.638936 |
| MAD | 1.8998090000000047 |
| CV | 0.0018725884813161175 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 3198.5323319999998 |
| MAD | 7.698500000000422 |
| CV | 0.003837033954537655 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 1844.404935 |
| MAD | 7.8135010000000875 |
| CV | 0.0054393837344507935 |
| Candidate-failed samples | 0 |

Run: `perf-v2-core-qwen-3-8-flash-next-perf-nvme-r2`
Execution backend: `hyperv`

## Scope and provenance

Original profile results remain independent; the index adds a derived provisional total with missing scores imputed as zero. Each profile lists its run and, where recorded, its execution backend; different backends are not treated as identical execution conditions.

`valid` means the profile evidence is valid, not that all cases passed or release eligibility is established. `candidate-failed` is not an infrastructure error. `diagnostic` is diagnostic-only; `noneligible` cannot enter rankings. Hyper-V performance is not release- or ranking-eligible; protocol-v2 Hyper-V runs on the fixed VHDX/memory devices form their own `hyperv-fixed-vhdx` pool, counted in the provisional total and not comparable with KVM/raw-NVMe scores.

Security-review and maintainability scores are not assessed. Reference is a separate calibration baseline, not a model-generation entry.
<!-- core-results:end -->
