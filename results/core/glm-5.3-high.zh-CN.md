# GLM 5.3 — Core — High

[English](glm-5.3-high.md)

本页汇总独立的 Core Eval profile 结果；各项分别列出评测状态与运行来源，未测维度不补造结果。

## 运行信息

| 字段 | 值 |
|---|---|
| 模型 | `glm-5.3` |
| Effort | High |
| Variant | Core |
| Agent harness | Pi |
| 生成运行次数 | 1（`n=1`） |
| SDK/POSIX 评测运行 | `eval-core-glm-5.3` |
| SDK/POSIX 评测日期 | 2026-09-06（UTC） |

实际 effort 为 High，由操作者确认 proxy override 的生效配置。Pi session 的本地 thinking-level 设置不代表 provider 侧的实际 effort。

## 提交代码规模

仅统计 `bench/` 下候选自行编写的源码。固定 SDK、FUSE adapter、依赖、文档、配置、lockfile 和生成产物均不计入。

| 组成 | 包含该组成的文件数 | 物理行数 | 代码行数 |
|---|---:|---:|---:|
| 实现（Rust） | 9 | 7,923 | 6,936 |
| Agent 自写测试及测试辅助代码（Rust） | 4 | 694 | 629 |
| Qualification 脚本 | 0 | 0 | 0 |

物理行数包含空行和注释。代码行数统计至少包含一个非注释 token 的行：空行、注释和文档注释（Rust 的 `///`、`//!`，Python docstring）不计入，`#[derive(...)]` 等 Rust 属性计为代码。2026-09-27 更正：本页早先的版本把文档注释计为代码、把 Rust 属性计为注释；文件数和物理行数不变。测试包括四个实现文件中的内联测试模块和三个测试专用辅助方法。文件数存在重叠，物理行数和代码行数互不重复。共有九个不同的 Rust 文件，没有独立测试文件或脚本。这些是源码规模数据，不是测试覆盖率或代码质量评分。

<!-- source-tree:begin -->
### 目录结构

`bench/` 下的全部文件，不含构建产物。数字为按上述规则统计的代码行，“测试”为其中属于测试代码的部分。

```text
bench/
├── src/
│   ├── btree.rs  (779 行代码，测试 225)
│   ├── core.rs  (3,475 行代码，测试 234)
│   ├── device.rs  (833 行代码，测试 119)
│   ├── format.rs  (1,352 行代码，测试 51)
│   ├── lib.rs  (6 行代码)
│   ├── main.rs  (59 行代码)
│   ├── mkfs.rs  (257 行代码)
│   ├── mkfs_main.rs  (67 行代码)
│   └── validate.rs  (737 行代码)
├── Cargo.toml
└── NOTES.md
```
<!-- source-tree:end -->

## 生成时间与活动量

| 指标 | 值 |
|---|---|
| Goal 开始 | 2026-09-04 16:40:57.061 UTC |
| Goal 完成 | 2026-09-05 18:10:08.085 UTC |
| Goal 活跃时间 | **7 小时 32 分 19 秒** |
| 自然经过时间，含活跃阶段之间的间隔 | 25 小时 29 分 11 秒 |
| 最终 goal 状态 | Complete（harness 记录） |
| Proxy API 请求数 / HTTP 响应数 | 629 / 629 |
| 含非零 token 用量的 assistant 响应记录数 | 613 |
| Assistant 错误记录数 | 16 |
| Agent 工具调用数 | 615 |

活跃时间来自 Pi goal 最终累计计数，为 27,139.120 秒。四个 goal identity 属于恢复继续的同一次生成，并继承此前计数，不能将快照相加。这是 harness 记账时间，不是模型推理耗时。Goal 完成不代表所有评测测试均通过。

请求数来自归档 proxy 日志；assistant 和工具调用数来自 Pi session。Proxy 记录了 613 条 HTTP 200 响应和 16 条 HTTP 429 响应。全部 16 条 assistant 错误记录的用量均为零。用量记录为零不证明失败请求没有产生上游费用。

### 工具使用统计

| 工具 | 请求次数 | 已记录失败次数 | 结果未知次数 |
|---|---:|---:|---:|
| `bash` | 585 | 45 | 0 |
| `edit` | 5 | 1 | 0 |
| `goal_complete` | 1 | 0 | 0 |
| `read` | 9 | 0 | 0 |
| `write` | 15 | 1 | 0 |
| **合计** | **615** | **47** | **0** |

归档 assistant 中每个 `toolCall` 按记录的工具名计一次请求，重试分别计数。通过 `toolCallId` 配对结果并核对工具名，仅将 `toolResult.isError: true` 计为已记录失败。结果缺失或无法解析时计为未知，不推定成功或失败。这些是工具级结果，不是模型 API 错误数或评测测试结果。

## Token 用量与 API 费用

| 指标 | 值 |
|---|---:|
| 非缓存输入 tokens（估算） | 15,248,264 |
| 缓存读取输入 tokens（估算） | 280,413,851 |
| 缓存写入输入 tokens | 此规则不估算独立写入量 |
| 输入 tokens 合计，含缓存 | 295,662,115 |
| 输出 tokens，含 thinking（`output`） | 541,837 |
| Thinking / reasoning tokens（留存文本估算） | ≈241,702 |
| **Tokens 总计** | **296,203,952** |
| 缓存命中率，按输入 tokens（估算） | 94.84% |
| API 费用估算（USD，前缀 + 五分钟启发式） | **$62.99** |

输入合计、输出和总 tokens 由 Pi assistant-message usage 记录求和，与归档 proxy 总量及最终累计 goal token 计数均一致。全部 613 条非零用量记录的 reasoning 字段均为零，但 304 条 assistant 记录保留了 thinking 文本；不能把零当作“没有 reasoning”的实测结果。输入总量包含跨请求反复发送的上下文，不是唯一上下文 token 数。

Reasoning 估算对 947,869 个字符进行分词，来自 613 条非零用量响应中的 304 条、共 304 个留存 thinking 块。采用 revision `aca966e4e02791568aa6a4ced368624b3d897f42` 的[公开 GLM 5.3 tokenizer](https://huggingface.co/zai-org/GLM-5.3/blob/aca966e4e02791568aa6a4ced368624b3d897f42/tokenizer.json)，使用 `tokenizers 0.23.2`。每个块独立编码，不添加特殊 token、聊天封装或签名；全部 304 个块均验证可解码还原原文。此数值只估算留存文本，不代表完整 reasoning 用量或 provider 计费 tokens，也不外推到没有留存文本的响应。不加入 output、总 tokens 或费用。

Pi 将全部 295,662,115 个 prompt tokens 记录在 `input` 中，`cacheRead` 和 `cacheWrite` 均为零。Proxy 日志仅独立保存 prompt、completion 和 total 计数，没有保存缓存拆分或原始 usage 响应。表格采用重建消息前缀加操作者指定的五分钟空闲失效规则；这些是估算值，不是 provider 缓存实测值。

沿 session 父记录链重建每次成功请求之前的消息历史，按内容比较规范化消息块：保留用户文本、assistant 文本及带签名的 thinking、工具调用和结果，将已持久化的 custom message 转为 user message；排除 error/aborted assistant 消息、usage/时间戳元数据、工具结果 details 和 goal-state 记账事件。此处理遵循相关 [Pi 消息转换](https://github.com/earendil-works/pi/blob/bfb004d4418ff05c6f909eaaab856cbe75c1fde0/packages/coding-agent/src/core/messages.ts)及[历史过滤](https://github.com/earendil-works/pi/blob/bfb004d4418ff05c6f909eaaab856cbe75c1fde0/packages/ai/src/api/transform-messages.ts)规则。612 次相邻成功请求比较中，上一轮重建历史均完整保留为本轮的精确消息级前缀。本次运行没有 compaction、分支、图片或需要合成结果的未完成工具调用。

首次成功请求按冷缓存处理。后续每次成功请求的空闲间隔，统一使用 host proxy 时钟，计算“上一条成功响应的 usage 日志时间 → 本次请求开始时间”。严格超过 300 秒时，本轮输入全部计为非缓存；否则，由于上一轮消息前缀完整保留，估算缓存读取为上一条成功请求报告的输入 tokens。16 条 HTTP 429 响应没有报告用量，不刷新假定缓存。计时终点是本次请求开始，而非本次响应结束。

613 条成功请求的输入计数与 proxy 序列完全一致，从 2,792 增长到 702,541 tokens。空闲规则触发 24 次失效；连同首次请求，共 25 次冷请求、588 次暖请求。相比不设过期的估算，减少 14,545,723 个缓存读取 tokens，得到缓存读取 280,413,851 tokens、非缓存输入 15,248,264 tokens，估算命中率 94.84%。本次前缀检查本身没有减少命中量，调整全部来自五分钟规则。

这是消息重建估算，不是最终 provider 请求的逐 token 比较。历史 system prompt、工具定义、hook 处理后的最终 payload 及 provider 渲染均未留档；将上一轮报告的输入量视为可复用 token 前缀，假定这些不可观测部分保持稳定且不会更早破坏前缀。五分钟是操作者定义的空闲 TTL，不是已验证的 GLM provider 策略。实际缓存路由、逐出、最小可缓存长度及独立缓存写入量仍未测得。

条件计价采用 [OpenRouter Models API](https://openrouter.ai/api/v1/models) 中 `z-ai/glm-5.3` 的非 batch 模型目录价格，查询日期为 2026-09-06。[GLM endpoint 价格](https://openrouter.ai/api/v1/models/z-ai/glm-5.3/endpoints)列出多个 provider 费率；不以较便宜的 endpoint 替换模型目录口径。这些是当前公开标价假设，不证明运行当日价格、实际 endpoint 路由或 provider 账单；不使用 session 中的 cost 字段。

| Token 类别 | USD / 百万 tokens | 估算费用（USD） |
|---|---:|---:|
| 非缓存输入（估算） | $1.40 | $21.347570 |
| 输出 | $4.40 | $2.384083 |
| 缓存读取（估算） | $0.14 | $39.257939 |

[OpenRouter 的 Z.AI 缓存规则](https://openrouter.ai/docs/guides/best-practices/prompt-caching#zai)在查询日期未列出单独写入或存储费，模型目录也未列出长上下文加价档位。采用估算的缓存拆分，未舍入费用为 `(15,248,264 × 1.40 + 280,413,851 × 0.14 + 541,837 × 4.40) / 1,000,000 = $62.98959154`，最终统一舍入为 **$62.99**。这是操作者定义的前缀及过期估算，不是恢复出的账单。失败请求未报告的用量仍未知。

生成时间、源码规模、tokens、活动量和费用均为描述性元数据，不是正确性评分。

## 评测结果

<!-- core-results:begin -->

### POSIX 正确性

| Profile | Classification | Pass | Expected fail | Fail | Unexpected pass | Timeout | Skip N/A | Skip impl | Pass rate | Category macro average |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `spec-tests` | `valid` | 44 | 0 | 0 | 0 | 0 | 7 | 0 | 100.00% | 100.00% |
| `pjdfstest-core` | `diagnostic` | 169 | 0 | 0 | 0 | 0 | 48 | 0 | 100.00% | 100.00% |
| `xfstests-core` | `valid` | 182 | 1 | 5 | 0 | 6 | 473 | 0 | 94.33% | 89.17% |

`spec-tests` run: `eval-core-glm-5.3`

`pjdfstest-core` run: `case-correction-core-glm-5.3-pjdfstest-core`

`xfstests-core` run: `eval-core-glm-5.3`

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

Run: `eval-submission-robustness-v2-core-glm-5-3-r3`

### 磁盘格式一致性

Profile: `canonical-format`

结果：`candidate-failed`；评测分类：`diagnostic`。

| Metric | Value |
|---|---:|
| Total | 479 |
| Pass | 417 |
| Fail | 62 |
| Timeout | 0 |
| Pass rate | 87.06% |
| Category macro average | 94.01% |
| Conformance score (0–100) | 94.05 |
| Full conformance | no |

Run: `core-canonical-glm-oracle3-r5`
执行后端: `hyperv`

### 崩溃一致性

Profile: `crash-core`

结果：`candidate-failed`；评测分类：`diagnostic`。

| Metric | Value |
|---|---:|
| Total | 216 |
| Pass | 215 |
| Fail | 1 |
| Timeout | 0 |
| Pass rate | 99.54% |
| Category macro average | 99.54% |

Run: `case-correction-core-glm-5.3-crash-core`

### 真实应用

Profile: `real-world`

结果：`candidate-failed`；评测分类：`diagnostic`。

| Metric | Value |
|---|---:|
| Total | 4 |
| Pass | 3 |
| Fail | 1 |
| Timeout | 0 |
| Pass rate | 75.00% |
| Category macro average | 83.33% |

Run: `core-realworld-glm-20260925-r1`
执行后端: `hyperv`

### CPU 性能

Profile: `perf-cpu`

结果：`pass`；评测分类：`diagnostic`。

性能分：**36.89**（比较池 `hyperv-fixed-vhdx`）。

| 类别 | 缓存 | 主指标 | 样本数 | 中位数 | MAD | CV | 候选失败样本 |
|---|---|---|---:|---:|---:|---:|---:|
| buffered-synchronized-writes | cold | operations_per_second | 3 | 1,611.7 | 1.4 | 0.1% | 0 |
| buffered-synchronized-writes | warm | operations_per_second | 3 | 1,599.4 | 2.9 | 0.4% | 0 |
| bulk-sequential-io | cold | MiB/s | 5 | 60.9 | 0.1 | 0.5% | 0 |
| bulk-sequential-io | warm | MiB/s | 5 | 61.1 | 0.5 | 0.7% | 0 |
| metadata-concurrency | cold | operations_per_second | 2 | 1,143.9 | 2.0 | 0.2% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 5,564.7 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 775.9 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 2 | 1,287.0 | 0.7 | 0.1% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 822.5 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 2 | 1,021.5 | 5.6 | 0.5% | 0 |
| metadata-concurrency | cold | operations_per_second | 3 | 421.3 | 2.1 | 0.8% | 0 |
| metadata-concurrency | warm | operations_per_second | 2 | 1,141.6 | 1.3 | 0.1% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 5,606.5 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 770.9 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 1,285.6 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 825.2 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 2 | 1,004.3 | 3.5 | 0.4% | 0 |
| metadata-concurrency | warm | operations_per_second | 3 | 420.4 | 0.8 | 0.8% | 0 |
| small-random-io | cold | iops | 3 | 2,885.0 | 2.7 | 0.1% | 0 |
| small-random-io | cold | iops | 5 | 3,258.6 | 648.8 | 43.5% | 0 |
| small-random-io | cold | iops | 3 | 7,023.8 | 36.3 | 0.6% | 0 |
| small-random-io | warm | iops | 3 | 2,886.1 | 9.6 | 0.3% | 0 |
| small-random-io | warm | iops | 5 | 2,999.3 | 398.8 | 37.7% | 0 |
| small-random-io | warm | iops | 5 | 7,025.9 | 15.9 | 0.4% | 0 |

Run: `perf-v2-core-glm-5-3-perf-cpu-r1`
执行后端: `hyperv`

### NVMe 性能

Profile: `perf-nvme`

结果：`pass`；评测分类：`diagnostic`。

性能分：**39.57**（比较池 `hyperv-fixed-vhdx`）。

| 类别 | 缓存 | 主指标 | 样本数 | 中位数 | MAD | CV | 候选失败样本 |
|---|---|---|---:|---:|---:|---:|---:|
| buffered-synchronized-writes | cold | operations_per_second | 3 | 896.5 | 7.7 | 1.0% | 0 |
| buffered-synchronized-writes | warm | operations_per_second | 3 | 902.5 | 0.4 | 0.3% | 0 |
| bulk-sequential-io | cold | MiB/s | 5 | 28.6 | 0.2 | 0.8% | 0 |
| bulk-sequential-io | warm | MiB/s | 5 | 28.8 | 0.1 | 0.5% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 673.6 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 5,531.9 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 523.0 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 733.2 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 422.3 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 444.1 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 3 | 263.8 | 1.2 | 0.7% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 658.7 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 5,670.1 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 521.8 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 734.7 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 458.0 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 441.2 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 3 | 261.9 | 0.5 | 0.9% | 0 |
| small-random-io | cold | iops | 3 | 1,507.0 | 0.3 | 0.1% | 0 |
| small-random-io | cold | iops | 4 | 2,497.7 | 694.4 | 28.3% | 0 |
| small-random-io | cold | iops | 4 | 2,907.0 | 140.6 | 6.2% | 0 |
| small-random-io | warm | iops | 3 | 1,505.6 | 0.8 | 0.2% | 0 |
| small-random-io | warm | iops | 4 | 1,369.7 | 83.2 | 21.9% | 0 |
| small-random-io | warm | iops | 4 | 3,099.2 | 49.9 | 2.0% | 0 |

Run: `perf-v2-core-glm-5-3-perf-nvme-r1`
执行后端: `hyperv`

## 评测范围与溯源

各 profile 原始结果独立保留；索引另列未测项暂计 0 的派生总分。每个 profile 列出其运行，并标明已记录的执行后端；不同后端不视为完全相同的执行条件。

`valid` 表示该 profile 的证据有效，不等于全部用例通过或全维度发布合格。`candidate-failed` 是候选失败，不是基础设施错误。`diagnostic` 仅诊断；`noneligible` 不具备排名资格。Hyper-V 性能不具备发布/排名资格；协议 v2 在固定 VHDX/内存设备上的 Hyper-V 运行单独组成 `hyperv-fixed-vhdx` 比较池计入暂计总分，与 KVM/raw-NVMe 分数不可比。

安全审查与可维护性评分尚未评定。Reference 单列为校准基线，不是模型生成条目。
<!-- core-results:end -->
