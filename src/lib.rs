//! # fiducia-orm-core
//!
//! Opaque database contexts plus generated compatibility projections for the
//! `fiducia-cloud` organization.
//!
//! Canonical DDL remains outside this crate. The commercial projections are
//! reproducibly generated from a normalized catalog pinned to reviewed SQL,
//! JSON Schema Draft 2020-12, and TypeSpec source blobs. DEN-3330 moves editable
//! persistence authority to `fiducia-lib-core`; until that repository is
//! provisioned, this crate is a transitional compatibility package only.
//!
//! Web/default consumers receive only [`ReadContext`] and named functions under
//! [`read`]. API consumers must explicitly enable the `read-write` feature to
//! compile [`WriteContext`] and [`write`]. Raw sessions remain private.

#[cfg(not(feature = "read-only"))]
compile_error!("fiducia-orm-core requires the read-only feature; read-write includes it");

mod connection;
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
pub use error::OrmError;
pub use generated::commercial_provenance::{
    COMMERCIAL_CATALOG_SHA256, COMMERCIAL_COLUMN_COUNT, COMMERCIAL_JSON_SCHEMA_GIT_BLOB_SHA1,
    COMMERCIAL_SQL_GIT_BLOB_SHA1, COMMERCIAL_TABLES, COMMERCIAL_TABLE_COUNT,
    COMMERCIAL_TYPESPEC_GIT_BLOB_SHA1,
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
