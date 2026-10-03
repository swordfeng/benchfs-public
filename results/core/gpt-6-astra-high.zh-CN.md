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


#### buffered-synchronized-writes / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 2016.7 |
| MAD | 0.2999999999999545 |
| CV | 0.0016040386251332393 |
| Candidate-failed samples | 0 |

#### buffered-synchronized-writes / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 1991.3 |
| MAD | 2.599999999999909 |
| CV | 0.0017173621670573643 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / cold

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 3 |
| Median | 451928156.0 |
| MAD | 5810374.0 |
| CV | 0.014364658586838205 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / warm

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 5 |
| Median | 464158179.0 |
| MAD | 5323742.0 |
| CV | 0.014495788642839362 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 1758.2609705597276 |
| MAD | 2.0559501672280476 |
| CV | 0.009187993408699576 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 7538.223321581903 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 1270.62371379296 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 2147.481219639872 |
| MAD | 2.8163670490880577 |
| CV | 0.003712315712767811 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 2 |
| Median | 1432.7695538869039 |
| MAD | 3.905264338112943 |
| CV | 0.0027256751286475244 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 2 |
| Median | 1423.1512633525172 |
| MAD | 5.054072325070251 |
| CV | 0.0035513247644276226 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 1752.746466213993 |
| MAD | 4.409546022942777 |
| CV | 0.00935607902584648 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 7552.04451299252 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 1272.383842158545 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 2 |
| Median | 2139.140253771102 |
| MAD | 1.2582492346450636 |
| CV | 0.0005882032430678116 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 2 |
| Median | 1431.3321704910556 |
| MAD | 10.544233985417804 |
| CV | 0.007366727446502045 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 2 |
| Median | 1417.4988892396577 |
| MAD | 0.2870635966826285 |
| CV | 0.00020251415987818414 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 3 |
| Median | 3622.1377869999997 |
| MAD | 1.3998600000004444 |
| CV | 0.0025856585250151475 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 6682.563487 |
| MAD | 767.5914569999995 |
| CV | 0.23103473547156247 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 4 |
| Median | 7227.674524 |
| MAD | 87.24115600000005 |
| CV | 0.022769829486477957 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 3615.638437 |
| MAD | 22.697729999999865 |
| CV | 0.025382836941608786 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 4710.586824 |
| MAD | 867.15551 |
| CV | 0.2948379051394322 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 3 |
| Median | 7420.857564000001 |
| MAD | 30.796810000000733 |
| CV | 0.003728010622902506 |
| Candidate-failed samples | 0 |

Run: `perf-v2-core-gpt-6-astra-perf-cpu-r2`
执行后端: `hyperv`

### NVMe 性能

Profile: `perf-nvme`

结果：`candidate-failed`；评测分类：`diagnostic`。


#### buffered-synchronized-writes / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 1235.2 |
| MAD | 1.0 |
| CV | 0.004949493692009951 |
| Candidate-failed samples | 0 |

#### buffered-synchronized-writes / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 1228.4 |
| MAD | 2.0 |
| CV | 0.001815326944709353 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / cold

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 3 |
| Median | 282044897.0 |
| MAD | 266057.0 |
| CV | 0.001065887481201038 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / warm

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 3 |
| Median | 282044804.0 |
| MAD | 1167145.0 |
| CV | 0.0038862443388754096 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 2 |
| Median | 920.8479870629533 |
| MAD | 1.7648198287055834 |
| CV | 0.0019165159217369636 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 7503.956113803032 |
| MAD | 25.07623462317588 |
| CV | 0.002763677507763717 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 591.9719803062826 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 1055.8225514762546 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 539.1769512254937 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 2 |
| Median | 518.366037248936 |
| MAD | 5.048444848338875 |
| CV | 0.009739150495144129 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 923.8369433916258 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 7555.86094464924 |
| MAD | 13.395035170756273 |
| CV | 0.001870328018302804 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 594.8331170075235 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 1052.5801694084232 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 536.1030019075382 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 2 |
| Median | 510.552692474307 |
| MAD | 2.044866826399641 |
| CV | 0.004005202316120479 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 3 |
| Median | 1969.50305 |
| MAD | 18.698131000000103 |
| CV | 0.008062528581553638 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 3140.671826 |
| MAD | 130.0739050000002 |
| CV | 0.1692574973276818 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 3224.6549889999997 |
| MAD | 12.239954999999554 |
| CV | 0.013261892719316585 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 3 |
| Median | 1966.80332 |
| MAD | 2.3997600000000148 |
| CV | 0.007649854477454516 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 1925.3511429999999 |
| MAD | 24.72289099999989 |
| CV | 0.07292572986824569 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 3102.4654219999998 |
| MAD | 67.48725300000024 |
| CV | 0.031087582168994043 |
| Candidate-failed samples | 0 |

Run: `perf-v2-core-gpt-6-astra-perf-nvme-r2`
执行后端: `hyperv`

## 评测范围与溯源

各 profile 原始结果独立保留；索引另列未测项暂计 0 的派生总分。每个 profile 列出其运行，并标明已记录的执行后端；不同后端不视为完全相同的执行条件。

`valid` 表示该 profile 的证据有效，不等于全部用例通过或全维度发布合格。`candidate-failed` 是候选失败，不是基础设施错误。`diagnostic` 仅诊断；`noneligible` 不具备排名资格。Hyper-V 性能不具备发布/排名资格；协议 v2 在固定 VHDX/内存设备上的 Hyper-V 运行单独组成 `hyperv-fixed-vhdx` 比较池计入暂计总分，与 KVM/raw-NVMe 分数不可比。

安全审查与可维护性评分尚未评定。Reference 单列为校准基线，不是模型生成条目。
<!-- core-results:end -->
