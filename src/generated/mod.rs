//! Generated compatibility projections and their immutable source provenance.
//!
//! Generated files live outside `src/` and are included here so `cargo fmt` does
//! not rewrite generator output. Regenerate them with
//! `python3 tools/generate_commercial_projections.py`.

pub mod commercial_provenance {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/generated/commercial_provenance.rs"
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
