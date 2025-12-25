//! Value conversion between Rust AttrValue and Python objects.

use crate::rational::PyRational;
use exiftool_attrs::AttrValue;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict, PyList};

/// Convert AttrValue to Python object.
/// Returns PyResult to properly propagate errors instead of panicking.
pub fn to_python(py: Python<'_>, value: &AttrValue) -> PyResult<Py<PyAny>> {
    let obj = match value {
        AttrValue::Bool(v) => (*v).into_pyobject(py)?.to_owned().into_any().unbind(),
        AttrValue::Str(v) => v.into_pyobject(py)?.into_any().unbind(),
        AttrValue::Int8(v) => v.into_pyobject(py)?.into_any().unbind(),
        AttrValue::Int(v) => v.into_pyobject(py)?.into_any().unbind(),
        AttrValue::UInt(v) => v.into_pyobject(py)?.into_any().unbind(),
        AttrValue::Int64(v) => v.into_pyobject(py)?.into_any().unbind(),
        AttrValue::UInt64(v) => v.into_pyobject(py)?.into_any().unbind(),
        AttrValue::Float(v) => v.into_pyobject(py)?.into_any().unbind(),
        AttrValue::Double(v) => v.into_pyobject(py)?.into_any().unbind(),

        AttrValue::Rational(n, d) => {
            let r = PyRational::from_signed(*n, *d);
            r.into_pyobject(py)?.into_any().unbind()
        }
        AttrValue::URational(n, d) => {
            let r = PyRational::from_unsigned(*n, *d);
            r.into_pyobject(py)?.into_any().unbind()
        }

        AttrValue::Bytes(v) => PyBytes::new(py, v).into_any().unbind(),

        AttrValue::DateTime(dt) => {
            // Return as ISO string
            dt.format("%Y-%m-%d %H:%M:%S").to_string().into_pyobject(py)?.into_any().unbind()
        }

        AttrValue::Vec3(arr) => {
            PyList::new(py, arr.iter())?.into_any().unbind()
        }
        AttrValue::Vec4(arr) => {
            PyList::new(py, arr.iter())?.into_any().unbind()
        }

        AttrValue::Uuid(u) => u.to_string().into_pyobject(py)?.into_any().unbind(),

        AttrValue::List(items) => {
            let converted: PyResult<Vec<_>> = items.iter().map(|v| to_python(py, v)).collect();
            PyList::new(py, converted?)?.into_any().unbind()
        }

        AttrValue::Map(map) => {
            let dict = PyDict::new(py);
            for (k, v) in map {
                dict.set_item(k, to_python(py, v)?)?;
            }
            dict.into_any().unbind()
        }

        AttrValue::Set(set) => {
            let converted: PyResult<Vec<_>> = set.iter().map(|v| to_python(py, v)).collect();
            PyList::new(py, converted?)?.into_any().unbind()
        }

        AttrValue::Json(s) => s.into_pyobject(py)?.into_any().unbind(),

        AttrValue::Group(attrs) => {
            // Convert nested Attrs to Python dict
            let dict = PyDict::new(py);
            for (k, v) in attrs.iter() {
                dict.set_item(k, to_python(py, v)?)?;
            }
            dict.into_any().unbind()
        }
    };
    Ok(obj)
}

/// Convert Python object to AttrValue.
pub fn from_python(obj: &Bound<'_, PyAny>) -> PyResult<AttrValue> {
    // Try string first (most common)
    if let Ok(s) = obj.extract::<String>() {
        return Ok(AttrValue::Str(s));
    }

    // Bool before int (bool is subclass of int in Python)
    if let Ok(b) = obj.extract::<bool>() {
        return Ok(AttrValue::Bool(b));
    }

    // Integer
    if let Ok(i) = obj.extract::<i64>() {
        if i >= i32::MIN as i64 && i <= i32::MAX as i64 {
            return Ok(AttrValue::Int(i as i32));
        }
        return Ok(AttrValue::Int64(i));
    }

    // Float
    if let Ok(f) = obj.extract::<f64>() {
        return Ok(AttrValue::Double(f));
    }

    // Bytes
    if let Ok(b) = obj.extract::<Vec<u8>>() {
        return Ok(AttrValue::Bytes(b));
    }

    // Rational
    if let Ok(r) = obj.extract::<PyRational>() {
        if r.num >= 0 && r.den >= 0 {
            return Ok(AttrValue::URational(r.num as u32, r.den as u32));
        }
        return Ok(AttrValue::Rational(r.num as i32, r.den as i32));
    }

    // Fallback to string representation
    Ok(AttrValue::Str(obj.str()?.to_string()))
}

/// Get display string for a value.
pub fn display_value(value: &AttrValue) -> String {
    match value {
        AttrValue::Bytes(b) => format!("<{} bytes>", b.len()),
        AttrValue::Rational(n, d) => format!("{}/{}", n, d),
        AttrValue::URational(n, d) => format!("{}/{}", n, d),
        _ => value.to_string(),
    }
}
