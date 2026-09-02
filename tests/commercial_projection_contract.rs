use std::collections::BTreeSet;

use fiducia_orm_core::{
    COMMERCIAL_CATALOG_SHA256, COMMERCIAL_COLUMN_COUNT, COMMERCIAL_JSON_SCHEMA_GIT_BLOB_SHA1,
    COMMERCIAL_SCHEMA, COMMERCIAL_SQL_GIT_BLOB_SHA1, COMMERCIAL_TABLES, COMMERCIAL_TABLE_COUNT,
    COMMERCIAL_TYPESPEC_GIT_BLOB_SHA1,
};

fn is_lower_hex(value: &str) -> bool {
    value
        .bytes()
        .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

#[test]
fn commercial_projection_manifest_is_complete_and_unique() {
    assert_eq!(COMMERCIAL_SCHEMA, "fiducia_commercial");
    assert_eq!(COMMERCIAL_TABLE_COUNT, 22);
    assert_eq!(COMMERCIAL_COLUMN_COUNT, 244);
    assert_eq!(COMMERCIAL_TABLES.len(), COMMERCIAL_TABLE_COUNT);

    let unique: BTreeSet<_> = COMMERCIAL_TABLES.iter().copied().collect();
    assert_eq!(unique.len(), COMMERCIAL_TABLE_COUNT);
}

#[test]
fn commercial_projection_source_digests_are_well_formed() {
    assert_eq!(COMMERCIAL_CATALOG_SHA256.len(), 64);
    assert!(is_lower_hex(COMMERCIAL_CATALOG_SHA256));

    for git_blob in [
        COMMERCIAL_SQL_GIT_BLOB_SHA1,
        COMMERCIAL_JSON_SCHEMA_GIT_BLOB_SHA1,
        COMMERCIAL_TYPESPEC_GIT_BLOB_SHA1,
    ] {
        assert_eq!(git_blob.len(), 40);
        assert!(is_lower_hex(git_blob));
    }
}

#[cfg(feature = "commercial-sea-orm")]
#[test]
fn every_sea_orm_model_is_reachable() {
    use std::any::TypeId;

    let _ = TypeId::of::<fiducia_orm_core::generated::commercial_sea_orm::organizations::Model>();
    let _ = TypeId::of::<fiducia_orm_core::generated::commercial_sea_orm::contacts::Model>();
    let _ = TypeId::of::<
        fiducia_orm_core::generated::commercial_sea_orm::organization_contact_roles::Model,
    >();
    let _ = TypeId::of::<
        fiducia_orm_core::generated::commercial_sea_orm::pre_interest_registrations::Model,
    >();
    let _ = TypeId::of::<fiducia_orm_core::generated::commercial_sea_orm::applications::Model>();
    let _ = TypeId::of::<
        fiducia_orm_core::generated::commercial_sea_orm::application_versions::Model,
    >();
    let _ = TypeId::of::<fiducia_orm_core::generated::commercial_sea_orm::attachments::Model>();
    let _ = TypeId::of::<fiducia_orm_core::generated::commercial_sea_orm::support_plans::Model>();
    let _ = TypeId::of::<
        fiducia_orm_core::generated::commercial_sea_orm::support_plan_versions::Model,
    >();
    let _ = TypeId::of::<fiducia_orm_core::generated::commercial_sea_orm::sla_policies::Model>();
    let _ =
        TypeId::of::<fiducia_orm_core::generated::commercial_sea_orm::sla_policy_versions::Model>();
    let _ =
        TypeId::of::<fiducia_orm_core::generated::commercial_sea_orm::contract_templates::Model>();
    let _ = TypeId::of::<
        fiducia_orm_core::generated::commercial_sea_orm::contract_template_versions::Model,
    >();
    let _ = TypeId::of::<fiducia_orm_core::generated::commercial_sea_orm::quotes::Model>();
    let _ = TypeId::of::<fiducia_orm_core::generated::commercial_sea_orm::quote_versions::Model>();
    let _ =
        TypeId::of::<fiducia_orm_core::generated::commercial_sea_orm::quote_line_items::Model>();
    let _ = TypeId::of::<
        fiducia_orm_core::generated::commercial_sea_orm::quote_contract_references::Model,
    >();
    let _ = TypeId::of::<
        fiducia_orm_core::generated::commercial_sea_orm::contract_acceptances::Model,
    >();
    let _ = TypeId::of::<
        fiducia_orm_core::generated::commercial_sea_orm::workflow_transition_rules::Model,
    >();
    let _ = TypeId::of::<fiducia_orm_core::generated::commercial_sea_orm::workflow_events::Model>();
    let _ =
        TypeId::of::<fiducia_orm_core::generated::commercial_sea_orm::idempotency_records::Model>();
    let _ = TypeId::of::<fiducia_orm_core::generated::commercial_sea_orm::outbox_events::Model>();
}

#[cfg(feature = "commercial-diesel")]
#[test]
fn every_diesel_model_is_reachable() {
    use std::any::TypeId;

    let _ = TypeId::of::<fiducia_orm_core::generated::commercial_diesel::models::Organization>();
    let _ = TypeId::of::<fiducia_orm_core::generated::commercial_diesel::models::Contact>();
    let _ = TypeId::of::<
        fiducia_orm_core::generated::commercial_diesel::models::OrganizationContactRole,
    >();
    let _ = TypeId::of::<
        fiducia_orm_core::generated::commercial_diesel::models::PreInterestRegistration,
    >();
    let _ = TypeId::of::<fiducia_orm_core::generated::commercial_diesel::models::Application>();
    let _ =
        TypeId::of::<fiducia_orm_core::generated::commercial_diesel::models::ApplicationVersion>();
    let _ = TypeId::of::<fiducia_orm_core::generated::commercial_diesel::models::Attachment>();
    let _ = TypeId::of::<fiducia_orm_core::generated::commercial_diesel::models::SupportPlan>();
    let _ =
        TypeId::of::<fiducia_orm_core::generated::commercial_diesel::models::SupportPlanVersion>();
    let _ = TypeId::of::<fiducia_orm_core::generated::commercial_diesel::models::SlaPolicy>();
    let _ =
        TypeId::of::<fiducia_orm_core::generated::commercial_diesel::models::SlaPolicyVersion>();
    let _ =
        TypeId::of::<fiducia_orm_core::generated::commercial_diesel::models::ContractTemplate>();
    let _ = TypeId::of::<
        fiducia_orm_core::generated::commercial_diesel::models::ContractTemplateVersion,
    >();
    let _ = TypeId::of::<fiducia_orm_core::generated::commercial_diesel::models::Quote>();
    let _ = TypeId::of::<fiducia_orm_core::generated::commercial_diesel::models::QuoteVersion>();
    let _ = TypeId::of::<fiducia_orm_core::generated::commercial_diesel::models::QuoteLineItem>();
    let _ = TypeId::of::<
        fiducia_orm_core::generated::commercial_diesel::models::QuoteContractReference,
    >();
    let _ =
        TypeId::of::<fiducia_orm_core::generated::commercial_diesel::models::ContractAcceptance>();
    let _ = TypeId::of::<
        fiducia_orm_core::generated::commercial_diesel::models::WorkflowTransitionRule,
    >();
    let _ = TypeId::of::<fiducia_orm_core::generated::commercial_diesel::models::WorkflowEvent>();
    let _ =
        TypeId::of::<fiducia_orm_core::generated::commercial_diesel::models::IdempotencyRecord>();
    let _ = TypeId::of::<fiducia_orm_core::generated::commercial_diesel::models::OutboxEvent>();
}
