use pyo3::prelude::*;

mod cube2x2;
mod cube3x3;

use cube2x2::Cube2x2;
use cube3x3::Cube3x3;

#[pymodule]
fn fastcube(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Cube2x2>()?;
    m.add_class::<Cube3x3>()?;
    Ok(())
}
