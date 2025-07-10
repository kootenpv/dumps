#!/usr/bin/env python3
import dumps
from datetime import datetime

data = {
    "name": "John",
    "age": 30,
    "items": [1, 2, 3],
    "meta": {
        "created": datetime.now(),
        "active": True
    }
}

print("Compact (indent=0, default):")
result = dumps.json(data)
print(result)
print()

print("Indent=2:")
result = dumps.json(data, indent=2)
print(result)
print()

print("Indent=4:")
result = dumps.json(data, indent=4)
print(result)
print()

print("Indent=8:")
result = dumps.json(data, indent=8)
print(result)