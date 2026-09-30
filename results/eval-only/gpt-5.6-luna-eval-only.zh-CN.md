# GPT 5.6 Luna：仅 Eval 证据

Submission: `core-gpt-5.6-luna`

[English](gpt-5.6-luna-eval-only.md)

本页仅报告 Eval 观察，不提供生成次数、耗时、token 或成本数据。各 profile 的候选和固定评测面身份独立记录；性能尚未评测。

缺少完整生成溯源不影响报告已完成的 Eval 观察；未取得有效结果的维度不是实测零分，索引暂计时另以 0 填充。

## 评测结果

<!-- core-results:begin -->

### 构建与 SDK 检查

| Profile | Classification | Pass | Expected fail | Fail | Unexpected pass | Timeout | Skip N/A | Skip impl | Pass rate | Category macro average |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `sdk-smoke` | `diagnostic` | 44 | 0 | 0 | 0 | 0 | 0 | 0 | 100.00% | — |

`sdk-smoke` run: `core-sdk-luna-20260925-r1`

### POSIX 正确性

| Profile | Classification | Pass | Expected fail | Fail | Unexpected pass | Timeout | Skip N/A | Skip impl | Pass rate | Category macro average |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `spec-tests` | `not-run` | — | — | — | — | — | — | — | — | — |
| `pjdfstest-core` | `not-run` | — | — | — | — | — | — | — | — | — |
| `xfstests-core` | `not-run` | — | — | — | — | — | — | — | — | — |

### 稳健性

Profile: `robustness`

结果：`candidate-failed`；评测分类：`valid`。

| Metric | Value |
|---|---:|
| Total | 47 |
| Pass | 0 |
| Fail | 47 |
| Timeout | 0 |
| Pass rate | 0.00% |
| Category macro average | 0.00% |

超时计失败，N/A 不计入分母，类别宏平均按类别等权。

Run: `eval-submission-robustness-v2-core-gpt-5-6-luna-r1`

### 磁盘格式一致性

Profile: `canonical-format`

结果：`candidate-failed`；评测分类：`diagnostic`。

| Metric | Value |
|---|---:|
| Total | 479 |
| Pass | 0 |
| Fail | 479 |
| Timeout | 0 |
| Pass rate | 0.00% |
| Category macro average | 0.00% |
| Conformance score (0–100) | 0.00 |
| Full conformance | no |

Run: `core-canonical-luna-oracle3-r6`
执行后端: `hyperv`

### 崩溃一致性

Profile: `crash-core`

结果：`candidate-failed`；评测分类：`diagnostic`。

| Metric | Value |
|---|---:|
| Total | 216 |
| Pass | 0 |
| Fail | 216 |
| Timeout | 0 |
| Pass rate | 0.00% |
| Category macro average | 0.00% |

Run: `core-crash-luna-20260925-r1`
执行后端: `hyperv`

### 真实应用

Profile: `real-world`

结果：`candidate-failed`；评测分类：`diagnostic`。

| Metric | Value |
|---|---:|
| Total | 4 |
| Pass | 0 |
| Fail | 4 |
| Timeout | 0 |
| Pass rate | 0.00% |
| Category macro average | 0.00% |

Run: `core-realworld-luna-20260925-r1`
执行后端: `hyperv`

### CPU 性能

Profile: `perf-cpu`

尚无可发布的评测结果（`not-run`），不是实测零分；索引暂计总分时按缺失项填 0。

### NVMe 性能

Profile: `perf-nvme`

尚无可发布的评测结果（`not-run`），不是实测零分；索引暂计总分时按缺失项填 0。

## 评测范围与溯源

各 profile 原始结果独立保留；索引另列未测项暂计 0 的派生总分。每个 profile 列出其运行，并标明已记录的执行后端；不同后端不视为完全相同的执行条件。

`valid` 表示该 profile 的证据有效，不等于全部用例通过或全维度发布合格。`candidate-failed` 是候选失败，不是基础设施错误。`diagnostic` 仅诊断；`noneligible` 不具备排名资格。Hyper-V 性能始终为诊断结果。

安全审查与可维护性评分尚未评定。Reference 单列为校准基线，不是模型生成条目。
<!-- core-results:end -->
