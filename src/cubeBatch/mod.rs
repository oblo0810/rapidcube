use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

use crate::cube2x2::Cube2x2;
use crate::cube3x3::Cube3x3;

#[pyclass(name = "cubeBatch", sequence)]
pub struct CubeBatch {
    cubes: Vec<Py<PyAny>>,
}

impl CubeBatch {
    fn validate_cube(cube: &Bound<'_, PyAny>) -> PyResult<()> {
        if cube.is_instance_of::<Cube2x2>() || cube.is_instance_of::<Cube3x3>() {
            Ok(())
        } else {
            Err(PyTypeError::new_err(
                "cubeBatch only accepts rapidcube.Cube2x2 or rapidcube.Cube3x3 objects",
            ))
        }
    }
}

#[pymethods]
impl CubeBatch {
    #[new]
    pub fn new(py: Python<'_>, cubes: Vec<Py<PyAny>>) -> PyResult<Self> {
        for cube in &cubes {
            Self::validate_cube(cube.bind(py))?;
        }

        Ok(Self { cubes })
    }

    #[getter]
    pub fn cubes(&self, py: Python<'_>) -> Vec<Py<PyAny>> {
        self.cubes.iter().map(|cube| cube.clone_ref(py)).collect()
    }

    pub fn __len__(&self) -> usize {
        self.cubes.len()
    }

    pub fn len(&self) -> usize {
        self.__len__()
    }

    pub fn __getitem__(&self, py: Python<'_>, index: isize) -> PyResult<Py<PyAny>> {
        let len = self.cubes.len() as isize;
        let adjusted_index = if index < 0 { len + index } else { index };

        if adjusted_index < 0 || adjusted_index >= len {
            return Err(pyo3::exceptions::PyIndexError::new_err("cubeBatch index out of range"));
        }

        Ok(self.cubes[adjusted_index as usize].clone_ref(py))
    }

    /// Alternate constructor: create `count` cubes of the given type.
    /// `cube_type` accepts "2x2" or "3x3" (case-insensitive) or the type names
    /// "Cube2x2" / "Cube3x3".
    #[staticmethod]
    pub fn from_count(py: Python<'_>, count: usize, cube_type: &str) -> PyResult<Self> {
        let mut cubes: Vec<Py<PyAny>> = Vec::with_capacity(count);

        let t = cube_type.to_lowercase();

        for _ in 0..count {
            let obj = if t == "2x2" || t == "2" || t == "cube2x2" {
                py.get_type::<Cube2x2>().call0()?
            } else if t == "3x3" || t == "3" || t == "cube3x3" {
                py.get_type::<Cube3x3>().call0()?
            } else {
                return Err(PyTypeError::new_err("unknown cube type: expected '2x2' or '3x3'"));
            };

            cubes.push(obj.into());
        }

        Ok(Self { cubes })
    }

    pub fn is_empty(&self) -> bool {
        self.cubes.is_empty()
    }

    pub fn append(&mut self, py: Python<'_>, cube: Py<PyAny>) -> PyResult<()> {
        Self::validate_cube(cube.bind(py))?;
        self.cubes.push(cube);
        Ok(())
    }
}
