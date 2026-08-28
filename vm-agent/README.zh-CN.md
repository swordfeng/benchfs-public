# Agent VM 内容目录

[English](README.md)

[`contents.json`](contents.json) 是 coding agent 在轨迹开始时可获得材料的机器可读目录。

## Workspace 摘要

| VM 路径 | 作用 | 公开方式 |
|---|---|---|
| `solution/docs/` | Run-scoped 任务材料 | 只列目录，不公开内容 |
| `solution/benchfs-sdk/` | 固定的参与者可见 Rust API | 快照位于 [`../crates/benchfs-sdk/`](../crates/benchfs-sdk/) |
| `solution/benchfs-fuse/` | 固定的参与者可见 FUSE adapter | 快照位于 [`../crates/benchfs-fuse/`](../crates/benchfs-fuse/) |
| `solution/bench/` | 可写的 NullFS 起始 scaffold | 源码快照位于 [`../crates/benchfs-nullfs/`](../crates/benchfs-nullfs/) |
| `solution/Cargo.toml` 和 lock/config 文件 | 冻结的 Rust workspace | 由公开 sample workspace 表示 |
| `solution/tools/` | 冻结的 direct-dependency policy | identity 由 benchmark release 记录 |
| `/opt/benchfs/` 公开反馈资产 | Dev-visible test 和 helper | 只公开汇总说明 |

任务材料的正文刻意不进入公开仓库。公开这些内容会增加 benchmark 答案进入未来模型训练数据的风险。隔离 VM 会获得该 run 冻结的任务材料；公开目录只证明材料类别，不复制实现要求。

## 访问边界

Agent 只能修改自己的实现 workspace 和普通 VM 状态。固定参与者代码和任务材料在 run 中以只读方式 staging。Hidden test、hidden manifest、evaluator seed、正式恢复 workload、evaluator oracle、credential、其他候选实现和 reference implementation 均不存在于 Dev VM。

## 候选之间的一致性

同一 benchmark 版本内，每个候选获得 byte-identical 的固定代码、任务材料、dependency policy、公开反馈资产、base image、初始可写 scaffold、harness policy 和 initial goal。每个 run 的 identity 和 hash 保留在私有审计记录中；可安全公开的 release identity 随正式结果发布。
