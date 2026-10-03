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


#### buffered-synchronized-writes / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 2635.4 |
| MAD | 2.300000000000182 |
| CV | 0.0032068774711178647 |
| Candidate-failed samples | 0 |

#### buffered-synchronized-writes / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 2623.5 |
| MAD | 4.900000000000091 |
| CV | 0.004942942668368523 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / cold

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 4 |
| Median | 36524933.5 |
| MAD | 279153.0 |
| CV | 0.00864026058015396 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / warm

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 4 |
| Median | 36591597.5 |
| MAD | 191719.5 |
| CV | 0.008325177727447898 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 1275.0321749492352 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 7964.602641224937 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 875.3453943875174 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 1401.057746048093 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 905.6045057843598 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 2 |
| Median | 1080.2367950605203 |
| MAD | 20.23541354746976 |
| CV | 0.01873238686184178 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 4 |
| Median | 64.69178174653288 |
| MAD | 0.9681509777001054 |
| CV | 0.02142373526333708 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 1274.2327935346277 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 8185.5252533593475 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 870.9536143177156 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 1396.637576957493 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 906.2743086691452 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 1080.0325720327198 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 4 |
| Median | 63.12868492643703 |
| MAD | 0.7792940805313293 |
| CV | 0.019285933176011492 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 3 |
| Median | 4574.742526 |
| MAD | 2.9997000000003027 |
| CV | 0.0012053194813702824 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 4 |
| Median | 6187.3596335 |
| MAD | 14.647865499999625 |
| CV | 0.003110238852712421 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 3 |
| Median | 2846.830634 |
| MAD | 15.167954999999893 |
| CV | 0.017963378618599077 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 3 |
| Median | 4573.3426660000005 |
| MAD | 0.09999000000061642 |
| CV | 0.0018579438802853105 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 4 |
| Median | 6174.6244934999995 |
| MAD | 14.00661749999972 |
| CV | 0.0022843210847767327 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 3 |
| Median | 2941.356889 |
| MAD | 4.673792999999478 |
| CV | 0.003207547667076699 |
| Candidate-failed samples | 0 |

Run: `perf-v2-core-gemini-3-8-flash-perf-cpu-r1`
执行后端: `hyperv`

### NVMe 性能

Profile: `perf-nvme`

结果：`pass`；评测分类：`diagnostic`。


#### buffered-synchronized-writes / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 2 |
| Median | 1804.8000000000002 |
| MAD | 5.100000000000023 |
| CV | 0.002825797872340438 |
| Candidate-failed samples | 0 |

#### buffered-synchronized-writes / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 2 |
| Median | 1789.35 |
| MAD | 42.05000000000007 |
| CV | 0.023500153687093118 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / cold

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 2 |
| Median | 16466728.0 |
| MAD | 125512.0 |
| CV | 0.007622157844594263 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / warm

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 2 |
| Median | 16497273.5 |
| MAD | 35714.5 |
| CV | 0.0021648728803580785 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 564.990302980749 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 6454.756067931315 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 382.3489768642001 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 563.9242841496209 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 354.2760021994249 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 354.1047569062702 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 28.92915651375103 |
| MAD | 0.09727410285352889 |
| CV | 0.00810859360922627 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 565.3846356275599 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 260338.67353905225 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 377.53730350923536 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 565.1136956168926 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 356.5201529637487 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 358.64638227195223 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 28.38634562962814 |
| MAD | 0.03346809750504676 |
| CV | 0.014030458908929342 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 2 |
| Median | 2653.0846915 |
| MAD | 25.847415500000125 |
| CV | 0.009742401206720062 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 2 |
| Median | 3309.0126014999996 |
| MAD | 7.889811499999951 |
| CV | 0.0023843401189900097 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 2 |
| Median | 1609.5175009999998 |
| MAD | 8.014256000000046 |
| CV | 0.004979290995606295 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 2 |
| Median | 2666.783322 |
| MAD | 9.449055000000044 |
| CV | 0.00354324062328152 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 2 |
| Median | 3293.366386 |
| MAD | 1.1582539999999426 |
| CV | 0.00035169302903061286 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 2 |
| Median | 1607.2826185 |
| MAD | 10.38106950000008 |
| CV | 0.006458770461717701 |
| Candidate-failed samples | 0 |

Run: `perf-v2-core-gemini-3-8-flash-perf-nvme-r5`
执行后端: `hyperv`

## 评测范围与溯源

各 profile 原始结果独立保留；索引另列未测项暂计 0 的派生总分。每个 profile 列出其运行，并标明已记录的执行后端；不同后端不视为完全相同的执行条件。

`valid` 表示该 profile 的证据有效，不等于全部用例通过或全维度发布合格。`candidate-failed` 是候选失败，不是基础设施错误。`diagnostic` 仅诊断；`noneligible` 不具备排名资格。Hyper-V 性能不具备发布/排名资格；协议 v2 在固定 VHDX/内存设备上的 Hyper-V 运行单独组成 `hyperv-fixed-vhdx` 比较池计入暂计总分，与 KVM/raw-NVMe 分数不可比。

安全审查与可维护性评分尚未评定。Reference 单列为校准基线，不是模型生成条目。
<!-- core-results:end -->
