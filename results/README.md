# Released Results

[中文](README.zh-CN.md)

No formal BenchFS model result has been released in this repository yet.

The current project activity is pre-release pilot validation. Pilot runs are used to validate difficulty, isolation, evaluator integrity, leakage resistance, and run-to-run behavior. They are permanently labeled as pilot data and are excluded from formal rankings. Infrastructure-invalid pilot attempts are not candidate scores.

When a formal result is released, this directory will contain an immutable, versioned record with:

- exact model, provider, harness, benchmark, public-code, and environment identities;
- variant, track, run count, budget, timestamps, and intervention status;
- build/mount status and approved aggregate score dimensions;
- correctness, robustness, conformance, real-world scenarios, crash testing, and qualified synthetic-performance aggregates as applicable;
- efficiency and audit metadata kept separate from correctness scores;
- manifest and artifact hashes sufficient to verify the record;
- an explicit validity and performance-eligibility classification.

Public records will not contain hidden case identifiers, hidden test bodies, evaluator oracles, seeds, traces, detailed implementation failures, or candidate source code.
