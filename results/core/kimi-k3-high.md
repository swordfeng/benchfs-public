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


#### buffered-synchronized-writes / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 0 |

#### buffered-synchronized-writes / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / cold

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / warm

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
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
| Median | 2348.7719945445206 |
| MAD | 4.633528585929525 |
| CV | 0.001775422258981002 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 7744.4277446444175 |
| MAD | 19.966363596741758 |
| CV | 0.0023711991536031567 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 1788.786778554594 |
| MAD | 2.7834541949878258 |
| CV | 0.0022744225014173455 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 3117.0347907840123 |
| MAD | 1.5638335368521439 |
| CV | 0.0031654404673946332 |
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
| Median | 1361.9293654003282 |
| MAD | 1.9097158500317164 |
| CV | 0.013146043597247378 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 260.66766337871485 |
| MAD | 0.8390844175892198 |
| CV | 0.0046806213777649186 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 2349.4035298587883 |
| MAD | 3.7911799898415666 |
| CV | 0.0022052207362298704 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 7891.237963548395 |
| MAD | 21.213850847522735 |
| CV | 0.006985789577955927 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 1784.5683334444602 |
| MAD | 4.822003404773568 |
| CV | 0.0025251244838076755 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 3122.1207134451643 |
| MAD | 4.299096987032044 |
| CV | 0.0012948724326589834 |
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
| Samples | 3 |
| Median | 1375.654311767467 |
| MAD | 2.6933100737649056 |
| CV | 0.0032886642030505804 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 4 |
| Median | 276.41746380693735 |
| MAD | 2.321378392644391 |
| CV | 0.018746127702632155 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 0 |

Run: `perf-v2-core-kimi-k3-perf-cpu-r1`
Execution backend: `hyperv`

### NVMe performance

Profile: `perf-nvme`

Outcome: `candidate-failed`; classification: `diagnostic`.


#### buffered-synchronized-writes / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 0 |

#### buffered-synchronized-writes / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / cold

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / warm

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
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
| Median | 2132.9005226566724 |
| MAD | 3.947617172849732 |
| CV | 0.004662640175124315 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 7552.478663462695 |
| MAD | 5.160007625869184 |
| CV | 0.0010216032758390047 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 1772.4106726442787 |
| MAD | 2.305487230459903 |
| CV | 0.003058859493913583 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 2830.3002596609977 |
| MAD | 1.7220455981382656 |
| CV | 0.0016538055882407397 |
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
| Median | 533.7330171683077 |
| MAD | 1.1896346627962657 |
| CV | 0.001897757587999066 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 4 |
| Median | 224.8356685361042 |
| MAD | 2.2799363482947825 |
| CV | 0.0185916068190902 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 2124.576155170269 |
| MAD | 3.6294299114856585 |
| CV | 0.0027554195659675196 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 7933.775055758935 |
| MAD | 15.087927294896872 |
| CV | 0.007784040801694263 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 1767.630150426573 |
| MAD | 2.9679541808588965 |
| CV | 0.0025432119291852666 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 2822.4221989455614 |
| MAD | 0.7625508177884512 |
| CV | 0.0013670345394234098 |
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
| Samples | 3 |
| Median | 531.8697481662329 |
| MAD | 1.0184117779567714 |
| CV | 0.004088481238853864 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 231.91914834650345 |
| MAD | 2.9583555582763097 |
| CV | 0.012602563220248072 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 0 |

Run: `perf-v2-core-kimi-k3-perf-nvme-r3`
Execution backend: `hyperv`

## Scope and provenance

Original profile results remain independent; the index adds a derived provisional total with missing scores imputed as zero. Each profile lists its run and, where recorded, its execution backend; different backends are not treated as identical execution conditions.

`valid` means the profile evidence is valid, not that all cases passed or release eligibility is established. `candidate-failed` is not an infrastructure error. `diagnostic` is diagnostic-only; `noneligible` cannot enter rankings. Hyper-V performance is not release- or ranking-eligible; protocol-v2 Hyper-V runs on the fixed VHDX/memory devices form their own `hyperv-fixed-vhdx` pool, counted in the provisional total and not comparable with KVM/raw-NVMe scores.

Security-review and maintainability scores are not assessed. Reference is a separate calibration baseline, not a model-generation entry.
<!-- core-results:end -->
