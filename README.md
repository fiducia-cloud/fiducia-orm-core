# fiducia-orm-core

Opaque, role-aware database contexts plus generated ORM compatibility projections for the `fiducia-cloud` organization.

Governed by [`fiducia-cloud/.github/SERVICE_AND_DATA_ARCHITECTURE.md`](https://github.com/fiducia-cloud/.github/blob/main/SERVICE_AND_DATA_ARCHITECTURE.md) and tracked by Linear `DEN-3330`.

## Authority and transition

Canonical DDL is **not** owned here. `fiducia-cloud/fiducia-infra` currently owns the reviewed commercial SQL migration, while `fiducia-cloud/fiducia-interfaces` owns the public Draft 2020-12 JSON Schema. TypeSpec owns the complementary transport operations and models.

`DEN-3330` moves the editable persistence/projection authority to a new `fiducia-cloud/fiducia-lib-core` repository. Until that repository can be provisioned, this repository may publish deterministic generated compatibility adapters, but it must not acquire migration authority or hand-maintained schema definitions.

[`commercial-projection.lock.json`](commercial-projection.lock.json) pins:

- the canonical commercial SQL blob;
- the canonical public JSON Schema blob;
- the complementary TypeSpec blob;
- the normalized structural catalog digest;
- every generated output digest.

The normalized catalog includes table names, ordered columns, logical SQL types, nullability, and primary keys. Foreign keys, unique/check constraints, indexes, row-level security, triggers, functions, and grants remain authoritative in canonical SQL and live-catalog verification.

## Generated commercial projections

The checked-in catalog currently projects all 22 `fiducia_commercial` tables and 244 columns for:

- SeaORM entities, enabled explicitly with `commercial-sea-orm`;
- Diesel `table!` declarations and `Queryable`/`Selectable`/`Identifiable` models, enabled explicitly with `commercial-diesel`.

Regenerate and verify:

```sh
python3 tools/generate_commercial_projections.py
python3 tools/generate_commercial_projections.py --check
```

Do not edit files under `generated/` manually. Change the reviewed catalog input, regenerate, and update the pinned upstream source blobs in the same pull request. The generator, checked-in outputs, and projection lock are tested together so included Rust modules cannot drift from the canonical catalog.

These entities are low-level compatibility types. Product code should continue to use named operations under `read` and `write`; it should not pass raw ORM sessions or persistence rows across HTTP/RPC boundaries.

## Runtime contract

| Consumer | Feature | Public surface |
| --- | --- | --- |
| Web/default consumer | `read-only` (default) | `ReadContext`, role-aware connection, and named functions under `read` |
| API server | `read-write` | Adds `WriteContext` and named functions under `write` |
| SeaORM compatibility consumer | `commercial-sea-orm` | Generated SeaORM entities; no migration or connection authority |
| Diesel compatibility consumer | `commercial-diesel` | Generated schema and Diesel models; no migration or connection authority |

ORM projection features are independent from the read/write context features. Enabling a compatibility model must not silently grant write APIs, and enabling `read-write` must not silently select an ORM projection.

Raw SeaORM/SQLx connections, entity managers, query builders, and backend error types stay private. A default consumer cannot import `WriteContext`, `connect_read_write`, or the `write` module; a compile-fail doctest enforces that. This is an intent-and-ergonomics boundary, not a security boundary: Cargo feature resolution is additive. The authoritative control is the SELECT-only database role.

`connect_read_only` pins `search_path=fiducia`, sets `default_transaction_read_only=on` in the PostgreSQL startup packet, and verifies both settings before returning an opaque context. `connect_read_write` is compiled only with `read-write` and rejects a transaction-read-only session.

## Original shared schema source

The original `fiducia` schema slice still comes from [`ORESoftware/k8s-libs-and-shared-defs`](https://github.com/ORESoftware/k8s-libs-and-shared-defs). [`shared-defs.lock.json`](shared-defs.lock.json) pins revision `c8bdc06d74746acc6439f9527ebd02697fdf028b`, organization slice `fiducia-cloud`, schema `fiducia`, and the generated Rust SeaORM adapter path.

The crate targets PostgreSQL and CockroachDB through SeaORM's `sqlx-postgres` backend. Shared code does not make the engines behaviorally identical; retry and transaction behavior must be tested per engine.

## Usage

Default web/read consumer:

```toml
fiducia-orm-core = { git = "https://github.com/fiducia-cloud/fiducia-orm-core.git", rev = "<merge-commit>" }
```

```rust,no_run
use fiducia_orm_core::{connect_read_only, read};

# async fn example() -> Result<(), fiducia_orm_core::OrmError> {
let context = connect_read_only("postgres://fiducia_web_ro@db/fiducia").await?;
read::ping(&context).await?;
# Ok(())
# }
```

API/write consumer:

```toml
fiducia-orm-core = {
  git = "https://github.com/fiducia-cloud/fiducia-orm-core.git",
  rev = "<merge-commit>",
  default-features = false,
  features = ["read-write"]
}
```

SeaORM compatibility consumer:

```toml
fiducia-orm-core = {
  git = "https://github.com/fiducia-cloud/fiducia-orm-core.git",
  rev = "<merge-commit>",
  features = ["commercial-sea-orm"]
}
```

Diesel compatibility consumer:

```toml
fiducia-orm-core = {
  git = "https://github.com/fiducia-cloud/fiducia-orm-core.git",
  rev = "<merge-commit>",
  features = ["commercial-diesel"]
}
```

## Migrations

There is no migration tooling in this crate. A separate `declarative-migrations`/`dpm` release job applies reviewed DDL with the project-scoped migrator identity. Runtime API and web identities do not receive DDL rights.

Specialized services such as `fiducia-node.rs` and `fiducia-brain.rs` remain on the separate Fiducia cluster; this library does not blur that cluster boundary or convey a shared writer credential.

## Validation

```sh
python3 -m py_compile tools/generate_commercial_projections.py
python3 tools/generate_commercial_projections.py --check
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo test --all-targets --all-features
cargo test --doc
```

A live denial probe is included but ignored by default because it performs an intentionally forbidden DDL statement against a disposable database:

```sh
ORM_CORE_TEST_DATABASE_URL='postgres://fiducia_web_ro@localhost/fiducia_test' \
  cargo test live_read_only_context_rejects_schema_ddl -- --ignored
```

Run that lane against both PostgreSQL and CockroachDB with a real SELECT-only web principal before releasing a consumer pin.
