# BenchFS 公开发布仓库

[English](README.md)

BenchFS 是一个评估 **AI 编程 agent 实现用户态文件系统能力**的基准。Agent 根据统一的任务规格、SDK 和 FUSE adapter，在隔离的开发虚拟机中编写实现；随后将实现的冻结副本放到全新的评测虚拟机中重新构建和测试。评测关注实际运行表现：文件系统正确性、稳健性、崩溃一致性、真实应用和性能。

本仓库公开基准介绍、参与者可见代码快照、测试套件汇总、评分规则和模型评测结果。它是基准的公开资料仓库，不是可用于生产的文件系统实现。

**从这里开始：**[模型成绩与报告](results/README.zh-CN.md) · [基准目的](docs/benchmark.zh-CN.md) · [评分方法](docs/scoring.zh-CN.md)

## 公开边界

本仓库有意**不公开**：

- 任务规格或存储格式文档；
- 实现指导、算法、架构或内部设计理由；
- 单个测试标识、测试正文、预期输出或 evaluator oracle；
- hidden manifest、seed、trace 或可能暴露评测 case 的失败细节；
- 候选实现。

该边界用于降低基准答案进入模型训练语料、进而使后续测量失效的风险。`crates/` 只包含固定的参与者可见 SDK、FUSE adapter 和刻意不具备功能的 NullFS scaffold。

## 目录

| 路径 | 公开内容 |
|---|---|
| [`docs/benchmark.zh-CN.md`](docs/benchmark.zh-CN.md) | 测试目的和测试对象 |
| [`docs/environment.zh-CN.md`](docs/environment.zh-CN.md) | 冻结的执行环境 |
| [`docs/features.zh-CN.md`](docs/features.zh-CN.md) | 高层、外部可观察的 feature 类别 |
| [`docs/test-suite.zh-CN.md`](docs/test-suite.zh-CN.md) | 测试套件和可见性汇总 |
| [`docs/scoring.zh-CN.md`](docs/scoring.zh-CN.md) | 测试与评分方法 |
| [`vm-agent/README.zh-CN.md`](vm-agent/README.zh-CN.md) | agent VM 中材料的目录 |
| [`harness/README.zh-CN.md`](harness/README.zh-CN.md) | harness 标识和运行协议 |
| [`crates/`](crates/) | SDK、FUSE adapter 和 NullFS scaffold 快照 |
| [`results/README.zh-CN.md`](results/README.zh-CN.md) | 模型成绩和详细评测报告 |

## 语言策略

默认语言为英文。每个 Markdown 文档都有中文对应版本，并在两个文件顶部互相链接。

## 版本管理

每次发布必须标识 benchmark 版本、harness 版本、环境 identity、公开代码 commit、suite manifest hash 和评分策略版本。任何冻结输入发生变化都必须产生新的 benchmark 版本；已经发布的结果不得在变更后的规则下被改写。
