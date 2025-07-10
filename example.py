#!/usr/bin/env python3
import dumps

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
    "bytes_data": b"Hello, bytes!"
}

# Serialize to JSON
json_str = dumps.json(data)
print("Compact JSON:")
print(json_str)
print()

# Pretty print
json_pretty = dumps.json(data, pretty=True)
print("Pretty JSON:")
print(json_pretty)
print()

# Deserialize back
restored = dumps.loads(json_str)
print("Restored data:")
print(restored)
print()

# Note: tuples and sets become lists after deserialization
print("Note: tuple became list:", type(restored["tuple_data"]))
print("Note: set became list:", type(restored["set_data"]))
print()

# Custom object example
class Person:
    def __init__(self, name, age):
        self.name = name
        self.age = age
    
    def __repr__(self):
        return f"Person(name={self.name!r}, age={self.age})"

person = Person("Alice", 25)
person_json = dumps.json({"person": person}, pretty=True)
print("Custom object serialization:")
print(person_json)