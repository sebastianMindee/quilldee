//! Python bindings for the Quilldee library.

#![allow(missing_docs)]

use pyo3::prelude::*;
use quilldee::geometry::point::Point;

#[pyclass(name = "Point", module = "quilldee.geometry")]
pub struct PyPoint(pub Point);

#[pymethods]
impl PyPoint {
    #[new]
    fn new(x: f64, y: f64) -> PyResult<Self> {
        Ok(Self(Point::new(x, y)))
    }

    #[getter]
    fn x(&self) -> f64 {
        self.0.x
    }

    #[getter]
    fn y(&self) -> f64 {
        self.0.y
    }

    #[setter]
    fn set_x(&mut self, value: f64) {
        self.0.x = value;
    }

    #[setter]
    fn set_y(&mut self, value: f64) {
        self.0.y = value;
    }

    /// Maps to Display
    #[pyo3(name = "__str__")]
    fn str_method(&self) -> PyResult<String> {
        Ok(format!("{}", self.0))
    }

    /// Maps to Debug
    #[pyo3(name = "__repr__")]
    fn repr_method(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    /// Maps to PartialEq
    #[pyo3(name = "__eq__")]
    fn eq_method(&self, other: &PyPoint) -> PyResult<bool> {
        Ok(self.0 == other.0)
    }

    /// Maps to Index
    #[pyo3(name = "__getitem__")]
    fn getitem_method(&self, index: usize) -> PyResult<f64> {
        Ok(self.0[index])
    }
}
