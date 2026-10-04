# GPT-6 Astra — Core — High

[中文](gpt-6-astra-high.zh-CN.md)

This page collects independent Core Eval profile results. Each profile lists its status and run provenance; unmeasured dimensions remain unmeasured.

## Run

| Field | Value |
|---|---|
| Model | `gpt-6-astra` |
| Effort | High |
| Variant | Core |
| Agent harness | Pi |
| Generation runs | 1 (`n=1`) |
| SDK/POSIX evaluation run | `eval-core-gpt-6-astra` |
| Initial SDK/POSIX evaluation date | 2026-09-05 (UTC) |
| SDK/POSIX results updated | 2026-09-06 (UTC) |

The effective effort is High, confirmed by the operator for the proxy override. The Pi session's local thinking-level setting does not represent the provider-side effort.

## Submission size

Only candidate-owned source under `bench/` is counted. The fixed SDK and FUSE adapter, dependencies, documentation, configuration, lockfiles, and generated artifacts are excluded.

| Component | Files containing this component | Physical lines | Code lines |
|---|---:|---:|---:|
| Implementation (Rust) | 9 | 4,184 | 4,114 |
| Agent-written tests and test support (Rust) | 5 | 1,110 | 1,093 |
| Qualification and diagnostic scripts (Python / shell) | 3 | 146 | 119 |

Physical lines include blanks and comments. Code lines count each line holding at least one non-comment token: blank lines, comments, and documentation comments (Rust `///` and `//!`, Python docstrings) are excluded, while Rust attributes such as `#[derive(...)]` count as code. Corrected on 2026-09-27: earlier versions of this page counted documentation comments as code and Rust attributes as comments; file and physical-line counts are unchanged. Tests include three dedicated files under `src/`, an inline test module, and its test-only module declaration. Two Rust files contain both implementation and test components; file counts overlap, while physical and code lines do not. There are 12 distinct Rust files and three scripts. These are source-size measurements, not test-coverage or code-quality scores.

<!-- source-tree:begin -->
### Directory structure

Every file under `bench/`, excluding build output. Numbers are code lines counted as above; "test" is the part of them in test code.

```text
bench/
├── scripts/
│   ├── check_mkfs.py  (108 code)
│   ├── service-daemon.sh  (10 code)
│   └── trace-daemon.sh  (1 code)
├── src/
│   ├── content.rs  (795 code)
│   ├── fault_tests.rs  (125 code, 125 test)
│   ├── format.rs  (706 code, 24 test)
│   ├── fs.rs  (1,161 code, 3 test)
│   ├── lib.rs  (8 code)
│   ├── main.rs  (43 code)
│   ├── mkfs.rs  (55 code)
│   ├── scale_tests.rs  (461 code, 461 test)
│   ├── storage.rs  (513 code)
│   ├── tests.rs  (480 code, 480 test)
│   ├── tree.rs  (563 code)
│   └── validate.rs  (297 code)
├── Cargo.toml
├── README.md
├── WORKLOG.md
└── qualification.json
```
<!-- source-tree:end -->

## Generation time and activity

| Metric | Value |
|---|---|
| Goal start | 2026-09-05 09:23:52.569 UTC |
| Goal completion | 2026-09-05 13:16:43.263 UTC |
| Goal active time | **3 h 52 min 51 s** |
| Elapsed time | 3 h 52 min 51 s |
| Final goal status | Complete (harness-recorded) |
| Proxy API requests / HTTP responses | 214 / 214 |
| Assistant response records with nonzero token usage | 209 |
| Assistant error records | 4 |
| Agent tool calls | 245 |

Active time is the final cumulative Pi goal counter, 13,970.694 seconds, from one goal. It is harness-accounted time, not model inference time. A completed goal does not imply that all evaluation tests passed.

Request counts come from the archived proxy log; assistant and tool-call counts come from the Pi session. The proxy records 213 HTTP 200 responses and one HTTP 405 response. The session has 213 assistant records, while 209 responses report nonzero usage. These are separate recorded counts, not interchangeable measures or a guarantee that every failed request was fully accounted for.

### Tool usage

| Tool | Requests | Recorded failures | Unknown outcomes |
|---|---:|---:|---:|
| `bash` | 83 | 7 | 0 |
| `edit` | 69 | 0 | 0 |
| `goal_complete` | 1 | 0 | 0 |
| `read` | 73 | 0 | 0 |
| `write` | 19 | 0 | 0 |
| **Total** | **245** | **7** | **0** |

Each archived assistant `toolCall` counts as one request under its recorded tool name, including retries. Results are matched by `toolCallId` and checked for the same tool name; only `toolResult.isError: true` counts as a recorded failure. Missing or unparseable results are unknown, not assumed successful or failed. These are tool-level outcomes, not model API errors or evaluation test results.

## Token usage and API cost

| Metric | Value |
|---|---:|
| Input tokens, excluding cache (`input`) | 617,656 |
| Cache-read input tokens (`cache_read`) | 41,738,752 |
| Cache-write input tokens (`cache_write`) | 0 |
| Total input tokens, including cache | 42,356,408 |
| Output tokens, including thinking (`output`) | 199,142 |
| Thinking / reasoning tokens | 117,099 |
| **Total tokens** | **42,555,550** |
| Cache hit rate (input tokens) | 98.54% |
| Estimated API cost (USD) | **$77.86** |

Token counts are summed from Pi assistant-message usage records and match both the archived proxy totals and the final cumulative goal token counter. Reasoning usage is explicitly recorded on all 209 responses with nonzero usage; it is a component of output and is not added again. Input totals include repeated context across requests, not unique context tokens.

Cost uses the non-batch model-catalog rates for `openai/gpt-6-astra` in [OpenRouter's Models API](https://openrouter.ai/api/v1/models), corroborated by the standard OpenAI endpoint in [Astra endpoint pricing](https://openrouter.ai/api/v1/models/openai/gpt-6-astra/endpoints), retrieved on 2026-09-06. Flex and priority endpoints are not used as the pricing basis. This is a current-list-price estimate, not evidence of the run-date price, actual endpoint routing, or a provider invoice; session cost fields are not used.

| Token class | USD / million tokens | Estimated cost (USD) |
|---|---:|---:|
| Standard-tier input, assumed to populate the cache | $12.50 | $6.422363 |
| Standard-tier output | $50.00 | $5.653200 |
| Standard-tier cache read | $1.00 | $26.745728 |
| Long-context input, assumed to populate the cache | $25.00 | $2.596675 |
| Long-context output | $75.00 | $6.455850 |
| Long-context cache read | $2.00 | $29.986048 |

[OpenRouter's OpenAI caching rules](https://openrouter.ai/docs/guides/best-practices/prompt-caching#openai) charge cache writes on GPT-5.6 and later at 1.25 times ordinary input pricing, including automatic caching. The session records zero separate cache writes but does not establish how many non-cached tokens populated the cache. All 617,656 non-cached input tokens are conservatively priced as writes, replacing rather than adding to the ordinary $10.00 / $20.00 input charge. This is a billing assumption, not a recovered write-token count; the usage table is unchanged.

Tiers are applied per response using total input, including cache reads and writes. Of 209 usage-bearing responses, 161 are below the 272,000-token threshold and 48 reach the long-context tier; the maximum input is 370,473 tokens. The standard tier contains 513,789 non-cached input, 26,745,728 cache-read, and 113,064 output tokens. The long-context tier contains 103,867 non-cached input, 14,993,024 cache-read, and 86,078 output tokens.

The unrounded cache-write-assumption total is $77.8598635, rounded once to **$77.86**. Pricing non-cached input at ordinary prompt rates instead, while retaining the same long-context tiers, yields $76.056056, or $76.06. Reasoning is already included in output. Unreported usage on failed requests remains unknown and is excluded.

Generation time, source size, tokens, activity, and cost are descriptive metadata, not correctness scores.

## Results

<!-- core-results:begin -->

### POSIX correctness

| Profile | Classification | Pass | Expected fail | Fail | Unexpected pass | Timeout | Skip N/A | Skip impl | Pass rate | Category macro average |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `spec-tests` | `valid` | 44 | 0 | 0 | 0 | 0 | 7 | 0 | 100.00% | 100.00% |
| `pjdfstest-core` | `diagnostic` | 169 | 0 | 0 | 0 | 0 | 48 | 0 | 100.00% | 100.00% |
| `xfstests-core` | `valid` | 178 | 1 | 11 | 0 | 4 | 473 | 0 | 92.27% | 89.31% |

`spec-tests` run: `eval-core-gpt-6-astra`

`pjdfstest-core` run: `case-correction-core-gpt-6-astra-pjdfstest-core`

`xfstests-core` run: `eval-core-gpt-6-astra`

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

Run: `eval-submission-robustness-v2-core-gpt-6-astra-r1`

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

Run: `core-canonical-astra-oracle3-r4`
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

Run: `case-correction-core-gpt-6-astra-crash-core`

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

Run: `core-realworld-astra-20260925-r2`
Execution backend: `qemu-kvm`

### CPU performance

Profile: `perf-cpu`

Outcome: `candidate-failed`; classification: `diagnostic`.

Performance score: **42.72** (comparison pool `hyperv-fixed-vhdx`).

| Family | Cache | Primary metric | Samples | Median | MAD | CV | Candidate-failed samples |
|---|---|---|---:|---:|---:|---:|---:|
| buffered-synchronized-writes | cold | operations_per_second | 3 | 2,016.7 | 0.3 | 0.2% | 0 |
| buffered-synchronized-writes | warm | operations_per_second | 3 | 1,991.3 | 2.6 | 0.2% | 0 |
| bulk-sequential-io | cold | MiB/s | 3 | 431.0 | 5.5 | 1.4% | 0 |
| bulk-sequential-io | warm | MiB/s | 5 | 442.7 | 5.1 | 1.4% | 0 |
| metadata-concurrency | cold | operations_per_second | 3 | 1,758.3 | 2.1 | 0.9% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 7,538.2 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 1,270.6 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 3 | 2,147.5 | 2.8 | 0.4% | 0 |
| metadata-concurrency | cold | operations_per_second | 2 | 1,432.8 | 3.9 | 0.3% | 0 |
| metadata-concurrency | cold | operations_per_second | 2 | 1,423.2 | 5.1 | 0.4% | 0 |
| metadata-concurrency | cold | operations_per_second | — | — | — | — | 0 |
| metadata-concurrency | warm | operations_per_second | 3 | 1,752.7 | 4.4 | 0.9% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 7,552.0 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 1,272.4 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 2 | 2,139.1 | 1.3 | 0.1% | 0 |
| metadata-concurrency | warm | operations_per_second | 2 | 1,431.3 | 10.5 | 0.7% | 0 |
| metadata-concurrency | warm | operations_per_second | 2 | 1,417.5 | 0.3 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | — | — | — | — | 0 |
| small-random-io | cold | iops | 3 | 3,622.1 | 1.4 | 0.3% | 0 |
| small-random-io | cold | iops | 5 | 6,682.6 | 767.6 | 23.1% | 0 |
| small-random-io | cold | iops | 4 | 7,227.7 | 87.2 | 2.3% | 0 |
| small-random-io | warm | iops | 5 | 3,615.6 | 22.7 | 2.5% | 0 |
| small-random-io | warm | iops | 5 | 4,710.6 | 867.2 | 29.5% | 0 |
| small-random-io | warm | iops | 3 | 7,420.9 | 30.8 | 0.4% | 0 |

Run: `perf-v2-core-gpt-6-astra-perf-cpu-r2`
Execution backend: `hyperv`

### NVMe performance

Profile: `perf-nvme`

Outcome: `candidate-failed`; classification: `diagnostic`.

Performance score: **45.46** (comparison pool `hyperv-fixed-vhdx`).

| Family | Cache | Primary metric | Samples | Median | MAD | CV | Candidate-failed samples |
|---|---|---|---:|---:|---:|---:|---:|
| buffered-synchronized-writes | cold | operations_per_second | 3 | 1,235.2 | 1.0 | 0.5% | 0 |
| buffered-synchronized-writes | warm | operations_per_second | 3 | 1,228.4 | 2.0 | 0.2% | 0 |
| bulk-sequential-io | cold | MiB/s | 3 | 269.0 | 0.3 | 0.1% | 0 |
| bulk-sequential-io | warm | MiB/s | 3 | 269.0 | 1.1 | 0.4% | 0 |
| metadata-concurrency | cold | operations_per_second | 2 | 920.8 | 1.8 | 0.2% | 0 |
| metadata-concurrency | cold | operations_per_second | 3 | 7,504.0 | 25.1 | 0.3% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 592.0 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 1,055.8 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 539.2 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 2 | 518.4 | 5.0 | 1.0% | 0 |
| metadata-concurrency | cold | operations_per_second | — | — | — | — | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 923.8 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 3 | 7,555.9 | 13.4 | 0.2% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 594.8 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 1,052.6 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 536.1 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 2 | 510.6 | 2.0 | 0.4% | 0 |
| metadata-concurrency | warm | operations_per_second | — | — | — | — | 0 |
| small-random-io | cold | iops | 3 | 1,969.5 | 18.7 | 0.8% | 0 |
| small-random-io | cold | iops | 5 | 3,140.7 | 130.1 | 16.9% | 0 |
| small-random-io | cold | iops | 5 | 3,224.7 | 12.2 | 1.3% | 0 |
| small-random-io | warm | iops | 3 | 1,966.8 | 2.4 | 0.8% | 0 |
| small-random-io | warm | iops | 5 | 1,925.4 | 24.7 | 7.3% | 0 |
| small-random-io | warm | iops | 5 | 3,102.5 | 67.5 | 3.1% | 0 |

Run: `perf-v2-core-gpt-6-astra-perf-nvme-r2`
Execution backend: `hyperv`

## Scope and provenance

Original profile results remain independent; the index adds a derived provisional total with missing scores imputed as zero. Each profile lists its run and, where recorded, its execution backend; different backends are not treated as identical execution conditions.

`valid` means the profile evidence is valid, not that all cases passed or release eligibility is established. `candidate-failed` is not an infrastructure error. `diagnostic` is diagnostic-only; `noneligible` cannot enter rankings. Hyper-V performance is not release- or ranking-eligible; protocol-v2 Hyper-V runs on the fixed VHDX/memory devices form their own `hyperv-fixed-vhdx` pool, counted in the provisional total and not comparable with KVM/raw-NVMe scores.

Security review is not assessed. Maintainability (`maint-v2-rev5`, 5% of the overall score): **88.35** / 100; components are in the README. Reference is a separate calibration baseline, not a model-generation entry.
<!-- core-results:end -->
