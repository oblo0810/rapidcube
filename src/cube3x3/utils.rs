use pyo3::prelude::*;

use super::Cube3x3;

#[pymethods]
impl Cube3x3 {
    /// Return the corner state as a 64-bit binary string.
    pub fn to_binary_corners(&self) -> PyResult<String> {
        Ok(format!("{:064b}", self.corners))
    }

    /// Return the edge state as a 64-bit binary string.
    pub fn to_binary_edges(&self) -> PyResult<String> {
        Ok(format!("{:064b}", self.edges))
    }

    /// Return the corner and edge states as a tuple of 64-bit binary strings.
    pub fn to_binary(&self) -> PyResult<(String, String)> {
        Ok((
            format!("{:064b}", self.corners),
            format!("{:064b}", self.edges),
        ))
    }

    /// Return the corner state as an array
    pub fn get_corners(&self) -> PyResult<Vec<(u8, u8)>> {
        let mut corners = Vec::new();
        for i in 0..8 {
            corners.push(self.get_corner_internal(i));
        }
        Ok(corners)
    }

    /// Return the edge state as an array
    pub fn get_edges(&self) -> PyResult<Vec<(u8, u8)>> {
        let mut edges = Vec::new();
        for i in 0..12 {
            edges.push(self.get_edge_internal(i));
        }
        Ok(edges)
    }

    /// Return True if the cube is solved
    pub fn is_solved(&self) -> PyResult<bool> {
        let mut solved_corners: u64 = 0;
        for i in 0..8 {
            solved_corners |= (i as u64) << (i * 5);
        }

        let mut solved_edges: u64 = 0;
        for i in 0..12 {
            solved_edges |= (i as u64) << (i * 5);
        }

        if self.corners == solved_corners && self.edges == solved_edges {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
