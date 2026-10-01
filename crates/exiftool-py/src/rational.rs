//! Rational number wrapper for Python.

use pyo3::prelude::*;

/// Rational number (numerator/denominator).
///
/// Represents EXIF rational values like ExposureTime (1/125) or FNumber (28/10).
#[pyclass(name = "Rational")]
#[derive(Clone)]
pub struct PyRational {
    /// Numerator
    #[pyo3(get)]
    pub num: i64,
    /// Denominator
    #[pyo3(get)]
    pub den: i64,
}

#[pymethods]
impl PyRational {
    #[new]
    fn new(num: i64, den: i64) -> Self {
        Self { num, den }
    }

    /// Get as float value.
    fn __float__(&self) -> f64 {
        if self.den == 0 {
            0.0
        } else {
            self.num as f64 / self.den as f64
        }
    }

    /// Get as int (truncated).
    fn __int__(&self) -> i64 {
        if self.den == 0 {
            0
        } else {
            self.num / self.den
        }
    }

    /// String representation: "num/den"
    fn __str__(&self) -> String {
        format!("{}/{}", self.num, self.den)
    }

    fn __repr__(&self) -> String {
        format!("Rational({}, {})", self.num, self.den)
    }

    /// Get as tuple (num, den).
    fn as_tuple(&self) -> (i64, i64) {
        (self.num, self.den)
    }

    /// Check if zero.
    #[getter]
    fn is_zero(&self) -> bool {
        self.num == 0
    }

    /// Get float value.
    #[getter]
    fn value(&self) -> f64 {
        self.__float__()
    }
}

impl PyRational {
    /// Create from unsigned rational.
    pub fn from_unsigned(num: u32, den: u32) -> Self {
        Self {
            num: num as i64,
            den: den as i64,
        }
    }

    /// Create from signed rational.
    pub fn from_signed(num: i32, den: i32) -> Self {
        Self {
            num: num as i64,
            den: den as i64,
        }
    }
}
