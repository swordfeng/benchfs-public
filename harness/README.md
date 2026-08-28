# Agent Harness

[中文](README.zh-CN.md)

The frozen pilot harness uses Pi Coding Agent 0.84.3 on Node.js 22.23.2. [`manifest.json`](manifest.json) records the public execution policy, [`agent-goal.txt`](agent-goal.txt) contains the initial goal template, and [`install.sh`](install.sh) installs the pinned public package.

## Execution protocol

- One continuous, one-shot trajectory per declared model/variant run.
- The operator starts the harness once and supplies only the frozen initial goal.
- No repair prompt, intermediate technical advice, source modification, or test-failure interpretation by a human.
- No model-specific prompt tuning, role routing, fallback model, subagent, memory, built-in browser/web access, or dynamic extension discovery.
- No BenchFS-specific tool API. The agent uses the harness's ordinary file, shell, and development tools inside the VM.
- The agent has normal local administrative access inside its disposable Dev VM.
- Model traffic passes through the audited endpoint proxy; other development traffic is limited by the frozen allowlist.
- The trajectory ends on agent completion or the declared run budget. The pilot candidate budget is 12 hours; formal releases state their own frozen budget.
- The final source is archived once, then evaluated separately without returning hidden feedback.

## Reproduction boundary

The npm package and version are public. Provider credentials, endpoint routing, task-material contents, hidden evaluator assets, and operator control-plane configuration are not included. A formal result records hashes for the exact harness profile, prompt, tool policy, and context policy used in that run.
