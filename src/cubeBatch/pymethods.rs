use pyo3::exceptions::PyIndexError;
use pyo3::prelude::*;
use pyo3::types::PyAny;

use super::CubeBatch;

#[pymethods]
impl CubeBatch {
    #[new]
    pub fn new(count: usize, cube_type: usize) -> PyResult<Self> {
        Self::from_count_internal(count, cube_type)
    }

    pub fn __len__(&self) -> usize {
        self.len_internal()
    }

    pub fn len(&self) -> usize {
        self.len_internal()
    }

    pub fn __getitem__(&self, py: Python<'_>, index: isize) -> PyResult<Py<PyAny>> {
        let len = self.len_internal() as isize;
        let adjusted_index = if index < 0 { len + index } else { index };

        if adjusted_index < 0 || adjusted_index >= len {
            return Err(PyIndexError::new_err("cubeBatch index out of range"));
        }

        self.item_at_internal(py, adjusted_index as usize)
    }

}
