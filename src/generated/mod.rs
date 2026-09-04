//! Generated compatibility projections and immutable source provenance.
//!
//! Generated files live outside `src/` and are included here so `cargo fmt` does
//! not rewrite generator output. Regenerate with the checked-in tools and always
//! run both generators in `--check` mode before review.

pub mod commercial_provenance {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/generated/commercial_provenance.rs"
    ));
}

pub mod dual_orm_runtime {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/generated/dual_orm_runtime.rs"
    ));
}

#[cfg(feature = "commercial-sea-orm")]
pub mod commercial_sea_orm {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/generated/commercial_sea_orm.rs"
    ));
}

#[cfg(feature = "commercial-diesel")]
pub mod commercial_diesel {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/generated/commercial_diesel.rs"
    ));
}
