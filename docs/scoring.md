# Testing and Scoring Method

[中文](scoring.zh-CN.md)

## Evaluation flow

1. Freeze the benchmark, public-code, harness, environment, suite-manifest, and scoring-policy identities.
2. Run one autonomous development trajectory in an isolated Dev VM.
3. Archive the final source once; development binaries and local test results are non-authoritative.
4. Rebuild the archive in a fresh Eval VM with the frozen dependency policy.
5. Exercise build, format, mount, correctness, robustness, variant, conformance, and qualified performance profiles.
6. Validate report completeness and infrastructure state before publishing aggregates.

Human technical intervention after generation starts is not allowed. A recovery from genuine infrastructure failure follows a predeclared rule and is recorded.

## Case outcomes

Each test case receives one terminal outcome:

- **pass** — required observation succeeded;
- **fail** — required observation failed;
- **skip N/A** — frozen profile classification says the case is not applicable;
- **skip implementation** — the implementation did not provide a required capability; counted as failure;
- **infrastructure error** — the evaluator could not produce a valid verdict.

N/A cases are excluded from scoring denominators. An infrastructure error invalidates the affected run rather than becoming a candidate failure.

## Correctness aggregates

For a valid profile:

$$
\text{raw pass rate} = \frac{\text{pass}}{\text{pass} + \text{fail} + \text{skip implementation}}
$$

The report also publishes an equal-weight macro average across frozen semantic categories. Raw pass rate and macro average remain separate; one does not hide weakness in the other. Mandatory-manifest 100% is required for the “fully conforming” label.

## Published dimensions

BenchFS publishes multiple dimensions rather than collapsing everything into one score:

- build, format, mount, and lifecycle status;
- Core semantic correctness;
- POSIX conformance;
- robustness and safety;
- format conformance;
- Journal/COW crash consistency, when applicable;
- real-world scenario completion, integrity, reopen behavior, and error handling;
- synthetic performance, resource use, and I/O amplification;
- post-run code-review findings;
- objective maintainability metrics.

Agent wall time, provider tokens, cost, tool calls, and human-intervention events are audit metadata, not correctness points.

## Synthetic performance measurement

Performance is reported as two independent profiles: memory-backed software overhead and dedicated persistent-storage end-to-end behavior. Timed workload families cover bulk sequential I/O, small random I/O, buffered synchronized writes, and metadata/concurrency activity. Integrity verification occurs outside the timed window.

Every timed workload uses a warm-up, a fixed measurement window, cold/warm cache labels, and repeated samples. Reports retain every sample plus median, median absolute deviation, and coefficient of variation. Outputs include throughput, IOPS, latency percentiles, candidate CPU, candidate peak memory, BlockDevice read/write/flush counts, space use, and applicable lifecycle timing.

Candidate RAM is measured from the filesystem daemon's isolated resource group; the workload generator and evaluator are excluded. Write amplification uses BlockDevice bytes written divided by application logical bytes written, with both values retained. Metadata workloads report absolute storage traffic per operation instead of a misleading ratio.

## Real-world workload assessment

The separate real-world suite is scored from application-level completion, integrity, reopen behavior, and error handling. Its result is not inferred from, or merged into, synthetic throughput and latency measurements. Exact applications and scenario oracles remain evaluator-only.

## Crash-test method

Crash evaluation runs after the agent trajectory. It interrupts frozen workloads without a clean shutdown, restarts from the resulting persistent state, and evaluates mount/recovery, required durable state, and structural validity under the selected Variant's policy. Infrastructure failure is distinct from a candidate crash-test failure. Exact crash workloads, interruption points, persistence selections, seeds, traces, and expected states remain evaluator-only.

## Performance qualification

Performance remains visible for every valid run, but enters formal comparison only after the frozen correctness and safety gates are met. The current policy requires complete build/mount/smoke and critical safety success, complete applicable durability-fence success, at least 95% POSIX semantic-category macro average, at least 80% in every major category, and at least 95% non-critical seeded robustness/recovery success.

Results from different variants, tracks, benchmark versions, or non-equivalent hardware pools are not merged.

## Result validity

A published result states whether it is formal, pilot, non-eligible, or infrastructure-invalid. Pilot data never enters the formal leaderboard. Reports include hashes for the frozen inputs and preserve machine-readable aggregates and audit artifacts; public narrative documents do not expose individual hidden cases or implementation-specific failure details.
