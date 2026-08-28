# Agent VM Content Catalog

[中文](README.zh-CN.md)

[`contents.json`](contents.json) is the machine-readable catalog of material made available to the coding agent at trajectory start.

## Workspace summary

| VM path | Role | Public release |
|---|---|---|
| `solution/docs/` | Run-scoped task material | Cataloged only; content withheld |
| `solution/benchfs-sdk/` | Fixed participant-facing Rust API | Snapshot under [`../crates/benchfs-sdk/`](../crates/benchfs-sdk/) |
| `solution/benchfs-fuse/` | Fixed participant-facing FUSE adapter | Snapshot under [`../crates/benchfs-fuse/`](../crates/benchfs-fuse/) |
| `solution/bench/` | Writable NullFS starting scaffold | Source snapshot under [`../crates/benchfs-nullfs/`](../crates/benchfs-nullfs/) |
| `solution/Cargo.toml` and lock/config files | Frozen Rust workspace | Represented by the public sample workspace |
| `solution/tools/` | Frozen direct-dependency policy | Identity recorded by the benchmark release |
| `/opt/benchfs/` public feedback assets | Dev-visible tests and helpers | Aggregate description only |

Task-material contents are deliberately absent from the public repository. Publishing them would risk putting benchmark answers into future model-training data. The isolated VM receives the frozen task material for that run; the public catalog proves what classes of material were present without reproducing implementation requirements.

## Access boundary

The agent can modify only its implementation workspace and ordinary VM state. Fixed participant code and task material are staged read-only for the run. Hidden tests, hidden manifests, evaluator seeds, formal recovery workloads, evaluator oracles, credentials, other candidates, and reference implementations are not present.

## Equality across candidates

Within one benchmark version, every compared candidate receives byte-identical fixed code, task material, dependency policy, public feedback assets, base image, initial writable scaffold, harness policy, and initial goal. Per-run identities and hashes are retained in the private audit record; safe release identities accompany formal results.
