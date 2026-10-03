/**
 * Independent TypeSpec authority for the opaque Fiducia dual-ORM runtime.
 * This file is never generated from JSON Schema. Both authorities must agree
 * before Rust or SQL output is emitted.
 */
namespace Fiducia.OrmRuntime;

enum OrmEngine {
  seaOrm: "sea_orm",
  diesel: "diesel",
}

enum AccessMode {
  readOnly: "read_only",
  readWrite: "read_write",
}

model ConnectionState {
  schemaName: string;
  transactionReadOnly: boolean;
}

model DualOrmOperation {
  operation: string;
  accessMode: AccessMode;
  engines: OrmEngine[];
  schemaName: string;
}
