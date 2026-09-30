# Fable 5.1 — Core — High

[English](fable-5.1-high.md)

本页汇总独立的 Core Eval profile 结果；各项分别列出评测状态与运行来源，未测维度不补造结果。

## 运行信息

| 字段 | 值 |
|---|---|
| 模型 | `fable-5.1` |
| Effort | High |
| Variant | Core |
| Agent harness | Pi |
| 生成运行次数 | 1（`n=1`） |
| SDK/POSIX 评测运行 | `eval-core-fable-5.1` |
| SDK/POSIX 首次评测日期 | 2026-09-04（UTC） |
| SDK/POSIX 成绩更新日期 | 2026-09-05（UTC） |

实际 effort 为 High，operator 已确认这是 proxy override 后的配置。Pi session 的本地 thinking-level 设置不代表 provider 侧的实际 effort。

## 提交规模

仅统计 `bench/` 下候选实现自有的源码。固定 SDK 和 FUSE adapter、依赖、文档、配置、锁文件及生成文件均不计入。

| 组成部分 | 包含该部分的文件数 | 总行数 | 代码行数 |
|---|---:|---:|---:|
| 实现（Rust） | 13 | 5,826 | 5,175 |
| Agent 编写的测试（Rust） | 2 | 395 | 347 |
| 测试运行脚本 | 0 | 0 | 0 |

总行数包含空行和注释。代码行数统计至少包含一个非注释 token 的行：空行、注释和文档注释（Rust 的 `///`、`//!`，Python docstring）不计入，`#[derive(...)]` 等 Rust 属性计为代码。2026-09-27 更正：本页早先的版本把文档注释计为代码、把 Rust 属性计为注释；文件数和总行数不变。测试位于独立文件中，没有内联测试模块，文件数不重叠。共有 15 个不同的 Rust 文件。这些数据仅衡量源码规模，不是测试覆盖率或代码质量评分。

<!-- source-tree:begin -->
### 目录结构

`bench/` 下的全部文件，不含构建产物。数字为按上述规则统计的代码行，“测试”为其中属于测试代码的部分。

```text
bench/
├── src/
│   ├── btree.rs  (527 行代码)
│   ├── checker.rs  (468 行代码)
│   ├── core.rs  (600 行代码)
│   ├── data.rs  (500 行代码)
│   ├── format.rs  (904 行代码)
│   ├── fs.rs  (923 行代码)
│   ├── fs_impl.rs  (304 行代码)
│   ├── lib.rs  (11 行代码)
│   ├── main.rs  (81 行代码)
│   ├── mkfs.rs  (68 行代码)
│   ├── mkfs_impl.rs  (86 行代码)
│   ├── node.rs  (245 行代码)
│   └── store.rs  (458 行代码)
├── tests/
│   ├── btree.rs  (182 行代码，测试 182)
│   └── lifecycle.rs  (165 行代码，测试 165)
├── Cargo.toml
└── NOTES.md
```
<!-- source-tree:end -->

## 生成时间与活动量

| 指标 | 值 |
|---|---|
| Goal 开始时间 | 2026-09-02 17:12:00.551 UTC |
| Goal 完成时间 | 2026-09-03 06:00:55.969 UTC |
| Goal 活跃时长 | **5 小时 16 分 10 秒** |
| 经过时长，含活跃时段之间的间隔 | 12 小时 48 分 55 秒 |
| 最终 goal 状态 | Complete（harness 记录） |
| Proxy API 请求数／HTTP 响应数 | 275 / 275 |
| Token usage 非零的 assistant 响应记录 | 273 |
| Assistant 错误记录 | 3 |
| Agent 工具调用次数 | 272 |

活跃时长采用 Pi goal 最终累计计数器的 18,970.246 秒。恢复后的 goal 继承了此前的计数，不能将四个 goal 的快照相加。活跃时长是 harness 记账的时间，不是模型推理时间。Goal 完成不代表全部评测测试通过。

请求数来自归档的 proxy 日志；assistant 和工具调用次数来自 Pi session。Session 包含 276 条 assistant 记录，而 proxy 记录了 275 次请求。它们属于不同的记录口径，不能互相替代，也不保证每次上游重试或失败请求均已计费或被完整记录。

### 工具使用统计

| 工具 | 请求次数 | 已记录失败次数 | 结果未知次数 |
|---|---:|---:|---:|
| `bash` | 245 | 18 | 0 |
| `edit` | 1 | 0 | 0 |
| `goal_complete` | 1 | 0 | 0 |
| `read` | 13 | 0 | 0 |
| `write` | 12 | 2 | 0 |
| **合计** | **272** | **20** | **0** |

归档 assistant 中每个 `toolCall` 按记录的工具名计一次请求，重试分别计数。通过 `toolCallId` 配对结果并核对工具名，仅将 `toolResult.isError: true` 计为已记录失败。结果缺失或无法解析时计为未知，不推定成功或失败。这些是工具级结果，不是模型 API 错误数或评测测试结果。

## Token 用量与 API 成本

| 指标 | 值 |
|---|---:|
| 输入 tokens，不含缓存（`input`） | 556 |
| 缓存读取输入 tokens（`cache_read`） | 97,277,903 |
| 缓存写入输入 tokens（`cache_write`） | 2,652,910 |
| 输入 tokens 合计，含缓存 | 99,931,369 |
| 输出 tokens，含 thinking（`output`） | 298,429 |
| Thinking／reasoning tokens | 52,185 |
| **总 tokens** | **100,229,798** |
| 缓存命中率（按输入 tokens） | 97.34% |
| API 成本估算（美元） | **$72.41** |

Token 数对 Pi assistant 消息中的 usage 记录求和，与归档的 proxy token 总数及最终 goal 的累计 token 计数均一致。全部 273 条 usage 非零的响应都明确记录了 reasoning 用量；它属于 output 的一部分，不重复相加。输入总量包含各次请求中重复发送的上下文，不是单次上下文大小，也不是去重后的 token 数。

成本采用 2026-09-05 查询到的 [OpenRouter Models API](https://openrouter.ai/api/v1/models) 中 `anthropic/claude-fable-5.1` 的非 batch 模型目录价，并与 [Fable endpoint 价格](https://openrouter.ai/api/v1/models/anthropic/claude-fable-5.1/endpoints) 交叉核对。下列单价单位为美元／百万 tokens。这是当前目录价下的估算，不是运行当日价格、实际 endpoint 路由或 provider 账单的证据，也不使用 session 成本字段。

| Token 类别 | 美元／百万 tokens | 费用估算（美元） |
|---|---:|---:|
| 非缓存输入 | $10.00 | $0.005560 |
| 输出 | $50.00 | $14.921450 |
| 缓存读取 | $0.25 | $24.319476 |
| 缓存写入，5 分钟 TTL | $12.50 | $33.161375 |
| 缓存写入，1 小时 TTL | $20.00 | $0.000000 |

未舍入的总费用为 $72.40786075，最后统一舍入为 **$72.41**。[OpenRouter 缓存 TTL 规则](https://openrouter.ai/docs/guides/best-practices/prompt-caching#cache-ttl-options) 区分 5 分钟和 1 小时写入。Session 记录了 2,652,910 个 cache-write tokens，`cacheWrite1h` 为零，因此按 5 分钟写入价计算。另列的 556 个 input tokens 按普通 prompt 单价计费，不再重复收取缓存写入费。Reasoning 已包含在 output 中。Token 总数仅涵盖已报告的 usage；失败请求中未报告的用量仍然未知。

生成时间、源码规模、tokens、活动量和成本均为描述性元数据，不是正确性评分。

## 评测结果

<!-- core-results:begin -->

### POSIX 正确性

| Profile | Classification | Pass | Expected fail | Fail | Unexpected pass | Timeout | Skip N/A | Skip impl | Pass rate | Category macro average |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `spec-tests` | `valid` | 44 | 0 | 0 | 0 | 0 | 7 | 0 | 100.00% | 100.00% |
| `pjdfstest-core` | `diagnostic` | 168 | 0 | 1 | 0 | 0 | 48 | 0 | 99.41% | 99.26% |
| `xfstests-core` | `valid` | 182 | 1 | 7 | 0 | 4 | 473 | 0 | 94.33% | 87.35% |

`spec-tests` run: `eval-core-fable-5.1`

`pjdfstest-core` run: `case-correction-core-fable-5.1-pjdfstest-core`

`xfstests-core` run: `eval-core-fable-5.1`

### 稳健性

Profile: `robustness`

结果：`candidate-failed`；评测分类：`valid`。

| Metric | Value |
|---|---:|
| Total | 47 |
| Pass | 44 |
| Fail | 3 |
| Timeout | 0 |
| Pass rate | 93.62% |
| Category macro average | 90.28% |

超时计失败，N/A 不计入分母，类别宏平均按类别等权。

Run: `eval-submission-robustness-v2-core-fable-5-1-r1`

### 磁盘格式一致性

Profile: `canonical-format`

结果：`candidate-failed`；评测分类：`diagnostic`。

| Metric | Value |
|---|---:|
| Total | 479 |
| Pass | 431 |
| Fail | 48 |
| Timeout | 0 |
| Pass rate | 89.98% |
| Category macro average | 95.45% |
| Conformance score (0–100) | 95.45 |
| Full conformance | no |

Run: `core-canonical-fable-oracle3-r6`
执行后端: `hyperv`

### 崩溃一致性

Profile: `crash-core`

结果：`candidate-failed`；评测分类：`diagnostic`。

| Metric | Value |
|---|---:|
| Total | 216 |
| Pass | 79 |
| Fail | 137 |
| Timeout | 0 |
| Pass rate | 36.57% |
| Category macro average | 36.57% |

Run: `case-correction-core-fable-5.1-crash-core`

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

Run: `core-realworld-fable-20260925-r1`
执行后端: `qemu-kvm`

### CPU 性能

Profile: `perf-cpu`

尚无可发布的评测结果（`not-run`），不是实测零分；索引暂计总分时按缺失项填 0。

### NVMe 性能

Profile: `perf-nvme`

尚无可发布的评测结果（`not-run`），不是实测零分；索引暂计总分时按缺失项填 0。

## 评测范围与溯源

各 profile 原始结果独立保留；索引另列未测项暂计 0 的派生总分。每个 profile 列出其运行，并标明已记录的执行后端；不同后端不视为完全相同的执行条件。

`valid` 表示该 profile 的证据有效，不等于全部用例通过或全维度发布合格。`candidate-failed` 是候选失败，不是基础设施错误。`diagnostic` 仅诊断；`noneligible` 不具备排名资格。Hyper-V 性能始终为诊断结果。

安全审查与可维护性评分尚未评定。Reference 单列为校准基线，不是模型生成条目。
<!-- core-results:end -->
