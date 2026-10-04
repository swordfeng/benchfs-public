# GPT-6 Astra — Core — High

[English](gpt-6-astra-high.md)

本页汇总独立的 Core Eval profile 结果；各项分别列出评测状态与运行来源，未测维度不补造结果。

## 运行信息

| 字段 | 值 |
|---|---|
| 模型 | `gpt-6-astra` |
| Effort | High |
| Variant | Core |
| Agent harness | Pi |
| 生成运行次数 | 1（`n=1`） |
| SDK/POSIX 评测运行 | `eval-core-gpt-6-astra` |
| SDK/POSIX 初次评测日期 | 2026-09-05（UTC） |
| SDK/POSIX 成绩更新日期 | 2026-09-06（UTC） |

实际 effort 为 High，由操作者确认 proxy override 的生效配置。Pi session 的本地 thinking-level 设置不代表 provider 侧的实际 effort。

## 提交代码规模

仅统计 `bench/` 下候选自行编写的源码。固定 SDK、FUSE adapter、依赖、文档、配置、lockfile 和生成产物均不计入。

| 组成 | 包含该组成的文件数 | 物理行数 | 代码行数 |
|---|---:|---:|---:|
| 实现（Rust） | 9 | 4,184 | 4,114 |
| Agent 自写测试及测试辅助代码（Rust） | 5 | 1,110 | 1,093 |
| Qualification 与诊断脚本（Python / shell） | 3 | 146 | 119 |

物理行数包含空行和注释。代码行数统计至少包含一个非注释 token 的行：空行、注释和文档注释（Rust 的 `///`、`//!`，Python docstring）不计入，`#[derive(...)]` 等 Rust 属性计为代码。2026-09-27 更正：本页早先的版本把文档注释计为代码、把 Rust 属性计为注释；文件数和物理行数不变。测试包括 `src/` 下三个独立文件、一个内联测试模块及测试专用模块声明。两个 Rust 文件同时含有实现与测试组成，因此文件数存在重叠，物理行数和代码行数不重复。共有 12 个不同的 Rust 文件和三个脚本。这些是源码规模数据，不是测试覆盖率或代码质量评分。

<!-- source-tree:begin -->
### 目录结构

`bench/` 下的全部文件，不含构建产物。数字为按上述规则统计的代码行，“测试”为其中属于测试代码的部分。

```text
bench/
├── scripts/
│   ├── check_mkfs.py  (108 行代码)
│   ├── service-daemon.sh  (10 行代码)
│   └── trace-daemon.sh  (1 行代码)
├── src/
│   ├── content.rs  (795 行代码)
│   ├── fault_tests.rs  (125 行代码，测试 125)
│   ├── format.rs  (706 行代码，测试 24)
│   ├── fs.rs  (1,161 行代码，测试 3)
│   ├── lib.rs  (8 行代码)
│   ├── main.rs  (43 行代码)
│   ├── mkfs.rs  (55 行代码)
│   ├── scale_tests.rs  (461 行代码，测试 461)
│   ├── storage.rs  (513 行代码)
│   ├── tests.rs  (480 行代码，测试 480)
│   ├── tree.rs  (563 行代码)
│   └── validate.rs  (297 行代码)
├── Cargo.toml
├── README.md
├── WORKLOG.md
└── qualification.json
```
<!-- source-tree:end -->

## 生成时间与活动量

| 指标 | 值 |
|---|---|
| Goal 开始 | 2026-09-05 09:23:52.569 UTC |
| Goal 完成 | 2026-09-05 13:16:43.263 UTC |
| Goal 活跃时间 | **3 小时 52 分 51 秒** |
| 自然经过时间 | 3 小时 52 分 51 秒 |
| 最终 goal 状态 | Complete（harness 记录） |
| Proxy API 请求数 / HTTP 响应数 | 214 / 214 |
| 含非零 token 用量的 assistant 响应记录数 | 209 |
| Assistant 错误记录数 | 4 |
| Agent 工具调用数 | 245 |

活跃时间来自唯一 Pi goal 最终累计计数，为 13,970.694 秒。这是 harness 记账时间，不是模型推理耗时。Goal 完成不代表所有评测测试均通过。

请求数来自归档 proxy 日志；assistant 和工具调用数来自 Pi session。Proxy 记录了 213 条 HTTP 200 响应和一条 HTTP 405 响应。Session 有 213 条 assistant 记录，其中 209 条响应有非零用量。这些是分别记录的计数，不可互换，也不保证每个失败请求的用量均被完整记录。

### 工具使用统计

| 工具 | 请求次数 | 已记录失败次数 | 结果未知次数 |
|---|---:|---:|---:|
| `bash` | 83 | 7 | 0 |
| `edit` | 69 | 0 | 0 |
| `goal_complete` | 1 | 0 | 0 |
| `read` | 73 | 0 | 0 |
| `write` | 19 | 0 | 0 |
| **合计** | **245** | **7** | **0** |

归档 assistant 中每个 `toolCall` 按记录的工具名计一次请求，重试分别计数。通过 `toolCallId` 配对结果并核对工具名，仅将 `toolResult.isError: true` 计为已记录失败。结果缺失或无法解析时计为未知，不推定成功或失败。这些是工具级结果，不是模型 API 错误数或评测测试结果。

## Token 用量与 API 费用

| 指标 | 值 |
|---|---:|
| 非缓存输入 tokens（`input`） | 617,656 |
| 缓存读取输入 tokens（`cache_read`） | 41,738,752 |
| 缓存写入输入 tokens（`cache_write`） | 0 |
| 输入 tokens 合计，含缓存 | 42,356,408 |
| 输出 tokens，含 thinking（`output`） | 199,142 |
| Thinking / reasoning tokens | 117,099 |
| **Tokens 总计** | **42,555,550** |
| 缓存命中率，按输入 tokens | 98.54% |
| API 费用估算（USD） | **$77.86** |

Token 数由 Pi assistant-message usage 记录求和，与归档 proxy 总量及最终累计 goal token 计数均一致。全部 209 条非零用量响应均明确记录 reasoning 用量；它属于 output 的组成部分，不再重复相加。输入总量包含跨请求反复发送的上下文，不是唯一上下文 token 数。

费用采用 [OpenRouter Models API](https://openrouter.ai/api/v1/models) 中 `openai/gpt-6-astra` 的非 batch 模型目录价格，并由 [Astra endpoint 价格](https://openrouter.ai/api/v1/models/openai/gpt-6-astra/endpoints)中的标准 OpenAI endpoint 佐证，查询日期为 2026-09-06。不以 Flex 或 priority endpoint 价格为基准。这是当前公开标价估算，不证明运行当日价格、实际 endpoint 路由或 provider 账单；不使用 session 中的 cost 字段。

| Token 类别 | USD / 百万 tokens | 估算费用（USD） |
|---|---:|---:|
| 普通档输入，假定用于填充缓存 | $12.50 | $6.422363 |
| 普通档输出 | $50.00 | $5.653200 |
| 普通档缓存读取 | $1.00 | $26.745728 |
| 长上下文档输入，假定用于填充缓存 | $25.00 | $2.596675 |
| 长上下文档输出 | $75.00 | $6.455850 |
| 长上下文档缓存读取 | $2.00 | $29.986048 |

[OpenRouter 的 OpenAI 缓存规则](https://openrouter.ai/docs/guides/best-practices/prompt-caching#openai)对 GPT-5.6 及更新模型的缓存写入按普通输入价格的 1.25 倍收费，包括自动缓存。Session 中单独的 cache-write 计数为零，但无法确定多少非缓存输入用于填充缓存。这里保守地将全部 617,656 个非缓存输入 tokens 按写入价估算，替代而非叠加普通 $10.00 / $20.00 输入费。这是计费假设，不是恢复出的写入 token 数；用量表保持原始记录。

按每条响应的总输入（含缓存读写）选择价格档位。209 条非零用量响应中，161 条低于 272,000-token 阈值，48 条达到长上下文档；最大输入为 370,473 tokens。普通档包含 513,789 个非缓存输入、26,745,728 个缓存读取和 113,064 个输出 tokens。长上下文档包含 103,867 个非缓存输入、14,993,024 个缓存读取和 86,078 个输出 tokens。

缓存写入假设下的未舍入总额为 $77.8598635，最终统一舍入为 **$77.86**。若非缓存输入改按普通 prompt 费率计费、仍保留相同长上下文档位，总额为 $76.056056，即 $76.06。Reasoning 已包含在 output 中。失败请求未报告的用量仍未知，不计入估算。

生成时间、源码规模、tokens、活动量和费用均为描述性元数据，不是正确性评分。

## 评测结果

<!-- core-results:begin -->

### POSIX 正确性

| Profile | Classification | Pass | Expected fail | Fail | Unexpected pass | Timeout | Skip N/A | Skip impl | Pass rate | Category macro average |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `spec-tests` | `valid` | 44 | 0 | 0 | 0 | 0 | 7 | 0 | 100.00% | 100.00% |
| `pjdfstest-core` | `diagnostic` | 169 | 0 | 0 | 0 | 0 | 48 | 0 | 100.00% | 100.00% |
| `xfstests-core` | `valid` | 178 | 1 | 11 | 0 | 4 | 473 | 0 | 92.27% | 89.31% |

`spec-tests` run: `eval-core-gpt-6-astra`

`pjdfstest-core` run: `case-correction-core-gpt-6-astra-pjdfstest-core`

`xfstests-core` run: `eval-core-gpt-6-astra`

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

Run: `eval-submission-robustness-v2-core-gpt-6-astra-r1`

### 磁盘格式一致性

Profile: `canonical-format`

结果：`pass`；评测分类：`diagnostic`。

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
执行后端: `hyperv`

### 崩溃一致性

Profile: `crash-core`

结果：`pass`；评测分类：`diagnostic`。

| Metric | Value |
|---|---:|
| Total | 216 |
| Pass | 216 |
| Fail | 0 |
| Timeout | 0 |
| Pass rate | 100.00% |
| Category macro average | 100.00% |

R 轴采用已观察能力分；上表仍为原始端到端结果。

| 能力 | 分数 / 100 | 满足 | 失败 | 未证明（暂计 0） |
|---|---:|---:|---:|---:|
| 结构安全 | 100.00 | 216 | 0 | 0 |
| 内容与持久性 | 100.00 | 216 | 0 | 0 |
| clean 生命周期 | 100.00 | 216 | 0 | 0 |

派生 crash 分 **100.00**；结构 / 内容 / clean 预算为 40% / 40% / 20%。合法安全拒绝 206；已观察两次生命周期 10；第二次被阻断 0；首次未观察 0。安全拒绝计契约满足，不表示恢复了内容；被阻断的第二次不计为通过。

Run: `case-correction-core-gpt-6-astra-crash-core`

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

Run: `core-realworld-astra-20260925-r2`
执行后端: `qemu-kvm`

### CPU 性能

Profile: `perf-cpu`

结果：`candidate-failed`；评测分类：`diagnostic`。

性能分：**56.88**（比较池 `hyperv-fixed-vhdx`）。

| 类别 | 缓存 | 主指标 | 样本数 | 中位数 | MAD | CV | 候选失败样本 |
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
执行后端: `hyperv`

### NVMe 性能

Profile: `perf-nvme`

结果：`candidate-failed`；评测分类：`diagnostic`。

性能分：**54.13**（比较池 `hyperv-fixed-vhdx`）。

| 类别 | 缓存 | 主指标 | 样本数 | 中位数 | MAD | CV | 候选失败样本 |
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
执行后端: `hyperv`

### Agent 代码评审

S：**90.91** / 100；状态：`complete`；用途：`diagnostic`。

| Critical | High | Medium | Low |
|---:|---:|---:|---:|
| 0 | 0 | 2 | 0 |

仅统计已确认、按根因去重的问题；以 20% 权重计入暂计总分。

Scoring rule: `agent-review-score-v2`.

## 评测范围与溯源

各 profile 原始结果独立保留；索引另列未测项暂计 0 的派生总分。每个 profile 列出其运行，并标明已记录的执行后端；不同后端不视为完全相同的执行条件。

`valid` 表示该 profile 的证据有效，不等于全部用例通过或全维度发布合格。`candidate-failed` 是候选失败，不是基础设施错误。`diagnostic` 仅诊断；`noneligible` 不具备排名资格。Hyper-V 性能不具备发布/排名资格；协议 v2 在固定 VHDX/内存设备上的 Hyper-V 运行单独组成 `hyperv-fixed-vhdx` 比较池计入暂计总分，与 KVM/raw-NVMe 分数不可比。

可维护性（`maint-v3-policy`，占总分 5%）：**87.63** / 100，各项见 README。Reference 单列为校准基线，不是模型生成条目。
<!-- core-results:end -->
