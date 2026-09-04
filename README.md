# fiducia-orm-core

Canonical opaque SeaORM and Diesel boundary for the `fiducia-cloud` organization.
It owns generated ORM projections, connection adapters, parity checks, and named
persistence operations. Request-serving repositories should keep at least 90%
of ORM-specific implementation here and consume only the capability they need.

Governed by `ORESoftware/my-ai/AGENTS.md`, the Fiducia organization policy, and
Linear `DEN-3330` / `DEN-2789`.

## Authority model

TypeSpec and JSON Schema are independent peer authorities. Neither is generated
from or subordinate to the other. The existing commercial projection generator
pins the reviewed SQL, TypeSpec, and Draft 2020-12 JSON Schema sources. The new
runtime generator independently reads:

- `schema/dual-orm-runtime.tsp`;
- `schema/dual-orm-runtime.schema.json`.

It normalizes both sources and stops for evaluation when they differ. Only a
matching pair may emit:

- `generated/dual_orm_runtime.rs`;
- `generated/dual_orm_runtime.sql`;
- `generated/dual_orm_runtime.receipt.json`.

The generated SQL is a read-only connection-policy probe used by both SeaORM and
Diesel. Canonical DDL and production migrations remain outside this crate and are
applied only by a reviewed, fenced one-shot migrator.

## Dual ORM runtime

SeaORM owns the asynchronous pooled context. Diesel independently opens a
libpq-backed connection in `spawn_blocking`. Both execute the same generated SQL
and their redacted policy states must match exactly. Raw sessions and ORM errors
never leave this crate.

Web/default consumers use a read-only database principal and:

```rust,no_run
# async fn example(url: &str) -> Result<(), fiducia_orm_core::OrmError> {
let evidence = fiducia_orm_core::dual::verify_read_only(url).await?;
assert!(evidence.transaction_read_only());
# Ok(())
# }
```

API consumers enable `read-write` and use:

```rust,no_run
# async fn example(url: &str) -> Result<(), fiducia_orm_core::OrmError> {
let evidence = fiducia_orm_core::dual::verify_read_write(url).await?;
assert!(!evidence.transaction_read_only());
# Ok(())
# }
```

Recommended Cargo/Zed feature profiles:

| Consumer | Features | Database authority |
| --- | --- | --- |
| Web / admin web | `commercial-dual-orm` | SELECT-only role, read-only transaction policy |
| API / admin API | `read-write`, `commercial-dual-orm` | Explicit product writer role |
| Migrator | not a request-serving consumer | Fenced one-shot DPM/declarative SQL job |

Cargo features express intent; database grants, workload identity, separate
credentials, and NetworkPolicy are the security boundary.

## Generated commercial projections

The checked-in catalog projects all 22 `fiducia_commercial` tables and 244
columns into independently generated SeaORM and Diesel compatibility types.
Neither ORM generates or certifies the other. Product code should add named,
tenant-scoped operations under `read` or `write`, not import raw query builders.

Regenerate and verify:

```sh
python3 tools/generate_commercial_projections.py
python3 tools/generate_dual_orm_runtime.py
python3 tools/generate_commercial_projections.py --check
python3 tools/generate_dual_orm_runtime.py --check
```

## Validation

```sh
python3 -m unittest -v tests/test_dual_orm_generation.py
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo test --all-targets --all-features
cargo test --doc
```

Live database lanes must use disposable PostgreSQL and CockroachDB databases with
distinct web, API, and migrator principals. No runtime service may apply DDL at
startup or fall back from a read-only credential to a writer credential.
