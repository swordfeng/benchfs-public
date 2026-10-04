# Reference 校准基线

Submission: `core-reference`

[English](reference-calibration.md)

本页仅报告 Eval 观察，不提供生成次数、耗时、token 或成本数据。各 profile 的候选和固定评测面身份独立记录；性能尚未评测。

SDK/POSIX 与其他 profile 使用的候选源码不完全相同，不能视为同一候选二进制的完整评测；各 profile 分别列出其运行。

## 评测结果

<!-- core-results:begin -->

### 构建与 SDK 检查

| Profile | Classification | Pass | Expected fail | Fail | Unexpected pass | Timeout | Skip N/A | Skip impl | Pass rate | Category macro average |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `sdk-smoke` | `valid` | 44 | 0 | 0 | 0 | 0 | 0 | 0 | 100.00% | — |

`sdk-smoke` run: `eval-core-reference`

### POSIX 正确性

| Profile | Classification | Pass | Expected fail | Fail | Unexpected pass | Timeout | Skip N/A | Skip impl | Pass rate | Category macro average |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `spec-tests` | `valid` | 44 | 0 | 0 | 0 | 0 | 7 | 0 | 100.00% | 100.00% |
| `pjdfstest-core` | `diagnostic` | 169 | 0 | 0 | 0 | 0 | 48 | 0 | 100.00% | 100.00% |
| `xfstests-core` | `valid` | 193 | 0 | 2 | 0 | 0 | 472 | 0 | 98.97% | 99.48% |

`spec-tests` run: `eval-core-reference`

`pjdfstest-core` run: `case-correction-core-reference-pjdfstest-core`

`xfstests-core` run: `eval-core-reference`

### 稳健性

Profile: `robustness`

结果：`candidate-failed`；评测分类：`diagnostic`。

| Metric | Value |
|---|---:|
| Total | 47 |
| Pass | 46 |
| Fail | 1 |
| Timeout | 0 |
| Pass rate | 97.87% |
| Category macro average | 97.50% |

超时计失败，N/A 不计入分母，类别宏平均按类别等权。

Run: `core-robustness-reference-20260925-r1`
执行后端: `hyperv`

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

Run: `core-canonical-reference-oracle3-r4`
执行后端: `hyperv`

### 崩溃一致性

Profile: `crash-core`

结果：`candidate-failed`；评测分类：`diagnostic`。

| Metric | Value |
|---|---:|
| Total | 216 |
| Pass | 212 |
| Fail | 4 |
| Timeout | 0 |
| Pass rate | 98.15% |
| Category macro average | 98.15% |

R 轴采用已观察能力分；上表仍为原始端到端结果。

| 能力 | 分数 / 100 | 满足 | 失败 | 未证明（暂计 0） |
|---|---:|---:|---:|---:|
| 结构安全 | 99.07 | 214 | 2 | 0 |
| 内容与持久性 | 100.00 | 216 | 0 | 0 |
| clean 生命周期 | 98.15 | 212 | 4 | 0 |

派生 crash 分 **99.26**；结构 / 内容 / clean 预算为 40% / 40% / 20%。合法安全拒绝 39；已观察两次生命周期 175；第二次被阻断 2；首次未观察 0。安全拒绝计契约满足，不表示恢复了内容；被阻断的第二次不计为通过。

Run: `case-correction-core-reference-crash-core`

### 真实应用

Profile: `real-world`

结果：`pass`；评测分类：`diagnostic`。

| Metric | Value |
|---|---:|
| Total | 4 |
| Pass | 4 |
| Fail | 0 |
| Timeout | 0 |
| Pass rate | 100.00% |
| Category macro average | 100.00% |

Run: `core-realworld-reference-oracle3-r6`
执行后端: `hyperv`

### CPU 性能

Profile: `perf-cpu`

尚无可发布的评测结果（`not-run`），不是实测零分；索引暂计总分时按缺失项填 0。

### NVMe 性能

Profile: `perf-nvme`

尚无可发布的评测结果（`not-run`），不是实测零分；索引暂计总分时按缺失项填 0。

### Agent 代码评审

状态：`incomplete`；S 尚无完整结果，显示为 —，仅在暂计总分中填 0。

Scoring rule: `agent-review-score-v2`.

## 评测范围与溯源

各 profile 原始结果独立保留；索引另列未测项暂计 0 的派生总分。每个 profile 列出其运行，并标明已记录的执行后端；不同后端不视为完全相同的执行条件。

`valid` 表示该 profile 的证据有效，不等于全部用例通过或全维度发布合格。`candidate-failed` 是候选失败，不是基础设施错误。`diagnostic` 仅诊断；`noneligible` 不具备排名资格。Hyper-V 性能不具备发布/排名资格；协议 v2 在固定 VHDX/内存设备上的 Hyper-V 运行单独组成 `hyperv-fixed-vhdx` 比较池计入暂计总分，与 KVM/raw-NVMe 分数不可比。

可维护性（`maint-v3-policy`，占总分 5%）：**67.39** / 100，各项见 README。Reference 单列为校准基线，不是模型生成条目。
<!-- core-results:end -->
