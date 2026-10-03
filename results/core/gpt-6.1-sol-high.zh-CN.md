# GPT 6.1 Sol · Pi — Core 测试报告

[English](gpt-6.1-sol-high.md) · [返回总表](../../README.zh-CN.md#results)

本次测试使用 GPT 6.1 Sol 与 Pi 完成 Core 开发任务。下文列出测试成绩、开发耗时、token 用量及费用估算。

## 运行设置

| 项目 | 设置 |
|---|---|
| 模型 | `gpt-6.1-sol` |
| 编程 Agent | Pi 0.84.3 |
| 推理设置 | High，由模型代理配置指定 |
| 开发次数 | 1 |
| 提交标识 | `core-gpt-6.1-sol` |
| 开发日期 | 2026-09-30（UTC） |

开发环境和工具配置见[实验环境](../../docs/environment.zh-CN.md)。各项测试的运行标识和执行后端列在下方结果中。

## 测试结果

<!-- core-results:begin -->

### 构建与 SDK 检查

| Profile | Classification | Pass | Expected fail | Fail | Unexpected pass | Timeout | Skip N/A | Skip impl | Pass rate | Category macro average |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `sdk-smoke` | `valid` | 54 | 0 | 0 | 0 | 0 | 0 | 0 | 100.00% | — |

`sdk-smoke` run: `eval-core-gpt-6-1-sol-sdk-smoke`

### POSIX 正确性

| Profile | Classification | Pass | Expected fail | Fail | Unexpected pass | Timeout | Skip N/A | Skip impl | Pass rate | Category macro average |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `spec-tests` | `valid` | 44 | 0 | 0 | 0 | 0 | 7 | 0 | 100.00% | 100.00% |
| `pjdfstest-core` | `valid` | 169 | 0 | 0 | 0 | 0 | 48 | 0 | 100.00% | 100.00% |
| `xfstests-core` | `valid` | 186 | 1 | 3 | 0 | 4 | 473 | 0 | 96.39% | 89.81% |

`spec-tests` run: `eval-core-gpt-6-1-sol-spec-tests`

`pjdfstest-core` run: `eval-core-gpt-6-1-sol-pjdfstest-core`

`xfstests-core` run: `eval-core-gpt-6-1-sol-xfstests-core`

### 稳健性

Profile: `robustness`

结果：`pass`；评测分类：`valid`。

| Metric | Value |
|---|---:|
| Total | 47 |
| Pass | 47 |
| Fail | 0 |
| Timeout | 0 |
| Pass rate | 100.00% |
| Category macro average | 100.00% |

超时计失败，N/A 不计入分母，类别宏平均按类别等权。

Run: `eval-core-gpt-6-1-sol-robustness`
执行后端: `qemu-kvm`

### 磁盘格式一致性

Profile: `canonical-format`

结果：`pass`；评测分类：`valid`。

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

Run: `eval-core-gpt-6-1-sol-canonical-format`
执行后端: `qemu-kvm`

### 崩溃一致性

Profile: `crash-core`

结果：`pass`；评测分类：`valid`。

| Metric | Value |
|---|---:|
| Total | 216 |
| Pass | 216 |
| Fail | 0 |
| Timeout | 0 |
| Pass rate | 100.00% |
| Category macro average | 100.00% |

Run: `eval-core-gpt-6-1-sol-crash-core-r2`
执行后端: `qemu-kvm`

### 真实应用

Profile: `real-world`

结果：`candidate-failed`；评测分类：`valid`。

| Metric | Value |
|---|---:|
| Total | 4 |
| Pass | 3 |
| Fail | 1 |
| Timeout | 0 |
| Pass rate | 75.00% |
| Category macro average | 83.33% |

Run: `eval-core-gpt-6-1-sol-real-world`
执行后端: `qemu-kvm`

### CPU 性能

Profile: `perf-cpu`

结果：`pass`；评测分类：`diagnostic`。

性能分：**42.44**（比较池 `hyperv-fixed-vhdx`）。

| 类别 | 缓存 | 主指标 | 样本数 | 中位数 | MAD | CV | 候选失败样本 |
|---|---|---|---:|---:|---:|---:|---:|
| buffered-synchronized-writes | cold | operations_per_second | 3 | 2,589.9 | 12.3 | 1.1% | 0 |
| buffered-synchronized-writes | warm | operations_per_second | 3 | 2,575.1 | 0.7 | 0.5% | 0 |
| bulk-sequential-io | cold | MiB/s | 5 | 328.3 | 1.3 | 0.5% | 0 |
| bulk-sequential-io | warm | MiB/s | 5 | 325.2 | 1.1 | 1.4% | 0 |
| metadata-concurrency | cold | operations_per_second | 3 | 2,487.9 | 6.8 | 0.4% | 0 |
| metadata-concurrency | cold | operations_per_second | 3 | 7,475.2 | 3.1 | 0.3% | 0 |
| metadata-concurrency | cold | operations_per_second | 3 | 1,741.3 | 14.2 | 0.8% | 0 |
| metadata-concurrency | cold | operations_per_second | 3 | 3,434.6 | 5.3 | 0.3% | 0 |
| metadata-concurrency | cold | operations_per_second | 3 | 2,628.7 | 0.2 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 3 | 1,280.2 | 2.5 | 0.4% | 0 |
| metadata-concurrency | cold | operations_per_second | 4 | 428.7 | 2.3 | 2.1% | 0 |
| metadata-concurrency | warm | operations_per_second | 3 | 2,490.1 | 6.5 | 0.4% | 0 |
| metadata-concurrency | warm | operations_per_second | 3 | 7,541.2 | 3.9 | 0.6% | 0 |
| metadata-concurrency | warm | operations_per_second | 3 | 1,734.6 | 3.6 | 0.3% | 0 |
| metadata-concurrency | warm | operations_per_second | 3 | 3,432.2 | 9.1 | 0.4% | 0 |
| metadata-concurrency | warm | operations_per_second | 3 | 2,618.9 | 4.8 | 0.2% | 0 |
| metadata-concurrency | warm | operations_per_second | 3 | 1,284.5 | 8.7 | 0.7% | 0 |
| metadata-concurrency | warm | operations_per_second | 3 | 432.7 | 2.4 | 1.2% | 0 |
| small-random-io | cold | iops | 3 | 4,594.4 | 41.8 | 1.0% | 0 |
| small-random-io | cold | iops | 5 | 7,248.6 | 39.2 | 1.6% | 0 |
| small-random-io | cold | iops | 3 | 7,290.2 | 17.7 | 0.4% | 0 |
| small-random-io | warm | iops | 3 | 4,593.1 | 38.9 | 0.8% | 0 |
| small-random-io | warm | iops | 5 | 7,204.1 | 57.6 | 1.5% | 0 |
| small-random-io | warm | iops | 3 | 7,274.4 | 58.4 | 0.8% | 0 |

Run: `perf-v2-core-gpt-6-1-sol-perf-cpu-r4`
执行后端: `hyperv`

### NVMe 性能

Profile: `perf-nvme`

结果：`pass`；评测分类：`diagnostic`。

性能分：**46.59**（比较池 `hyperv-fixed-vhdx`）。

| 类别 | 缓存 | 主指标 | 样本数 | 中位数 | MAD | CV | 候选失败样本 |
|---|---|---|---:|---:|---:|---:|---:|
| buffered-synchronized-writes | cold | operations_per_second | 3 | 1,772.7 | 6.3 | 0.4% | 0 |
| buffered-synchronized-writes | warm | operations_per_second | 3 | 1,761.4 | 1.8 | 0.7% | 0 |
| bulk-sequential-io | cold | MiB/s | 5 | 271.4 | 0.5 | 0.3% | 0 |
| bulk-sequential-io | warm | MiB/s | 5 | 272.9 | 1.6 | 0.8% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 2,219.3 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 7,579.4 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 1,718.7 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 3,049.1 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 1,918.1 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 498.8 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 400.4 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 2,214.4 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 7,670.1 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 1,735.2 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 3,034.1 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 1,910.6 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 487.2 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 402.4 | 0.0 | 0.0% | 0 |
| small-random-io | cold | iops | 3 | 2,664.7 | 3.6 | 0.2% | 0 |
| small-random-io | cold | iops | 5 | 3,231.6 | 24.1 | 1.5% | 0 |
| small-random-io | cold | iops | 5 | 3,172.1 | 33.8 | 1.3% | 0 |
| small-random-io | warm | iops | 3 | 2,668.5 | 13.7 | 0.5% | 0 |
| small-random-io | warm | iops | 5 | 3,379.5 | 44.6 | 2.4% | 0 |
| small-random-io | warm | iops | 5 | 3,126.7 | 46.2 | 1.7% | 0 |

Run: `perf-v2-core-gpt-6-1-sol-perf-nvme-r7`
执行后端: `hyperv`

## 评测范围与溯源

各 profile 原始结果独立保留；索引另列未测项暂计 0 的派生总分。每个 profile 列出其运行，并标明已记录的执行后端；不同后端不视为完全相同的执行条件。

`valid` 表示该 profile 的证据有效，不等于全部用例通过或全维度发布合格。`candidate-failed` 是候选失败，不是基础设施错误。`diagnostic` 仅诊断；`noneligible` 不具备排名资格。Hyper-V 性能不具备发布/排名资格；协议 v2 在固定 VHDX/内存设备上的 Hyper-V 运行单独组成 `hyperv-fixed-vhdx` 比较池计入暂计总分，与 KVM/raw-NVMe 分数不可比。

安全审查与可维护性评分尚未评定。Reference 单列为校准基线，不是模型生成条目。
<!-- core-results:end -->

## 开发耗时

| 项目 | 记录 |
|---|---|
| 任务开始 | 2026-09-30 08:04:55.609 UTC |
| 任务完成 | 2026-09-30 19:45:06.327 UTC |
| 活跃时长 | **7 小时 14 分 12 秒** |
| 起止时间差 | 11 小时 40 分 11 秒 |
| 任务状态 | 已完成（Agent 记录） |

活跃时长取自 Pi 的任务累计计时，共 26,052.273 秒；它与起止时间差分开记录，不代表模型的推理耗时。

## Token 用量与费用

| 项目 | 数量 |
|---|---:|
| 未缓存输入 tokens | 2,000,258 |
| 缓存读取 tokens | 29,043,712 |
| 单独记录的缓存写入 tokens | 0 |
| 输入 tokens 合计 | 31,043,970 |
| 输出 tokens（含推理） | 184,515 |
| 其中：推理 tokens | 112,947 |
| 总 tokens | 31,228,485 |
| 估算费用（美元） | **$12.87** |

用量汇总自 172 条带有 token 记录的回复，与归档的代理汇总一致。推理 tokens 已包含在输出中，不重复计费。输入包含多次请求中重复发送的上下文。

费用按 2026-09-30 查询的 [OpenAI GPT 6.1 Sol 标价](https://developers.openai.com/api/docs/models/gpt-6.1-sol)估算。标准档每百万 tokens：输入 $2、缓存读取 $0.10、缓存写入 $2.50、输出 $10。单次请求的总输入超过 272,000 tokens 时，输入和缓存费率翻倍，输出费率乘以 1.5；按整次请求计算。

| 请求档位 | 回复数 | 未缓存输入 | 缓存读取 | 输出（含推理） |
|---|---:|---:|---:|---:|
| 标准 | 150 | 1,078,628 | 23,389,312 | 135,009 |
| 长上下文 | 22 | 921,630 | 5,654,400 | 49,506 |

记录未单独拆出缓存写入量。为与此前 Sol 报告的估算口径一致，这里将未缓存输入全部按缓存写入费率计算，替代普通输入费率，两者不叠加。由此得到 $12.8672112，四舍五入为 **$12.87**；若全部按普通输入费率计算，则为 $11.41。这是基于记录用量和公开标价的估算，不是账单金额。

## 源码规模

仅统计提交中 `bench/` 下的实现、测试和脚本，不含固定 SDK、FUSE adapter、依赖、文档、配置及生成文件。

| 组成 | 含该类代码的文件数 | 物理行数 | 代码行数 |
|---|---:|---:|---:|
| Rust 实现 | 10 | 4,032 | 3,960 |
| Agent 编写的 Rust 测试 | 4 | 1,113 | 1,100 |
| 脚本 | 1 | 95 | 82 |

物理行数包含空行和注释；代码行数排除空行、注释和文档注释。内联测试与实现可能位于同一文件，文件数不能直接相加；Rust 文件共 13 个。源码规模、耗时和费用仅作描述，不参与评分。
