#!/usr/bin/env python3
"""Generate the dual-ORM Rust/SQL runtime only when TypeSpec and JSON Schema agree."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
TYPESPEC_PATH = ROOT / "schema" / "dual-orm-runtime.tsp"
JSON_SCHEMA_PATH = ROOT / "schema" / "dual-orm-runtime.schema.json"
RUST_PATH = ROOT / "generated" / "dual_orm_runtime.rs"
SQL_PATH = ROOT / "generated" / "dual_orm_runtime.sql"
RECEIPT_PATH = ROOT / "generated" / "dual_orm_runtime.receipt.json"

SCHEMA_VERSION = "fiducia.dual-orm-runtime.v1"
EXPECTED_SCHEMA = "fiducia"
EXPECTED_ENGINES = ("sea_orm", "diesel")
EXPECTED_OPERATIONS = (
    ("read_connection_state", "read_only"),
    ("write_connection_state", "read_write"),
)


class GenerationError(RuntimeError):
    """A fail-closed source/parity/generation error."""


@dataclass(frozen=True)
class Contract:
    schema_version: str
    schema_name: str
    engines: tuple[str, ...]
    operations: tuple[tuple[str, str], ...]

    def as_dict(self) -> dict[str, Any]:
        return {
            "schema_version": self.schema_version,
            "schema_name": self.schema_name,
            "engines": list(self.engines),
            "operations": [
                {"operation": operation, "access_mode": access_mode}
                for operation, access_mode in self.operations
            ],
        }


def digest_bytes(value: bytes) -> str:
    return hashlib.sha256(value).hexdigest()


def canonical_json(value: Any) -> bytes:
    return (json.dumps(value, sort_keys=True, separators=(",", ":")) + "\n").encode()


def parse_typespec(source: str) -> Contract:
    if "namespace Fiducia.OrmRuntime;" not in source:
        raise GenerationError("TypeSpec namespace is not Fiducia.OrmRuntime")
    if not re.search(
        r"model\s+ConnectionState\s*\{[^}]*schemaName:\s*string;[^}]*transactionReadOnly:\s*boolean;",
        source,
        re.S,
    ):
        raise GenerationError("TypeSpec ConnectionState fields are missing or reordered")

    engine_block = re.search(r"enum\s+OrmEngine\s*\{(?P<body>[^}]*)\}", source, re.S)
    access_block = re.search(r"enum\s+AccessMode\s*\{(?P<body>[^}]*)\}", source, re.S)
    if engine_block is None or access_block is None:
        raise GenerationError("TypeSpec engine/access enums are missing")
    engines = tuple(re.findall(r':\s*"([a-z_]+)"\s*,', engine_block.group("body")))
    access_modes = tuple(re.findall(r':\s*"([a-z_]+)"\s*,', access_block.group("body")))
    if engines != EXPECTED_ENGINES:
        raise GenerationError(f"TypeSpec engines differ: {engines!r}")
    if access_modes != tuple(value for _, value in EXPECTED_OPERATIONS):
        raise GenerationError(f"TypeSpec access modes differ: {access_modes!r}")
    if not re.search(
        r"model\s+DualOrmOperation\s*\{[^}]*operation:\s*string;[^}]*accessMode:\s*AccessMode;[^}]*engines:\s*OrmEngine\[\];[^}]*schemaName:\s*string;",
        source,
        re.S,
    ):
        raise GenerationError("TypeSpec DualOrmOperation fields are missing or reordered")
    return Contract(SCHEMA_VERSION, EXPECTED_SCHEMA, engines, EXPECTED_OPERATIONS)


def require_object(value: Any, field: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise GenerationError(f"{field} must be an object")
    return value


def parse_json_schema(value: Any) -> Contract:
    root = require_object(value, "JSON Schema")
    if root.get("$schema") != "https://json-schema.org/draft/2020-12/schema":
        raise GenerationError("JSON Schema must use Draft 2020-12")
    if root.get("additionalProperties") is not False:
        raise GenerationError("JSON Schema root must be closed")
    props = require_object(root.get("properties"), "JSON Schema properties")
    version = require_object(props.get("schemaVersion"), "schemaVersion").get("const")
    schema_name = require_object(props.get("schemaName"), "schemaName").get("const")
    engine_items = require_object(props.get("engines"), "engines").get("prefixItems")
    operation_items = require_object(props.get("operations"), "operations").get("prefixItems")
    if not isinstance(engine_items, list) or not isinstance(operation_items, list):
        raise GenerationError("JSON Schema peer arrays must use prefixItems")
    engines = tuple(require_object(item, "engine item").get("const") for item in engine_items)
    operations: list[tuple[str, str]] = []
    for item in operation_items:
        item_props = require_object(
            require_object(item, "operation item").get("properties"),
            "operation properties",
        )
        operations.append(
            (
                require_object(item_props.get("operation"), "operation").get("const"),
                require_object(item_props.get("accessMode"), "accessMode").get("const"),
            )
        )
    contract = Contract(str(version), str(schema_name), engines, tuple(operations))
    expected = Contract(
        SCHEMA_VERSION, EXPECTED_SCHEMA, EXPECTED_ENGINES, EXPECTED_OPERATIONS
    )
    if contract != expected:
        raise GenerationError(f"JSON Schema contract differs: {contract.as_dict()!r}")
    return contract


def render_rust(contract: Contract) -> bytes:
    operations = "\n".join(
        f'    ("{operation}", "{access_mode}"),'
        for operation, access_mode in contract.operations
    )
    engines = ", ".join(f'"{engine}"' for engine in contract.engines)
    sql = (
        "SELECT current_schema()::text AS schema_name, "
        "current_setting('default_transaction_read_only')::text AS transaction_read_only"
    )
    return (
        "// AUTOGENERATED by tools/generate_dual_orm_runtime.py; do not edit.\n"
        f'pub const DUAL_ORM_RUNTIME_SCHEMA_VERSION: &str = "{contract.schema_version}";\n'
        f'pub const DUAL_ORM_SCHEMA_NAME: &str = "{contract.schema_name}";\n'
        f"pub const DUAL_ORM_ENGINES: &[&str] = &[{engines}];\n"
        "pub const DUAL_ORM_OPERATIONS: &[(&str, &str)] = &[\n"
        f"{operations}\n"
        "];\n"
        f'pub const CONNECTION_STATE_SQL: &str = "{sql}";\n'
    ).encode()


def render_sql(contract: Contract) -> bytes:
    return (
        "-- AUTOGENERATED by tools/generate_dual_orm_runtime.py; do not edit.\n"
        f"-- schema-version: {contract.schema_version}\n"
        "-- peer-authorities: schema/dual-orm-runtime.tsp + schema/dual-orm-runtime.schema.json\n"
        "-- This probe is read-only and shared by SeaORM and Diesel adapters.\n"
        "SELECT current_schema()::text AS schema_name,\n"
        "       current_setting('default_transaction_read_only')::text AS transaction_read_only;\n"
    ).encode()


def expected_outputs() -> dict[Path, bytes]:
    typespec_bytes = TYPESPEC_PATH.read_bytes()
    json_schema_bytes = JSON_SCHEMA_PATH.read_bytes()
    typespec_contract = parse_typespec(typespec_bytes.decode())
    json_contract = parse_json_schema(json.loads(json_schema_bytes))
    if typespec_contract != json_contract:
        raise GenerationError(
            "STOPPED_FOR_EVALUATION: TypeSpec and JSON Schema normalized contracts differ"
        )
    rust = render_rust(typespec_contract)
    sql = render_sql(typespec_contract)
    receipt = {
        "schema_version": "fiducia.dual-orm-generation-receipt.v1",
        "status": "PASS",
        "normalized_contract": typespec_contract.as_dict(),
        "inputs": {
            "typespec": {
                "path": str(TYPESPEC_PATH.relative_to(ROOT)),
                "sha256": digest_bytes(typespec_bytes),
            },
            "json_schema": {
                "path": str(JSON_SCHEMA_PATH.relative_to(ROOT)),
                "sha256": digest_bytes(json_schema_bytes),
            },
        },
        "outputs": {
            "rust": {
                "path": str(RUST_PATH.relative_to(ROOT)),
                "sha256": digest_bytes(rust),
            },
            "sql": {
                "path": str(SQL_PATH.relative_to(ROOT)),
                "sha256": digest_bytes(sql),
            },
        },
        "invariants": {
            "type_spec_and_json_schema_are_independent_peers": True,
            "both_orm_engines_required": True,
            "no_raw_connection_export": True,
            "generated_sql_is_read_only": True,
        },
    }
    return {RUST_PATH: rust, SQL_PATH: sql, RECEIPT_PATH: canonical_json(receipt)}


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    try:
        outputs = expected_outputs()
        mismatches: list[str] = []
        for path, expected in outputs.items():
            if args.check:
                actual = path.read_bytes() if path.exists() else b""
                if actual != expected:
                    mismatches.append(str(path.relative_to(ROOT)))
            else:
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_bytes(expected)
        if mismatches:
            raise GenerationError("generated output drift: " + ", ".join(mismatches))
        print(
            json.dumps(
                {
                    "status": "PASS",
                    "checked": args.check,
                    "outputs": [str(path.relative_to(ROOT)) for path in outputs],
                },
                sort_keys=True,
            )
        )
        return 0
    except (OSError, UnicodeError, json.JSONDecodeError, GenerationError) as exc:
        print(f"error: {exc}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
