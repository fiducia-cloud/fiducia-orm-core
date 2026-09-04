//! Opaque runtime parity checks that execute the same generated probe through
//! SeaORM and Diesel. No raw connection, query builder, or ORM error escapes
//! this crate.

use diesel::{
    connection::SimpleConnection, Connection, PgConnection, QueryableByName, RunQueryDsl,
};
use tokio::task;

use crate::{
    connection::InternalConnectionState,
    generated::dual_orm_runtime::{CONNECTION_STATE_SQL, DUAL_ORM_ENGINES},
    read::{self, ConnectionState},
    OrmError, ReadContext, ORG_SCHEMA,
};

#[cfg(feature = "read-write")]
use crate::{write, WriteContext};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AccessMode {
    ReadOnly,
    #[cfg(feature = "read-write")]
    ReadWrite,
}

#[derive(QueryableByName)]
struct DieselConnectionState {
    #[diesel(sql_type = diesel::sql_types::Text)]
    schema_name: String,
    #[diesel(sql_type = diesel::sql_types::Text)]
    transaction_read_only: String,
}

/// Redacted, implementation-independent evidence that both ORM engines reached
/// the same database policy state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DualOrmConnectionState {
    sea_orm: ConnectionState,
    diesel: ConnectionState,
}

impl DualOrmConnectionState {
    /// The generated contract always requires SeaORM and Diesel, in that order.
    #[must_use]
    pub const fn engines() -> &'static [&'static str] {
        DUAL_ORM_ENGINES
    }

    #[must_use]
    pub fn sea_orm(&self) -> &ConnectionState {
        &self.sea_orm
    }

    #[must_use]
    pub fn diesel(&self) -> &ConnectionState {
        &self.diesel
    }

    #[must_use]
    pub fn schema(&self) -> &str {
        self.sea_orm.schema()
    }

    #[must_use]
    pub fn transaction_read_only(&self) -> bool {
        self.sea_orm.transaction_read_only()
    }
}

/// Execute the generated connection-state query through SeaORM and Diesel
/// using independently opened, read-only sessions.
pub async fn verify_read_only(database_url: &str) -> Result<DualOrmConnectionState, OrmError> {
    let sea_context = crate::connect_read_only(database_url).await?;
    verify_read_context(&sea_context, database_url).await
}

/// Reuse an already verified SeaORM read context and independently verify the
/// same policy through Diesel.
pub async fn verify_read_context(
    sea_context: &ReadContext,
    database_url: &str,
) -> Result<DualOrmConnectionState, OrmError> {
    let sea_state = read::connection_state(sea_context).await?;
    let diesel_state = diesel_state_async(database_url, AccessMode::ReadOnly).await?;
    reconcile_states(sea_state, diesel_state, true)
}

/// Execute the generated connection-state query through SeaORM and Diesel
/// using independently opened read/write sessions. API consumers only.
#[cfg(feature = "read-write")]
pub async fn verify_read_write(database_url: &str) -> Result<DualOrmConnectionState, OrmError> {
    let sea_context = crate::connect_read_write(database_url).await?;
    verify_write_context(&sea_context, database_url).await
}

/// Reuse an already verified SeaORM write context and independently verify the
/// same policy through Diesel. API consumers only.
#[cfg(feature = "read-write")]
pub async fn verify_write_context(
    sea_context: &WriteContext,
    database_url: &str,
) -> Result<DualOrmConnectionState, OrmError> {
    let sea_state = write::connection_state(sea_context).await?;
    let diesel_state = diesel_state_async(database_url, AccessMode::ReadWrite).await?;
    reconcile_states(sea_state, diesel_state, false)
}

async fn diesel_state_async(
    database_url: &str,
    access_mode: AccessMode,
) -> Result<ConnectionState, OrmError> {
    let database_url = database_url.to_owned();
    task::spawn_blocking(move || diesel_state(&database_url, access_mode))
        .await
        .map_err(OrmError::database)?
}

fn diesel_state(database_url: &str, access_mode: AccessMode) -> Result<ConnectionState, OrmError> {
    let mut connection = PgConnection::establish(database_url).map_err(OrmError::database)?;
    match access_mode {
        AccessMode::ReadOnly => connection
            .batch_execute(&format!(
                "SET search_path TO {ORG_SCHEMA}; SET default_transaction_read_only TO on"
            ))
            .map_err(OrmError::database)?,
        #[cfg(feature = "read-write")]
        AccessMode::ReadWrite => connection
            .batch_execute(&format!("SET search_path TO {ORG_SCHEMA}"))
            .map_err(OrmError::database)?,
    }

    let row = diesel::sql_query(CONNECTION_STATE_SQL)
        .get_result::<DieselConnectionState>(&mut connection)
        .map_err(OrmError::database)?;
    Ok(ConnectionState::from_internal(InternalConnectionState {
        schema: row.schema_name,
        transaction_read_only: row.transaction_read_only == "on",
    }))
}

fn reconcile_states(
    sea_orm: ConnectionState,
    diesel: ConnectionState,
    expected_read_only: bool,
) -> Result<DualOrmConnectionState, OrmError> {
    if sea_orm != diesel {
        return Err(OrmError::policy(format!(
            "SeaORM and Diesel connection policy disagree: sea_orm={sea_orm:?}, diesel={diesel:?}"
        )));
    }
    if sea_orm.schema() != ORG_SCHEMA {
        return Err(OrmError::policy(format!(
            "dual ORM probe resolved schema {:?}; expected {ORG_SCHEMA:?}",
            sea_orm.schema()
        )));
    }
    if sea_orm.transaction_read_only() != expected_read_only {
        return Err(OrmError::policy(
            "dual ORM probe returned an unexpected transaction policy",
        ));
    }
    Ok(DualOrmConnectionState { sea_orm, diesel })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_contract_requires_both_engines() {
        assert_eq!(DualOrmConnectionState::engines(), &["sea_orm", "diesel"]);
        assert!(CONNECTION_STATE_SQL.starts_with("SELECT current_schema()"));
        assert!(!CONNECTION_STATE_SQL.to_ascii_lowercase().contains("insert"));
        assert!(!CONNECTION_STATE_SQL.to_ascii_lowercase().contains("update"));
        assert!(!CONNECTION_STATE_SQL.to_ascii_lowercase().contains("delete"));
    }

    #[test]
    fn parity_rejects_engine_disagreement() {
        let sea = ConnectionState::from_internal(InternalConnectionState {
            schema: ORG_SCHEMA.to_owned(),
            transaction_read_only: true,
        });
        let diesel = ConnectionState::from_internal(InternalConnectionState {
            schema: ORG_SCHEMA.to_owned(),
            transaction_read_only: false,
        });
        assert!(reconcile_states(sea, diesel, true).is_err());
    }
}
