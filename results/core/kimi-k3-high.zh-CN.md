# Kimi K3 — Core — High

[English](kimi-k3-high.md)

本页汇总独立的 Core Eval profile 结果；各项分别列出评测状态与运行来源，未测维度不补造结果。

## 运行信息

| 字段 | 值 |
|---|---|
| 模型 | `kimi-k3` |
| Effort | High |
| Variant | Core |
| Agent harness | Pi |
| 生成运行次数 | 1（`n=1`） |
| SDK/POSIX 评测运行 | `eval-core-kimi-k3` |
| SDK/POSIX 首次评测日期 | 2026-09-04（UTC） |
| SDK/POSIX 成绩更新日期 | 2026-09-05（UTC） |

实际 effort 为 High，由 proxy override 强制设置。Pi session 的本地 thinking-level 设置不代表 provider 侧的实际 effort。

## 提交规模

仅统计 `bench/` 下候选实现自有的源码。固定 SDK 和 FUSE adapter、依赖、文档、配置、锁文件及生成文件均不计入。

| 组成部分 | 包含该部分的文件数 | 总行数 | 代码行数 |
|---|---:|---:|---:|
| 实现（Rust） | 10 | 7,580 | 6,707 |
| Agent 编写的测试（Rust） | 5：4 个独立文件＋1 个共享文件 | 376 | 332 |
| 测试运行脚本（Shell） | 1 | 43 | 32 |

总行数包含空行和注释。代码行数统计至少包含一个非注释 token 的行：空行、注释和文档注释（Rust 的 `///`、`//!`，Python docstring）不计入，`#[derive(...)]` 等 Rust 属性计为代码。2026-09-27 更正：本页早先的版本把文档注释计为代码、把 Rust 属性计为注释；文件数和总行数不变。内联测试模块及其 test-only attribute 计入测试，而非实现。一个 Rust 文件同时包含实现和测试，因此文件数有重叠，行数不重复。共有 14 个不同的 Rust 文件。这些数据仅衡量源码规模，不是测试覆盖率或代码质量评分。

## 生成时间与活动量

| 指标 | 值 |
|---|---|
| Goal 开始时间 | 2026-09-02 17:18:34.870 UTC |
| Goal 完成时间 | 2026-09-03 06:05:53.540 UTC |
| Goal 活跃时长 | **6 小时 52 分 8 秒** |
| 经过时长，含活跃时段之间的间隔 | 12 小时 47 分 19 秒 |
| 最终 goal 状态 | Complete（harness 记录） |
| Proxy API 请求数／HTTP 响应数 | 537 / 537 |
| Token usage 非零的 assistant 响应记录 | 535 |
| Assistant 错误记录 | 2 |
| Agent 工具调用次数 | 548 |

活跃时长采用 Pi goal 最终累计计数器的 24,727.690 秒。恢复后的 goal 继承了此前的计数，不能将两个 goal 的快照相加。活跃时长是 harness 记账的时间，不是模型推理时间。Goal 完成不代表全部评测测试通过。

请求数来自归档的 proxy 日志；assistant 和工具调用次数来自 Pi session。这些数字描述已记录的活动，不保证每次上游重试或失败请求均已计费或被完整记录。

### 工具使用统计

| 工具 | 请求次数 | 已记录失败次数 | 结果未知次数 |
|---|---:|---:|---:|
| `bash` | 450 | 26 | 0 |
| `edit` | 32 | 1 | 0 |
| `goal_complete` | 1 | 0 | 0 |
| `read` | 46 | 0 | 0 |
| `write` | 19 | 0 | 0 |
| **合计** | **548** | **27** | **0** |

归档 assistant 中每个 `toolCall` 按记录的工具名计一次请求，重试分别计数。通过 `toolCallId` 配对结果并核对工具名，仅将 `toolResult.isError: true` 计为已记录失败。结果缺失或无法解析时计为未知，不推定成功或失败。这些是工具级结果，不是模型 API 错误数或评测测试结果。

## Token 用量与 API 成本

| 指标 | 值 |
|---|---:|
| 输入 tokens，不含缓存（`input`） | 5,491,719 |
| 缓存读取输入 tokens（`cache_read`） | 222,793,117 |
| 缓存写入输入 tokens（`cache_write`） | 0 |
| 输入 tokens 合计，含缓存 | 228,284,836 |
| 输出 tokens，含 thinking（`output`） | 478,897 |
| Thinking／reasoning tokens（留存文本估算） | ≈43,834 |
| **总 tokens** | **228,763,733** |
| 缓存命中率（按输入 tokens） | 97.59% |
| API 成本估算（美元） | **$90.50** |

Token 数对 Pi assistant 消息中的 usage 记录求和，与最终 goal 的累计 token 计数一致。Proxy token summary 没有 usage 记录，因此不采用其中为零的 token 总数。输入总量包含各次请求中重复发送的上下文，不是单次上下文大小，也不是去重后的 token 数。

Provider usage 记录没有独立的 reasoning-token 计数。本次估算对 183,427 个字符进行分词，来自 26 条 assistant 记录、共 28 个留存 thinking 块：其中 25 条属于全部 535 条非零用量响应，另一条为保留了 thinking 但未报告用量的错误记录。采用 revision `f831ab66814297da540d832a5235f8e904f29d06` 的[公开 Kimi K3 tokenizer](https://huggingface.co/moonshotai/Kimi-K3/blob/f831ab66814297da540d832a5235f8e904f29d06/tokenization_kimi.py)及词表，使用 `tiktoken 0.11.0`。每个块作为普通文本独立编码，不计聊天封装和签名。此数值仅衡量留存的 thinking 文本，不代表整次运行的完整 reasoning 用量或 provider 计费 token 数，也不外推到其他响应。计入错误记录中的留存文本不等于恢复了其计费用量。估算值不加入 output、总 tokens 或费用。失败请求中未报告的用量仍然未知。

成本按 2026-09-05 查询到的 [OpenRouter Models API](https://openrouter.ai/api/v1/models) 中 `moonshotai/kimi-k3` 的非 batch 模型目录价重新估算。下列单价单位为美元／百万 tokens。这是当前目录价下的重估，不是运行当日价格、实际 endpoint 路由或 provider 账单的证据，也不使用 session 成本字段。

| Token 类别 | 美元／百万 tokens | 费用估算（美元） |
|---|---:|---:|
| 非缓存输入／缓存未命中 | $3.00 | $16.475157 |
| 输出 | $15.00 | $7.183455 |
| 缓存读取 | $0.30 | $66.837935 |

未舍入的总费用为 $90.4965471，最后统一舍入为 **$90.50**。[OpenRouter 缓存文档](https://openrouter.ai/docs/guides/best-practices/prompt-caching#moonshot-ai) 未对 Moonshot 单独收取缓存写入费用，K3 模型目录也未列出缓存写入溢价。非缓存 input 即使用于建立缓存，也仅按 prompt 单价计费一次；不能因 cache-write 字段为零就增加附加费。

此前的 $77.14 使用 operator 提供的 $2.55 / $12.75 / $0.256 单价，与 [K3 endpoint 价格](https://openrouter.ai/api/v1/models/moonshotai/kimi-k3/endpoints) 中较低的 Makora 报价一致。新值与其他报告统一采用模型目录价，并不宣称本次运行实际使用了不同的 endpoint。失败请求中未报告的用量不计入。

生成时间、源码规模、tokens、活动量和成本均为描述性元数据，不是正确性评分。

## 评测结果

<!-- core-results:begin -->

### POSIX 正确性

| Profile | Classification | Pass | Expected fail | Fail | Unexpected pass | Timeout | Skip N/A | Skip impl | Pass rate | Category macro average |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `spec-tests` | `valid` | 44 | 0 | 0 | 0 | 0 | 7 | 0 | 100.00% | 100.00% |
| `pjdfstest-core` | `diagnostic` | 169 | 0 | 0 | 0 | 0 | 48 | 0 | 100.00% | 100.00% |
| `xfstests-core` | `valid` | 185 | 1 | 3 | 0 | 5 | 473 | 0 | 95.88% | 89.75% |

`spec-tests` run: `eval-core-kimi-k3`

`pjdfstest-core` run: `case-correction-core-kimi-k3-pjdfstest-core`

`xfstests-core` run: `eval-core-kimi-k3`

### 稳健性

Profile: `robustness`

结果：`candidate-failed`；评测分类：`valid`。

| Metric | Value |
|---|---:|
| Total | 47 |
| Pass | 46 |
| Fail | 1 |
| Timeout | 0 |
| Pass rate | 97.87% |
| Category macro average | 97.50% |

超时计失败，N/A 不计入分母，类别宏平均按类别等权。

Run: `eval-submission-robustness-v2-core-kimi-k3-r2`

### 磁盘格式一致性

Profile: `canonical-format`

结果：`candidate-failed`；评测分类：`diagnostic`。

| Metric | Value |
|---|---:|
| Total | 479 |
| Pass | 410 |
| Fail | 69 |
| Timeout | 0 |
| Pass rate | 85.59% |
| Category macro average | 83.27% |
| Conformance score (0–100) | 84.17 |
| Full conformance | no |

Run: `core-canonical-kimi-oracle3-r5`
执行后端: `hyperv`

### 崩溃一致性

Profile: `crash-core`

结果：`candidate-failed`；评测分类：`diagnostic`。

| Metric | Value |
|---|---:|
| Total | 216 |
| Pass | 205 |
| Fail | 11 |
| Timeout | 0 |
| Pass rate | 94.91% |
| Category macro average | 94.91% |

Run: `case-correction-core-kimi-k3-crash-core`

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
| Category macro average | 66.67% |

Run: `core-realworld-kimi-20260925-r1`
执行后端: `hyperv`

### CPU 性能

Profile: `perf-cpu`

尚无可发布的评测结果（`not-run`），不是实测零分；索引暂计总分时按缺失项填 0。

### NVMe 性能

Profile: `perf-nvme`

尚无可发布的评测结果（`not-run`），不是实测零分；索引暂计总分时按缺失项填 0。

## 评测范围与溯源

[JSON](../data/core/kimi-k3-high.json)

各 profile 原始结果独立保留；索引另列未测项暂计 0 的派生总分。JSON 保留逐 profile 的运行、manifest、报告、候选及固定评测面身份，并标明已记录的执行后端；不同后端不视为完全相同的执行条件。

`valid` 表示该 profile 的证据有效，不等于全部用例通过或全维度发布合格。`candidate-failed` 是候选失败，不是基础设施错误。`diagnostic` 仅诊断；`noneligible` 不具备排名资格。Hyper-V 性能始终为诊断结果。

安全审查与可维护性评分尚未评定。Reference 单列为校准基线，不是模型生成条目。
<!-- core-results:end -->
