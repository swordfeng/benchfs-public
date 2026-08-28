# Benchmark Purpose and Evaluated Objects

[中文](benchmark.zh-CN.md)

## Purpose

BenchFS measures whether an autonomous coding agent can complete a substantial, stateful systems-programming task within a frozen environment and budget. It emphasizes executable outcomes rather than subjective code style.

The benchmark is intended to measure:

- sustained autonomous engineering over a long task;
- correct use of a fixed participant-facing API and runtime boundary;
- externally observable filesystem correctness, robustness, and durability;
- the ability to build, mount, test, debug, and finish without human technical intervention;
- resource efficiency and performance after correctness qualification.

It is not intended to compare programming languages, evaluate Linux kernel-module development, or replace production-filesystem certification.

## Evaluated object

A result applies to one frozen tuple:

```text
model × agent harness × benchmark version × variant × track
```

Changing any tuple member creates a different result. Provider name, exact model identifier, harness commit/version, benchmark commit, environment identity, and run count must accompany every release.

## Variants

- **Core:** baseline filesystem capability set.
- **Journal:** Core plus crash-consistency requirements for journaled operation.
- **COW:** Core plus copy-on-write, snapshot, clone, and reclamation capability families.

Variants are reported separately. A higher variant does not receive credit for omitting Core capabilities.

## Tracks

- **Independent Implementation:** starts from the same non-functional scaffold and clean context.
- **Seeded Extension:** starts from one frozen common Core seed and is reported separately from independent work.

Results from different variants or tracks are not merged.

## Experimental unit

Each released trajectory states its run count. A single trajectory is reported as `n=1`; its case pass rate is an implementation result, not an estimate of the model's independent success probability. Repeated runs, when present, are released individually before any predeclared aggregation is applied.
