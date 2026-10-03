# Commercial projection validation

The generated SeaORM and Diesel modules in this repository are compatibility projections for the reviewed `fiducia_commercial` catalog. They do not own migrations or restate PostgreSQL row-level security, triggers, functions, grants, indexes, foreign keys, or check constraints.

A projection change is reviewable only when the same commit preserves all of these checks:

```sh
python3 -m py_compile tools/generate_commercial_projections.py
python3 tools/generate_commercial_projections.py --check
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo test --all-targets --all-features
cargo test --doc
```

The lock file pins the canonical SQL, Draft 2020-12 JSON Schema, TypeSpec source, normalized catalog digest, and generated output digests. A changed upstream blob requires a reviewed catalog update and regeneration; a hand edit under `generated/` is invalid.

Live database verification remains a separate release gate. It must compare the projected table/column/type/nullability/primary-key subset against a disposable PostgreSQL and CockroachDB catalog while independently testing the canonical SQL constraints and tenant isolation.
