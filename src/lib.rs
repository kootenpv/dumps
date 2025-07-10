use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyTuple, PySet, PyBool, PyBytes, PyString};
use serde_json::{json, Value};
use serde::Serialize;
use base64::{Engine as _, engine::general_purpose};

fn python_to_json_value(obj: &Bound<'_, PyAny>, dt_mode: &str, bytes_mode: &str) -> PyResult<Value> {
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
    } else if obj.hasattr("isoformat")? {
        match dt_mode {
            "raise" => {
                Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>("datetime objects are not JSON serializable"))
            }
            "iso" => {
                if let Ok(iso_str) = obj.call_method0("isoformat").and_then(|r| r.extract::<String>()) {
                    Ok(Value::String(iso_str))
                } else {
                    let class_name = obj.get_type().name()?.to_string();
                    Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!("Object of type '{}' is not JSON serializable", class_name)))
                }
            }
            format_str => {
                if let Ok(formatted) = obj.call_method1("strftime", (format_str,)).and_then(|r| r.extract::<String>()) {
                    Ok(Value::String(formatted))
                } else {
                    let class_name = obj.get_type().name()?.to_string();
                    Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!("Failed to format datetime with format string: {}", format_str)))
                }
            }
        }
    } else if let Ok(bytes) = obj.downcast::<PyBytes>() {
        let b = bytes.as_bytes();
        match bytes_mode {
            "raise" => {
                Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>("bytes objects are not JSON serializable"))
            }
            encoding => {
                match std::str::from_utf8(b) {
                    Ok(s) if encoding == "utf-8" || encoding == "utf8" => {
                        Ok(Value::String(s.to_string()))
                    }
                    _ => {
                        match encoding {
                            "ascii" => {
                                if b.iter().all(|&byte| byte.is_ascii()) {
                                    Ok(Value::String(String::from_utf8_lossy(b).into_owned()))
                                } else {
                                    Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("bytes contain non-ASCII characters"))
                                }
                            }
                            "utf-8" | "utf8" => {
                                match String::from_utf8(b.to_vec()) {
                                    Ok(s) => Ok(Value::String(s)),
                                    Err(_) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("bytes contain invalid UTF-8"))
                                }
                            }
                            "base64" => {
                                let encoded = general_purpose::STANDARD.encode(b);
                                Ok(Value::String(encoded))
                            }
                            _ => {
                                Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("unsupported encoding: {}", encoding)))
                            }
                        }
                    }
                }
            }
        }
    } else if let Ok(list) = obj.downcast::<PyList>() {
        let mut vec = Vec::new();
        for item in list.iter() {
            vec.push(python_to_json_value(&item, dt_mode, bytes_mode)?);
        }
        Ok(Value::Array(vec))
    } else if let Ok(tuple) = obj.downcast::<PyTuple>() {
        let mut vec = Vec::new();
        for item in tuple.iter() {
            vec.push(python_to_json_value(&item, dt_mode, bytes_mode)?);
        }
        Ok(Value::Array(vec))
    } else if let Ok(set) = obj.downcast::<PySet>() {
        let mut vec = Vec::new();
        for item in set.iter() {
            vec.push(python_to_json_value(&item, dt_mode, bytes_mode)?);
        }
        Ok(Value::Array(vec))
    } else if let Ok(dict) = obj.downcast::<PyDict>() {
        let mut map = serde_json::Map::new();
        for (key, value) in dict.iter() {
            let key_str = key.str()?.to_string();
            map.insert(key_str, python_to_json_value(&value, dt_mode, bytes_mode)?);
        }
        Ok(Value::Object(map))
    } else {
        let class_name = obj.get_type().name()?.to_string();
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!("Object of type '{}' is not JSON serializable", class_name)))
    }
}

#[pyfunction(signature = (obj, indent=None, dt=None, bytes=None))]
fn json(obj: &Bound<'_, PyAny>, indent: Option<usize>, dt: Option<String>, bytes: Option<String>) -> PyResult<String> {
    let dt_mode = dt.unwrap_or_else(|| "iso".to_string());
    let bytes_mode = bytes.unwrap_or_else(|| "raise".to_string());
    let value = python_to_json_value(obj, &dt_mode, &bytes_mode)?;
    
    let result = match indent.unwrap_or(0) {
        0 => serde_json::to_string(&value)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("JSON serialization error: {}", e))),
        spaces => {
            let indent_bytes = vec![b' '; spaces];
            let formatter = serde_json::ser::PrettyFormatter::with_indent(&indent_bytes);
            let mut ser = serde_json::Serializer::with_formatter(Vec::new(), formatter);
            value.serialize(&mut ser).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("JSON serialization error: {}", e)))?;
            String::from_utf8(ser.into_inner()).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("UTF-8 error: {}", e)))
        }
    };
    
    result
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