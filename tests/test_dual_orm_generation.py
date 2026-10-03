from __future__ import annotations

import importlib.util
import json
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location(
    "generate_dual_orm_runtime", ROOT / "tools" / "generate_dual_orm_runtime.py"
)
assert SPEC is not None and SPEC.loader is not None
module = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = module
SPEC.loader.exec_module(module)


class DualOrmGenerationTests(unittest.TestCase):
    def test_checked_in_peer_contracts_normalize_identically(self) -> None:
        typespec = module.parse_typespec(module.TYPESPEC_PATH.read_text())
        schema = module.parse_json_schema(json.loads(module.JSON_SCHEMA_PATH.read_text()))
        self.assertEqual(typespec, schema)
        self.assertEqual(typespec.engines, ("sea_orm", "diesel"))

    def test_typespec_engine_drift_stops_generation(self) -> None:
        source = module.TYPESPEC_PATH.read_text().replace('diesel: "diesel",', '')
        with self.assertRaises(module.GenerationError):
            module.parse_typespec(source)

    def test_json_schema_operation_drift_stops_generation(self) -> None:
        value = json.loads(module.JSON_SCHEMA_PATH.read_text())
        value["properties"]["operations"]["prefixItems"][1]["properties"]["operation"][
            "const"
        ] = "unsafe_write"
        with self.assertRaises(module.GenerationError):
            module.parse_json_schema(value)

    def test_generated_sql_is_read_only(self) -> None:
        sql = module.render_sql(
            module.parse_typespec(module.TYPESPEC_PATH.read_text())
        ).decode()
        lowered = sql.lower()
        self.assertIn("select current_schema()", lowered)
        for mutation in ("insert ", "update ", "delete ", "alter ", "drop ", "create "):
            self.assertNotIn(mutation, lowered)


if __name__ == "__main__":
    unittest.main()
