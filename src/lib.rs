use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyTuple, PySet, PyBool, PyBytes, PyString};
use serde_json::{json, Value};
use base64::{Engine as _, engine::general_purpose};

fn python_to_json_value(obj: &Bound<'_, PyAny>) -> PyResult<Value> {
    if obj.is_none() {
        Ok(Value::Null)
    } else if let Ok(b) = obj.downcast::<PyBool>() {
        Ok(Value::Bool(b.is_true()))
    } else if let Ok(i) = obj.extract::<i64>() {
        Ok(json!(i))
    } else if let Ok(f) = obj.extract::<f64>() {
        Ok(json!(f))
    } else if let Ok(s) = obj.downcast::<PyString>() {
        Ok(Value::String(s.to_string()))
    } else if let Ok(bytes) = obj.downcast::<PyBytes>() {
        let b = bytes.as_bytes();
        let encoded = general_purpose::STANDARD.encode(b);
        Ok(json!({"__bytes__": encoded}))
    } else if let Ok(list) = obj.downcast::<PyList>() {
        let mut vec = Vec::new();
        for item in list.iter() {
            vec.push(python_to_json_value(&item)?);
        }
        Ok(Value::Array(vec))
    } else if let Ok(tuple) = obj.downcast::<PyTuple>() {
        let mut vec = Vec::new();
        for item in tuple.iter() {
            vec.push(python_to_json_value(&item)?);
        }
        Ok(Value::Array(vec))
    } else if let Ok(set) = obj.downcast::<PySet>() {
        let mut vec = Vec::new();
        for item in set.iter() {
            vec.push(python_to_json_value(&item)?);
        }
        Ok(Value::Array(vec))
    } else if let Ok(dict) = obj.downcast::<PyDict>() {
        let mut map = serde_json::Map::new();
        for (key, value) in dict.iter() {
            let key_str = key.str()?.to_string();
            map.insert(key_str, python_to_json_value(&value)?);
        }
        Ok(Value::Object(map))
    } else {
        let class_name = obj.get_type().name()?.to_string();
        let repr = obj.repr()?.to_string();
        Ok(json!({
            "__class__": class_name,
            "__repr__": repr
        }))
    }
}

#[pyfunction(signature = (obj, pretty=None))]
fn json(obj: &Bound<'_, PyAny>, pretty: Option<bool>) -> PyResult<String> {
    let value = python_to_json_value(obj)?;
    
    let result = if pretty.unwrap_or(false) {
        serde_json::to_string_pretty(&value)
    } else {
        serde_json::to_string(&value)
    };
    
    result.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("JSON serialization error: {}", e)))
}

#[pyfunction]
fn loads(json_str: &str) -> PyResult<PyObject> {
    Python::with_gil(|py| {
        let value: Value = serde_json::from_str(json_str)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("JSON parsing error: {}", e)))?;
        
        json_value_to_python(py, &value)
    })
}

fn json_value_to_python(py: Python<'_>, value: &Value) -> PyResult<PyObject> {
    match value {
        Value::Null => Ok(py.None()),
        Value::Bool(b) => Ok((*b).into_py(py)),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.into_py(py))
            } else if let Some(f) = n.as_f64() {
                Ok(f.into_py(py))
            } else {
                Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid number"))
            }
        }
        Value::String(s) => Ok(s.clone().into_py(py)),
        Value::Array(arr) => {
            let list = PyList::empty(py);
            for item in arr {
                list.append(json_value_to_python(py, item)?)?;
            }
            Ok(list.into())
        }
        Value::Object(map) => {
            if let Some(Value::String(encoded)) = map.get("__bytes__") {
                let decoded = general_purpose::STANDARD.decode(encoded)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Base64 decode error: {}", e)))?;
                Ok(PyBytes::new(py, &decoded).into())
            } else if map.contains_key("__class__") && map.contains_key("__repr__") {
                let dict = PyDict::new(py);
                for (k, v) in map {
                    dict.set_item(k, json_value_to_python(py, v)?)?;
                }
                Ok(dict.into())
            } else {
                let dict = PyDict::new(py);
                for (k, v) in map {
                    dict.set_item(k, json_value_to_python(py, v)?)?;
                }
                Ok(dict.into())
            }
        }
    }
}

#[pymodule]
#[pyo3(name = "dumps")]
fn dumps_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(json, m)?)?;
    m.add_function(wrap_pyfunction!(loads, m)?)?;
    Ok(())
}