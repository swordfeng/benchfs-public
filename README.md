# BenchFS Public Release

[中文](README.zh-CN.md)

BenchFS evaluates how well an autonomous coding agent can complete a substantial systems-programming task under a fixed, reproducible protocol. This repository is the public release surface for benchmark metadata, participant-facing code snapshots, aggregate suite information, scoring rules, and released results.

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
| [`results/README.md`](results/README.md) | Released-result status |

## Language policy

English is the default. Every Markdown document has a Chinese counterpart linked at the top of both files.

## Versioning

A release must identify the benchmark version, harness version, environment identity, public-code commit, suite manifest hashes, and scoring-policy version. Changes to any frozen input require a new benchmark version; existing results are never rewritten under changed rules.
