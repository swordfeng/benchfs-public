# Opus 5.5 — Core — High — Claude Code (`-cc`)

[English](opus-5.5-high-cc.md)

本页记录 Opus 5.5 的 Core 生成运行及八项通过独立验证的非性能 profile 结果。各次运行和二进制身份分别保留在下方。性能、代码审查和可维护性仍未评测。

## 运行信息

| 字段 | 值 |
|---|---|
| 模型 | `claude-opus-5-5` |
| Effort | High（`--effort high`） |
| Variant | Core |
| Agent harness | Claude Code 2.1.283，精简配置（见下文） |
| 生成运行次数 | 1（`n=1`） |
| 生成日期 | 2026-09-27（UTC） |
| 评测 | 八项非性能 profile 结果已独立核验 |

**Harness 差异。** 本次运行使用 Claude Code，而不是其他 Core 报告使用的 Pi harness，原因是模型通过 Claude 订阅访问。因此结果不能与 Pi harness 的报告直接比较。任务面没有变化：Dev VM 镜像、agent 可见文档和 prompt 文本都相同。harness 只保留 `Bash`、`Read`、`Edit`、`Write` 四个工具，没有 subagent、MCP server、Web 工具或记忆。模型流量经环境的出网白名单访问 Anthropic，而不是经过 model proxy。Prompt 作为 `/goal` 条件只输入一次。与 Pi 的 goal 扩展由 agent 自行宣布完成不同，Claude Code 的 `/goal` 在每个 turn 之后由另一个小模型（Haiku 4.5）判断是否完成；条件尚未满足时，会把判断理由回传给主模型。Claude Code 还会加入自己的 system prompt、工具说明和上下文提醒；使用订阅登录时，其中包含账户身份信息。

## 提交规模

仅统计 `bench/` 下候选实现自有的源码。固定 SDK 和 FUSE adapter、依赖、文档、配置、锁文件及生成文件均不计入。

| 组成部分 | 包含该部分的文件数 | 总行数 | 代码行数 |
|---|---:|---:|---:|
| 实现（Rust） | 32 | 8,318 | 7,139 |
| Agent 编写的测试及测试支持代码（Rust） | 11 | 2,792 | 2,627 |
| 测试运行脚本 | 0 | 0 | 0 |

总行数包含空行和注释。代码行数统计至少包含一个非注释 token 的行：空行、注释和文档注释（Rust 的 `///`、`//!`，Python docstring）不计入，`#[derive(...)]` 等 Rust 属性计为代码。测试包括 `tests/` 下的 9 个独立文件（其中一个是共享辅助模块），以及两个实现文件（`src/crc.rs`、`src/format/mod.rs`）中的内联 `#[cfg(test)]` 模块（含属性行）。这两个文件在文件数上重叠；总行数和代码行数按行划分，不重叠。共有 41 个不同的 Rust 文件。这些数据仅衡量源码规模，不是测试覆盖率或代码质量评分。

<!-- source-tree:begin -->
### 目录结构

`bench/` 下的全部文件，不含构建产物。数字为按上述规则统计的代码行，“测试”为其中属于测试代码的部分。

```text
bench/
├── src/
│   ├── btree/
│   │   ├── mod.rs  (8 行代码)
│   │   ├── node.rs  (362 行代码)
│   │   ├── ops.rs  (554 行代码)
│   │   └── walk.rs  (97 行代码)
│   ├── format/
│   │   ├── geometry.rs  (284 行代码)
│   │   ├── inode.rs  (313 行代码)
│   │   ├── meta.rs  (130 行代码)
│   │   ├── mod.rs  (182 行代码，测试 29)
│   │   └── records.rs  (155 行代码)
│   ├── fs/
│   │   ├── data/
│   │   │   ├── mod.rs  (107 行代码)
│   │   │   ├── range.rs  (288 行代码)
│   │   │   ├── read.rs  (132 行代码)
│   │   │   └── write.rs  (328 行代码)
│   │   ├── handles.rs  (179 行代码)
│   │   ├── mod.rs  (487 行代码)
│   │   ├── namespace.rs  (697 行代码)
│   │   ├── ops.rs  (551 行代码)
│   │   └── xattr.rs  (285 行代码)
│   ├── alloc.rs  (254 行代码)
│   ├── bin_fsck.rs  (37 行代码)
│   ├── cache.rs  (253 行代码)
│   ├── check.rs  (427 行代码)
│   ├── crc.rs  (93 行代码，测试 18)
│   ├── dev.rs  (109 行代码)
│   ├── devlock.rs  (22 行代码)
│   ├── extmap.rs  (259 行代码)
│   ├── image.rs  (128 行代码)
│   ├── lib.rs  (14 行代码)
│   ├── main.rs  (89 行代码)
│   ├── mkfs.rs  (70 行代码)
│   ├── prof.rs  (49 行代码)
│   └── store.rs  (243 行代码)
├── tests/
│   ├── common/
│   │   └── mod.rs  (65 行代码，测试 65)
│   ├── basic.rs  (408 行代码，测试 408)
│   ├── btree.rs  (138 行代码，测试 138)
│   ├── concurrent.rs  (172 行代码，测试 172)
│   ├── corrupt.rs  (138 行代码，测试 138)
│   ├── crash.rs  (220 行代码，测试 220)
│   ├── faults.rs  (184 行代码，测试 184)
│   ├── model.rs  (713 行代码，测试 713)
│   └── spec.rs  (542 行代码，测试 542)
├── Cargo.toml
└── README.md
```
<!-- source-tree:end -->

## 生成时间与活动量

| 指标 | 值 |
|---|---|
| Goal 开始时间 | 2026-09-27 01:01:54.452 UTC |
| Goal 完成时间 | 2026-09-27 04:09:03.143 UTC |
| Goal 活跃时长 | **3 小时 7 分 9 秒** |
| 经过时长，含活跃时段之间的间隔 | 3 小时 7 分 9 秒 |
| 最终 goal 状态 | Met（由 `/goal` 评估器判定） |
| 含 usage 的主模型 API 响应 | 249 |
| 完成评估器判定次数 | 1 |
| API 错误记录 | 0 |
| Agent 工具调用次数 | 256 |
| API 时间／工具执行时间 | 1 小时 10 分 14 秒／1 小时 56 分 51 秒 |

Goal 以一个连续的 turn 运行：Claude Code 记录的 turn 时长为 11,228.721 秒，与设置 goal 到评估器给出判定之间的间隔一致。期间没有额度暂停、API 错误记录或上下文压缩；Claude Code 的会话状态中有 1.1 秒 API 时间属于重试。Operator 输入给模型的只有 `/goal` 条件。API 时间和工具时间来自 Claude Code 的会话累计状态。活跃时长是 harness 记录的墙钟时间，不是模型推理时间。Goal 判定完成不代表评测测试通过。

响应数来自归档的 Claude Code transcript，按 message ID 去重：Claude Code 为每个内容块写一条记录，249 次响应共有 576 条 assistant 记录。该 harness 没有 model proxy 日志。环境出网日志显示，除 Anthropic 外，agent 只访问了 `crates.io`。Claude Code 启动时曾尝试连接 `downloads.claude.ai` 和 `github.com` 各一次，均在设置 goal 之前被白名单拒绝。

### 工具使用统计

| 工具 | 请求次数 | 已记录失败次数 | 结果未知次数 |
|---|---:|---:|---:|
| `Bash` | 229 | 7 | 0 |
| `Edit` | 1 | 0 | 0 |
| `Read` | 7 | 0 | 0 |
| `Write` | 19 | 0 | 0 |
| **合计** | **256** | **7** | **0** |

每个 `tool_use` 块按工具名计一次请求。通过 `tool_use_id` 配对结果，仅将 `tool_result.is_error: true` 计为已记录失败。每次调用都有对应结果。这些是工具级结果，不是模型 API 错误数或评测测试结果。

## Token 用量与 API 成本

主模型（`claude-opus-5-5`）：

| 指标 | 值 |
|---|---:|
| 输入 tokens，不含缓存（`input_tokens`） | 498 |
| 缓存读取输入 tokens（`cache_read_input_tokens`） | 119,028,042 |
| 缓存写入输入 tokens（`cache_creation_input_tokens`） | 733,087 |
| 输入 tokens 合计，含缓存 | 119,761,627 |
| 输出 tokens，含 thinking（`output_tokens`） | 472,273 |
| Thinking tokens | 211,358 |
| **总 tokens** | **120,233,900** |
| 缓存命中率（按输入 tokens） | 99.39% |

完成评估器（`claude-haiku-4-5-20251001`，由 `/goal` 使用）：输入 3、缓存写入 131,770、缓存读取 0、输出 211 tokens，合计 **131,984 tokens**。

| | API 成本估算（美元） |
|---|---:|
| 主模型 | $39.12 |
| 完成评估器 | $0.17 |
| **合计** | **$39.28** |

主模型 token 数对 transcript 中每次响应的 usage 求和，与 Claude Code 的会话累计值完全一致。评估器不以响应的形式出现在 transcript 中，其 token 数只来自会话累计值。Thinking tokens 属于 output 的一部分，不重复相加。输入总量包含各次请求中重复发送的上下文，不是单次上下文大小，也不是去重后的 token 数。

成本采用 2026-09-27 查询到的 [OpenRouter Models API](https://openrouter.ai/api/v1/models) 中 `anthropic/claude-opus-5.5` 和 `anthropic/claude-haiku-4.5` 的非 batch 模型目录价。下列单价单位为美元／百万 tokens。这是当前目录价下的估算，不是实际费用：本次运行使用的是订阅。

| Token 类别（主模型） | 美元／百万 tokens | 费用估算（美元） |
|---|---:|---:|
| 非缓存输入 | $4.00 | $0.001992 |
| 输出 | $20.00 | $9.445460 |
| 缓存读取 | $0.20 | $23.805608 |
| 缓存写入，5 分钟 TTL | $5.00 | $0.000000 |
| 缓存写入，1 小时 TTL | $8.00 | $5.864696 |

主模型未舍入成本为 $39.1177564。全部 733,087 个缓存写入 tokens 都记录为 1 小时写入（`ephemeral_1h_input_tokens`），因此按 1 小时单价计算；结果与 Claude Code 自身记录的估算值相同。会话累计值没有按 TTL 拆分评估器的缓存写入。按 5 分钟单价（每百万 $1.25，输入 $1.00、输出 $5.00）计算，评估器成本为 $0.1657705，与 Claude Code 记录的估算值相同；按 1 小时单价则为 $0.2645980。合计采用 5 分钟口径：$39.2835269，最后统一舍入为 **$39.28**。Token 总数仅涵盖已报告的 usage。

生成时间、源码规模、tokens、活动量和成本均为描述性元数据，不是正确性评分。

## 评测结果

八组历史 profile 使用的候选二进制、Cargo.lock 和固定表面哈希完全一致。

xfstests 为 188 项通过、1 项预期失败、3 项失败、2 项超时、473 项 N/A。Canonical-format 为 479 个检查点通过。Crash-core 为 216 项通过。

历史 SDK、Spec、PJD、xfstests 和 robustness profile 尚未确立完整的环境审计覆盖。

<!-- core-results:begin -->

### 构建与 SDK 检查

| Profile | Classification | Pass | Expected fail | Fail | Unexpected pass | Timeout | Skip N/A | Skip impl | Pass rate | Category macro average |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `sdk-smoke` | `diagnostic` | 59 | 0 | 0 | 0 | 0 | 0 | 0 | 100.00% | — |

`sdk-smoke` run: `cc-opus-sdk-20260927-r1`

### POSIX 正确性

| Profile | Classification | Pass | Expected fail | Fail | Unexpected pass | Timeout | Skip N/A | Skip impl | Pass rate | Category macro average |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `spec-tests` | `diagnostic` | 44 | 0 | 0 | 0 | 0 | 7 | 0 | 100.00% | 100.00% |
| `pjdfstest-core` | `diagnostic` | 169 | 0 | 0 | 0 | 0 | 48 | 0 | 100.00% | 100.00% |
| `xfstests-core` | `diagnostic` | 188 | 1 | 3 | 0 | 2 | 473 | 0 | 97.42% | 98.90% |

`spec-tests` run: `cc-opus-spec-20260927-r1`

`pjdfstest-core` run: `case-correction-core-opus-5.5-cc-pjdfstest-core`

`xfstests-core` run: `cc-opus-xfs-20260927-r1`

### 稳健性

Profile: `robustness`

结果：`pass`；评测分类：`diagnostic`。

| Metric | Value |
|---|---:|
| Total | 47 |
| Pass | 47 |
| Fail | 0 |
| Timeout | 0 |
| Pass rate | 100.00% |
| Category macro average | 100.00% |

超时计失败，N/A 不计入分母，类别宏平均按类别等权。

Run: `cc-opus-robust-20260927-r1`
执行后端: `hyperv`

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

Run: `cc-opus-canonical-20260927-r1`
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

Run: `cc-opus-crash-oracle6-r2`
执行后端: `hyperv`

### 真实应用

Profile: `real-world`

结果：`pass`；评测分类：`diagnostic`。

| Metric | Value |
|---|---:|
| Total | 4 |
| Pass | 4 |
| Fail | 0 |
| Timeout | 0 |
| Pass rate | 100.00% |
| Category macro average | 100.00% |

Run: `cc-opus-realworld-20260927-r1`
执行后端: `hyperv`

### CPU 性能

Profile: `perf-cpu`

结果：`pass`；评测分类：`diagnostic`。

性能分：**45.43**（比较池 `hyperv-fixed-vhdx`）。

| 类别 | 缓存 | 主指标 | 样本数 | 中位数 | MAD | CV | 候选失败样本 |
|---|---|---|---:|---:|---:|---:|---:|
| buffered-synchronized-writes | cold | operations_per_second | 3 | 3,618.6 | 13.4 | 0.3% | 0 |
| buffered-synchronized-writes | warm | operations_per_second | 3 | 3,625.3 | 4.4 | 0.2% | 0 |
| bulk-sequential-io | cold | MiB/s | 5 | 382.3 | 2.5 | 0.9% | 0 |
| bulk-sequential-io | warm | MiB/s | 5 | 381.6 | 0.8 | 0.4% | 0 |
| metadata-concurrency | cold | operations_per_second | 3 | 2,829.1 | 3.6 | 0.2% | 0 |
| metadata-concurrency | cold | operations_per_second | 3 | 8,755.7 | 26.8 | 0.4% | 0 |
| metadata-concurrency | cold | operations_per_second | 3 | 2,264.0 | 0.7 | 0.1% | 0 |
| metadata-concurrency | cold | operations_per_second | 3 | 3,923.7 | 9.7 | 0.4% | 0 |
| metadata-concurrency | cold | operations_per_second | 3 | 4,617.6 | 11.9 | 0.3% | 0 |
| metadata-concurrency | cold | operations_per_second | 3 | 1,648.9 | 2.8 | 0.6% | 0 |
| metadata-concurrency | cold | operations_per_second | 5 | 438.7 | 5.2 | 5.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 3 | 2,835.3 | 8.7 | 0.6% | 0 |
| metadata-concurrency | warm | operations_per_second | 3 | 8,927.1 | 54.8 | 0.6% | 0 |
| metadata-concurrency | warm | operations_per_second | 3 | 2,262.4 | 0.5 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 3 | 3,916.2 | 5.5 | 0.2% | 0 |
| metadata-concurrency | warm | operations_per_second | 3 | 4,621.0 | 7.7 | 0.4% | 0 |
| metadata-concurrency | warm | operations_per_second | 3 | 1,646.2 | 0.1 | 0.3% | 0 |
| metadata-concurrency | warm | operations_per_second | 4 | 447.4 | 4.1 | 2.4% | 0 |
| small-random-io | cold | iops | 3 | 5,662.6 | 22.1 | 0.4% | 0 |
| small-random-io | cold | iops | 5 | 10,987.8 | 68.1 | 1.1% | 0 |
| small-random-io | cold | iops | 5 | 11,604.7 | 59.1 | 1.7% | 0 |
| small-random-io | warm | iops | 3 | 5,690.2 | 8.9 | 0.3% | 0 |
| small-random-io | warm | iops | 5 | 10,954.1 | 78.9 | 1.1% | 0 |
| small-random-io | warm | iops | 5 | 11,554.9 | 64.6 | 0.7% | 0 |

Run: `perf-v2-core-opus-5-5-cc-perf-cpu-r2`
执行后端: `hyperv`

### NVMe 性能

Profile: `perf-nvme`

结果：`pass`；评测分类：`diagnostic`。

性能分：**48.19**（比较池 `hyperv-fixed-vhdx`）。

| 类别 | 缓存 | 主指标 | 样本数 | 中位数 | MAD | CV | 候选失败样本 |
|---|---|---|---:|---:|---:|---:|---:|
| buffered-synchronized-writes | cold | operations_per_second | 3 | 2,264.0 | 4.0 | 0.2% | 0 |
| buffered-synchronized-writes | warm | operations_per_second | 3 | 2,251.7 | 3.4 | 0.2% | 0 |
| bulk-sequential-io | cold | MiB/s | 5 | 274.4 | 1.6 | 1.3% | 0 |
| bulk-sequential-io | warm | MiB/s | 5 | 276.4 | 0.6 | 1.4% | 0 |
| metadata-concurrency | cold | operations_per_second | 2 | 2,755.1 | 39.9 | 1.4% | 0 |
| metadata-concurrency | cold | operations_per_second | 3 | 8,592.1 | 5.1 | 0.7% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 2,224.7 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 2 | 3,850.3 | 34.0 | 0.9% | 0 |
| metadata-concurrency | cold | operations_per_second | 2 | 4,367.7 | 7.6 | 0.2% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 452.6 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 347.3 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 2 | 2,797.7 | 5.3 | 0.2% | 0 |
| metadata-concurrency | warm | operations_per_second | 3 | 8,881.5 | 49.1 | 0.9% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 2,227.2 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 2 | 3,856.9 | 9.4 | 0.2% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 4,334.7 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 463.4 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 344.9 | 0.0 | 0.0% | 0 |
| small-random-io | cold | iops | 4 | 3,252.3 | 9.7 | 0.6% | 0 |
| small-random-io | cold | iops | 4 | 5,058.5 | 38.3 | 1.9% | 0 |
| small-random-io | cold | iops | 4 | 3,348.4 | 6.3 | 0.7% | 0 |
| small-random-io | warm | iops | 3 | 3,267.7 | 11.2 | 0.3% | 0 |
| small-random-io | warm | iops | 4 | 5,080.2 | 45.3 | 1.5% | 0 |
| small-random-io | warm | iops | 3 | 3,296.5 | 17.1 | 0.9% | 0 |

Run: `perf-v2-core-opus-5-5-cc-perf-nvme-r2`
执行后端: `hyperv`

### Agent 代码评审

S：**62.50** / 100；状态：`complete`；用途：`diagnostic`。

| Critical | High | Medium | Low |
|---:|---:|---:|---:|
| 0 | 1 | 9 | 0 |

仅统计已确认、按根因去重的问题；以 20% 权重计入暂计总分。

Scoring rule: `agent-review-score-v2`.

## 评测范围与溯源

各 profile 原始结果独立保留；索引另列未测项暂计 0 的派生总分。每个 profile 列出其运行，并标明已记录的执行后端；不同后端不视为完全相同的执行条件。

`valid` 表示该 profile 的证据有效，不等于全部用例通过或全维度发布合格。`candidate-failed` 是候选失败，不是基础设施错误。`diagnostic` 仅诊断；`noneligible` 不具备排名资格。Hyper-V 性能不具备发布/排名资格；协议 v2 在固定 VHDX/内存设备上的 Hyper-V 运行单独组成 `hyperv-fixed-vhdx` 比较池计入暂计总分，与 KVM/raw-NVMe 分数不可比。

可维护性（`maint-v2-rev5`，占总分 5%）：**96.05** / 100，各项见 README。Reference 单列为校准基线，不是模型生成条目。
<!-- core-results:end -->
