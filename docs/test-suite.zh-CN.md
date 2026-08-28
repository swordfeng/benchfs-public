# 测试 Suite 概览

[English](test-suite.md)

BenchFS 组合 benchmark-specific semantic check、上游 POSIX suite、robustness workload，以及 Variant 特定的正确性、恢复、格式符合性和性能评测。单个 case 标识、case 正文、预期输出、seed 和 hidden selection 细节都不会公开。

## 冻结的汇总数量

下表描述当前 Core correctness manifest。“Selected”包含在模型运行前被归类为不适用的 case；“Scored”排除这些预分类 N/A 项。

| Suite | Selected | Dev-visible | Scored | 预分类 N/A |
|---|---:|---:|---:|---:|
| BenchFS semantic check | 51 | 33 | 44 | 7 |
| pjdfstest Core profile | 217 | 217 | 215 | 2 |
| xfstests generic profile | 667 | 31 | 184 | 483 |
| Seeded robustness workload | 2 | 0 | 2 | 0 |

### xfstests 对比

Evaluator 总共选取 **667** 个上游 generic case。Dev VM 暴露一个冻结的 **31-case** 子集，并且这些 case 全部属于 evaluator 的选取集合。因此，约 4.6% 的已选 xfstests 表面对 Dev 可见。公开仓库只发布这些汇总数量，绝不发布具体 case number。

如果一个已选 case 的前置条件属于 benchmark profile 以外的文件系统或环境能力，它可以被预分类为 N/A。该分类必须在观察候选结果之前冻结。

## 可见性规则

- **Dev-visible：** 存在于 Dev VM，可用于 agent 开发反馈。
- **Eval-hidden：** 不存在于 Dev VM，只在 agent 轨迹结束后执行。
- Hidden check 可以更广泛地测试相同的已发布能力类别，但不能引入新的、未说明的 feature 要求。
- Hidden 测试材料、manifest、精确 identity 和详细失败证据不进入本公开仓库。

## 下一版本状态

下面说明的正式真实 workload 和性能 profile 属于下一 benchmark version 的 requirements。它们不会追加入当前 pilot；runner、环境、metric 和 oracle 完成 qualification 并冻结之前，不发布正式结果。

## 合成性能 Workload

性能使用两个分开报告的环境：memory-backed profile 测量软件开销，dedicated persistent-storage profile 测量端到端行为。Evaluator 通过普通 Linux syscall 和 FUSE 操作真实挂载的文件系统。Workload 类别覆盖大块顺序 I/O、小块随机 I/O、buffered synchronized write、metadata 和目录操作、容量压力、并发，以及适用的 Variant 操作。

Integrity precheck 和 postcheck 位于 timed window 之外。每个 timed workload 都有 warm-up、固定 measurement window、cold/warm cache 分类和完整保留的重复样本。精确 job file、参数和操作序列属于冻结 evaluator 数据，不在本页公开。

## 真实 Workload Suite

下一版本另有一套小型 application-level correctness suite。它将 completion、data integrity、reopen behavior 和 error handling 与合成性能分开报告。本仓库不公开精确 application、scenario step、fixture、单个 identity 或 expected output。

## Crash Test

Crash test 是独立的评测路径，不属于合成性能或真实 workload 标签。Evaluator 在冻结的位置中断运行中的文件系统，从产生的持久状态重新启动，并检查它是否按照所选 Variant 完成 mount 或 recovery、保留必需的 durable state，以及保持结构有效。

Crash workload identity、interruption point、persistence selection、seed、trace 和 expected state 仅供 evaluator 使用，不在本页公开。

## 其他维度

其他 profile 测量格式符合性、客观 maintainability metric 和运行后 code review。正式结果把真实 workload scenario、crash test 和合成性能作为独立维度报告，不把它们统一归入“workload”。
