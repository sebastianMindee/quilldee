//! Python bindings for the polygon class.
#![allow(missing_docs)]

use crate::geometry::point::PyPoint;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::PyAnyMethods;
use pyo3::types::PySlice;
use pyo3::types::PySliceMethods;
use pyo3::{Bound, FromPyObject, IntoPyObjectExt, Py, PyAny, PyRef, PyResult, pyclass, pymethods};
use quilldee::geometry::point::Point;
use quilldee::geometry::polygon::Polygon;

#[pyclass(name = "Polygon", module = "quilldee.geometry")]
pub struct PyPolygon(Polygon);

#[derive(FromPyObject)]
pub enum PointInput<'a> {
    Point(PyRef<'a, PyPoint>),
    Tuple((f64, f64)),
}

#[pymethods]
impl PyPolygon {
    #[new]
    fn new(points: Vec<PointInput>) -> PyResult<Self> {
        let core_polygon = points.into_iter().map(|input| match input {
            PointInput::Point(p) => p.0.clone(),
            PointInput::Tuple((x, y)) => Point::new(x, y),
        });

        Ok(Self(Polygon::new(core_polygon)))
    }
    /// Maps to Display through to_string_representation()
    #[pyo3(name = "__str__")]
    fn str_method(&self) -> PyResult<String> {
        Ok(self.0.to_string_representation())
    }

    /// Maps to Debug
    #[pyo3(name = "__repr__")]
    fn repr_method(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }

    /// Maps to PartialEq
    #[pyo3(name = "__eq__")]
    fn eq_method(&self, other: &PyPolygon) -> PyResult<bool> {
        Ok(self.0 == other.0)
    }

    /// Maps to Index
    #[pyo3(name = "__getitem__")]
    fn getitem_method(&self, key: &Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
        let py = key.py();
        if let Ok(index) = key.extract::<isize>() {
            let len = self.0.0.len() as isize;
            let actual_index = if index < 0 { index + len } else { index };
            if actual_index < 0 {
                return Err(pyo3::exceptions::PyIndexError::new_err(
                    "polygon index out of range",
                ));
            }
            let point = self
                .0
                .0
                .get(actual_index as usize)
                .copied()
                .map(PyPoint)
                .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("index out of range"));
            return Ok(point?.into_py_any(py)?);
        }

        if let Ok(slice) = key.cast::<PySlice>() {
            let indices = slice.indices(self.0.0.len() as isize)?;

            let mut result = Vec::new();
            let mut i = indices.start;
            while if indices.step > 0 {
                i < indices.stop
            } else {
                i > indices.stop
            } {
                if let Some(point) = self.0.0.get(i as usize).copied().map(PyPoint) {
                    result.push(point);
                }
                i += indices.step;
            }
            return Ok(PyPolygon(Polygon::new(result.into_iter().map(|p| p.0))).into_py_any(py)?);
        }

        Err(PyTypeError::new_err("indices must be integers or slices"))
    }

    // Length
    #[pyo3(name = "__len__")]
    fn len_method(&self) -> PyResult<usize> {
        Ok(self.0.0.len())
    }

    // Centroid method
    fn centroid(&self) -> PyResult<Option<PyPoint>> {
        Ok(self.0.centroid().map(PyPoint))
    }
}
