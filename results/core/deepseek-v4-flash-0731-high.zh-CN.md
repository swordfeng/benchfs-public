# DeepSeek V4 Flash 0731 — Core — High

[English](deepseek-v4-flash-0731-high.md)

本页汇总独立的 Core Eval profile 结果；各项分别列出评测状态与运行来源，未测维度不补造结果。

## 运行信息

| 字段 | 值 |
|---|---|
| 模型 | `deepseek-v4-flash-0731` |
| Effort | High |
| Variant | Core |
| Agent harness | Pi |
| 生成运行次数 | 1（`n=1`） |
| SDK/POSIX 评测运行 | `eval-core-deepseek-v4-flash-0731` |
| SDK/POSIX 初次评测日期 | 2026-09-05（UTC） |
| SDK/POSIX 成绩更新日期 | 2026-09-06（UTC） |

实际 effort 为 High，由操作者确认 proxy override 的生效配置。Pi session 的本地 thinking-level 设置不代表 provider 侧的实际 effort。

## 提交代码规模

仅统计 `bench/` 下候选自行编写的源码。固定 SDK、FUSE adapter、依赖、文档、配置、lockfile 和生成产物均不计入。

| 组成 | 包含该组成的文件数 | 物理行数 | 代码行数 |
|---|---:|---:|---:|
| 实现（Rust） | 12 | 7,144 | 6,226 |
| Agent 自写测试及测试辅助代码（Rust） | 2 | 643 | 572 |
| Qualification 脚本 | 0 | 0 | 0 |

物理行数包含空行和注释。代码行数统计至少包含一个非注释 token 的行：空行、注释和文档注释（Rust 的 `///`、`//!`，Python docstring）不计入，`#[derive(...)]` 等 Rust 属性计为代码。2026-09-27 更正：本页早先的版本把文档注释计为代码、把 Rust 属性计为注释；文件数和物理行数不变。测试包括一个独立 integration-test 文件，以及实现文件中明确注明供测试使用的辅助函数。该文件在两类文件数中重复出现，但物理行数和代码行数互不重复。共有 13 个不同的 Rust 文件。这些是源码规模数据，不是测试覆盖率或代码质量评分。

<!-- source-tree:begin -->
### 目录结构

`bench/` 下的全部文件，不含构建产物。数字为按上述规则统计的代码行，“测试”为其中属于测试代码的部分。

```text
bench/
├── src/
│   ├── fs/
│   │   ├── btree.rs  (744 行代码)
│   │   ├── checker.rs  (668 行代码，测试 7)
│   │   ├── format.rs  (780 行代码)
│   │   ├── format_image.rs  (106 行代码)
│   │   ├── inode.rs  (514 行代码)
│   │   ├── layout.rs  (55 行代码)
│   │   ├── mod.rs  (269 行代码)
│   │   ├── ops.rs  (2,337 行代码)
│   │   └── state.rs  (621 行代码)
│   ├── lib.rs  (1 行代码)
│   ├── main.rs  (47 行代码)
│   └── mkfs.rs  (91 行代码)
├── tests/
│   └── core.rs  (565 行代码，测试 565)
└── Cargo.toml
```
<!-- source-tree:end -->

## 生成时间与活动量

| 指标 | 值 |
|---|---|
| Goal 开始 | 2026-09-01 16:38:42.243 UTC |
| 最后一次 goal 状态更新 | 2026-09-02 04:58:31.350 UTC |
| 最后记录的 goal 活跃时间 | **12 小时 19 分 49 秒** |
| 截至最后 goal 状态更新的自然经过时间 | 12 小时 19 分 49 秒 |
| 最后记录的 goal 状态 | Blocked；harness 未记录完成 |
| 最后一条 session 记录 | 2026-09-02 05:01:41.760 UTC |
| Proxy API 请求数 / HTTP 响应数 | 476 / 476 |
| 含非零 token 用量的 assistant 响应记录数 | 476 |
| Assistant 错误记录数 | 2 |
| Agent 工具调用数 | 482 |

该唯一 goal 最后累计的活跃时间为 44,389.107 秒。此后还有三条 assistant 记录报告了 835,162 tokens；其用量已计入下表，但停止更新的 goal 计时器没有计入这段时间。因此，归档 session 不能确立最终生成完成时间。活跃时间是 harness 记账时间，不是模型推理耗时。独立完成的 Eval 评测的是归档候选，不受已记录 goal 状态的概念混淆影响。

请求数来自归档 proxy 日志；assistant 和工具调用数来自 Pi session。476 条 HTTP 响应均为 200。Session 有 478 条 assistant 记录，包括两条没有用量的错误记录。这些计数不是可互换的指标，也不保证未报告用量的失败请求没有产生费用。

### 工具使用统计

| 工具 | 请求次数 | 已记录失败次数 | 结果未知次数 |
|---|---:|---:|---:|
| `bash` | 459 | 51 | 1 |
| `edit` | 1 | 0 | 0 |
| `h` | 1 | 1 | 0 |
| `read` | 15 | 0 | 0 |
| `write` | 6 | 1 | 0 |
| **合计** | **482** | **53** | **1** |

归档 assistant 中每个 `toolCall` 按记录的工具名计一次请求，重试分别计数。通过 `toolCallId` 配对结果并核对工具名，仅将 `toolResult.isError: true` 计为已记录失败。结果缺失或无法解析时计为未知，不推定成功或失败。这些是工具级结果，不是模型 API 错误数或评测测试结果。

一次 `bash` 请求没有匹配的归档结果，因此 53 是已观测失败次数，不是完整失败总数。请求过的工具 `h` 按原名保留。

## Token 用量与 API 费用

| 指标 | 值 |
|---|---:|
| 非缓存输入 tokens（`input`） | 11,723,837 |
| 缓存读取输入 tokens（`cache_read`） | 150,212,352 |
| 缓存写入输入 tokens（`cache_write`） | 0 |
| 输入 tokens 合计，含缓存 | 161,936,189 |
| 输出 tokens，含 thinking（`output`） | 518,078 |
| Thinking / reasoning tokens | 257,581 |
| **Tokens 总计** | **162,454,267** |
| 缓存命中率，按输入 tokens | 92.76% |
| API 费用估算（USD） | **$3.26** |

Token 数由 Pi assistant-message usage 记录求和，与归档 proxy 总量一致。最后 goal token 计数仅为 161,619,105；加上后续报告的 835,162 tokens 后与 session 总量吻合。全部 476 条非零用量响应均明确记录 reasoning 用量；它属于 output 的组成部分，不再重复相加。输入总量包含跨请求反复发送的上下文，不是唯一上下文 token 数。

费用采用 [OpenRouter Models API](https://openrouter.ai/api/v1/models) 中 `deepseek/deepseek-v4-flash-0731` 的非 batch 模型目录价格；[DeepSeek endpoint 价格](https://openrouter.ai/api/v1/models/deepseek/deepseek-v4-flash-0731/endpoints)中的 Relace endpoint 也列出相同费率，查询日期为 2026-09-06。这是当前公开标价估算，不证明运行当日价格、实际 endpoint 路由或 provider 账单；不使用 session 中的 cost 字段。

| Token 类别 | USD / 百万 tokens | 估算费用（USD） |
|---|---:|---:|
| 非缓存输入 / 缓存未命中 | $0.065 | $0.762049 |
| 输出 | $0.18 | $0.093254 |
| 缓存读取 | $0.016 | $2.403398 |

[OpenRouter 的 DeepSeek 缓存规则](https://openrouter.ai/docs/guides/best-practices/prompt-caching#deepseek)按普通输入价格收取缓存写入费用。因此非缓存输入只计费一次，不额外添加写入溢价。该模型目录未列出长上下文加价档位。未舍入总额为 $3.258701077，最终统一舍入为 **$3.26**。失败请求未报告的用量仍未知，不计入估算。

生成时间、源码规模、tokens、活动量和费用均为描述性元数据，不是正确性评分。

## 评测结果

<!-- core-results:begin -->

### POSIX 正确性

| Profile | Classification | Pass | Expected fail | Fail | Unexpected pass | Timeout | Skip N/A | Skip impl | Pass rate | Category macro average |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `spec-tests` | `valid` | 44 | 0 | 0 | 0 | 0 | 7 | 0 | 100.00% | 100.00% |
| `pjdfstest-core` | `diagnostic` | 169 | 0 | 0 | 0 | 0 | 48 | 0 | 100.00% | 100.00% |
| `xfstests-core` | `valid` | 166 | 1 | 13 | 0 | 14 | 473 | 0 | 86.08% | 84.02% |

`spec-tests` run: `eval-core-deepseek-v4-flash-0731`

`pjdfstest-core` run: `case-correction-core-deepseek-v4-flash-0731-pjdfstest-core`

`xfstests-core` run: `eval-core-deepseek-v4-flash-0731`

### 稳健性

Profile: `robustness`

结果：`candidate-failed`；评测分类：`valid`。

| Metric | Value |
|---|---:|
| Total | 47 |
| Pass | 35 |
| Fail | 9 |
| Timeout | 3 |
| Pass rate | 74.47% |
| Category macro average | 69.58% |

超时计失败，N/A 不计入分母，类别宏平均按类别等权。

Run: `eval-submission-robustness-v2-core-deepseek-v4-flash-0731-r2`

### 磁盘格式一致性

Profile: `canonical-format`

结果：`candidate-failed`；评测分类：`diagnostic`。

| Metric | Value |
|---|---:|
| Total | 479 |
| Pass | 388 |
| Fail | 91 |
| Timeout | 0 |
| Pass rate | 81.00% |
| Category macro average | 73.56% |
| Conformance score (0–100) | 74.86 |
| Full conformance | no |

Run: `core-canonical-deepseek-oracle3-r5`
执行后端: `hyperv`

### 崩溃一致性

Profile: `crash-core`

结果：`candidate-failed`；评测分类：`diagnostic`。

| Metric | Value |
|---|---:|
| Total | 216 |
| Pass | 206 |
| Fail | 10 |
| Timeout | 0 |
| Pass rate | 95.37% |
| Category macro average | 95.37% |

Run: `case-correction-core-deepseek-v4-flash-0731-crash-core`

### 真实应用

Profile: `real-world`

结果：`candidate-failed`；评测分类：`diagnostic`。

| Metric | Value |
|---|---:|
| Total | 4 |
| Pass | 2 |
| Fail | 2 |
| Timeout | 0 |
| Pass rate | 50.00% |
| Category macro average | 66.67% |

Run: `core-realworld-deepseek-oracle3-r6`
执行后端: `hyperv`

### CPU 性能

Profile: `perf-cpu`

结果：`pass`；评测分类：`diagnostic`。

性能分：**39.63**（比较池 `hyperv-fixed-vhdx`）。

| 类别 | 缓存 | 主指标 | 样本数 | 中位数 | MAD | CV | 候选失败样本 |
|---|---|---|---:|---:|---:|---:|---:|
| buffered-synchronized-writes | cold | operations_per_second | 2 | 1,990.7 | 6.8 | 0.3% | 0 |
| buffered-synchronized-writes | warm | operations_per_second | 2 | 2,007.2 | 1.3 | 0.1% | 0 |
| bulk-sequential-io | cold | MiB/s | 3 | 34.8 | 0.1 | 0.6% | 0 |
| bulk-sequential-io | warm | MiB/s | 3 | 34.7 | 0.0 | 0.8% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 1,062.4 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 5,302.3 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 504.2 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 1,242.3 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 546.4 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 1,390.0 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 36.3 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 1,052.3 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 6,679.6 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 505.6 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 1,241.8 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 544.1 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 1,388.1 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 38.3 | 0.0 | 0.0% | 0 |
| small-random-io | cold | iops | 2 | 3,468.8 | 51.3 | 1.5% | 0 |
| small-random-io | cold | iops | 2 | 6,671.2 | 110.4 | 1.7% | 0 |
| small-random-io | cold | iops | 2 | 6,973.4 | 10.9 | 0.2% | 0 |
| small-random-io | warm | iops | 2 | 3,464.6 | 50.2 | 1.5% | 0 |
| small-random-io | warm | iops | 2 | 4,971.4 | 1,167.6 | 23.5% | 0 |
| small-random-io | warm | iops | 2 | 6,925.4 | 10.3 | 0.1% | 0 |

Run: `perf-v2-core-deepseek-v4-flash-0731-perf-cpu-r6`
执行后端: `hyperv`

### NVMe 性能

Profile: `perf-nvme`

结果：`pass`；评测分类：`diagnostic`。

性能分：**40.10**（比较池 `hyperv-fixed-vhdx`）。

| 类别 | 缓存 | 主指标 | 样本数 | 中位数 | MAD | CV | 候选失败样本 |
|---|---|---|---:|---:|---:|---:|---:|
| buffered-synchronized-writes | cold | operations_per_second | 2 | 1,227.8 | 8.2 | 0.7% | 0 |
| buffered-synchronized-writes | warm | operations_per_second | 2 | 1,221.2 | 3.8 | 0.3% | 0 |
| bulk-sequential-io | cold | MiB/s | 2 | 15.6 | 0.2 | 1.1% | 0 |
| bulk-sequential-io | warm | MiB/s | 2 | 15.6 | 0.1 | 0.4% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 661.5 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 5,323.9 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 336.5 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 752.9 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 315.3 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 556.5 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 21.2 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 664.7 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 6,770.2 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 332.9 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 750.3 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 305.6 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 557.4 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 22.3 | 0.0 | 0.0% | 0 |
| small-random-io | cold | iops | 2 | 1,922.9 | 15.1 | 0.8% | 0 |
| small-random-io | cold | iops | 2 | 2,394.0 | 60.5 | 2.5% | 0 |
| small-random-io | cold | iops | 1 | 3,061.8 | 0.0 | 0.0% | 0 |
| small-random-io | warm | iops | 2 | 1,901.6 | 41.1 | 2.2% | 0 |
| small-random-io | warm | iops | 2 | 1,847.9 | 6.5 | 0.3% | 0 |
| small-random-io | warm | iops | 1 | 2,974.7 | 0.0 | 0.0% | 0 |

Run: `perf-v2-core-deepseek-v4-flash-0731-perf-nvme-r6`
执行后端: `hyperv`

## 评测范围与溯源

各 profile 原始结果独立保留；索引另列未测项暂计 0 的派生总分。每个 profile 列出其运行，并标明已记录的执行后端；不同后端不视为完全相同的执行条件。

`valid` 表示该 profile 的证据有效，不等于全部用例通过或全维度发布合格。`candidate-failed` 是候选失败，不是基础设施错误。`diagnostic` 仅诊断；`noneligible` 不具备排名资格。Hyper-V 性能不具备发布/排名资格；协议 v2 在固定 VHDX/内存设备上的 Hyper-V 运行单独组成 `hyperv-fixed-vhdx` 比较池计入暂计总分，与 KVM/raw-NVMe 分数不可比。

安全审查尚未评定。可维护性（`maint-v2-rev5`，占总分 5%）：**79.85** / 100，各项见 README。Reference 单列为校准基线，不是模型生成条目。
<!-- core-results:end -->
