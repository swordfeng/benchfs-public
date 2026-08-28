# Execution Environment

[中文](environment.zh-CN.md)

## Frozen pilot environment

| Component | Value |
|---|---|
| Guest OS | Ubuntu Server 24.04.4 LTS, x86-64 |
| Guest kernel | Ubuntu GA 6.8 (`6.8.0-138-generic` for the frozen pilot image) |
| VM resources | 8 vCPU, 16 GiB static RAM, swap disabled, ballooning disabled |
| Data devices | Two independent 64 GiB disposable devices for test and scratch roles |
| Rust | 1.97.1 |
| libfuse | 3.18.2 |
| Node.js | 22.23.2 |
| Coding agent | Pi Coding Agent 0.84.3 |

Every formal result must name the exact image digest. Package versions, firmware, VM launch configuration, public code, and tool configuration are frozen by the corresponding benchmark release rather than inferred from this summary.

## Isolation

Development and evaluation use separate disposable VMs created from the frozen image.

- The development VM contains only agent-visible materials and public feedback tests.
- Hidden evaluator assets are absent from the development VM.
- Development network access is restricted to the release allowlist and the model endpoint proxy.
- Evaluation rebuilds the archived source in a clean VM and does not return hidden feedback to the agent.
- Evaluation network access is locked before scored execution.
- VM and data devices are destroyed after evaluation; artifacts are pulled into the run record first.

## Reproducibility metadata

A released result includes, at minimum:

- guest image and benchmark commit identities;
- harness, model, toolchain, and library versions;
- VM CPU, memory, disk-role, and backend configuration;
- public-code and suite-manifest identities;
- start/end time, run count, infrastructure status, and human-intervention status.

Performance results additionally identify the physical host, CPU isolation, frequency policy, NUMA placement, storage device, host kernel, and controller version. Raw performance from different hardware pools is not directly combined.
