# GPT 5.6 Sol — Core — High

[English](gpt-5.6-sol-high.md)

本页汇总独立的 Core Eval profile 结果；各项分别列出评测状态与运行来源，未测维度不补造结果。

## 运行信息

| 字段 | 值 |
|---|---|
| 模型 | `gpt-5.6-sol` |
| Effort | High |
| Variant | Core |
| Agent harness | Pi |
| 生成运行次数 | 1（`n=1`） |
| 提交留档 | `core-gpt-5.6-sol-v2` |
| SDK/POSIX 评测运行 | `eval-core-gpt-5-6-sol-v2-r1` |
| SDK/POSIX 首次评测日期 | 2026-09-02（UTC） |
| SDK/POSIX 成绩更新日期 | 2026-09-05（UTC） |

实际 effort 为 High，operator 已确认这是 proxy override 后的配置。Pi session 的本地 thinking-level 设置不代表 provider 侧的实际 effort。

## 提交规模

仅统计 `bench/` 下候选实现自有的源码。固定 SDK 和 FUSE adapter、依赖、文档、配置、锁文件及生成文件均不计入。

| 组成部分 | 包含该部分的文件数 | 总行数 | 代码行数 |
|---|---:|---:|---:|
| 实现（Rust） | 4 | 3,311 | 3,236 |
| Agent 编写的测试（Rust） | 0 | 0 | 0 |
| 测试运行脚本 | 0 | 0 | 0 |

总行数包含空行和注释。代码行数统计至少包含一个非注释 token 的行：空行、注释和文档注释（Rust 的 `///`、`//!`，Python docstring）不计入，`#[derive(...)]` 等 Rust 属性计为代码。2026-09-27 更正：本页早先的版本把文档注释计为代码、把 Rust 属性计为注释；文件数和总行数不变。归档的 `bench/` 目录中没有独立测试文件、内联测试模块或脚本。这不代表 agent 在开发期间没有运行测试。这些数据仅衡量源码规模，不是测试覆盖率或代码质量评分。

<!-- source-tree:begin -->
### 目录结构

`bench/` 下的全部文件，不含构建产物。数字为按上述规则统计的代码行，“测试”为其中属于测试代码的部分。

```text
bench/
├── src/
│   ├── format.rs  (486 行代码)
│   ├── fs.rs  (2,687 行代码)
│   ├── main.rs  (45 行代码)
│   └── mkfs.rs  (18 行代码)
└── Cargo.toml
```
<!-- source-tree:end -->

## 生成时间与活动量

| 指标 | 值 |
|---|---|
| Goal 开始时间 | 2026-08-31 16:50:48.032 UTC |
| Goal 完成时间 | 2026-08-31 19:33:45.460 UTC |
| Goal 活跃时长 | **2 小时 42 分 57 秒** |
| 经过时长 | 2 小时 42 分 57 秒 |
| 最终 goal 状态 | Complete（harness 记录） |
| Proxy API 请求数／HTTP 响应数 | 197 / 197 |
| Token usage 非零的 assistant 响应记录 | 195 |
| Assistant 错误记录 | 1 |
| Agent 工具调用次数 | 234 |

活跃时长采用单个 Pi goal 最终累计计数器的 9,777.428 秒。活跃时长是 harness 记账的时间，不是模型推理时间。Goal 完成不代表全部评测测试通过。

请求数来自归档的 proxy 日志；assistant 和工具调用次数来自 Pi session。Session 包含 196 条 assistant 记录，而 proxy 记录了 197 次请求。它们属于不同的记录口径，不能互相替代，也不保证每次上游重试或失败请求均已计费或被完整记录。

### 工具使用统计

| 工具 | 请求次数 | 已记录失败次数 | 结果未知次数 |
|---|---:|---:|---:|
| `bash` | 84 | 7 | 0 |
| `edit` | 78 | 11 | 1 |
| `goal_complete` | 1 | 0 | 0 |
| `read` | 67 | 0 | 0 |
| `write` | 4 | 0 | 0 |
| **合计** | **234** | **18** | **1** |

归档 assistant 中每个 `toolCall` 按记录的工具名计一次请求，重试分别计数。通过 `toolCallId` 配对结果并核对工具名，仅将 `toolResult.isError: true` 计为已记录失败。结果缺失或无法解析时计为未知，不推定成功或失败。这些是工具级结果，不是模型 API 错误数或评测测试结果。

一次 `edit` 请求没有匹配的归档结果，因此 18 是已观测失败次数，不是完整失败总数。

## Token 用量与 API 成本

| 指标 | 值 |
|---|---:|
| 输入 tokens，不含缓存（`input`） | 323,794 |
| 缓存读取输入 tokens（`cache_read`） | 30,650,368 |
| 缓存写入输入 tokens（`cache_write`） | 0 |
| 输入 tokens 合计，含缓存 | 30,974,162 |
| 输出 tokens，含 thinking（`output`） | 87,220 |
| Thinking／reasoning tokens | 26,681 |
| **总 tokens** | **31,061,382** |
| 缓存命中率（按输入 tokens） | 98.95% |
| API 成本估算（美元） | **$7.81** |

Token 数对 Pi assistant 消息中的 usage 记录求和，与归档的 proxy token 总数及最终 goal 的累计 token 计数均一致。全部 195 条 usage 非零的响应都明确记录了 reasoning 用量；它属于 output 的一部分，不重复相加。输入总量包含各次请求中重复发送的上下文，不是单次上下文大小，也不是去重后的 token 数。

成本采用 2026-09-05 查询到的 [OpenRouter Models API](https://openrouter.ai/api/v1/models) 中 `openai/gpt-5.6-sol` 的非 batch 模型目录价，并与 [Sol endpoint 价格](https://openrouter.ai/api/v1/models/openai/gpt-5.6-sol/endpoints) 中的标准 OpenAI endpoint 交叉核对。下列单价单位为美元／百万 tokens。这是当前目录价下的估算，不是运行当日价格、实际 endpoint 路由或 provider 账单的证据，也不使用 session 成本字段。

| Token 类别 | 美元／百万 tokens | 费用估算（美元） |
|---|---:|---:|
| 输入，假定用于建立缓存 | $2.50 | $0.809485 |
| 输出 | $10.00 | $0.872200 |
| 缓存读取 | $0.20 | $6.130074 |

[OpenRouter 的 OpenAI 缓存规则](https://openrouter.ai/docs/guides/best-practices/prompt-caching#openai) 对 GPT-5.6 系列的自动缓存写入按普通输入单价的 1.25 倍计费。Session 中独立的 cache-write 计数为零，但无法据此确定多少非缓存 input 实际写入了缓存。本次估算保守地将全部 323,794 个 input tokens 按 $2.50 的写入价计算，替代而非叠加普通的 $2.00 prompt 费用。这是计费假设，不是恢复出的 cache-write token 数；原始 usage 表不变。如果所有 input 均按普通 prompt 单价计算，总费用则为 $7.6498616，即 $7.65。

价格档位按每条响应的总输入核对，包括缓存读取和写入。全部 195 条带 usage 的响应均低于 272,000 tokens 的长上下文门槛，最大值为 215,066，因此不加收长上下文溢价。按缓存写入假设计算的未舍入总费用为 $7.8117586，最后统一舍入为 **$7.81**。Reasoning 已包含在 output 中。Token 总数仅涵盖已报告的 usage；失败请求中未报告的用量仍然未知。

生成时间、源码规模、tokens、活动量和成本均为描述性元数据，不是正确性评分。

## 评测结果

<!-- core-results:begin -->

### POSIX 正确性

| Profile | Classification | Pass | Expected fail | Fail | Unexpected pass | Timeout | Skip N/A | Skip impl | Pass rate | Category macro average |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `spec-tests` | `valid` | 44 | 0 | 0 | 0 | 0 | 7 | 0 | 100.00% | 100.00% |
| `pjdfstest-core` | `diagnostic` | 169 | 0 | 0 | 0 | 0 | 48 | 0 | 100.00% | 100.00% |
| `xfstests-core` | `valid` | 183 | 1 | 4 | 0 | 6 | 473 | 0 | 94.85% | 89.62% |

`spec-tests` run: `eval-core-gpt-5-6-sol-v2-r1`

`pjdfstest-core` run: `case-correction-core-gpt-5.6-sol-v2-pjdfstest-core`

`xfstests-core` run: `eval-core-gpt-5-6-sol-v2-r1`

### 稳健性

Profile: `robustness`

结果：`candidate-failed`；评测分类：`valid`。

| Metric | Value |
|---|---:|
| Total | 47 |
| Pass | 40 |
| Fail | 7 |
| Timeout | 0 |
| Pass rate | 85.11% |
| Category macro average | 78.75% |

超时计失败，N/A 不计入分母，类别宏平均按类别等权。

Run: `eval-submission-robustness-v2-core-gpt-5-6-sol-v2-r1`

### 磁盘格式一致性

Profile: `canonical-format`

结果：`candidate-failed`；评测分类：`diagnostic`。

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
| 结构安全 | 99.54 | 215 | 1 | 0 |
| 内容与持久性 | 100.00 | 216 | 0 | 0 |
| clean 生命周期 | 98.15 | 212 | 4 | 0 |

派生 crash 分 **99.44**；结构 / 内容 / clean 预算为 40% / 40% / 20%。合法安全拒绝 38；已观察两次生命周期 176；第二次被阻断 2；首次未观察 0。安全拒绝计契约满足，不表示恢复了内容；被阻断的第二次不计为通过。

Run: `case-correction-core-gpt-5.6-sol-v2-crash-core`

### 真实应用

Profile: `real-world`

结果：`candidate-failed`；评测分类：`diagnostic`。

| Metric | Value |
|---|---:|
| Total | 4 |
| Pass | 2 |
| Fail | 0 |
| Timeout | 2 |
| Pass rate | 50.00% |
| Category macro average | 33.33% |

Run: `core-realworld-sol-oracle3-r6`
执行后端: `hyperv`

### CPU 性能

Profile: `perf-cpu`

结果：`candidate-failed`；评测分类：`diagnostic`。

性能分：**26.68**（比较池 `hyperv-fixed-vhdx`）。

| 类别 | 缓存 | 主指标 | 样本数 | 中位数 | MAD | CV | 候选失败样本 |
|---|---|---|---:|---:|---:|---:|---:|
| buffered-synchronized-writes | cold | operations_per_second | 5 | 102.5 | 3.1 | 8.0% | 0 |
| buffered-synchronized-writes | warm | operations_per_second | 5 | 105.4 | 1.0 | 5.6% | 0 |
| bulk-sequential-io | cold | MiB/s | 5 | 28.4 | 0.4 | 4.7% | 0 |
| bulk-sequential-io | warm | MiB/s | 5 | 28.9 | 0.8 | 7.9% | 0 |
| metadata-concurrency | cold | operations_per_second | 3 | 2,902.4 | 5.7 | 0.2% | 0 |
| metadata-concurrency | cold | operations_per_second | — | — | — | — | 0 |
| metadata-concurrency | cold | operations_per_second | — | — | — | — | 0 |
| metadata-concurrency | cold | operations_per_second | 2 | 4,024.6 | 21.4 | 0.5% | 0 |
| metadata-concurrency | cold | operations_per_second | — | — | — | — | 1 |
| metadata-concurrency | cold | operations_per_second | 2 | 420.8 | 24.6 | 5.9% | 0 |
| metadata-concurrency | cold | operations_per_second | 5 | 883.3 | 48.0 | 5.8% | 0 |
| metadata-concurrency | warm | operations_per_second | 2 | 2,894.6 | 10.9 | 0.4% | 0 |
| metadata-concurrency | warm | operations_per_second | — | — | — | — | 0 |
| metadata-concurrency | warm | operations_per_second | — | — | — | — | 0 |
| metadata-concurrency | warm | operations_per_second | 2 | 4,026.1 | 14.2 | 0.4% | 0 |
| metadata-concurrency | warm | operations_per_second | — | — | — | — | 1 |
| metadata-concurrency | warm | operations_per_second | 2 | 429.6 | 13.4 | 3.1% | 0 |
| metadata-concurrency | warm | operations_per_second | 4 | 967.8 | 8.6 | 1.5% | 0 |
| small-random-io | cold | iops | 5 | 361.9 | 38.2 | 8.5% | 0 |
| small-random-io | cold | iops | 5 | 2,662.0 | 1,506.9 | 54.6% | 0 |
| small-random-io | cold | iops | 5 | 6,803.6 | 123.3 | 2.6% | 0 |
| small-random-io | warm | iops | 5 | 377.7 | 27.7 | 6.3% | 0 |
| small-random-io | warm | iops | 5 | 491.6 | 258.4 | 109.8% | 0 |
| small-random-io | warm | iops | 5 | 6,653.3 | 90.1 | 3.4% | 0 |

Run: `perf-v2-core-gpt-5-6-sol-v2-perf-cpu-r1`
执行后端: `hyperv`

### NVMe 性能

Profile: `perf-nvme`

结果：`candidate-failed`；评测分类：`diagnostic`。

性能分：**20.51**（比较池 `hyperv-fixed-vhdx`）。

| 类别 | 缓存 | 主指标 | 样本数 | 中位数 | MAD | CV | 候选失败样本 |
|---|---|---|---:|---:|---:|---:|---:|
| buffered-synchronized-writes | cold | operations_per_second | 5 | 20.1 | 0.3 | 2.2% | 0 |
| buffered-synchronized-writes | warm | operations_per_second | 3 | 20.5 | 0.0 | 0.6% | 0 |
| bulk-sequential-io | cold | MiB/s | 5 | 13.3 | 0.1 | 0.7% | 0 |
| bulk-sequential-io | warm | MiB/s | 5 | 24.9 | 0.1 | 0.6% | 0 |
| metadata-concurrency | cold | operations_per_second | 2 | 2,888.2 | 10.0 | 0.3% | 0 |
| metadata-concurrency | cold | operations_per_second | — | — | — | — | 0 |
| metadata-concurrency | cold | operations_per_second | — | — | — | — | 0 |
| metadata-concurrency | cold | operations_per_second | 2 | 3,983.9 | 46.4 | 1.2% | 0 |
| metadata-concurrency | cold | operations_per_second | — | — | — | — | 1 |
| metadata-concurrency | cold | operations_per_second | — | — | — | — | 0 |
| metadata-concurrency | cold | operations_per_second | 3 | 869.8 | 5.9 | 0.7% | 0 |
| metadata-concurrency | warm | operations_per_second | 2 | 2,868.9 | 11.3 | 0.4% | 0 |
| metadata-concurrency | warm | operations_per_second | — | — | — | — | 0 |
| metadata-concurrency | warm | operations_per_second | — | — | — | — | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 4,023.6 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | — | — | — | — | 1 |
| metadata-concurrency | warm | operations_per_second | — | — | — | — | 0 |
| metadata-concurrency | warm | operations_per_second | 5 | 879.0 | 42.2 | 6.5% | 0 |
| small-random-io | cold | iops | 5 | 89.0 | 1.3 | 1.8% | 0 |
| small-random-io | cold | iops | 5 | 296.0 | 231.4 | 114.3% | 0 |
| small-random-io | cold | iops | 5 | 1,605.1 | 367.5 | 44.7% | 0 |
| small-random-io | warm | iops | 5 | 88.2 | 0.8 | 1.2% | 0 |
| small-random-io | warm | iops | 5 | 60.1 | 10.6 | 65.9% | 0 |
| small-random-io | warm | iops | 5 | 1,736.9 | 560.2 | 51.4% | 0 |

Run: `perf-v2-core-gpt-5-6-sol-v2-perf-nvme-r2`
执行后端: `hyperv`

### Agent 代码评审

S：**31.15** / 100；状态：`complete`；用途：`diagnostic`。

| Critical | High | Medium | Low |
|---:|---:|---:|---:|
| 2 | 5 | 13 | 1 |

仅统计已确认、按根因去重的问题；以 20% 权重计入暂计总分。

Scoring rule: `agent-review-score-v2`.

## 评测范围与溯源

各 profile 原始结果独立保留；索引另列未测项暂计 0 的派生总分。每个 profile 列出其运行，并标明已记录的执行后端；不同后端不视为完全相同的执行条件。

`valid` 表示该 profile 的证据有效，不等于全部用例通过或全维度发布合格。`candidate-failed` 是候选失败，不是基础设施错误。`diagnostic` 仅诊断；`noneligible` 不具备排名资格。Hyper-V 性能不具备发布/排名资格；协议 v2 在固定 VHDX/内存设备上的 Hyper-V 运行单独组成 `hyperv-fixed-vhdx` 比较池计入暂计总分，与 KVM/raw-NVMe 分数不可比。

可维护性（`maint-v3-policy`，占总分 5%）：**75.69** / 100，各项见 README。Reference 单列为校准基线，不是模型生成条目。
<!-- core-results:end -->
