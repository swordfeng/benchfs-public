# Scoring

[中文](scoring.zh-CN.md) · [Back to results](../README.md#results)

BenchFS measures executable behavior, confirmed code-review defects, and maintainability. The overview shows the combined score and its individual dimensions.

## Pass rates and profile scores

```text
pass rate = (pass + expected fail) / applicable tests
applicable tests = pass + expected fail + fail + unexpected pass + timeout + missing required functionality
```

N/A tests are excluded. Timeouts and missing required functionality count as failures. When a test requires an operation to be rejected, an expected failure counts as meeting that requirement.

Semantic tests, pjdfstest, xfstests, robustness, crash consistency, and applications use an **equal-weight category average**: calculate each category's pass rate, average the categories with applicable tests, and multiply by 100. Categories with more tests do not receive more weight simply because of their size.

**The format profile uses its native conformance score.** Its check pass rate is supplementary and does not replace that score. Build and SDK smoke checks are unscored.

## Overall score

The current rule is `core-overall-v3-zero-fill`, revision `2026-10-04-r2`. Each dimension is scored on its own justified scale, then combined directly:

```text
total = 0.35 × C + 0.20 × R + 0.20 × S + 0.10 × W + 0.10 × P + 0.05 × M
```

| Dimension | Calculation | Overall weight | Reason for internal budgets |
|---|---|---:|---|
| Correctness C | 30% semantic + 30% pjdfstest + 15% xfstests + 25% format | 35% | Semantics retain 75%; overlapping Linux regressions receive a smaller share than contract and broad POSIX tests |
| Robustness R | 40% robustness + 60% observed crash capabilities | 20% | Persisted-state commitments receive priority; contract-valid safe rejection still passes |
| Code review S | Confirmed, deduplicated defect burden curve below | 20% | Severity reflects harm and reach; no second utility transform |
| Applications W | 25% version control + 50% database + 25% archive | 10% | Concurrent database content and crash/reopen each consume half of the database budget |
| Performance P | Equal CPU/NVMe shares; each is 80% speed + 20% RAM | 10% | Both device scenarios and resource costs retain fixed budgets |
| Maintainability M | Eleven static proxies grouped into six risk budgets | 5% | Related indicators share a budget; syntactic diagnostics have a limited share |

The former shared utility function U is removed. The profile tables continue to show their native results; weighted dimensions can differ from their equal-category profile scores. Scores are retrospective measurements of existing evidence. Agent-facing instructions, generated implementations, finalized reviews and raw benchmark outcomes are unchanged.

Missing inputs contribute zero only when calculating provisional totals; tables show “—” to distinguish them from measured zeroes. Missing shares retain their original weight, and partially measured dimensions are marked with `*`. Computation retains source precision and ordering uses unrounded totals. Diagnostic evidence keeps its original eligibility status; a provisional total is not a formal release result.

[New and historical scores](../results/overall-score-v3.json) retain the v2 totals and dimension inputs. [Frozen numerical calibration](../results/scoring-calibration-v3.json) supplies every weight and anchor. These parameters are engineering and policy judgements, not a statistically fitted optimum. No cohort z-score, variance equalization or distribution scaling is applied; adding another submission cannot change an existing score.

## Crash capability scale in R

The crash share uses a fixed linear budget: **40% structural safety, 40% content and durability, 20% clean lifecycle**. The first two protect persisted data; the last measures valid clean publication and any observed fresh reopening. The 80/20 split distinguishes data harm from lifecycle usability. These are explicit policy judgements, declared before recalculation, rather than weights fitted to model rankings. The 40/60 robustness/crash split and all main-axis weights remain fixed.

Each case receives one budget per capability. Independent offline checks prove structure, online intent/fence oracles prove content and durability, and lifecycle results plus independently checked clean state prove clean lifecycle. An observed failure in either lifecycle overrides a pass for that capability; differing content across reopening fails content. Repeated observations never add budget. G0–G8 retain equal weight, with 24 cases per category.

An accepted, unchanged safe rejection fulfills the original contract and receives full compliance credit. It is counted separately and proves no recovered content. A dirty marker after successful unmount still fails clean lifecycle; proved structure and content retain their own shares. Structural corruption can fail both structure and valid-clean publication. This mapping applies uniformly to every submission and does not count or forgive root causes.

A domain with no proof receives zero in its original share, with an unproved count and a `*` on R. A successor blocked after an earlier failure remains unobserved; it neither adds a pass nor cancels an existing independent proof. The score measures **observed contract capabilities**, with coverage reported separately, and does not establish two-lifecycle reliability for blocked cases. Raw case failures, category macro averages and end-to-end recovery/safe-rejection counts remain unchanged.

The README and model reports show the new components alongside raw outcomes and blocked reopening counts. [The r1 scores](../results/overall-score-v3-r1.json), [r1 parameters](../results/scoring-calibration-v3-r1.json) and [r1 audit](../results/scoring-audit-v3-r1.json) remain frozen for comparison. No benchmarks or reviews were rerun.

## Performance scale

The full-score production target for each workload and cache state is the **fixed geometric mean of retained ext4, XFS and Btrfs rates**. Both device scenarios use this common target. This measures progress toward a production capability target, rather than claiming a paired same-device speed experiment: references used one 120-second sample, while candidates use the retained protocol-v2 adaptive samples.

```text
reference = geometric_mean(ext4_rate, xfs_rate, btrfs_rate)
speed_score = 100 × clamp(log(candidate_rate / floor) / log(reference / floor), 0, 1)
```

Equal multiplicative improvements receive equal increments. Reaching the production target scores 100; faster rates are retained but scores cap at 100. The three-filesystem portfolio is fixed rather than selecting a favorable reference for each candidate. A portfolio target of 100 does not mean every member scores 100 in every cell.

Speed budgets are sequential I/O 30%, random I/O 40% (single/multi/shared: 20/10/10%), and synchronized overwrite 30%; cold and warm share each budget equally. Metadata measurements remain separate diagnostics. Minimum rates are declared deployment targets, independent of the candidate distribution:

| Workload | CPU floor | NVMe floor |
|---|---:|---:|
| Sequential | 100 MiB/s | 50 MiB/s |
| Random single | 1000 IOPS | 500 IOPS |
| Random multi | 2000 IOPS | 1000 IOPS |
| Random shared | 1000 IOPS | 500 IOPS |
| Synchronized overwrite | 50 ops/s | 20 ops/s |

These floors retain the published operating envelope: sequential floors budget roughly 10.24/20.48 seconds per GiB of transfer; parallel random has twice the single-request processing target; synchronized floors budget roughly 20/50 ms per completed operation in the stream. They are policy targets, not test deadlines or independently measured user requirements. Synchronized operations/s is not a standalone fsync latency measurement.

RAM uses an absolute, piecewise linear budget: 512 MiB or less scores 100, 2 GiB scores 50, 8 GiB or more scores 0. These represent roughly 1.56%, 6.25% and 25% of the 32 GiB guest. The maximum retained candidate cgroup peak includes initialization and resident charges; it is not compared to kernel-filesystem memory. A failed speed cell scores zero; an unmeasured cell remains missing. RAM earns no bonus unless the full fio load was observed.

## Maintainability scale

`maint-v3-policy` reuses the frozen `maint-v2-rev5` raw extraction and thresholds. It changes the aggregation: local complexity 30%, architecture 10%, duplication 15%, unsafe proof obligations 15%, error paths 20%, and diagnostic tooling 10%. Documentation of unsafe receives more weight than its mere occurrence; error-handling proxies receive more than syntactic lint counts. These are maintenance-risk proxies, not confirmed defects.

| Proxy | Weight | Raw anchors for 100 / 60 / 0 points |
|---|---:|---|
| Function SLOC p95 | 10% | 40 / 100 / 250 |
| Branching p95 | 15% | 8 / 15 / 30 |
| Nesting p95 | 5% | 3 / 5 / 8 |
| File SLOC p95 | 5% | 600 / 1500 / 3000 |
| Dependency cycle share | 5% | .25 / .50 / .80 |
| Duplication % | 15% | 3 / 10 / 25 |
| Unsafe / KLOC | 5% | 2 / 10 / 30 |
| Undocumented unsafe / KLOC | 10% | 1 / 5 / 15 |
| Error risk / KLOC | 20% | 2 / 10 / 30 |
| Lint / KLOC | 5% | 1 / 5 / 15 |
| Suppressions / KLOC | 5% | .5 / 2 / 6 |

Each component interpolates linearly between anchors and saturates outside them. Size and branching represent increasing local reasoning scope; dependency cycles represent propagation; repetition represents coordinated repair sites; unsafe and error paths represent proof obligations; diagnostics have a small budget because they are noisy and correlated. Threshold locations are explicit engineering warning bands and have not been validated against independent maintenance tasks.

## Agent code review

Agents independently review each implementation, consolidate duplicate root causes, and discuss findings and severity before confirming the result. Only confirmed, deduplicated defects contribute to S. Rejected or unproven allegations are excluded.

```text
burden = 40 × critical + 15 × high + 5 × medium + low
S = 10000 / (100 + burden)
```

This is `agent-review-score-v2`: burden 0 gives 100, burden 100 gives 50, and burden 500 gives 16.67. Additional defects always lower the score, with diminishing deductions. Severity weights are policy equivalents: five low roots equal one medium, three medium equal one high, and eight medium equal one critical; they are not measured risk probabilities. Scale 100 sets twenty medium-equivalent roots at half score. A complete review with no confirmed defects scores 100; it is not proof of correctness. An incomplete or unperformed review has no S score, not a zero-defect result.

The README and model reports publish completion status, severity counts and diagnostic scores. [Aggregate JSON](../results/agent-review.json) also includes opaque evidence hashes. Detailed findings and agent conversations remain private.

## Calibration sensitivity

Current submissions are an audit cohort, not an independent validation set. The [aggregate audit](../results/scoring-audit-v3.json) retains per-axis standard deviations, covariance, saturation, performance noise and parameter sensitivity. No distribution scaling is justified by the available repeatability evidence. Changing the review scale from 100 to 200 makes Opus lead in the sensitivity scenario; the first-place ranking is therefore conditional on the stated risk policy. Median ± MAD perturbations are descriptive checks, not confidence intervals.

## Development statistics

Time, token usage, cost, and source size are reported separately. They do not affect scores or break ties. Each entry represents one development run, rather than an average across runs, and identifies both the model and its coding agent.
