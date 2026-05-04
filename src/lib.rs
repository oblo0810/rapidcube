use pyo3::prelude::*;

mod cube2x2;
mod cube3x3;

use cube2x2::Cube2x2;
use cube3x3::Cube3x3;

/// Return the inverse of the given scramble string.
#[pyfunction]
fn inverse_scramble(scramble: &str) -> String {
    let mut inverse_moves = Vec::new();
    for move_str in scramble.split_whitespace().rev() {
        let (base_move, suffix) = if move_str.ends_with("2") {
            (&move_str[..move_str.len() - 1], "2")
        } else if move_str.ends_with("'") {
            (&move_str[..move_str.len() - 1], "'")
        } else {
            (move_str, "")
        };

        let inverse_base = match base_move {
            "U" => "U'",
            "U'" => "U",
            "R" => "R'",
            "R'" => "R",
            "F" => "F'",
            "F'" => "F",
            "D" => "D'",
            "D'" => "D",
            "L" => "L'",
            "L'" => "L",
            "B" => "B'",
            "B'" => "B",
            _ => base_move,
        };
        inverse_moves.push(format!("{}{}", inverse_base, suffix));
    }
    inverse_moves.join(" ")
}

#[pymodule]
fn rapidcube(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Cube2x2>()?;
    m.add_class::<Cube3x3>()?;
    m.add_function(wrap_pyfunction!(inverse_scramble, m)?)?;
    Ok(())
}
