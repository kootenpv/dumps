#!/usr/bin/env python3
import dumps
from datetime import datetime

# Example data structures
data = {
    "name": "John Doe",
    "age": 30,
    "active": True,
    "balance": 1234.56,
    "tags": ["python", "rust", "json"],
    "metadata": {
        "created": "2024-01-01",
        "modified": "2024-01-10"
    },
    "tuple_data": (1, 2, 3),
    "set_data": {1, 2, 3, 2, 1},  # Will be [1, 2, 3] in JSON
    "none_value": None,
    "bytes_data": b"Hello, bytes!",
    "utf8_bytes": "Hello, 世界".encode('utf-8'),
    "timestamp": datetime(2025, 7, 10, 11, 51, 57, 960798)
}

# Serialize to JSON with base64 encoding for bytes
json_str = dumps.json(data, bytes="base64")
print("Compact JSON (bytes as base64):")
print(json_str)
print()

# Indented print with UTF-8 encoding for bytes
json_indented = dumps.json(data, indent=2, bytes="utf-8")
print("Indented JSON (2 spaces, bytes as utf-8):")
print(json_indented)
print()

# Deserialize back
restored = dumps.loads(json_str)
print("Restored data:")
print(restored)
print()

# Note: tuples and sets become lists after deserialization
print("Note: tuple became list:", type(restored["tuple_data"]))
print("Note: set became list:", type(restored["set_data"]))
print("Note: datetime became string:", type(restored["timestamp"]), restored["timestamp"])
print("Note: bytes became string:", type(restored["bytes_data"]), restored["bytes_data"])
print()

# Bytes handling examples
print("Bytes handling examples:")
print("1. Default behavior (raise error):")
try:
    result = dumps.json({"data": b"test"})
    print(result)
except Exception as e:
    print(f"Error: {e}")

print("\n2. UTF-8 encoding:")
result = dumps.json({"data": "Hello, 世界".encode('utf-8')}, bytes="utf-8", indent=2)
print(result)

print("\n3. Base64 encoding:")
result = dumps.json({"data": b"\x00\x01\x02\x03"}, bytes="base64", indent=2)
print(result)