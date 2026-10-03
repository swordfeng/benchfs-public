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


#### buffered-synchronized-writes / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 5 |
| Median | 102.5 |
| MAD | 3.0894410558944116 |
| CV | 0.07998257876308118 |
| Candidate-failed samples | 0 |

#### buffered-synchronized-writes / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 5 |
| Median | 105.35767734211315 |
| MAD | 1.0423226578868565 |
| CV | 0.05630207933460487 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / cold

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 5 |
| Median | 29805573.0 |
| MAD | 422727.0 |
| CV | 0.047043897921454234 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / warm

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 5 |
| Median | 30284194.0 |
| MAD | 874613.0 |
| CV | 0.07850364039066114 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 2902.3578312508257 |
| MAD | 5.698238086249148 |
| CV | 0.002378216532555285 |
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

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 2 |
| Median | 4024.5800539465363 |
| MAD | 21.36359462198311 |
| CV | 0.00530827920817075 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 1 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 2 |
| Median | 420.7652807481952 |
| MAD | 24.62848695912018 |
| CV | 0.058532602583859505 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 5 |
| Median | 883.2723980530152 |
| MAD | 47.99959109309259 |
| CV | 0.05765300601775447 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 2 |
| Median | 2894.564593796541 |
| MAD | 10.941880495739724 |
| CV | 0.0037801472868111883 |
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

#### metadata-concurrency / warm

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
| Samples | 2 |
| Median | 4026.0529035827967 |
| MAD | 14.16806665512263 |
| CV | 0.0035190959966061113 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 1 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 2 |
| Median | 429.5526760904163 |
| MAD | 13.374037792442522 |
| CV | 0.031134802637400934 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 4 |
| Median | 967.7824769758114 |
| MAD | 8.603357347726956 |
| CV | 0.014693734248041535 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 361.86381400000005 |
| MAD | 38.156181999999944 |
| CV | 0.08535991637977407 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 2661.9558690000003 |
| MAD | 1506.9086380000003 |
| CV | 0.5455865970703212 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 6803.617643 |
| MAD | 123.33169700000053 |
| CV | 0.026481254245043866 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 377.744329 |
| MAD | 27.715125 |
| CV | 0.06289218313440963 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 491.618511 |
| MAD | 258.439032 |
| CV | 1.0975531627162665 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 6653.330602999999 |
| MAD | 90.08855900000071 |
| CV | 0.033795974738737376 |
| Candidate-failed samples | 0 |

Run: `perf-v2-core-gpt-5-6-sol-v2-perf-cpu-r1`
执行后端: `hyperv`

### NVMe 性能

Profile: `perf-nvme`

结果：`candidate-failed`；评测分类：`diagnostic`。


#### buffered-synchronized-writes / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 5 |
| Median | 20.073903924897632 |
| MAD | 0.29565999322247194 |
| CV | 0.021689791994687194 |
| Candidate-failed samples | 0 |

#### buffered-synchronized-writes / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 20.471340123826643 |
| MAD | 0.004089360791812879 |
| CV | 0.006370344563300322 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / cold

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 5 |
| Median | 13924394.0 |
| MAD | 67137.0 |
| CV | 0.0066181278557804315 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / warm

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 5 |
| Median | 26148319.0 |
| MAD | 93491.0 |
| CV | 0.005841812291712969 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 2 |
| Median | 2888.2346682987054 |
| MAD | 9.992155689531955 |
| CV | 0.0034596065891757213 |
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

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 2 |
| Median | 3983.8589060051218 |
| MAD | 46.40702325133816 |
| CV | 0.011648761752427007 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 1 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 869.8073587939201 |
| MAD | 5.856284789961023 |
| CV | 0.0068073847989696306 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 2 |
| Median | 2868.946205382835 |
| MAD | 11.31432183182369 |
| CV | 0.003943720454080071 |
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

#### metadata-concurrency / warm

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
| Median | 4023.635189800146 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | — |
| Median | — |
| MAD | — |
| CV | — |
| Candidate-failed samples | 1 |

#### metadata-concurrency / warm

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
| Samples | 5 |
| Median | 879.0187915067572 |
| MAD | 42.22329150605492 |
| CV | 0.06514855939623476 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 89.01307299999999 |
| MAD | 1.275193999999999 |
| CV | 0.01813248376229172 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 295.972685 |
| MAD | 231.434131 |
| CV | 1.142901048333523 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 1605.095315 |
| MAD | 367.45504099999994 |
| CV | 0.44711327641288473 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 88.169341 |
| MAD | 0.8479300000000052 |
| CV | 0.012198910781949629 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 60.096154 |
| MAD | 10.619426000000004 |
| CV | 0.6586409374516524 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 1736.866802 |
| MAD | 560.2481899999998 |
| CV | 0.5143026553289984 |
| Candidate-failed samples | 0 |

Run: `perf-v2-core-gpt-5-6-sol-v2-perf-nvme-r2`
执行后端: `hyperv`

## 评测范围与溯源

各 profile 原始结果独立保留；索引另列未测项暂计 0 的派生总分。每个 profile 列出其运行，并标明已记录的执行后端；不同后端不视为完全相同的执行条件。

`valid` 表示该 profile 的证据有效，不等于全部用例通过或全维度发布合格。`candidate-failed` 是候选失败，不是基础设施错误。`diagnostic` 仅诊断；`noneligible` 不具备排名资格。Hyper-V 性能不具备发布/排名资格；协议 v2 在固定 VHDX/内存设备上的 Hyper-V 运行单独组成 `hyperv-fixed-vhdx` 比较池计入暂计总分，与 KVM/raw-NVMe 分数不可比。

安全审查与可维护性评分尚未评定。Reference 单列为校准基线，不是模型生成条目。
<!-- core-results:end -->
