use pyo3::prelude::*;

mod cube2x2;

use cube2x2::Cube2x2;

#[pymodule]
fn fastcube(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Cube2x2>()?;
    Ok(())
}
