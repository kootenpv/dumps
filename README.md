# dumps - Fast JSON Serialization with Rust

A Python library that uses Rust for fast JSON serialization of Python objects.

## Installation

```bash
# Install maturin first
pip install maturin

# Build and install the library
maturin develop
```

## Usage

```python
import dumps

# Serialize Python objects to JSON
data = {"name": "John", "age": 30, "items": [1, 2, 3]}
json_str = dumps.dumps(data)

# Pretty print
json_pretty = dumps.dumps(data, pretty=True)

# Deserialize JSON back to Python
restored = dumps.loads(json_str)
```

## Features

- Fast serialization using Rust
- Supports basic Python types: dict, list, tuple, str, int, float, bool, None
- Handles bytes objects (base64 encoded)
- Pretty printing option
- Custom object serialization with class name and repr

## Building

```bash
# Development build
maturin develop

# Release build
maturin build --release
```