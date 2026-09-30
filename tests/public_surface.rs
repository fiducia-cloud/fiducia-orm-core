use std::{fs, path::PathBuf};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read(relative: &str) -> String {
    fs::read_to_string(root().join(relative))
        .unwrap_or_else(|error| panic!("failed to read {relative}: {error}"))
}

#[test]
fn raw_orm_types_are_not_reexported() {
    let library = read("src/lib.rs");
    for forbidden in [
        "pub use sea_orm",
        "pub type ReadContext = DatabaseConnection",
        "pub type WriteContext = DatabaseConnection",
    ] {
        assert!(
            !library.contains(forbidden),
            "public surface leaked {forbidden}"
        );
    }

    let connection = read("src/connection.rs");
    assert!(connection.contains("connection: DatabaseConnection"));
    assert!(connection.contains("pub(crate) fn connection"));
    assert!(!connection.contains("pub fn connection(&self)"));
}

#[test]
fn every_write_symbol_is_feature_gated() {
    let library = read("src/lib.rs");
    assert!(library.contains("#[cfg(feature = \"read-write\")]\npub mod write;"));
    assert!(library.contains("#[cfg(feature = \"read-write\")]\npub use connection"));
    assert!(library.contains("compile_fail"));

    let cargo = read("Cargo.toml");
    assert!(cargo.contains("default = [\"read-only\"]"));
    assert!(cargo.contains("read-write = [\"read-only\"]"));
}

#[test]
fn product_local_contract_home_is_explicit() {
    let zpkg = read(".zpkg.toml");
    for dependency in [
        "\"fiducia-cloud/fiducia-interfaces\"",
        "\"fiducia-cloud/fiducia-lib-core\"",
    ] {
        assert!(
            zpkg.contains(dependency),
            "product-local contract dependency missing {dependency}"
        );
    }
    assert!(
        !zpkg.contains("oresoftware/k8s-libs-and-shared-defs"),
        "centralized shared-defs must not remain ORM authority"
    );

    let source_lock = read("contracts/source-lock.toml");
    assert!(source_lock.contains("interfaces_repository = \"fiducia-cloud/fiducia-interfaces\""));
    assert!(source_lock.contains("format = \"ores.core-source-lock/v1\""));
}

#[test]
fn live_denial_probe_remains_available_but_opt_in() {
    let connection = read("src/connection.rs");
    assert!(connection
        .contains("#[ignore = \"requires a dedicated ORM_CORE_TEST_DATABASE_URL database\"]"));
    assert!(connection.contains("live_read_only_context_rejects_schema_ddl"));
    assert!(connection.contains("read-only context unexpectedly executed DDL"));
}
