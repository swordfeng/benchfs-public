# Qwen 3.8 Flash Next — Core — High

[English](qwen-3.8-flash-next-high.md)

本页汇总独立的 Core Eval profile 结果；各项分别列出评测状态与运行来源，未测维度不补造结果。

## 运行信息

| 字段 | 值 |
|---|---|
| 模型 | `qwen-3.8-flash-next` |
| Effort | High |
| Variant | Core |
| Agent harness | Pi |
| 生成运行次数 | 1（`n=1`） |
| SDK/POSIX 评测运行 | `eval-core-qwen-3.8-flash-next` |
| SDK/POSIX 评测日期 | 2026-09-06（UTC） |

实际 effort 为 High，由操作者确认 proxy override 的生效配置。Pi session 的本地 thinking-level 设置不代表 provider 侧的实际 effort。

## 提交代码规模

仅统计 `bench/` 下候选自行编写的源码。固定 SDK、FUSE adapter、依赖、文档、配置、lockfile 和生成产物均不计入。

| 组成 | 包含该组成的文件数 | 物理行数 | 代码行数 |
|---|---:|---:|---:|
| 实现（Rust） | 13 | 8,740 | 7,530 |
| Agent 自写测试及测试辅助代码（Rust） | 3 | 829 | 763 |
| Qualification 脚本 | 0 | 0 | 0 |

物理行数包含空行和注释。代码行数统计至少包含一个非注释 token 的行：空行、注释和文档注释（Rust 的 `///`、`//!`，Python docstring）不计入，`#[derive(...)]` 等 Rust 属性计为代码。2026-09-27 更正：本页早先的版本把文档注释计为代码、把 Rust 属性计为注释；文件数和物理行数不变。测试包括一个独立 integration-test 文件，以及两个实现文件中的内联测试模块。这两个文件在组成文件数中重复出现，但物理行数和代码行数互不重复。共有 14 个不同的 Rust 文件。这些是源码规模数据，不是测试覆盖率或代码质量评分。

<!-- source-tree:begin -->
### 目录结构

`bench/` 下的全部文件，不含构建产物。数字为按上述规则统计的代码行，“测试”为其中属于测试代码的部分。

```text
bench/
├── src/
│   ├── boot.rs  (516 行代码)
│   ├── check.rs  (687 行代码)
│   ├── device.rs  (50 行代码)
│   ├── engine.rs  (2,392 行代码)
│   ├── format.rs  (1,121 行代码)
│   ├── formatter.rs  (214 行代码)
│   ├── fs.rs  (1,563 行代码)
│   ├── geom.rs  (402 行代码，测试 36)
│   ├── lib.rs  (9 行代码)
│   ├── main.rs  (106 行代码)
│   ├── mkfs.rs  (95 行代码)
│   ├── sync.rs  (250 行代码)
│   └── tree.rs  (205 行代码，测试 44)
├── tests/
│   └── model.rs  (683 行代码，测试 683)
└── Cargo.toml
```
<!-- source-tree:end -->

## 生成时间与活动量

| 指标 | 值 |
|---|---|
| Goal 开始 | 2026-09-04 16:46:44.784 UTC |
| 最后 goal 状态更新 / 最后一条 session 记录 | 2026-09-05 09:13:21.102 UTC |
| 最后记录的 goal 活跃时间 | **12 小时 0 分 54 秒** |
| 截至最后 goal 状态更新的自然经过时间，含间隔 | 16 小时 26 分 36 秒 |
| 最后记录的 goal 状态 | Active；harness 未记录完成 |
| 归档 proxy API 请求数 / HTTP 响应数（仅部分时段） | 37 / 36 |
| 含非零 token 用量的 assistant 响应记录数 | 519 |
| Assistant 错误记录数 | 41 |
| Assistant 中止记录数 | 3 |
| Agent 工具调用数 | 522 |

Pi goal 最后累计记录的活跃时间为 43,253.875 秒。七个 goal identity 属于恢复继续的同一次生成，不能将快照相加。归档结束时 goal 仍为 active，不能确立最终生成完成时间。活跃时间是 harness 记账时间，不是模型推理耗时。独立完成的 Eval 评测的是归档候选，不受已记录 goal 状态的概念混淆影响。

Proxy 归档只覆盖尾段：用量时间戳范围为 2026-09-05 08:45:19.305732–09:05:08.231703 UTC。它记录了 37 个请求、36 条 HTTP 200 响应及仅 33 条 usage。这是局部计数，不是完整生成过程的请求量。Pi session 有 563 条 assistant 记录，其中 519 条含非零用量。两行格式损坏的 session 内容均为 tool-result，不是 assistant-usage，解析时排除；未发现损坏的 assistant-usage 记录。工具调用从 assistant 记录统计，不依赖 tool-result 行数。

### 工具使用统计

| 工具 | 请求次数 | 已记录失败次数 | 结果未知次数 |
|---|---:|---:|---:|
| `bash` | 481 | 24 | 2 |
| `edit` | 7 | 1 | 0 |
| `read` | 13 | 0 | 0 |
| `write` | 21 | 1 | 0 |
| **合计** | **522** | **26** | **2** |

归档 assistant 中每个 `toolCall` 按记录的工具名计一次请求，重试分别计数。通过 `toolCallId` 配对结果并核对工具名，仅将 `toolResult.isError: true` 计为已记录失败。结果缺失或无法解析时计为未知，不推定成功或失败。这些是工具级结果，不是模型 API 错误数或评测测试结果。

排除两条格式损坏的工具结果记录后，522 次请求中有 520 条结果能够配对。两次 `bash` 的结果仍未知；26 是已观测失败次数，不是完整失败总数。

## Token 用量与 API 费用

| 指标 | 值 |
|---|---:|
| 非缓存输入 tokens（`input`） | 2,582,958 |
| 缓存读取输入 tokens（`cache_read`） | 151,479,808 |
| 缓存写入输入 tokens（`cache_write`） | 0 |
| 输入 tokens 合计，含缓存 | 154,062,766 |
| 输出 tokens，含 thinking（`output`） | 734,568 |
| Thinking / reasoning tokens | 480,599 |
| **Tokens 总计** | **154,797,334** |
| 缓存命中率，按输入 tokens | 98.32% |
| API 费用估算（USD） | **$3.29** |

Token 数由 Pi assistant-message usage 记录求和，不采用仅含尾段的 proxy 汇总值 14,748,770。记录中发生过 goal-token 重置，不能单独使用最后计数：重置前 89,446,529 加上最终计数 65,350,805，等于 154,797,334，与 session 求和吻合。不累加中间累计快照。全部 519 条非零用量响应均明确记录 reasoning 用量；它属于 output 的组成部分，不再重复相加。输入总量包含跨请求反复发送的上下文，不是唯一上下文 token 数。

费用采用 [OpenRouter Models API](https://openrouter.ai/api/v1/models) 中 `qwen/qwen3.8-flash` 的非 batch 模型目录价格，并由 [Qwen endpoint 价格](https://openrouter.ai/api/v1/models/qwen/qwen3.8-flash/endpoints)佐证，查询日期为 2026-09-06。目录将其 Hugging Face 仓库标为 `Qwen/Qwen3.8-Flash-Next`，据此对应目录简称与提交名称。这是当前公开标价估算，不证明运行当日价格、实际 endpoint 路由或 provider 账单；不使用 session 中的 cost 字段。

| Token 类别 | USD / 百万 tokens | 估算费用（USD） |
|---|---:|---:|
| 输入，假定用于填充缓存 | $0.20 | $0.516592 |
| 输出 | $0.47 | $0.345247 |
| 缓存读取 | $0.016 | $2.423677 |

模型目录及 Alibaba endpoint 列出的缓存写入价为 $0.20，普通 prompt 价为 $0.15。Session 中单独的写入计数为零，但无法确定多少非缓存输入用于填充缓存。这里保守地将全部 2,582,958 个非缓存输入 tokens 按列出的写入价估算，替代而非叠加普通输入费。[OpenRouter 的 Alibaba 缓存文档](https://openrouter.ai/docs/guides/best-practices/prompt-caching#alibaba-qwen)说明了独立写入计价；此处采用该模型目录费率，而非文档中的通用倍率。这是计费假设，不是恢复出的 cache-write 数量；用量表保持原始记录。

该模型目录未列出长上下文加价档位。缓存写入假设下的未舍入总额为 $3.285515488，最终统一舍入为 **$3.29**。若非缓存输入改按普通 prompt 费率计价，总额为 $3.156367588，即 $3.16。失败或中止请求未报告的用量仍未知，不计入估算。

生成时间、源码规模、tokens、活动量和费用均为描述性元数据，不是正确性评分。

## 评测结果

<!-- core-results:begin -->

### POSIX 正确性

| Profile | Classification | Pass | Expected fail | Fail | Unexpected pass | Timeout | Skip N/A | Skip impl | Pass rate | Category macro average |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `spec-tests` | `valid` | 44 | 0 | 0 | 0 | 0 | 7 | 0 | 100.00% | 100.00% |
| `pjdfstest-core` | `diagnostic` | 169 | 0 | 0 | 0 | 0 | 48 | 0 | 100.00% | 100.00% |
| `xfstests-core` | `valid` | 167 | 1 | 24 | 0 | 2 | 473 | 0 | 86.60% | 81.52% |

`spec-tests` run: `eval-core-qwen-3.8-flash-next`

`pjdfstest-core` run: `case-correction-core-qwen-3.8-flash-next-pjdfstest-core`

`xfstests-core` run: `eval-core-qwen-3.8-flash-next`

### 稳健性

Profile: `robustness`

结果：`candidate-failed`；评测分类：`valid`。

| Metric | Value |
|---|---:|
| Total | 47 |
| Pass | 32 |
| Fail | 15 |
| Timeout | 0 |
| Pass rate | 68.09% |
| Category macro average | 66.25% |

超时计失败，N/A 不计入分母，类别宏平均按类别等权。

Run: `eval-submission-robustness-v2-core-qwen-3-8-flash-next-r1`

### 磁盘格式一致性

Profile: `canonical-format`

结果：`candidate-failed`；评测分类：`diagnostic`。

| Metric | Value |
|---|---:|
| Total | 479 |
| Pass | 368 |
| Fail | 111 |
| Timeout | 0 |
| Pass rate | 76.83% |
| Category macro average | 67.51% |
| Conformance score (0–100) | 75.33 |
| Full conformance | no |

Run: `core-canonical-qwen-oracle3-r5`
执行后端: `hyperv`

### 崩溃一致性

Profile: `crash-core`

结果：`candidate-failed`；评测分类：`diagnostic`。

| Metric | Value |
|---|---:|
| Total | 216 |
| Pass | 150 |
| Fail | 66 |
| Timeout | 0 |
| Pass rate | 69.44% |
| Category macro average | 69.44% |

Run: `case-correction-core-qwen-3.8-flash-next-crash-core`

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
| Category macro average | 33.33% |

Run: `core-realworld-qwen-oracle3-r6`
执行后端: `hyperv`

### CPU 性能

Profile: `perf-cpu`

结果：`candidate-failed`；评测分类：`diagnostic`。

性能分：**40.38**（比较池 `hyperv-fixed-vhdx`）。

| 类别 | 缓存 | 主指标 | 样本数 | 中位数 | MAD | CV | 候选失败样本 |
|---|---|---|---:|---:|---:|---:|---:|
| buffered-synchronized-writes | cold | operations_per_second | 3 | 2,623.1 | 3.5 | 0.4% | 0 |
| buffered-synchronized-writes | warm | operations_per_second | 3 | 2,627.0 | 15.7 | 0.5% | 0 |
| bulk-sequential-io | cold | MiB/s | 5 | 244.9 | 1.7 | 1.0% | 0 |
| bulk-sequential-io | warm | MiB/s | 5 | 244.0 | 3.3 | 1.2% | 0 |
| metadata-concurrency | cold | operations_per_second | — | — | — | — | 1 |
| metadata-concurrency | cold | operations_per_second | 3 | 7,261.3 | 4.3 | 0.2% | 0 |
| metadata-concurrency | cold | operations_per_second | 3 | 1,945.8 | 0.3 | 0.1% | 0 |
| metadata-concurrency | cold | operations_per_second | 3 | 3,510.7 | 9.5 | 0.5% | 0 |
| metadata-concurrency | cold | operations_per_second | 3 | 2,881.7 | 7.1 | 0.2% | 0 |
| metadata-concurrency | cold | operations_per_second | 3 | 969.4 | 9.1 | 1.1% | 0 |
| metadata-concurrency | cold | operations_per_second | — | — | — | — | 1 |
| metadata-concurrency | warm | operations_per_second | 1 | 2,474.2 | 0.0 | 0.0% | 1 |
| metadata-concurrency | warm | operations_per_second | 3 | 9,160.0 | 102.4 | 1.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 3 | 1,945.7 | 0.4 | 0.2% | 0 |
| metadata-concurrency | warm | operations_per_second | 3 | 3,512.2 | 14.6 | 0.4% | 0 |
| metadata-concurrency | warm | operations_per_second | 3 | 2,894.3 | 1.8 | 0.2% | 0 |
| metadata-concurrency | warm | operations_per_second | 4 | 949.2 | 19.4 | 2.1% | 0 |
| metadata-concurrency | warm | operations_per_second | — | — | — | — | 1 |
| small-random-io | cold | iops | 3 | 4,580.5 | 20.3 | 0.5% | 0 |
| small-random-io | cold | iops | 5 | 6,368.1 | 4.5 | 0.2% | 0 |
| small-random-io | cold | iops | 5 | 3,666.4 | 20.0 | 0.9% | 0 |
| small-random-io | warm | iops | 3 | 4,567.2 | 1.8 | 0.6% | 0 |
| small-random-io | warm | iops | 3 | 6,380.8 | 8.1 | 0.2% | 0 |
| small-random-io | warm | iops | 5 | 3,695.2 | 32.3 | 0.9% | 0 |

Run: `perf-v2-core-qwen-3-8-flash-next-perf-cpu-r1`
执行后端: `hyperv`

### NVMe 性能

Profile: `perf-nvme`

结果：`candidate-failed`；评测分类：`diagnostic`。

性能分：**43.24**（比较池 `hyperv-fixed-vhdx`）。

| 类别 | 缓存 | 主指标 | 样本数 | 中位数 | MAD | CV | 候选失败样本 |
|---|---|---|---:|---:|---:|---:|---:|
| buffered-synchronized-writes | cold | operations_per_second | 3 | 1,753.9 | 6.1 | 0.6% | 0 |
| buffered-synchronized-writes | warm | operations_per_second | 3 | 1,749.7 | 8.9 | 0.5% | 0 |
| bulk-sequential-io | cold | MiB/s | 5 | 178.2 | 0.3 | 0.8% | 0 |
| bulk-sequential-io | warm | MiB/s | 5 | 177.3 | 0.5 | 0.6% | 0 |
| metadata-concurrency | cold | operations_per_second | — | — | — | — | 1 |
| metadata-concurrency | cold | operations_per_second | 2 | 5,624.7 | 61.0 | 1.1% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 1,563.2 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 3 | 3,082.4 | 5.3 | 0.5% | 0 |
| metadata-concurrency | cold | operations_per_second | 3 | 1,978.7 | 5.2 | 0.9% | 0 |
| metadata-concurrency | cold | operations_per_second | 4 | 378.0 | 2.1 | 1.9% | 0 |
| metadata-concurrency | cold | operations_per_second | — | — | — | — | 1 |
| metadata-concurrency | warm | operations_per_second | — | — | — | — | 1 |
| metadata-concurrency | warm | operations_per_second | 2 | 8,998.9 | 145.6 | 1.6% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 1,571.1 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 3 | 3,083.7 | 4.6 | 0.6% | 0 |
| metadata-concurrency | warm | operations_per_second | 4 | 2,034.7 | 15.1 | 1.7% | 0 |
| metadata-concurrency | warm | operations_per_second | 4 | 378.4 | 5.3 | 2.0% | 0 |
| metadata-concurrency | warm | operations_per_second | — | — | — | — | 1 |
| small-random-io | cold | iops | 3 | 2,638.2 | 17.9 | 0.6% | 0 |
| small-random-io | cold | iops | 5 | 3,196.1 | 8.0 | 0.3% | 0 |
| small-random-io | cold | iops | 5 | 1,826.4 | 5.5 | 0.6% | 0 |
| small-random-io | warm | iops | 3 | 2,610.6 | 1.9 | 0.2% | 0 |
| small-random-io | warm | iops | 5 | 3,198.5 | 7.7 | 0.4% | 0 |
| small-random-io | warm | iops | 5 | 1,844.4 | 7.8 | 0.5% | 0 |

Run: `perf-v2-core-qwen-3-8-flash-next-perf-nvme-r2`
执行后端: `hyperv`

## 评测范围与溯源

各 profile 原始结果独立保留；索引另列未测项暂计 0 的派生总分。每个 profile 列出其运行，并标明已记录的执行后端；不同后端不视为完全相同的执行条件。

`valid` 表示该 profile 的证据有效，不等于全部用例通过或全维度发布合格。`candidate-failed` 是候选失败，不是基础设施错误。`diagnostic` 仅诊断；`noneligible` 不具备排名资格。Hyper-V 性能不具备发布/排名资格；协议 v2 在固定 VHDX/内存设备上的 Hyper-V 运行单独组成 `hyperv-fixed-vhdx` 比较池计入暂计总分，与 KVM/raw-NVMe 分数不可比。

安全审查与可维护性评分尚未评定。Reference 单列为校准基线，不是模型生成条目。
<!-- core-results:end -->
