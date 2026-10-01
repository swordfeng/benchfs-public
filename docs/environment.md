# Experimental environment

[中文](environment.zh-CN.md) · [Back to overview](../README.md)

Models develop in Linux VMs. Submitted source is rebuilt in separate evaluation VMs, so files and configuration left behind during development are not treated as part of the implementation.

## Development configuration

| Component | Configuration |
|---|---|
| OS | Ubuntu Server 24.04.4 LTS, x86-64 |
| Kernel | Ubuntu GA 6.8; base image version `6.8.0-138-generic` |
| VM resources | 8 vCPU, 16 GiB fixed RAM; swap and ballooning disabled |
| Data devices | Separate 64 GiB test and scratch devices |
| Rust | 1.97.1 |
| libfuse | 3.18.2 |
| Node.js | 22.23.2 |
| Pi Coding Agent | 0.84.3 |
| Claude Code | 2.1.283; labeled separately in results |

Each model / agent combination runs once, at High effort. The development budget is up to 12 hours. Active time and elapsed time are recorded separately; individual reports provide details.

## Tools and human involvement

Agents use file operations, shell access, and local development tools to build, test, and debug. Subagents, MCP, memory, and built-in web tools are disabled. Claude Code retains Bash, Read, Edit, and Write, and uses a different task-completion mechanism from Pi.

The operator provides the initial task and manages the environment, without editing candidate code or giving technical guidance. Development network access is limited to required dependency sources and model services. Individual reports retain run-specific settings and recovery records.

## Independent evaluation

Development VMs contain task materials and feedback tests; the full evaluation material is used only during testing. Evaluation rebuilds the submitted source and locks network access before executing tests. Its feedback does not enter development.

Tests run against actual participant formatting and filesystem binaries. Resources, data devices, and virtualization backends are retained per test run; the development configuration is not a substitute for those settings. Existing reports identify Hyper-V or KVM separately.
