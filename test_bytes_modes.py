#!/usr/bin/env python3
import dumps

# Test data with different byte strings
utf8_bytes = "Hello, 世界".encode('utf-8')
ascii_bytes = b"Hello, ASCII"
binary_bytes = b"\x00\x01\x02\x03\xff"

print("=== Testing bytes handling modes ===\n")

# Test default behavior (should be "raise")
print("1. Default (raise) mode:")
try:
    result = dumps.json({"data": utf8_bytes})
    print(f"Result: {result}")
except Exception as e:
    print(f"Error: {e}")
print()

# Test explicit "raise" mode
print("2. Explicit raise mode:")
try:
    result = dumps.json({"data": utf8_bytes}, bytes="raise")
    print(f"Result: {result}")
except Exception as e:
    print(f"Error: {e}")
print()

# Test UTF-8 encoding
print("3. UTF-8 encoding:")
try:
    result = dumps.json({"data": utf8_bytes}, bytes="utf-8")
    print(f"Result: {result}")
except Exception as e:
    print(f"Error: {e}")
print()

# Test ASCII encoding with ASCII bytes
print("4. ASCII encoding (valid ASCII):")
try:
    result = dumps.json({"data": ascii_bytes}, bytes="ascii")
    print(f"Result: {result}")
except Exception as e:
    print(f"Error: {e}")
print()

# Test ASCII encoding with non-ASCII bytes
print("5. ASCII encoding (invalid ASCII):")
try:
    result = dumps.json({"data": utf8_bytes}, bytes="ascii")
    print(f"Result: {result}")
except Exception as e:
    print(f"Error: {e}")
print()

# Test base64 encoding
print("6. Base64 encoding:")
try:
    result = dumps.json({"data": binary_bytes}, bytes="base64")
    print(f"Result: {result}")
except Exception as e:
    print(f"Error: {e}")
print()

# Test invalid encoding
print("7. Invalid encoding:")
try:
    result = dumps.json({"data": utf8_bytes}, bytes="invalid")
    print(f"Result: {result}")
except Exception as e:
    print(f"Error: {e}")
print()

# Test with nested data
print("8. Nested data with bytes:")
try:
    data = {
        "text": "hello",
        "items": [ascii_bytes, utf8_bytes],
        "meta": {"binary": binary_bytes}
    }
    result = dumps.json(data, bytes="base64", indent=2)
    print(f"Result: {result}")
except Exception as e:
    print(f"Error: {e}")