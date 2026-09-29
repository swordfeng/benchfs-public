# BenchFS Public Release

[中文](README.zh-CN.md)

BenchFS is a benchmark for **AI coding agents implementing a userspace filesystem**. Agents work from a fixed task specification, SDK and FUSE adapter in isolated development VMs. A frozen copy of each implementation is then rebuilt and tested in a fresh evaluation VM. The benchmark measures executable filesystem behavior: correctness, robustness, crash consistency, real-world applications and performance.

This repository publishes the benchmark overview, participant-facing code snapshots, aggregate test-suite information, scoring rules and model results. It is the public companion to the benchmark, not a production filesystem implementation.

**Start here:** [Model scores and reports](results/README.md) · [Benchmark purpose](docs/benchmark.md) · [Scoring](docs/scoring.md)

## Publication boundary

This repository intentionally does **not** publish:

- task specifications or storage-format documents;
- implementation guidance, algorithms, architecture, or internal design rationale;
- individual test identifiers, test bodies, expected outputs, or evaluator oracles;
- hidden manifests, seeds, traces, or failure details that could reveal evaluation cases;
- candidate implementations.

This boundary reduces the risk that benchmark answers enter model-training corpora and invalidate later measurements. The code under `crates/` is limited to the fixed participant-facing SDK, FUSE adapter, and intentionally non-functional NullFS scaffold.

## Contents

| Path | Public content |
|---|---|
| [`docs/benchmark.md`](docs/benchmark.md) | Purpose and evaluated objects |
| [`docs/environment.md`](docs/environment.md) | Frozen execution environment |
| [`docs/features.md`](docs/features.md) | High-level, externally visible feature families |
| [`docs/test-suite.md`](docs/test-suite.md) | Aggregate suite and visibility overview |
| [`docs/scoring.md`](docs/scoring.md) | Testing and scoring method |
| [`vm-agent/README.md`](vm-agent/README.md) | Catalog of material present in the agent VM |
| [`harness/README.md`](harness/README.md) | Harness identity and execution protocol |
| [`crates/`](crates/) | SDK, FUSE adapter, and NullFS scaffold snapshots |
| [`results/README.md`](results/README.md) | Model scores and detailed evaluation reports |

## Language policy

English is the default. Every Markdown document has a Chinese counterpart linked at the top of both files.

## Versioning

A release must identify the benchmark version, harness version, environment identity, public-code commit, suite manifest hashes, and scoring-policy version. Changes to any frozen input require a new benchmark version; existing results are never rewritten under changed rules.
