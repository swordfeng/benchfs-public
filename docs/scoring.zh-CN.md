# 测试与评分方法

[English](scoring.md)

## 评测流程

1. 冻结 benchmark、公开代码、harness、环境、suite manifest 和评分策略 identity。
2. 在隔离的 Dev VM 中运行一次自主开发轨迹。
3. 只留档一次最终源码；开发期 binary 和本地测试结果不具备权威性。
4. 在全新 Eval VM 中按冻结 dependency policy 重新构建留档。
5. 执行 build、format、mount、correctness、robustness、Variant、conformance 和具备资格后的 performance profile。
6. 验证报告完整性和基础设施状态，再发布汇总结果。

Generation 开始后不允许人工技术干预。真正的基础设施故障只能按预先声明的规则恢复，并且必须记录。

## Case outcome

每个测试 case 只能得到一个最终 outcome：

- **pass**：必需观测成功；
- **fail**：必需观测失败；
- **skip N/A**：冻结的 profile 分类认定该 case 不适用；
- **skip implementation**：实现缺少必需能力，按失败计；
- **infrastructure error**：evaluator 无法给出有效 verdict。

N/A case 不进入评分分母。基础设施错误会使受影响 run 无效，而不是转化为候选实现失败。

## 正确性汇总

对有效 profile：

$$
\text{raw pass rate} = \frac{\text{pass}}{\text{pass} + \text{fail} + \text{skip implementation}}
$$

报告还发布冻结 semantic category 的等权 macro average。Raw pass rate 和 macro average 分开保留，不能互相掩盖弱项。只有 mandatory manifest 达到 100% 才能标记为“fully conforming”。

## 发布维度

BenchFS 发布多个维度，不把所有结果压缩为单一分数：

- build、format、mount 和 lifecycle 状态；
- Core semantic correctness；
- POSIX conformance；
- robustness 和 safety；
- format conformance；
- 适用时的 Journal/COW crash consistency；
- 真实 workload scenario 的 completion、integrity、reopen behavior 和 error handling；
- 合成 performance、resource use 和 I/O amplification；
- 运行后 code-review finding；
- 客观 maintainability metric。

Agent wall time、provider token、cost、tool call 和 human-intervention event 是审计元数据，不是正确性分数。

## 合成性能测量

性能分为两个独立 profile：memory-backed software overhead 和 dedicated persistent-storage end-to-end behavior。Timed workload 类别覆盖大块顺序 I/O、小块随机 I/O、buffered synchronized write，以及 metadata/concurrency activity。Integrity verification 位于 timed window 之外。

每个 timed workload 都使用 warm-up、固定 measurement window、cold/warm cache label 和重复样本。报告保留全部样本以及 median、median absolute deviation 和 coefficient of variation。输出包括 throughput、IOPS、latency percentile、candidate CPU、candidate peak memory、BlockDevice read/write/flush count、space use 和适用的 lifecycle timing。

Candidate RAM 从 filesystem daemon 的隔离 resource group 测量，不包含 workload generator 和 evaluator。写放大使用 BlockDevice written bytes 除以 application logical written bytes，并保留两个原始值。Metadata workload 报告每 operation 的绝对 storage traffic，不发布误导性的 ratio。

## 真实 Workload 评估

独立的真实 workload suite 根据 application-level completion、integrity、reopen behavior 和 error handling 评分。它的结果不从合成 throughput/latency 测量中推断，也不与其合并。精确 application 和 scenario oracle 仅供 evaluator 使用。

## Crash Test 方法

Crash evaluation 在 agent 轨迹结束后运行。它在不 clean shutdown 的情况下中断冻结 workload，从产生的持久状态重新启动，并按照所选 Variant 的策略评估 mount/recovery、必需的 durable state 和结构有效性。基础设施故障与候选实现的 crash-test failure 分开。精确 crash workload、interruption point、persistence selection、seed、trace 和 expected state 仅供 evaluator 使用。

## 性能资格

每个有效 run 的性能结果都保留，但只有满足冻结的正确性和安全门槛后才能进入正式比较。当前策略要求 build/mount/smoke 和 critical safety 全部成功，适用的 durability-fence 全部成功，POSIX semantic-category macro average 至少 95%，每个主要 category 至少 80%，非关键 seeded robustness/recovery 至少 95%。

不同 Variant、Track、benchmark version 或不等价硬件池的结果不合并。

## 结果有效性

发布结果必须标记为 formal、pilot、non-eligible 或 infrastructure-invalid。Pilot 数据永不进入正式 leaderboard。报告包含冻结输入的 hash，并保留机器可读汇总和审计 artifact；公开叙述文档不暴露单个 hidden case 或实现特定的详细失败信息。
