# 执行环境

[English](environment.md)

## 冻结的 pilot 环境

| 组件 | 值 |
|---|---|
| Guest OS | Ubuntu Server 24.04.4 LTS，x86-64 |
| Guest kernel | Ubuntu GA 6.8（冻结 pilot 镜像为 `6.8.0-138-generic`） |
| VM 资源 | 8 vCPU、16 GiB static RAM、禁用 swap、禁用 ballooning |
| 数据设备 | 两个独立的 64 GiB disposable device，分别用于 test 和 scratch |
| Rust | 1.97.1 |
| libfuse | 3.18.2 |
| Node.js | 22.23.2 |
| Coding agent | Pi Coding Agent 0.84.3 |

每条正式结果必须给出精确的镜像 digest。软件包版本、firmware、VM 启动配置、公开代码和工具配置由对应 benchmark release 冻结，不能只根据本页摘要推断。

## 隔离

开发和评测使用从冻结镜像创建的、相互独立的 disposable VM。

- Dev VM 只包含 agent 可见材料和公开反馈测试。
- Dev VM 中不存在 hidden evaluator 资产。
- 开发期网络只允许访问 release allowlist 和 model endpoint proxy。
- Eval 在干净 VM 中重新构建留档源码，不向 agent 返回 hidden feedback。
- 计分执行前锁定 Eval 网络。
- 拉取运行记录后销毁 Eval VM 和数据设备。

## 可复现元数据

一条发布结果至少包含：

- guest 镜像和 benchmark commit identity；
- harness、model、toolchain 和 library 版本；
- VM CPU、内存、磁盘角色和 backend 配置；
- 公开代码和 suite manifest identity；
- 开始/结束时间、运行次数、基础设施状态和人工干预状态。

性能结果还必须标识 physical host、CPU isolation、frequency policy、NUMA placement、storage device、host kernel 和 controller 版本。不同硬件池的原始性能数据不得直接合并。
