# Reference 校准基线

Submission: `core-reference`

[English](reference-calibration.md)

本页仅报告 Eval 观察，不提供生成次数、耗时、token 或成本数据。各 profile 的候选和固定评测面身份独立记录。

SDK/POSIX 与其他 profile 使用的候选源码不完全相同，不能视为同一候选二进制的完整评测；各 profile 分别列出其运行。

## 评测结果

<!-- core-results:begin -->

### 构建与 SDK 检查

| Profile | Classification | Pass | Expected fail | Fail | Unexpected pass | Timeout | Skip N/A | Skip impl | Pass rate | Category macro average |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `sdk-smoke` | `diagnostic` | 82 | 0 | 0 | 0 | 0 | 0 | 0 | 100.00% | — |

`sdk-smoke` run: `reference-review-sdk-20261004-r1`

### POSIX 正确性

| Profile | Classification | Pass | Expected fail | Fail | Unexpected pass | Timeout | Skip N/A | Skip impl | Pass rate | Category macro average |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `spec-tests` | `diagnostic` | 44 | 0 | 0 | 0 | 0 | 7 | 0 | 100.00% | 100.00% |
| `pjdfstest-core` | `diagnostic` | 169 | 0 | 0 | 0 | 0 | 48 | 0 | 100.00% | 100.00% |
| `xfstests-core` | `diagnostic` | 190 | 1 | 2 | 0 | 1 | 473 | 0 | 98.45% | 99.42% |

`spec-tests` run: `reference-readdir-spec-20261004-r1`

`pjdfstest-core` run: `reference-readdir-pjd-20261004-r1`

`xfstests-core` run: `case-correction-core-reference-xfstests-core`

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

Run: `reference-review-robustness-20261004-r1`
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

Run: `reference-readdir-canonical-20261004-r1`
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

上表按用例是否满足全部要求统计结果。下表分别计算各项已验证能力，用于稳健性 R。

| 检查项目 | 分数 / 100 | 满足要求（项） | 未满足（项） | 未验证（计 0） |
|---|---:|---:|---:|---:|
| 磁盘结构 | 100.00 | 216 | 0 | 0 |
| 数据正确性 | 100.00 | 216 | 0 | 0 |
| 卸载与重开 | 100.00 | 216 | 0 | 0 |

崩溃分 **100.00**。权重：磁盘结构 40%、数据正确性 40%、卸载与重开 20%。

允许的安全拒绝：204 项；两轮已测试：12 项；失败后未再次打开：0 项；首轮未测试：0 项。

每轮包括挂载、检查和卸载。安全拒绝计满足要求，不表示数据已恢复。前一步失败导致未测试的再次打开不计通过。各检查项目的含义见[评分方法](../../docs/scoring.zh-CN.md)。

Run: `reference-readdir-crash-20261004-r1`
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

Run: `reference-review-realworld-20261004-r2`
执行后端: `hyperv`

### CPU 性能

Profile: `perf-cpu`

结果：`pass`；评测分类：`diagnostic`。

性能分：**61.00**（比较池 `hyperv-fixed-vhdx`）。已判定预算 100.0%；未测项对应的得分范围 61.00–61.00，不是置信区间。

分项（满分 100）：文件读写 45.90；元数据 39.64；RAM 100.00；写入效率 99.66。
RAM 峰值：62.1 MiB。元数据：14 项有有效速率，0 项候选失败，0 项未测。

写入放大 = 计时窗口内控制器写入字节数 ÷ 应用逻辑写入字节数；不是 SSD 内部写入放大。元数据操作不使用这个比值。“—”不代表零；候选失败得 0，未测项保留预算并暂计 0。* 表示部分预算未判定。

| 负载 | 缓存 | 样本数 | 速率 | MAD | CV | 候选失败样本 | 写入放大 | 速度分 / 100 |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| 顺序读写 | cold | 5 | 323.5 MiB/s | 6.9 | 6.5% | 0 | 0.998× | 34.87 |
| 顺序读写 | warm | 5 | 323.0 MiB/s | 2.2 | 9.4% | 0 | 1.001× | 34.89 |
| 随机读写（单线程） | cold | 3 | 4,207.9 IOPS | 23.7 | 0.5% | 0 | 0.996× | 27.93 |
| 随机读写（单线程） | warm | 3 | 4,159.2 IOPS | 42.6 | 1.1% | 0 | 0.996× | 27.65 |
| 随机读写（多线程） | cold | 5 | 7,018.3 IOPS | 20.8 | 1.1% | 0 | 0.008× | 25.87 |
| 随机读写（多线程） | warm | 5 | 7,051.6 IOPS | 35.5 | 0.5% | 0 | 0.000× | 25.97 |
| 随机读写（共享区域） | cold | 5 | 6,947.5 IOPS | 19.4 | 0.7% | 0 | 0.017× | 37.64 |
| 随机读写（共享区域） | warm | 5 | 6,929.9 IOPS | 83.3 | 1.0% | 0 | 0.000× | 37.56 |
| 同步覆盖写 | cold | 3 | 2,590.8 ops/s | 22.8 | 0.9% | 0 | 1.031× | 78.34 |
| 同步覆盖写 | warm | 3 | 2,614.4 ops/s | 6.4 | 0.6% | 0 | 1.032× | 78.49 |
| 创建、查询并删除 | cold | 3 | 2,686.0 ops/s | 14.2 | 0.4% | 0 | — | 24.17 |
| 创建、查询并删除 | warm | 3 | 2,665.4 ops/s | 4.3 | 0.4% | 0 | — | 24.67 |
| 深层与大目录查找 | cold | 3 | 7,144.4 ops/s | 30.5 | 0.4% | 0 | — | 14.58 |
| 深层与大目录查找 | warm | 3 | 7,284.5 ops/s | 37.1 | 0.6% | 0 | — | 15.36 |
| 目录内重命名 | cold | 3 | 1,715.2 ops/s | 15.0 | 0.8% | 0 | — | 27.23 |
| 目录内重命名 | warm | 3 | 1,716.0 ops/s | 0.3 | 0.4% | 0 | — | 26.92 |
| 同目录并发修改 | cold | 3 | 3,614.3 ops/s | 5.3 | 0.6% | 0 | — | 29.51 |
| 同目录并发修改 | warm | 3 | 3,589.4 ops/s | 3.7 | 0.7% | 0 | — | 29.88 |
| 跨目录并发创建 | cold | 3 | 3,506.6 ops/s | 31.1 | 1.4% | 0 | — | 35.36 |
| 跨目录并发创建 | warm | 3 | 3,594.4 ops/s | 3.4 | 0.1% | 0 | — | 35.74 |
| 并发同步落盘 | cold | 3 | 1,418.9 ops/s | 4.6 | 1.2% | 0 | — | 100.00 |
| 并发同步落盘 | warm | 3 | 1,402.9 ops/s | 0.3 | 1.1% | 0 | — | 100.00 |
| 接近满盘时回收空间 | cold | 4 | 396.7 ops/s | 4.4 | 1.8% | 0 | — | 50.93 |
| 接近满盘时回收空间 | warm | 5 | 389.4 ops/s | 7.7 | 4.6% | 0 | — | 49.39 |

Run: `reference-fallocate-perf-cpu-20261004-r1`
执行后端: `hyperv`

### NVMe 性能

Profile: `perf-nvme`

结果：`pass`；评测分类：`diagnostic`。

性能分：**60.24**（比较池 `hyperv-fixed-vhdx`）。已判定预算 100.0%；未测项对应的得分范围 60.24–60.24，不是置信区间。

分项（满分 100）：文件读写 45.25；元数据 37.25；RAM 100.00；写入效率 99.67。
RAM 峰值：67.9 MiB。元数据：14 项有有效速率，0 项候选失败，0 项未测。

写入放大 = 计时窗口内控制器写入字节数 ÷ 应用逻辑写入字节数；不是 SSD 内部写入放大。元数据操作不使用这个比值。“—”不代表零；候选失败得 0，未测项保留预算并暂计 0。* 表示部分预算未判定。

| 负载 | 缓存 | 样本数 | 速率 | MAD | CV | 候选失败样本 | 写入放大 | 速度分 / 100 |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| 顺序读写 | cold | 5 | 248.8 MiB/s | 3.4 | 2.0% | 0 | 0.999× | 39.53 |
| 顺序读写 | warm | 5 | 251.2 MiB/s | 0.7 | 1.5% | 0 | 1.000× | 39.82 |
| 随机读写（单线程） | cold | 5 | 2,536.5 IOPS | 16.1 | 0.7% | 0 | 1.000× | 27.82 |
| 随机读写（单线程） | warm | 5 | 2,533.3 IOPS | 15.1 | 0.6% | 0 | 1.000× | 27.75 |
| 随机读写（多线程） | cold | 5 | 3,161.9 IOPS | 0.7 | 0.7% | 0 | 0.007× | 20.76 |
| 随机读写（多线程） | warm | 5 | 3,205.4 IOPS | 25.8 | 2.1% | 0 | 0.000× | 21.01 |
| 随机读写（共享区域） | cold | 5 | 3,155.0 IOPS | 15.0 | 0.7% | 0 | 0.011× | 31.52 |
| 随机读写（共享区域） | warm | 5 | 3,156.1 IOPS | 8.6 | 0.5% | 0 | 0.000× | 31.51 |
| 同步覆盖写 | cold | 3 | 1,761.9 ops/s | 21.9 | 1.3% | 0 | 1.030× | 75.20 |
| 同步覆盖写 | warm | 3 | 1,761.8 ops/s | 8.1 | 0.4% | 0 | 1.032× | 75.18 |
| 创建、查询并删除 | cold | 3 | 2,669.0 ops/s | 0.0 | 0.2% | 0 | — | 24.03 |
| 创建、查询并删除 | warm | 3 | 2,649.6 ops/s | 6.2 | 0.3% | 0 | — | 24.54 |
| 深层与大目录查找 | cold | 1 | 6,968.2 ops/s | 0.0 | 0.0% | 0 | — | 14.04 |
| 深层与大目录查找 | warm | 1 | 7,331.6 ops/s | 0.0 | 0.0% | 0 | — | 15.50 |
| 目录内重命名 | cold | 1 | 1,692.3 ops/s | 0.0 | 0.0% | 0 | — | 26.94 |
| 目录内重命名 | warm | 1 | 1,671.1 ops/s | 0.0 | 0.0% | 0 | — | 26.34 |
| 同目录并发修改 | cold | 3 | 3,593.7 ops/s | 10.6 | 0.3% | 0 | — | 29.39 |
| 同目录并发修改 | warm | 3 | 3,578.2 ops/s | 7.0 | 0.3% | 0 | — | 29.81 |
| 跨目录并发创建 | cold | 2 | 3,386.6 ops/s | 2.2 | 0.1% | 0 | — | 34.61 |
| 跨目录并发创建 | warm | 2 | 3,361.8 ops/s | 35.3 | 1.1% | 0 | — | 34.29 |
| 并发同步落盘 | cold | 2 | 526.6 ops/s | 14.3 | 2.7% | 0 | — | 89.21 |
| 并发同步落盘 | warm | 1 | 533.4 ops/s | 0.0 | 0.0% | 0 | — | 89.37 |
| 接近满盘时回收空间 | cold | 5 | 314.4 ops/s | 7.5 | 3.7% | 0 | — | 45.88 |
| 接近满盘时回收空间 | warm | 5 | 309.2 ops/s | 10.0 | 5.5% | 0 | — | 44.38 |

Run: `reference-fallocate-perf-nvme-20261004-r1`
执行后端: `hyperv`

### Agent 代码评审

状态：`incomplete`；S 尚无完整结果，显示为 —，仅在暂计总分中填 0。

Scoring rule: `agent-review-score-v2`.

## 评测范围与溯源

各 profile 原始结果独立保留；索引另列未测项暂计 0 的派生总分。每个 profile 列出其运行，并标明已记录的执行后端；不同后端不视为完全相同的执行条件。

`valid` 表示该 profile 的证据有效，不等于全部用例通过或全维度发布合格。`candidate-failed` 是候选失败，不是基础设施错误。`diagnostic` 仅诊断；`noneligible` 不具备排名资格。Hyper-V 性能不具备发布/排名资格；协议 v2 在固定 VHDX/内存设备上的 Hyper-V 运行单独组成 `hyperv-fixed-vhdx` 比较池计入暂计总分，与 KVM/raw-NVMe 分数不可比。

可维护性（`maint-v3-policy`，占总分 5%）：**84.63** / 100，各项见 README。Reference 单列为校准基线，不是模型生成条目。
<!-- core-results:end -->
