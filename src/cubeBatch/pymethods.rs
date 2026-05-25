use numpy::{IntoPyArray, PyArray2, PyReadonlyArray1};
use pyo3::exceptions::PyIndexError;
use pyo3::prelude::*;
use pyo3::types::PyAny;

use super::CubeBatch;

#[pymethods]
impl CubeBatch {
    #[new]
    /// Create a batch of solved cubes.
    ///
    /// Args:
    ///     count: Number of cubes in the batch.
    ///     cube_type: Cube size (2 or 3).
    ///
    /// Raises:
    ///     TypeError: If `cube_type` is not 2 or 3.
    pub fn new(count: usize, cube_type: usize) -> PyResult<Self> {
        Self::from_count_internal(count, cube_type)
    }

    /// Return successor states for all cubes using the 12 quarter-turn moves.
    ///
    /// Returns a NumPy int64 array with shape (len(self) * 12, sticker_dim).
    pub fn get_next_states(&self, py: Python<'_>) -> PyResult<Py<PyArray2<i64>>> {
        let states = py.detach(|| self.next_states_internal())?;
        Ok(states.into_pyarray(py).unbind())
    }

    /// Scramble each cube with the corresponding number of random moves.
    ///
    /// The `scramble_lengths` array is zipped with the batch. If it is shorter
    /// than the batch, only the first N cubes are scrambled. Extra lengths are
    /// ignored.
    pub fn scramble(&mut self, scramble_lengths: PyReadonlyArray1<'_, i64>) -> PyResult<()> {
        let scramble_lengths_rust: &[i64] = scramble_lengths.as_slice().unwrap();
        self.scramble_batch_internal(scramble_lengths_rust)
    }

    /// Return the number of cubes in the batch.
    pub fn __len__(&self) -> usize {
        self.len_internal()
    }

    /// Return the number of cubes in the batch.
    pub fn len(&self) -> usize {
        self.len_internal()
    }

    /// Return the cube at `index` (supports negative indexing).
    ///
    /// Raises:
    ///     IndexError: If `index` is out of range.
    pub fn __getitem__(&self, py: Python<'_>, index: isize) -> PyResult<Py<PyAny>> {
        let len = self.len_internal() as isize;
        let adjusted_index = if index < 0 { len + index } else { index };

        if adjusted_index < 0 || adjusted_index >= len {
            return Err(PyIndexError::new_err("cubeBatch index out of range"));
        }

        self.item_at_internal(py, adjusted_index as usize)
    }
}
