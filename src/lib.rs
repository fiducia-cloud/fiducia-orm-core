//! # fiducia-orm-core
//!
//! Canonical Fiducia persistence boundary: opaque database contexts, named
//! operations, generated SeaORM and Diesel projections, and dual-engine runtime
//! parity checks. At least 90% of ORM implementation belongs here rather than in
//! request-serving binaries.
//!
//! TypeSpec and JSON Schema are independent peer authorities. Candidate Rust
//! and read-only SQL are emitted only after normalized parity; neither source nor
//! ORM engine may silently win a discrepancy.
//!
//! Web/default consumers receive only [`ReadContext`] and named functions under
//! [`read`]. API consumers explicitly enable `read-write` for [`WriteContext`]
//! and [`write`]. Raw sessions remain private.

#[cfg(not(feature = "read-only"))]
compile_error!("fiducia-orm-core requires the read-only feature; read-write includes it");

mod connection;
#[cfg(feature = "commercial-diesel")]
pub mod dual;
mod error;
#[doc(hidden)]
pub mod generated;
pub mod read;
mod schema;

#[cfg(feature = "read-write")]
pub mod write;

pub use connection::{
    connect_read_only, connect_read_only_with_policy, ConnectPolicy, ReadContext,
};
#[cfg(feature = "read-write")]
pub use connection::{connect_read_write, connect_read_write_with_policy, WriteContext};
#[cfg(feature = "commercial-diesel")]
pub use dual::DualOrmConnectionState;
pub use error::OrmError;
pub use generated::commercial_provenance::{
    COMMERCIAL_CATALOG_SHA256, COMMERCIAL_COLUMN_COUNT, COMMERCIAL_JSON_SCHEMA_GIT_BLOB_SHA1,
    COMMERCIAL_SQL_GIT_BLOB_SHA1, COMMERCIAL_TABLES, COMMERCIAL_TABLE_COUNT,
    COMMERCIAL_TYPESPEC_GIT_BLOB_SHA1,
};
pub use generated::dual_orm_runtime::{
    CONNECTION_STATE_SQL, DUAL_ORM_ENGINES, DUAL_ORM_OPERATIONS,
    DUAL_ORM_RUNTIME_SCHEMA_VERSION, DUAL_ORM_SCHEMA_NAME,
};
pub use schema::{
    COMMERCIAL_SCHEMA, ORG_SCHEMA, SHARED_DEFS_ORG_SLICE, SHARED_DEFS_REVISION,
    SHARED_DEFS_SEA_ORM_ADAPTER,
};

/// Default consumers cannot import write symbols. This doctest is compiled only
/// for the default/read-only surface; all-feature API builds omit it.
#[cfg(not(feature = "read-write"))]
#[doc = r#"
```compile_fail
use fiducia_orm_core::{WriteContext, connect_read_write, write};
```
"#]
pub mod default_surface_compile_fail {}
