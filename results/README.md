# Core Results

[中文](README.zh-CN.md)

BenchFS evaluates **AI coding agents implementing a userspace filesystem**. Agents develop against a fixed specification, SDK and FUSE adapter in isolated VMs; frozen implementations are rebuilt and tested in fresh evaluation VMs. The aim is to measure whether an agent can complete a substantial systems-programming task and produce a working filesystem.

This public repository contains benchmark information, SDK/adapter snapshots, scoring rules and released results. This page collects the **Core model scores and detailed reports**. See the [project overview](../README.md) for the repository contents, or [benchmark purpose](../docs/benchmark.md) for the evaluated capabilities and experimental setup.

## About these results

The original eight models each have one archived generation (`n=1`) on the **Core** track, using **Pi** and operator-confirmed **High effort**. Additional **Claude Code** submissions are explicitly labeled and retain their own generation and intervention metadata. Each model/harness submission is an individual generation, not an average across repeated generations; cross-harness differences are not controlled model comparisons. Entries are alphabetical, not ordered by score.

The reports cover build and SDK-smoke checks, POSIX correctness (`spec-tests`, `pjdfstest-core`, `xfstests-core`), robustness, format conformance, crash consistency and real-world applications. CPU and NVMe performance, code review and maintainability have not yet been measured.

Click a model name for its detailed report: case counts, pass rates, category macro averages and run provenance, plus source size, generation time, token usage and estimated cost where available. Generation goal-state telemetry is separate from candidate evaluation: DeepSeek and Qwen retain their incomplete goal states alongside completed Eval observations. Luna and Reference do not assert model-generation metadata.

**Reading the scores:** raw pass rates count applicable cases; category macro averages weight categories equally. Timeouts count as failures, while N/A cases are excluded. Format uses its native conformance score, not its case pass rate. Missing results remain unmeasured in the underlying reports; only the provisional totals below substitute zero. SDK-smoke checks and generation cost do not contribute to the total.

<!-- core-results:begin -->
## Totals and dimension scores

All 12 results; click a model name for its report. Reference is a calibration baseline; Luna has evaluation records only. Scores are 0–100. Unmeasured items count as zero; `*` marks zero-imputed values.

C correctness, R robustness, S code review, W real-world applications, P performance, M maintainability. S, P, M are unmeasured; the provisional ceiling is 65.00, without redistributing weights.

| Model | Effort | Total / 100 | C | R | S | W | P | M | JSON |
|---|---|---:|---:|---:|---:|---:|---:|---:|---|
| [DeepSeek V4 Flash 0731](core/deepseek-v4-flash-0731-high.md) | High | 61.15* | 89.72 | 82.48 | 0.00* | 66.67 | 0.00* | 0.00* | [JSON](data/core/deepseek-v4-flash-0731-high.json) |
| [Fable 5.1](core/fable-5.1-high.md) | High | 60.81* | 95.52 | 63.43 | 0.00* | 83.33 | 0.00* | 0.00* | [JSON](data/core/fable-5.1-high.json) |
| [Fable 5.1 (Claude Code)](core/fable-5.1-high-cc.md) | High | 61.56* | 97.45 | 68.52 | 0.00* | 83.33 | 0.00* | 0.00* | [JSON](data/core/fable-5.1-high-cc.json) |
| [Gemini 3.8 Flash](core/gemini-3.8-flash-high.md) | High | 59.85* | 90.57 | 78.59 | 0.00* | 50.00 | 0.00* | 0.00* | [JSON](data/core/gemini-3.8-flash-high.json) |
| [GLM 5.3](core/glm-5.3-high.md) | High | 63.89* | 95.81 | 99.77 | 0.00* | 83.33 | 0.00* | 0.00* | [JSON](data/core/glm-5.3-high.json) |
| [GPT 5.6 Luna](eval-only/gpt-5.6-luna-eval-only.md) | — | 0.00* | 0.00* | 0.00 | 0.00* | 0.00 | 0.00* | 0.00* | [JSON](data/core/gpt-5.6-luna-eval-only.json) |
| [GPT 5.6 Sol](core/gpt-5.6-sol-high.md) | High | 59.67* | 95.44 | 88.45 | 0.00* | 33.33 | 0.00* | 0.00* | [JSON](data/core/gpt-5.6-sol-high.json) |
| [GPT-6 Astra](core/gpt-6-astra-high.md) | High | 64.09* | 97.33 | 100.00 | 0.00* | 83.33 | 0.00* | 0.00* | [JSON](data/core/gpt-6-astra-high.json) |
| [Kimi K3](core/kimi-k3-high.md) | High | 62.57* | 93.48 | 96.20 | 0.00* | 66.67 | 0.00* | 0.00* | [JSON](data/core/kimi-k3-high.json) |
| [Opus 5.5 (Claude Code)](core/opus-5.5-high-cc.md) | High | 64.97* | 99.73 | 100.00 | 0.00* | 100.00 | 0.00* | 0.00* | [JSON](data/core/opus-5.5-high-cc.json) |
| [Qwen 3.8 Flash Next](core/qwen-3.8-flash-next-high.md) | High | 57.10* | 89.21 | 67.85 | 0.00* | 33.33 | 0.00* | 0.00* | [JSON](data/core/qwen-3.8-flash-next-high.json) |
| [Reference](eval-only/reference-calibration.md) | — | 61.50* | 74.87 | 97.82 | 0.00* | 100.00 | 0.00* | 0.00* | [JSON](data/core/reference-calibration.json) |

### Eval profile breakdown

Spec/PJD/XFS/Robustness/Crash/Real-world are category macro averages × 100; Format is native conformance; CPU/NVMe combine normalized fio and peak RAM. SDK smoke is unscored.

| Model | Effort | Spec | PJD | XFS | Format | Robustness | Crash | Real-world | CPU | NVMe |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| DeepSeek V4 Flash 0731 | High | 100.00 | 100.00 | 84.02 | 74.86 | 69.58 | 95.37 | 66.67 | 0.00* | 0.00* |
| Fable 5.1 | High | 100.00 | 99.26 | 87.35 | 95.45 | 90.28 | 36.57 | 83.33 | 0.00* | 0.00* |
| Fable 5.1 (Claude Code) | High | 100.00 | 100.00 | 94.36 | 95.45 | 100.00 | 37.04 | 83.33 | 0.00* | 0.00* |
| Gemini 3.8 Flash | High | 100.00 | 100.00 | 84.61 | 77.66 | 89.58 | 67.59 | 50.00 | 0.00* | 0.00* |
| GLM 5.3 | High | 100.00 | 100.00 | 89.17 | 94.05 | 100.00 | 99.54 | 83.33 | 0.00* | 0.00* |
| GPT 5.6 Luna | — | 0.00* | 0.00* | 0.00* | 0.00 | 0.00 | 0.00 | 0.00 | 0.00* | 0.00* |
| GPT 5.6 Sol | High | 100.00 | 100.00 | 89.62 | 92.15 | 78.75 | 98.15 | 33.33 | 0.00* | 0.00* |
| GPT-6 Astra | High | 100.00 | 100.00 | 89.31 | 100.00 | 100.00 | 100.00 | 83.33 | 0.00* | 0.00* |
| Kimi K3 | High | 100.00 | 100.00 | 89.75 | 84.17 | 97.50 | 94.91 | 66.67 | 0.00* | 0.00* |
| Opus 5.5 (Claude Code) | High | 100.00 | 100.00 | 98.90 | 100.00 | 100.00 | 100.00 | 100.00 | 0.00* | 0.00* |
| Qwen 3.8 Flash Next | High | 100.00 | 100.00 | 81.52 | 75.33 | 66.25 | 69.44 | 33.33 | 0.00* | 0.00* |
| Reference | — | 100.00 | 100.00 | 99.48 | 0.00 | 97.50 | 98.15 | 100.00 | 0.00* | 0.00* |

### Scoring

`C=(Spec+PJD+XFS+Format)/4; R=(Robustness+Crash)/2; W=Real-world; P=(CPU+NVMe)/2`

`Total=0.35U(C)+0.20U(R)+0.20U(S)+0.10U(W)+0.10U(P)+0.05U(M)`

U linearly interpolates `(0,0), (20,45), (40,68), (60,83), (80,93), (100,100)` after dimension aggregation. Calculations retain full precision; displayed scores use two decimals. Rule: `core-overall-v1-zero-fill`.

These provisional values combine retained observations, not a complete same-build evaluation; reports retain run identities.
<!-- core-results:end -->

## Evaluation status

Fable CC's eight non-performance profiles are complete. Its provisional total is **61.56/100**, with C **97.45**, R **68.52**, and W **83.33**. Performance, security review and maintainability remain unmeasured.
