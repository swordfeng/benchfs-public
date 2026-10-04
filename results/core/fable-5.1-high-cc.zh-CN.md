# Fable 5.1 — Core — High — Claude Code (`-cc`)

[English](fable-5.1-high-cc.md)

本页记录 Fable 5.1 的第二次 Core 生成运行，用于与 [Pi harness 下的 Fable 5.1](fable-5.1-high.zh-CN.md) 对照。八项非性能 profile 已全部完成：SDK、51-case Spec、PJD、xfstests、robustness、canonical-format、crash-core、real-world。

## 运行信息

| 字段 | 值 |
|---|---|
| 模型 | `claude-fable-5-1` |
| Effort | High（`--effort high`） |
| Variant | Core |
| Agent harness | Claude Code 2.1.283，精简配置（见下文） |
| 生成运行次数 | 1（`n=1`） |
| 生成日期 | 2026-09-27（UTC） |
| 评测 | 八项非性能 profile 全部完成并通过独立验证 |

**Harness 差异。** 本次运行通过 Claude 订阅使用 Claude Code，而不是 Pi harness，因此不能与 Pi harness 的报告直接比较；与同一模型的 Pi 运行对照正是这次运行的目的。任务面没有变化：Dev VM 镜像、agent 可见文档和 prompt 文本都相同。harness 只保留 `Bash`、`Read`、`Edit`、`Write` 四个工具，没有 subagent、MCP server、Web 工具或记忆。模型流量经环境的出网白名单访问 Anthropic，而不是经过 model proxy。Prompt 作为 `/goal` 条件只输入一次。与 Pi 的 goal 扩展由 agent 自行宣布完成不同，Claude Code 的 `/goal` 在每个 turn 之后由另一个小模型（Haiku 4.5）判断是否完成；条件尚未满足时，会把判断理由回传给主模型。Claude Code 还会加入自己的 system prompt、工具说明和上下文提醒；使用订阅登录时，其中包含账户身份信息。

**中断。** 生成会话曾中断，随后通过 `--continue` 和一条不含技术内容的 `continue` 消息恢复。恢复时调整了进程 scope 策略，因此环境与早期运行不同。Goal 条件以及中断前后 agent 的全部工作仍在同一个连续的 transcript 中；下方时间表保留中断时长。

## 提交规模

仅统计 `bench/` 下候选实现自有的源码。固定 SDK 和 FUSE adapter、依赖、文档、配置、锁文件及生成文件均不计入。

| 组成部分 | 包含该部分的文件数 | 总行数 | 代码行数 |
|---|---:|---:|---:|
| 实现（Rust） | 11 | 7,106 | 6,353 |
| Agent 编写的测试及测试支持代码（Rust） | 5 | 867 | 810 |
| 测试运行脚本 | 0 | 0 | 0 |

总行数包含空行和注释。代码行数统计至少包含一个非注释 token 的行：空行、注释和文档注释（Rust 的 `///`、`//!`，Python docstring）不计入，`#[derive(...)]` 等 Rust 属性计为代码。测试包括 `tests/` 下的 1 个独立文件，以及四个实现文件（`src/bitmap.rs`、`src/btree.rs`、`src/cache.rs`、`src/format.rs`）中的内联 `#[cfg(test)]` 模块（含属性行）。这四个文件在文件数上重叠；总行数和代码行数按行划分，不重叠。共有 12 个不同的 Rust 文件。这些数据仅衡量源码规模，不是测试覆盖率或代码质量评分。

<!-- source-tree:begin -->
### 目录结构

`bench/` 下的全部文件，不含构建产物。数字为按上述规则统计的代码行，“测试”为其中属于测试代码的部分。

```text
bench/
├── src/
│   ├── bitmap.rs  (253 行代码，测试 48)
│   ├── btree.rs  (1,159 行代码，测试 234)
│   ├── cache.rs  (320 行代码，测试 50)
│   ├── check.rs  (473 行代码)
│   ├── core.rs  (1,061 行代码)
│   ├── format.rs  (1,084 行代码，测试 55)
│   ├── fs.rs  (1,912 行代码)
│   ├── lib.rs  (8 行代码)
│   ├── main.rs  (181 行代码)
│   ├── mkfs.rs  (75 行代码)
│   └── mount.rs  (214 行代码)
├── tests/
│   └── memfs.rs  (423 行代码，测试 423)
├── Cargo.toml
├── NOTES.md
└── README.md
```
<!-- source-tree:end -->

## 生成时间与活动量

| 指标 | 值 |
|---|---|
| Goal 开始时间 | 2026-09-27 06:08:15.712 UTC |
| Goal 完成时间 | 2026-09-27 09:16:03.855 UTC |
| Goal 活跃时长 | **2 小时 59 分 12 秒** |
| 经过时长，含中断 | 3 小时 7 分 48 秒 |
| Harness 中断 | 8 分 36 秒（UTC 06:41:43.526 – 06:50:19.294） |
| 最终 goal 状态 | Met（由 `/goal` 评估器判定） |
| 含 usage 的主模型 API 响应 | 127 |
| API 错误记录 | 0 |
| Agent 工具调用次数 | 173 |
| API 时间／工具执行时间 | 1 小时 5 分 18 秒／1 小时 54 分 14 秒 |

活跃时长为经过时长减去中断时长；中断区间从 harness 被终止前的最后一条 transcript 记录，到恢复后的会话记录被中断调用结果的那一条记录为止。期间没有额度暂停、API 错误记录或上下文压缩；Claude Code 的会话状态中有不到 1 秒的 API 时间属于重试。除 goal 条件外，operator 输入给模型的只有恢复后的那条 `continue`。API 时间和工具时间来自 Claude Code 的会话累计状态，恢复后的会话沿用了这些累计值。活跃时长是 harness 记录的墙钟时间，不是模型推理时间。Goal 判定完成不代表评测测试通过。

响应数来自归档的 Claude Code transcript，按 message ID 去重：Claude Code 为每个内容块写一条记录，127 次响应共有 348 条 assistant 记录。该 harness 没有 model proxy 日志。环境出网日志显示，除 Anthropic 外，agent 只访问了 `crates.io`。Claude Code 在设置 goal 之前和 goal 完成之后启动时，曾尝试连接 `downloads.claude.ai` 和 `github.com`，均被白名单拒绝。

### 工具使用统计

| 工具 | 请求次数 | 已记录失败次数 | 结果未知次数 |
|---|---:|---:|---:|
| `Bash` | 163 | 4 | 1 |
| `Read` | 10 | 0 | 0 |
| **合计** | **173** | **4** | **1** |

每个 `tool_use` 块按工具名计一次请求。通过 `tool_use_id` 配对结果，`tool_result.is_error: true` 计为已记录失败，但 harness 被终止时正在执行的那次调用除外：Claude Code 把它的结果标为错误，同时说明其结果未知，因此计为结果未知。每次调用都有对应结果。Agent 通过 `Bash` 写文件，没有使用 `Edit` 或 `Write`。这些是工具级结果，不是模型 API 错误数或评测测试结果。

## Token 用量与 API 成本

主模型（`claude-fable-5-1`）：

| 指标 | 值 |
|---|---:|
| 输入 tokens，不含缓存（`input_tokens`） | 3,704 |
| 缓存读取输入 tokens（`cache_read_input_tokens`） | 49,120,546 |
| 缓存写入输入 tokens（`cache_creation_input_tokens`） | 559,750 |
| 输入 tokens 合计，含缓存 | 49,684,000 |
| 输出 tokens，含 thinking（`output_tokens`） | 340,562 |
| Thinking tokens | 135,351 |
| **总 tokens** | **50,024,562** |
| 缓存命中率（按输入 tokens） | 98.87% |

完成评估器（`claude-haiku-4-5-20251001`，由 `/goal` 使用）：输入 3、缓存写入 115,555、缓存读取 0、输出 345 tokens，合计 **115,903 tokens**。

| | API 成本估算（美元） |
|---|---:|
| 主模型 | $40.54 |
| 完成评估器 | $0.15 |
| **合计** | **$40.69** |

主模型 token 数对 transcript 中每次响应的 usage 求和，与 Claude Code 的会话累计值完全一致。恢复后的会话退出时写入的最后一份会话状态快照退回到了中断前的数值，因此合计采用累计量最大的快照。评估器不以响应的形式出现在 transcript 中，其 token 数只来自会话累计值。Thinking tokens 属于 output 的一部分，不重复相加。输入总量包含各次请求中重复发送的上下文，不是单次上下文大小，也不是去重后的 token 数。

成本采用 2026-09-27 查询到的 [OpenRouter Models API](https://openrouter.ai/api/v1/models) 中 `anthropic/claude-fable-5.1` 和 `anthropic/claude-haiku-4.5` 的非 batch 模型目录价；主模型单价与 Pi 运行所用的相同。下列单价单位为美元／百万 tokens。这是当前目录价下的估算，不是实际费用：本次运行使用的是订阅。

| Token 类别（主模型） | 美元／百万 tokens | 费用估算（美元） |
|---|---:|---:|
| 非缓存输入 | $10.00 | $0.037040 |
| 输出 | $50.00 | $17.028100 |
| 缓存读取 | $0.25 | $12.280137 |
| 缓存写入，5 分钟 TTL | $12.50 | $0.000000 |
| 缓存写入，1 小时 TTL | $20.00 | $11.195000 |

主模型未舍入成本为 $40.5402765。全部 559,750 个缓存写入 tokens 都记录为 1 小时写入（`ephemeral_1h_input_tokens`），因此按 1 小时单价计算；结果与 Claude Code 自身记录的估算值相同。Pi 运行的缓存写入是 5 分钟写入，因此两次运行的缓存写入成本不能直接比较。会话累计值没有按 TTL 拆分评估器的缓存写入。按 5 分钟单价（每百万 $1.25，输入 $1.00、输出 $5.00）计算，评估器成本为 $0.14617175，与 Claude Code 记录的估算值相同；按 1 小时单价则为 $0.232838。合计采用 5 分钟口径：$40.68644825，最后统一舍入为 **$40.69**。Token 总数仅涵盖已报告的 usage。

生成时间、源码规模、tokens、活动量和成本均为描述性元数据，不是正确性评分。

## 评测结果

SDK 为 54 项通过；Spec 为 44 项通过、7 项 N/A；xfstests 为 187 项通过、1 项预期失败、3 项失败、3 项超时、473 项 N/A。PJD 为 169 项通过、48 项 N/A。Robustness 为 47/47 通过。Canonical-format 完成 479 个检查点：431 项通过、48 项失败；原生一致性分为 95.45，不使用检查点通过率代替该分数。

**暂计总分：61.56/100；C：97.45；R：68.52；W：83.33。** 非性能输入已全部实测。性能、代码审查和可维护性仍未测，按既有规则零填充，因此不是完整六维成绩。

<!-- core-results:begin -->

### 构建与 SDK 检查

| Profile | Classification | Pass | Expected fail | Fail | Unexpected pass | Timeout | Skip N/A | Skip impl | Pass rate | Category macro average |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `sdk-smoke` | `diagnostic` | 54 | 0 | 0 | 0 | 0 | 0 | 0 | 100.00% | — |

`sdk-smoke` run: `cc-fable-sdk-20260927-r1`

### POSIX 正确性

| Profile | Classification | Pass | Expected fail | Fail | Unexpected pass | Timeout | Skip N/A | Skip impl | Pass rate | Category macro average |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `spec-tests` | `diagnostic` | 44 | 0 | 0 | 0 | 0 | 7 | 0 | 100.00% | 100.00% |
| `pjdfstest-core` | `diagnostic` | 169 | 0 | 0 | 0 | 0 | 48 | 0 | 100.00% | 100.00% |
| `xfstests-core` | `diagnostic` | 187 | 1 | 3 | 0 | 3 | 473 | 0 | 96.91% | 94.36% |

`spec-tests` run: `cc-fable-spec-20260927-r1`

`pjdfstest-core` run: `case-correction-core-fable-5.1-cc-pjdfstest-core`

`xfstests-core` run: `cc-fable-xfs-20260927-r1`

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

Run: `cc-fable-robust-approved-r1`
执行后端: `qemu-kvm`

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

Run: `cc-fable-canonical-approved-r1`
执行后端: `qemu-kvm`

### 崩溃一致性

Profile: `crash-core`

结果：`candidate-failed`；评测分类：`valid`。

| Metric | Value |
|---|---:|
| Total | 216 |
| Pass | 80 |
| Fail | 136 |
| Timeout | 0 |
| Pass rate | 37.04% |
| Category macro average | 37.04% |

Run: `cc-fable-crash-approved-r2`
执行后端: `qemu-kvm`

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

Run: `cc-fable-realworld-approved-r1`
执行后端: `qemu-kvm`

### CPU 性能

Profile: `perf-cpu`

结果：`pass`；评测分类：`diagnostic`。

性能分：**47.69**（比较池 `hyperv-fixed-vhdx`）。

| 类别 | 缓存 | 主指标 | 样本数 | 中位数 | MAD | CV | 候选失败样本 |
|---|---|---|---:|---:|---:|---:|---:|
| buffered-synchronized-writes | cold | operations_per_second | 3 | 2,617.4 | 6.2 | 0.2% | 0 |
| buffered-synchronized-writes | warm | operations_per_second | 3 | 2,623.6 | 9.9 | 0.5% | 0 |
| bulk-sequential-io | cold | MiB/s | 3 | 632.5 | 0.4 | 0.1% | 0 |
| bulk-sequential-io | warm | MiB/s | 3 | 632.4 | 0.7 | 0.2% | 0 |
| metadata-concurrency | cold | operations_per_second | 3 | 2,520.6 | 2.0 | 0.1% | 0 |
| metadata-concurrency | cold | operations_per_second | 3 | 8,881.6 | 8.2 | 0.3% | 0 |
| metadata-concurrency | cold | operations_per_second | 3 | 2,323.3 | 1.2 | 0.1% | 0 |
| metadata-concurrency | cold | operations_per_second | 3 | 3,943.7 | 9.8 | 0.3% | 0 |
| metadata-concurrency | cold | operations_per_second | 3 | 3,993.6 | 9.3 | 0.3% | 0 |
| metadata-concurrency | cold | operations_per_second | 5 | 1,659.9 | 37.0 | 3.1% | 0 |
| metadata-concurrency | cold | operations_per_second | 4 | 574.2 | 6.1 | 1.9% | 0 |
| metadata-concurrency | warm | operations_per_second | 3 | 2,515.0 | 5.0 | 0.2% | 0 |
| metadata-concurrency | warm | operations_per_second | 3 | 8,923.8 | 8.0 | 0.2% | 0 |
| metadata-concurrency | warm | operations_per_second | 3 | 2,317.1 | 1.6 | 0.2% | 0 |
| metadata-concurrency | warm | operations_per_second | 3 | 3,940.6 | 5.6 | 0.2% | 0 |
| metadata-concurrency | warm | operations_per_second | 3 | 3,991.4 | 12.1 | 0.3% | 0 |
| metadata-concurrency | warm | operations_per_second | 3 | 1,697.4 | 13.5 | 0.8% | 0 |
| metadata-concurrency | warm | operations_per_second | 3 | 573.5 | 1.0 | 1.4% | 0 |
| small-random-io | cold | iops | 3 | 5,093.0 | 11.6 | 0.3% | 0 |
| small-random-io | cold | iops | 5 | 15,168.1 | 11.7 | 0.2% | 0 |
| small-random-io | cold | iops | 5 | 10,482.9 | 53.6 | 0.6% | 0 |
| small-random-io | warm | iops | 3 | 5,088.6 | 3.9 | 0.1% | 0 |
| small-random-io | warm | iops | 5 | 15,116.7 | 15.3 | 0.2% | 0 |
| small-random-io | warm | iops | 5 | 10,515.0 | 14.0 | 0.4% | 0 |

Run: `perf-v2-core-fable-5-1-cc-perf-cpu-r2`
执行后端: `hyperv`

### NVMe 性能

Profile: `perf-nvme`

结果：`pass`；评测分类：`diagnostic`。

性能分：**52.11**（比较池 `hyperv-fixed-vhdx`）。

| 类别 | 缓存 | 主指标 | 样本数 | 中位数 | MAD | CV | 候选失败样本 |
|---|---|---|---:|---:|---:|---:|---:|
| buffered-synchronized-writes | cold | operations_per_second | 3 | 1,796.7 | 1.0 | 0.3% | 0 |
| buffered-synchronized-writes | warm | operations_per_second | 3 | 1,797.1 | 8.0 | 0.5% | 0 |
| bulk-sequential-io | cold | MiB/s | 4 | 323.8 | 0.6 | 0.2% | 0 |
| bulk-sequential-io | warm | MiB/s | 4 | 327.4 | 0.7 | 0.3% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 2,213.6 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 8,849.3 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 2,333.3 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 3,640.2 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 3,309.6 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 566.8 | 0.0 | 0.0% | 0 |
| metadata-concurrency | cold | operations_per_second | 1 | 376.5 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 2,218.1 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 8,897.3 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 2,342.8 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 3,779.9 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 3,275.7 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 630.0 | 0.0 | 0.0% | 0 |
| metadata-concurrency | warm | operations_per_second | 1 | 380.0 | 0.0 | 0.0% | 0 |
| small-random-io | cold | iops | 3 | 3,377.3 | 18.9 | 0.7% | 0 |
| small-random-io | cold | iops | 3 | 8,226.8 | 2.2 | 0.1% | 0 |
| small-random-io | cold | iops | 5 | 4,872.2 | 0.6 | 0.2% | 0 |
| small-random-io | warm | iops | 3 | 3,414.5 | 4.7 | 0.8% | 0 |
| small-random-io | warm | iops | 3 | 8,183.6 | 16.4 | 0.2% | 0 |
| small-random-io | warm | iops | 5 | 4,880.8 | 13.9 | 0.5% | 0 |

Run: `perf-v2-core-fable-5-1-cc-perf-nvme-r1`
执行后端: `hyperv`

### Agent 代码评审

S：**44.84** / 100；状态：`complete`；用途：`diagnostic`。

| Critical | High | Medium | Low |
|---:|---:|---:|---:|
| 1 | 0 | 16 | 3 |

仅统计已确认、按根因去重的问题；以 20% 权重计入暂计总分。

Scoring rule: `agent-review-score-v2`.

## 评测范围与溯源

各 profile 原始结果独立保留；索引另列未测项暂计 0 的派生总分。每个 profile 列出其运行，并标明已记录的执行后端；不同后端不视为完全相同的执行条件。

`valid` 表示该 profile 的证据有效，不等于全部用例通过或全维度发布合格。`candidate-failed` 是候选失败，不是基础设施错误。`diagnostic` 仅诊断；`noneligible` 不具备排名资格。Hyper-V 性能不具备发布/排名资格；协议 v2 在固定 VHDX/内存设备上的 Hyper-V 运行单独组成 `hyperv-fixed-vhdx` 比较池计入暂计总分，与 KVM/raw-NVMe 分数不可比。

可维护性（`maint-v2-rev5`，占总分 5%）：**84.39** / 100，各项见 README。Reference 单列为校准基线，不是模型生成条目。
<!-- core-results:end -->
