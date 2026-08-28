# Agent Harness

[English](README.md)

冻结的 pilot harness 使用运行于 Node.js 22.23.2 的 Pi Coding Agent 0.84.3。[`manifest.json`](manifest.json) 记录公开运行策略，[`agent-goal.txt`](agent-goal.txt) 包含 initial goal 模板，[`install.sh`](install.sh) 安装固定版本的公开 package。

## 运行协议

- 每个已声明的 model/Variant run 只有一条 continuous one-shot trajectory。
- Operator 只启动一次 harness，并且只提供冻结的 initial goal。
- 不允许人工追加 repair prompt、提供中途技术建议、修改源码或解释测试失败。
- 禁止按模型调整 prompt、role routing、fallback model、subagent、memory、内建 browser/web 和动态 extension discovery。
- 不增加 BenchFS 专用 tool API；agent 只使用 harness 在 VM 内提供的普通文件、shell 和开发工具。
- Agent 在 disposable Dev VM 内拥有正常的本地管理员权限。
- Model 流量经过可审计 endpoint proxy；其他开发流量受冻结 allowlist 限制。
- Agent 完成或达到声明的 run 预算时轨迹结束。Pilot 候选预算是 12 小时；正式 release 声明自己的冻结预算。
- 最终源码只留档一次，之后单独评测，不返回 hidden feedback。

## 复现边界

npm package 和版本公开。Provider credential、endpoint routing、任务材料正文、hidden evaluator 资产和 operator control-plane 配置不进入本仓库。正式结果会记录该 run 使用的精确 harness profile、prompt、tool policy 和 context policy 的 hash。
