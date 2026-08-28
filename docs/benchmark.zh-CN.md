# 测试目的和测试对象

[English](benchmark.md)

## 目的

BenchFS 测量自主编程 agent 能否在冻结的环境和预算内完成大型、有状态的系统编程任务。评价以可执行结果为主，而不是主观代码风格。

本基准用于测量：

- 在长任务中持续自主完成工程工作的能力；
- 正确使用固定的参与者可见 API 和运行时边界；
- 文件系统外部可观察的正确性、鲁棒性和持久性；
- 在没有人工技术干预的情况下完成构建、挂载、测试、调试和交付；
- 通过正确性资格门槛后的资源效率和性能。

本基准不用于比较编程语言，不评价 Linux 内核模块开发，也不替代生产文件系统认证。

## 测试对象

一条结果只适用于一个冻结的组合：

```text
model × agent harness × benchmark version × variant × track
```

其中任何一项变化都会产生不同的结果。每次发布必须附带 provider 名称、精确 model 标识、harness commit/version、benchmark commit、环境 identity 和运行次数。

## Variant

- **Core：** 基础文件系统能力集合。
- **Journal：** Core 加上 journal 运行方式的崩溃一致性要求。
- **COW：** Core 加上 copy-on-write、snapshot、clone 和空间回收能力类别。

不同 Variant 分开报告。高级 Variant 不能通过缺少 Core 能力获得成绩。

## Track

- **Independent Implementation：** 从相同的无功能 scaffold 和干净 context 开始。
- **Seeded Extension：** 从同一个冻结的公共 Core seed 开始，并与独立实现分开报告。

不同 Variant 或 Track 的结果不合并。

## 实验单位

每条发布轨迹必须标明运行次数。单条轨迹标记为 `n=1`；其 case 通过率是该实现的结果，不能解释为模型独立成功概率的估计。存在重复运行时，先逐条发布，再应用预先声明的聚合规则。
