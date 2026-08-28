# BenchFS Feature Overview

[中文](features.zh-CN.md)

This page lists externally visible capability families only. It is not a task specification and intentionally omits behavioral edge cases, required layouts, algorithms, constants, and test mappings.

## Core

| Family | High-level capability |
|---|---|
| Lifecycle | Format, mount, unmount, and remount |
| Namespace | Files, directories, creation, removal, and rename |
| Links | Hard links and symbolic links |
| Handles | File and directory open/close and directory iteration |
| Metadata | Ownership, modes, timestamps, file types, attributes, and filesystem statistics |
| Data I/O | Positioned and sequential reads/writes, truncation, and file growth |
| Sparse storage | Holes, sparse-file navigation, allocation, zeroing, and reclamation |
| Extended attributes | User extended-attribute operations |
| Synchronization | File, data, and directory synchronization |
| Kernel-visible behavior | Buffered I/O, memory mapping, advisory locking, and cache coherence |
| Capacity behavior | Space exhaustion, reuse after deletion, and large-file/large-directory operation |
| Special nodes | Metadata and namespace lifecycle for supported special inode types |
| Persistence | State preservation across clean unmount and remount |

## Journal

Journal includes every Core family and adds crash-consistent recovery and durability behavior for the journaled variant. Public documentation does not disclose the required log organization, transaction protocol, recovery procedure, or crash cases.

## COW

COW includes every Core family and adds snapshots, read-only snapshot views, file cloning/reflink behavior, copy-on-write isolation, and unreachable-space reclamation. Public documentation does not disclose metadata organization, update algorithms, recovery procedure, or evaluator workloads.

## What this overview does not define

Capability names are informational. The authoritative task materials are delivered only inside the isolated agent run and are intentionally excluded from this repository. No implementation can claim conformance from this overview alone; conformance is determined by the frozen evaluator for the named benchmark version.
