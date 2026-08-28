# BenchFS Feature 概览

[English](features.md)

本页只列出外部可观察的能力类别。它不是任务规格，并有意省略行为边界、必需布局、算法、常量和测试映射。

## Core

| 类别 | 高层能力 |
|---|---|
| 生命周期 | format、mount、unmount 和 remount |
| Namespace | 文件、目录、创建、删除和 rename |
| 链接 | hard link 和 symbolic link |
| Handle | 文件和目录的 open/close，以及目录遍历 |
| Metadata | owner、mode、timestamp、文件类型、attribute 和文件系统统计 |
| 数据 I/O | 定位和顺序读写、truncate 和文件增长 |
| 稀疏存储 | hole、稀疏文件导航、allocation、zeroing 和回收 |
| Extended attribute | user extended-attribute 操作 |
| 同步 | 文件、数据和目录同步 |
| Kernel 可见行为 | buffered I/O、memory mapping、advisory locking 和 cache coherence |
| 容量行为 | 空间耗尽、删除后复用，以及大文件和大目录操作 |
| Special node | 支持的特殊 inode 类型的 metadata 和 namespace 生命周期 |
| 持久性 | clean unmount/remount 后保留状态 |

## Journal

Journal 包含全部 Core 类别，并增加 journal Variant 的崩溃一致恢复和持久性行为。公开文档不披露必需的 log 组织、transaction protocol、恢复过程或 crash case。

## COW

COW 包含全部 Core 类别，并增加 snapshot、只读 snapshot view、文件 clone/reflink、copy-on-write isolation 和不可达空间回收。公开文档不披露 metadata 组织、更新算法、恢复过程或 evaluator workload。

## 本页不定义的内容

能力名称仅用于说明。权威任务材料只在隔离的 agent run 内提供，并有意不进入本仓库。任何实现都不能只根据本概览声称符合要求；符合性由指定 benchmark 版本的冻结 evaluator 判定。
