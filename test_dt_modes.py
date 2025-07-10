#!/usr/bin/env python3
import dumps
from datetime import datetime

dt = datetime(2025, 7, 10, 11, 51, 57, 960798)

# Test default behavior (should be "iso")
print("Default (iso):")
result = dumps.json({"dt": dt})
print(result)
print()

# Test explicit "iso" mode
print("Explicit iso:")
result = dumps.json({"dt": dt}, dt="iso")
print(result)
print()

# Test "raise" mode
print("Raise mode:")
try:
    result = dumps.json({"dt": dt}, dt="raise")
    print(result)
except Exception as e:
    print(f"Error: {e}")
print()

# Test custom format
print("Custom format %Y-%m-%d:")
result = dumps.json({"dt": dt}, dt="%Y-%m-%d")
print(result)
print()

# Test another custom format
print("Custom format %B %d, %Y:")
result = dumps.json({"dt": dt}, dt="%B %d, %Y")
print(result)
print()

# Test time format
print("Custom format %H:%M:%S:")
result = dumps.json({"dt": dt}, dt="%H:%M:%S")
print(result)