# 已发布结果

[English](README.md)

本仓库目前还没有发布正式 BenchFS 模型结果。

当前项目活动属于发布前 pilot 验证。Pilot run 用于验证难度、隔离、evaluator 完整性、防泄漏能力和多次运行行为。它们永久标记为 pilot 数据，不进入正式排名。Infrastructure-invalid 的 pilot 尝试不属于候选成绩。

正式结果发布后，本目录将包含不可变、带版本的记录，至少包括：

- 精确的 model、provider、harness、benchmark、公开代码和环境 identity；
- Variant、Track、运行次数、预算、时间和人工干预状态；
- build/mount 状态和获准公开的汇总评分维度；
- 适用的 correctness、robustness、conformance、真实 workload scenario、crash test 和具备资格的合成 performance 汇总；
- 与正确性分数分离的效率和审计元数据；
- 足以验证记录的 manifest 和 artifact hash；
- 明确的有效性和性能资格分类。

公开记录不会包含 hidden case 标识、hidden test 正文、evaluator oracle、seed、trace、实现特定的详细失败信息或候选源码。
