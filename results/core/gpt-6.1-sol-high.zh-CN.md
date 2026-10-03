# GPT 6.1 Sol · Pi — Core 测试报告

[English](gpt-6.1-sol-high.md) · [返回总表](../../README.zh-CN.md#results)

本次测试使用 GPT 6.1 Sol 与 Pi 完成 Core 开发任务。下文列出测试成绩、开发耗时、token 用量及费用估算。

## 运行设置

| 项目 | 设置 |
|---|---|
| 模型 | `gpt-6.1-sol` |
| 编程 Agent | Pi 0.84.3 |
| 推理设置 | High，由模型代理配置指定 |
| 开发次数 | 1 |
| 提交标识 | `core-gpt-6.1-sol` |
| 开发日期 | 2026-09-30（UTC） |

开发环境和工具配置见[实验环境](../../docs/environment.zh-CN.md)。各项测试的运行标识和执行后端列在下方结果中。

## 测试结果

<!-- core-results:begin -->

### 构建与 SDK 检查

| Profile | Classification | Pass | Expected fail | Fail | Unexpected pass | Timeout | Skip N/A | Skip impl | Pass rate | Category macro average |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `sdk-smoke` | `valid` | 54 | 0 | 0 | 0 | 0 | 0 | 0 | 100.00% | — |

`sdk-smoke` run: `eval-core-gpt-6-1-sol-sdk-smoke`

### POSIX 正确性

| Profile | Classification | Pass | Expected fail | Fail | Unexpected pass | Timeout | Skip N/A | Skip impl | Pass rate | Category macro average |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `spec-tests` | `valid` | 44 | 0 | 0 | 0 | 0 | 7 | 0 | 100.00% | 100.00% |
| `pjdfstest-core` | `valid` | 169 | 0 | 0 | 0 | 0 | 48 | 0 | 100.00% | 100.00% |
| `xfstests-core` | `valid` | 186 | 1 | 3 | 0 | 4 | 473 | 0 | 96.39% | 89.81% |

`spec-tests` run: `eval-core-gpt-6-1-sol-spec-tests`

`pjdfstest-core` run: `eval-core-gpt-6-1-sol-pjdfstest-core`

`xfstests-core` run: `eval-core-gpt-6-1-sol-xfstests-core`

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

Run: `eval-core-gpt-6-1-sol-robustness`
执行后端: `qemu-kvm`

### 磁盘格式一致性

Profile: `canonical-format`

结果：`pass`；评测分类：`valid`。

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

Run: `eval-core-gpt-6-1-sol-canonical-format`
执行后端: `qemu-kvm`

### 崩溃一致性

Profile: `crash-core`

结果：`pass`；评测分类：`valid`。

| Metric | Value |
|---|---:|
| Total | 216 |
| Pass | 216 |
| Fail | 0 |
| Timeout | 0 |
| Pass rate | 100.00% |
| Category macro average | 100.00% |

Run: `eval-core-gpt-6-1-sol-crash-core-r2`
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

Run: `eval-core-gpt-6-1-sol-real-world`
执行后端: `qemu-kvm`

### CPU 性能

Profile: `perf-cpu`

结果：`pass`；评测分类：`diagnostic`。


#### buffered-synchronized-writes / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 2589.9 |
| MAD | 12.300000000000182 |
| CV | 0.010634977592942763 |
| Candidate-failed samples | 0 |

#### buffered-synchronized-writes / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 2575.1 |
| MAD | 0.6999999999998181 |
| CV | 0.005009175776058064 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / cold

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 5 |
| Median | 344266330.0 |
| MAD | 1343721.0 |
| CV | 0.004567858205404987 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / warm

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 5 |
| Median | 341036094.0 |
| MAD | 1189710.0 |
| CV | 0.014480246901825276 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 2487.936957251538 |
| MAD | 6.824729674292485 |
| CV | 0.004449545859151276 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 7475.16981178434 |
| MAD | 3.1392913399140525 |
| CV | 0.002599565134307536 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 1741.3359903548012 |
| MAD | 14.213507644051106 |
| CV | 0.007916727349025047 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 3434.6284469153957 |
| MAD | 5.29287290630964 |
| CV | 0.0034689536489952394 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 2628.693659772316 |
| MAD | 0.19285512903661584 |
| CV | 8.216933484033831e-05 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 1280.172633225453 |
| MAD | 2.5024326215852852 |
| CV | 0.0036888234443918696 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 4 |
| Median | 428.65068790029625 |
| MAD | 2.250445796413601 |
| CV | 0.020676974519239177 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 2490.0855496630143 |
| MAD | 6.533544822847944 |
| CV | 0.004027740717027429 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 7541.235595387099 |
| MAD | 3.9051664299859112 |
| CV | 0.0063323238423675324 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 1734.6490644058342 |
| MAD | 3.6090870499558605 |
| CV | 0.0027039560228499654 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 3432.1666903519776 |
| MAD | 9.125102063893337 |
| CV | 0.004319153530145256 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 2618.9141105267854 |
| MAD | 4.768780328462981 |
| CV | 0.0015159235272029568 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 1284.540433212674 |
| MAD | 8.652845708976201 |
| CV | 0.00651471508561826 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 432.6613765640889 |
| MAD | 2.366026464568961 |
| CV | 0.012499628419648615 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 3 |
| Median | 4594.440556 |
| MAD | 41.795820000000276 |
| CV | 0.009837651237396538 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 7248.550289999999 |
| MAD | 39.17122699999891 |
| CV | 0.01609814472232201 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 3 |
| Median | 7290.170943 |
| MAD | 17.698049999999967 |
| CV | 0.00418346223358922 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 3 |
| Median | 4593.140686000001 |
| MAD | 38.89611100000002 |
| CV | 0.0075593074838785764 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 7204.070981999999 |
| MAD | 57.57129000000077 |
| CV | 0.01450394288933547 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 3 |
| Median | 7274.389714 |
| MAD | 58.41395000000011 |
| CV | 0.007644529229799184 |
| Candidate-failed samples | 0 |

Run: `perf-v2-core-gpt-6-1-sol-perf-cpu-r4`
执行后端: `hyperv`

### NVMe 性能

Profile: `perf-nvme`

结果：`pass`；评测分类：`diagnostic`。


#### buffered-synchronized-writes / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 1772.7 |
| MAD | 6.2999999999999545 |
| CV | 0.00363696834126647 |
| Candidate-failed samples | 0 |

#### buffered-synchronized-writes / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 3 |
| Median | 1761.4 |
| MAD | 1.800000000000182 |
| CV | 0.006654794394648604 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / cold

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 5 |
| Median | 284546142.0 |
| MAD | 566714.0 |
| CV | 0.0034392725064478573 |
| Candidate-failed samples | 0 |

#### bulk-sequential-io / warm

| Metric | Value |
|---|---:|
| Primary metric | throughput_bytes_per_second |
| Samples | 5 |
| Median | 286132523.0 |
| MAD | 1691342.0 |
| CV | 0.008107017532512925 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 2219.3449838175734 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 7579.417969536214 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 1718.655883839244 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 3049.111745094359 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 1918.076050003212 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 498.7744451629674 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / cold

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 400.37479885670575 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 2214.449216364136 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 7670.12375092865 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 1735.1704840784162 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 3034.103712731025 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 1910.5668652252132 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 487.1631681580132 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### metadata-concurrency / warm

| Metric | Value |
|---|---:|
| Primary metric | operations_per_second |
| Samples | 1 |
| Median | 402.44571627724565 |
| MAD | 0.0 |
| CV | 0.0 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 3 |
| Median | 2664.7335270000003 |
| MAD | 3.5996399999994537 |
| CV | 0.002418763063490954 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 3231.553569 |
| MAD | 24.09518100000014 |
| CV | 0.015097262488616118 |
| Candidate-failed samples | 0 |

#### small-random-io / cold

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 3172.065047 |
| MAD | 33.79366100000016 |
| CV | 0.01296737490477681 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 3 |
| Median | 2668.533147 |
| MAD | 13.698629999999866 |
| CV | 0.005450133010804131 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 3379.5399829999997 |
| MAD | 44.640452999999525 |
| CV | 0.024289937223036035 |
| Candidate-failed samples | 0 |

#### small-random-io / warm

| Metric | Value |
|---|---:|
| Primary metric | iops |
| Samples | 5 |
| Median | 3126.7366309999998 |
| MAD | 46.19419700000026 |
| CV | 0.017480171952505463 |
| Candidate-failed samples | 0 |

Run: `perf-v2-core-gpt-6-1-sol-perf-nvme-r7`
执行后端: `hyperv`

## 评测范围与溯源

各 profile 原始结果独立保留；索引另列未测项暂计 0 的派生总分。每个 profile 列出其运行，并标明已记录的执行后端；不同后端不视为完全相同的执行条件。

`valid` 表示该 profile 的证据有效，不等于全部用例通过或全维度发布合格。`candidate-failed` 是候选失败，不是基础设施错误。`diagnostic` 仅诊断；`noneligible` 不具备排名资格。Hyper-V 性能不具备发布/排名资格；协议 v2 在固定 VHDX/内存设备上的 Hyper-V 运行单独组成 `hyperv-fixed-vhdx` 比较池计入暂计总分，与 KVM/raw-NVMe 分数不可比。

安全审查与可维护性评分尚未评定。Reference 单列为校准基线，不是模型生成条目。
<!-- core-results:end -->

## 开发耗时

| 项目 | 记录 |
|---|---|
| 任务开始 | 2026-09-30 08:04:55.609 UTC |
| 任务完成 | 2026-09-30 19:45:06.327 UTC |
| 活跃时长 | **7 小时 14 分 12 秒** |
| 起止时间差 | 11 小时 40 分 11 秒 |
| 任务状态 | 已完成（Agent 记录） |

活跃时长取自 Pi 的任务累计计时，共 26,052.273 秒；它与起止时间差分开记录，不代表模型的推理耗时。

## Token 用量与费用

| 项目 | 数量 |
|---|---:|
| 未缓存输入 tokens | 2,000,258 |
| 缓存读取 tokens | 29,043,712 |
| 单独记录的缓存写入 tokens | 0 |
| 输入 tokens 合计 | 31,043,970 |
| 输出 tokens（含推理） | 184,515 |
| 其中：推理 tokens | 112,947 |
| 总 tokens | 31,228,485 |
| 估算费用（美元） | **$12.87** |

用量汇总自 172 条带有 token 记录的回复，与归档的代理汇总一致。推理 tokens 已包含在输出中，不重复计费。输入包含多次请求中重复发送的上下文。

费用按 2026-09-30 查询的 [OpenAI GPT 6.1 Sol 标价](https://developers.openai.com/api/docs/models/gpt-6.1-sol)估算。标准档每百万 tokens：输入 $2、缓存读取 $0.10、缓存写入 $2.50、输出 $10。单次请求的总输入超过 272,000 tokens 时，输入和缓存费率翻倍，输出费率乘以 1.5；按整次请求计算。

| 请求档位 | 回复数 | 未缓存输入 | 缓存读取 | 输出（含推理） |
|---|---:|---:|---:|---:|
| 标准 | 150 | 1,078,628 | 23,389,312 | 135,009 |
| 长上下文 | 22 | 921,630 | 5,654,400 | 49,506 |

记录未单独拆出缓存写入量。为与此前 Sol 报告的估算口径一致，这里将未缓存输入全部按缓存写入费率计算，替代普通输入费率，两者不叠加。由此得到 $12.8672112，四舍五入为 **$12.87**；若全部按普通输入费率计算，则为 $11.41。这是基于记录用量和公开标价的估算，不是账单金额。

## 源码规模

仅统计提交中 `bench/` 下的实现、测试和脚本，不含固定 SDK、FUSE adapter、依赖、文档、配置及生成文件。

| 组成 | 含该类代码的文件数 | 物理行数 | 代码行数 |
|---|---:|---:|---:|
| Rust 实现 | 10 | 4,032 | 3,960 |
| Agent 编写的 Rust 测试 | 4 | 1,113 | 1,100 |
| 脚本 | 1 | 95 | 82 |

物理行数包含空行和注释；代码行数排除空行、注释和文档注释。内联测试与实现可能位于同一文件，文件数不能直接相加；Rust 文件共 13 个。源码规模、耗时和费用仅作描述，不参与评分。
