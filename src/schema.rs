/// PostgreSQL/CockroachDB schema owned by the Fiducia Cloud service boundary.
pub const ORG_SCHEMA: &str = "fiducia";

/// Organization slice consumed from the canonical shared-definitions repo.
pub const SHARED_DEFS_ORG_SLICE: &str = "fiducia-cloud";

/// Exact reviewed shared-definitions revision for generated entity input.
pub const SHARED_DEFS_REVISION: &str =
    "c8bdc06d74746acc6439f9527ebd02697fdf028b";

/// Generated adapter location within the shared-definitions repository.
pub const SHARED_DEFS_SEA_ORM_ADAPTER: &str =
    "pg-defs/generated/rust/sea-orm";
