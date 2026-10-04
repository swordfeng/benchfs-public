# Gemini 3.8 Flash — Core — High

[English](gemini-3.8-flash-high.md)

本页汇总独立的 Core Eval profile 结果；各项分别列出评测状态与运行来源，未测维度不补造结果。

## 运行信息

| 字段 | 值 |
|---|---|
| 模型 | `gemini-3.8-flash` |
| Effort | High |
| Variant | Core |
| Agent harness | Pi |
| 生成运行次数 | 1（`n=1`） |
| SDK/POSIX 评测运行 | `eval-core-gemini-3.8-flash` |
| SDK/POSIX 初次评测日期 | 2026-09-05（UTC） |
| SDK/POSIX 成绩更新日期 | 2026-09-06（UTC） |

实际 effort 为 High，由操作者确认 proxy override 的生效配置。Pi session 的本地 thinking-level 设置不代表 provider 侧的实际 effort。

## 提交代码规模

仅统计 `bench/` 下候选自行编写的源码。固定 SDK、FUSE adapter、依赖、文档、配置、lockfile 和生成产物均不计入。

| 组成 | 包含该组成的文件数 | 物理行数 | 代码行数 |
|---|---:|---:|---:|
| 实现（Rust） | 11 | 5,244 | 4,417 |
| Agent 自写测试（Rust） | 0 | 0 | 0 |
| Qualification 脚本 | 0 | 0 | 0 |

物理行数包含空行和注释。代码行数统计至少包含一个非注释 token 的行：空行、注释和文档注释（Rust 的 `///`、`//!`，Python docstring）不计入，`#[derive(...)]` 等 Rust 属性计为代码。2026-09-27 更正：本页早先的版本把文档注释计为代码、把 Rust 属性计为注释；文件数和物理行数不变。归档 `bench/` 目录共有 11 个不同的 Rust 文件，没有独立测试、内联测试模块或脚本。这不代表 agent 在开发期间没有运行测试。这些是源码规模数据，不是测试覆盖率或代码质量评分。

<!-- source-tree:begin -->
### 目录结构

`bench/` 下的全部文件，不含构建产物。数字为按上述规则统计的代码行，“测试”为其中属于测试代码的部分。

```text
bench/
├── src/
│   ├── btree.rs  (318 行代码)
│   ├── btree_mgr.rs  (334 行代码)
│   ├── directory.rs  (150 行代码)
│   ├── extent.rs  (458 行代码)
│   ├── format.rs  (477 行代码)
│   ├── fs.rs  (1,372 行代码)
│   ├── main.rs  (85 行代码)
│   ├── mkfs.rs  (283 行代码)
│   ├── storage.rs  (427 行代码)
│   ├── superblock.rs  (268 行代码)
│   └── xattr.rs  (245 行代码)
└── Cargo.toml
```
<!-- source-tree:end -->

## 生成时间与活动量

| 指标 | 值 |
|---|---|
| Goal 开始 | 2026-09-05 06:40:31.117 UTC |
| Goal 完成 | 2026-09-05 10:06:00.977 UTC |
| Goal 活跃时间 | **3 小时 5 分 17 秒** |
| 自然经过时间，含活跃阶段之间的间隔 | 3 小时 25 分 30 秒 |
| 最终 goal 状态 | Complete（harness 记录） |
| Proxy API 请求数 / HTTP 响应数 | 652 / 652 |
| 含非零 token 用量的 assistant 响应记录数 | 650 |
| Assistant 错误记录数 | 8 |
| Agent 工具调用数 | 642 |

活跃时间来自 Pi goal 最终累计计数，为 11,116.962 秒。三个 goal identity 属于恢复继续的同一次生成，并继承此前计数，不能将快照相加。这是 harness 记账时间，不是模型推理耗时。Goal 完成不代表所有评测测试均通过。

请求数来自归档 proxy 日志；assistant 和工具调用数来自 Pi session。Proxy 记录了 650 条 HTTP 200 响应和两条 HTTP 402 响应；session 有 652 条 assistant 记录。八条 assistant 错误记录中有六条仍报告了用量，已计入 token 总量。错误记录与零用量响应不是同一类别。

### 工具使用统计

| 工具 | 请求次数 | 已记录失败次数 | 结果未知次数 |
|---|---:|---:|---:|
| `bash` | 190 | 34 | 0 |
| `edit` | 57 | 5 | 0 |
| `goal_complete` | 1 | 0 | 0 |
| `read` | 383 | 5 | 0 |
| `write` | 11 | 0 | 0 |
| **合计** | **642** | **44** | **0** |

归档 assistant 中每个 `toolCall` 按记录的工具名计一次请求，重试分别计数。通过 `toolCallId` 配对结果并核对工具名，仅将 `toolResult.isError: true` 计为已记录失败。结果缺失或无法解析时计为未知，不推定成功或失败。这些是工具级结果，不是模型 API 错误数或评测测试结果。

## Token 用量与 API 费用

| 指标 | 值 |
|---|---:|
| 非缓存输入 tokens（`input`） | 16,383,967 |
| 缓存读取输入 tokens（`cache_read`） | 244,400,079 |
| 缓存写入输入 tokens（`cache_write`） | 0 |
| 输入 tokens 合计，含缓存 | 260,784,046 |
| 输出 tokens，含 thinking（`output`） | 267,908 |
| Thinking / reasoning tokens（留存文本粗估） | ≈4,000 |
| **Tokens 总计** | **261,051,954** |
| 缓存命中率，按输入 tokens | 93.72% |
| API 费用估算（USD） | **$31.62** |

Token 数由全部 Pi assistant-message usage 记录求和，包括有用量的错误记录，与归档 proxy 总量及最终累计 goal token 计数均一致。记录中没有独立 reasoning-token 字段；这不表示 reasoning 为零。不在已记录的 output 或 total 之外重复添加 reasoning。输入总量包含跨请求反复发送的上下文，不是唯一上下文 token 数。

留存文本估算覆盖 650 条非零用量响应中的五条、共五个 thinking 块，合计 16,108 个字符。按 [Google 给出的约四字符/token 经验值](https://ai.google.dev/gemini-api/docs/tokens)，`16,108 / 4 = 4,027`，报告为**约 4,000 tokens**，而非精确 tokenizer 计数。这是本地字符粗估，没有运行 Gemini tokenizer，也不是 provider 计费数。[Google 区分思考摘要与内部推理](https://ai.google.dev/gemini-api/docs/thinking#thought-summaries)；留存摘要不能恢复完整内部 thinking 用量。估算不外推到其他响应，也不加入 output、总 tokens 或费用。未将留存文本发送到外部 token-counting API。

费用采用 [OpenRouter Models API](https://openrouter.ai/api/v1/models) 中 `google/gemini-3.8-flash` 的非 batch 模型目录价格，并由 [Gemini endpoint 价格](https://openrouter.ai/api/v1/models/google/gemini-3.8-flash/endpoints)中的标准 Google AI Studio endpoint 佐证，查询日期为 2026-09-06。不以 Flex 或 priority endpoint 价格为基准。这是当前公开标价估算，不证明运行当日价格、实际 endpoint 路由或 provider 账单；不使用 session 中的 cost 字段。

| Token 类别 | USD / 百万 tokens | 估算费用（USD） |
|---|---:|---:|
| 非缓存输入 | $0.75 | $12.287975 |
| 输出，含已计入 output 的 reasoning | $3.75 | $1.004655 |
| 缓存读取 | $0.075 | $18.330006 |

[OpenRouter 的 Google 缓存规则](https://openrouter.ai/docs/guides/best-practices/prompt-caching#google-gemini)区分没有写入或存储费的隐式缓存与显式缓存存储。估算采用已记录的输入、输出和缓存读取，假定没有需要单独计费的显式存储；归档报告零 cache writes，但不能独立确定存储计费模式。目录列出的五分钟缓存写入存储费约为每百万 tokens $0.0416667。作为敏感性计算，若每个非缓存输入 token 均收取一次该费用，将增加约 $0.6826653，总额约为 $32.31。这不是恢复出的写入 token 数或实测费用。

该模型目录未列出长上下文加价档位。按已记录用量计算、未包含可能未记录的显式存储费的总额为 $31.622636175，最终统一舍入为 **$31.62**。目录单列的 reasoning 费率不在 output 之外重复计费。失败请求未报告的用量仍未知，不计入估算。

生成时间、源码规模、tokens、活动量和费用均为描述性元数据，不是正确性评分。

## 评测结果

<!-- core-results:begin -->

### POSIX 正确性

| Profile | Classification | Pass | Expected fail | Fail | Unexpected pass | Timeout | Skip N/A | Skip impl | Pass rate | Category macro average |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `spec-tests` | `valid` | 44 | 0 | 0 | 0 | 0 | 7 | 0 | 100.00% | 100.00% |
| `pjdfstest-core` | `diagnostic` | 169 | 0 | 0 | 0 | 0 | 48 | 0 | 100.00% | 100.00% |
| `xfstests-core` | `valid` | 174 | 1 | 4 | 0 | 15 | 473 | 0 | 90.21% | 84.61% |

`spec-tests` run: `eval-core-gemini-3.8-flash`

`pjdfstest-core` run: `case-correction-core-gemini-3.8-flash-pjdfstest-core`

`xfstests-core` run: `eval-core-gemini-3.8-flash`

### 稳健性

Profile: `robustness`

结果：`candidate-failed`；评测分类：`valid`。

| Metric | Value |
|---|---:|
| Total | 47 |
| Pass | 43 |
| Fail | 4 |
| Timeout | 0 |
| Pass rate | 91.49% |
| Category macro average | 89.58% |

超时计失败，N/A 不计入分母，类别宏平均按类别等权。

Run: `eval-submission-robustness-v2-core-gemini-3-8-flash-r1`

### 磁盘格式一致性

Profile: `canonical-format`

结果：`candidate-failed`；评测分类：`diagnostic`。

| Metric | Value |
|---|---:|
| Total | 479 |
| Pass | 419 |
| Fail | 60 |
| Timeout | 0 |
| Pass rate | 87.47% |
| Category macro average | 71.79% |
| Conformance score (0–100) | 77.66 |
| Full conformance | no |

Run: `core-canonical-gemini-oracle3-r6`
执行后端: `hyperv`

### 崩溃一致性

Profile: `crash-core`

结果：`candidate-failed`；评测分类：`diagnostic`。

| Metric | Value |
|---|---:|
| Total | 216 |
| Pass | 146 |
| Fail | 70 |
| Timeout | 0 |
| Pass rate | 67.59% |
| Category macro average | 67.59% |

上表按用例是否满足全部要求统计结果。下表分别计算各项已验证能力，用于稳健性 R。

| 检查项目 | 分数 / 100 | 满足要求（项） | 未满足（项） | 未验证（计 0） |
|---|---:|---:|---:|---:|
| 磁盘结构 | 67.59 | 146 | 44 | 26 |
| 数据正确性 | 87.96 | 190 | 0 | 26 |
| 卸载与重开 | 67.59 | 146 | 70 | 0 |

崩溃分 **75.74**。权重：磁盘结构 40%、数据正确性 40%、卸载与重开 20%。

允许的安全拒绝：23 项；两轮已测试：123 项；失败后未再次打开：70 项；首轮未测试：0 项。

每轮包括挂载、检查和卸载。安全拒绝计满足要求，不表示数据已恢复。前一步失败导致未测试的再次打开不计通过。各检查项目的含义见[评分方法](../../docs/scoring.zh-CN.md)。

Run: `case-correction-core-gemini-3.8-flash-crash-core`

### 真实应用

Profile: `real-world`

结果：`candidate-failed`；评测分类：`diagnostic`。

| Metric | Value |
|---|---:|
| Total | 4 |
| Pass | 2 |
| Fail | 1 |
| Timeout | 1 |
| Pass rate | 50.00% |
| Category macro average | 50.00% |

Run: `core-realworld-gemini-oracle3-r6`
执行后端: `hyperv`

### CPU 性能

Profile: `perf-cpu`

结果：`pass`；评测分类：`diagnostic`。

性能分：**51.93**（比较池 `hyperv-fixed-vhdx`）。已判定预算 100.0%；未测项对应的得分范围 51.93–51.93，不是置信区间。

分项（满分 100）：文件读写 33.88；元数据 23.85；RAM 100.00；写入效率 99.67。
RAM 峰值：24.2 MiB。元数据：14 项有有效速率，0 项候选失败，0 项未测。

写入放大 = 计时窗口内控制器写入字节数 ÷ 应用逻辑写入字节数；不是 SSD 内部写入放大。元数据操作不使用这个比值。“—”不代表零；候选失败得 0，未测项保留预算并暂计 0。* 表示部分预算未判定。

| 负载 | 缓存 | 样本数 | 速率 | MAD | CV | 候选失败样本 | 写入放大 | 速度分 / 100 |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| 顺序读写 | cold | 4 | 34.8 MiB/s | 0.3 | 0.9% | 0 | 1.000× | 0.00 |
| 顺序读写 | warm | 4 | 34.9 MiB/s | 0.2 | 0.8% | 0 | 1.000× | 0.00 |
| 随机读写（单线程） | cold | 3 | 4,574.7 IOPS | 3.0 | 0.1% | 0 | 1.000× | 29.56 |
| 随机读写（单线程） | warm | 3 | 4,573.3 IOPS | 0.1 | 0.2% | 0 | 1.000× | 29.49 |
| 随机读写（多线程） | cold | 4 | 6,187.4 IOPS | 14.6 | 0.3% | 0 | 1.000× | 23.28 |
| 随机读写（多线程） | warm | 4 | 6,174.6 IOPS | 14.0 | 0.2% | 0 | 1.000× | 23.23 |
| 随机读写（共享区域） | cold | 3 | 2,846.8 IOPS | 15.2 | 1.8% | 0 | 1.000× | 20.31 |
| 随机读写（共享区域） | warm | 3 | 2,941.4 IOPS | 4.7 | 0.3% | 0 | 1.000× | 20.93 |
| 同步覆盖写 | cold | 3 | 2,635.4 ops/s | 2.3 | 0.3% | 0 | 1.031× | 78.68 |
| 同步覆盖写 | warm | 3 | 2,623.5 ops/s | 4.9 | 0.5% | 0 | 1.031× | 78.56 |
| 创建、查询并删除 | cold | 1 | 1,275.0 ops/s | 0.0 | 0.0% | 0 | — | 7.99 |
| 创建、查询并删除 | warm | 1 | 1,274.2 ops/s | 0.0 | 0.0% | 0 | — | 8.64 |
| 深层与大目录查找 | cold | 1 | 7,964.6 ops/s | 0.0 | 0.0% | 0 | — | 16.94 |
| 深层与大目录查找 | warm | 1 | 8,185.5 ops/s | 0.0 | 0.0% | 0 | — | 17.89 |
| 目录内重命名 | cold | 1 | 875.3 ops/s | 0.0 | 0.0% | 0 | — | 12.63 |
| 目录内重命名 | warm | 1 | 871.0 ops/s | 0.0 | 0.0% | 0 | — | 12.19 |
| 同目录并发修改 | cold | 1 | 1,401.1 ops/s | 0.0 | 0.0% | 0 | — | 8.93 |
| 同目录并发修改 | warm | 1 | 1,396.6 ops/s | 0.0 | 0.0% | 0 | — | 9.38 |
| 跨目录并发创建 | cold | 1 | 905.6 ops/s | 0.0 | 0.0% | 0 | — | 5.96 |
| 跨目录并发创建 | warm | 1 | 906.3 ops/s | 0.0 | 0.0% | 0 | — | 5.83 |
| 并发同步落盘 | cold | 2 | 1,080.2 ops/s | 20.2 | 1.9% | 0 | — | 100.00 |
| 并发同步落盘 | warm | 1 | 1,080.0 ops/s | 0.0 | 0.0% | 0 | — | 100.00 |
| 接近满盘时回收空间 | cold | 4 | 64.7 ops/s | 1.0 | 2.1% | 0 | — | 11.55 |
| 接近满盘时回收空间 | warm | 4 | 63.1 ops/s | 0.8 | 1.9% | 0 | — | 9.88 |

Run: `perf-v2-core-gemini-3-8-flash-perf-cpu-r1`
执行后端: `hyperv`

### NVMe 性能

Profile: `perf-nvme`

结果：`pass`；评测分类：`diagnostic`。

性能分：**50.55**（比较池 `hyperv-fixed-vhdx`）。已判定预算 100.0%；未测项对应的得分范围 50.55–50.55，不是置信区间。

分项（满分 100）：文件读写 32.53；元数据 20.00；RAM 100.00；写入效率 99.67。
RAM 峰值：19.5 MiB。元数据：14 项有有效速率，0 项候选失败，0 项未测。

写入放大 = 计时窗口内控制器写入字节数 ÷ 应用逻辑写入字节数；不是 SSD 内部写入放大。元数据操作不使用这个比值。“—”不代表零；候选失败得 0，未测项保留预算并暂计 0。* 表示部分预算未判定。

| 负载 | 缓存 | 样本数 | 速率 | MAD | CV | 候选失败样本 | 写入放大 | 速度分 / 100 |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| 顺序读写 | cold | 2 | 15.7 MiB/s | 0.1 | 0.8% | 0 | 1.000× | 0.00 |
| 顺序读写 | warm | 2 | 15.7 MiB/s | 0.0 | 0.2% | 0 | 1.000× | 0.00 |
| 随机读写（单线程） | cold | 2 | 2,653.1 IOPS | 25.8 | 1.0% | 0 | 1.000× | 28.59 |
| 随机读写（单线程） | warm | 2 | 2,666.8 IOPS | 9.4 | 0.4% | 0 | 1.000× | 28.62 |
| 随机读写（多线程） | cold | 2 | 3,309.0 IOPS | 7.9 | 0.2% | 0 | 1.000× | 21.58 |
| 随机读写（多线程） | warm | 2 | 3,293.4 IOPS | 1.2 | 0.0% | 0 | 1.000× | 21.49 |
| 随机读写（共享区域） | cold | 2 | 1,609.5 IOPS | 8.0 | 0.5% | 0 | 1.000× | 20.01 |
| 随机读写（共享区域） | warm | 2 | 1,607.3 IOPS | 10.4 | 0.6% | 0 | 1.000× | 19.97 |
| 同步覆盖写 | cold | 2 | 1,804.8 ops/s | 5.1 | 0.3% | 0 | 1.031× | 75.60 |
| 同步覆盖写 | warm | 2 | 1,789.3 ops/s | 42.1 | 2.4% | 0 | 1.031× | 75.44 |
| 创建、查询并删除 | cold | 1 | 565.0 ops/s | 0.0 | 0.0% | 0 | — | 0.00 |
| 创建、查询并删除 | warm | 1 | 565.4 ops/s | 0.0 | 0.0% | 0 | — | 0.00 |
| 深层与大目录查找 | cold | 1 | 6,454.8 ops/s | 0.0 | 0.0% | 0 | — | 12.38 |
| 深层与大目录查找 | warm | 1 | 260,338.7 ops/s | 0.0 | 0.0% | 0 | — | 93.01 |
| 目录内重命名 | cold | 1 | 382.3 ops/s | 0.0 | 0.0% | 0 | — | 0.00 |
| 目录内重命名 | warm | 1 | 377.5 ops/s | 0.0 | 0.0% | 0 | — | 0.00 |
| 同目录并发修改 | cold | 1 | 563.9 ops/s | 0.0 | 0.0% | 0 | — | 0.00 |
| 同目录并发修改 | warm | 1 | 565.1 ops/s | 0.0 | 0.0% | 0 | — | 0.00 |
| 跨目录并发创建 | cold | 1 | 354.3 ops/s | 0.0 | 0.0% | 0 | — | 0.00 |
| 跨目录并发创建 | warm | 1 | 356.5 ops/s | 0.0 | 0.0% | 0 | — | 0.00 |
| 并发同步落盘 | cold | 1 | 354.1 ops/s | 0.0 | 0.0% | 0 | — | 80.59 |
| 并发同步落盘 | warm | 1 | 358.6 ops/s | 0.0 | 0.0% | 0 | — | 80.75 |
| 接近满盘时回收空间 | cold | 3 | 28.9 ops/s | 0.1 | 0.8% | 0 | — | 0.00 |
| 接近满盘时回收空间 | warm | 3 | 28.4 ops/s | 0.0 | 1.4% | 0 | — | 0.00 |

Run: `perf-v2-core-gemini-3-8-flash-perf-nvme-r5`
执行后端: `hyperv`

### Agent 代码评审

S：**17.86** / 100；状态：`complete`；用途：`diagnostic`。

| Critical | High | Medium | Low |
|---:|---:|---:|---:|
| 8 | 1 | 23 | 10 |

仅统计已确认、按根因去重的问题；以 20% 权重计入暂计总分。

Scoring rule: `agent-review-score-v2`.

## 评测范围与溯源

各 profile 原始结果独立保留；索引另列未测项暂计 0 的派生总分。每个 profile 列出其运行，并标明已记录的执行后端；不同后端不视为完全相同的执行条件。

`valid` 表示该 profile 的证据有效，不等于全部用例通过或全维度发布合格。`candidate-failed` 是候选失败，不是基础设施错误。`diagnostic` 仅诊断；`noneligible` 不具备排名资格。Hyper-V 性能不具备发布/排名资格；协议 v2 在固定 VHDX/内存设备上的 Hyper-V 运行单独组成 `hyperv-fixed-vhdx` 比较池计入暂计总分，与 KVM/raw-NVMe 分数不可比。

可维护性（`maint-v3-policy`，占总分 5%）：**68.11** / 100，各项见 README。Reference 单列为校准基线，不是模型生成条目。
<!-- core-results:end -->
