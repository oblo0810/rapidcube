use pyo3::prelude::*;

use super::Cube3x3;

#[pymethods]
impl Cube3x3 {
    #[new]
    /// Create a solved 3x3 cube state.
    pub fn new() -> PyResult<Self> {
        Ok(Self::new_solved())
    }

    /// Return an ANSI-colored string rendering of the cube.
    fn __str__(&self) -> String {
        self.render_ansi_string()
    }

    /// Return the corner state as a 64-bit binary string.
    pub fn to_binary_corners(&self) -> PyResult<String> {
        Ok(self.binary_corners_string())
    }

    /// Return the edge state as a 64-bit binary string.
    pub fn to_binary_edges(&self) -> PyResult<String> {
        Ok(self.binary_edges_string())
    }

    /// Return the corner and edge states as a tuple of 64-bit binary strings.
    pub fn to_binary(&self) -> PyResult<(String, String)> {
        Ok(self.binary_state())
    }

    /// Return the corner state as an array.
    pub fn get_corners(&self) -> PyResult<Vec<(u8, u8)>> {
        Ok(self.corners_state())
    }

    /// Return the edge state as an array.
    pub fn get_edges(&self) -> PyResult<Vec<(u8, u8)>> {
        Ok(self.edges_state())
    }

    /// Return true if the cube is solved.
    pub fn is_solved(&self) -> PyResult<bool> {
        Ok(self.is_solved_internal())
    }

    /// Apply the U move (clockwise top face turn).
    pub fn do_u_move(&mut self) -> PyResult<()> {
        self.do_u_move_corners();
        self.do_u_move_edges();
        Ok(())
    }

    /// Apply the U' move (counterclockwise top face turn).
    pub fn do_u_prime_move(&mut self) -> PyResult<()> {
        self.do_u_prime_move_corners();
        self.do_u_prime_move_edges();
        Ok(())
    }

    /// Apply the D move (clockwise bottom face turn).
    pub fn do_d_move(&mut self) -> PyResult<()> {
        self.do_d_move_corners();
        self.do_d_move_edges();
        Ok(())
    }

    /// Apply the D' move (counterclockwise bottom face turn).
    pub fn do_d_prime_move(&mut self) -> PyResult<()> {
        self.do_d_prime_move_corners();
        self.do_d_prime_move_edges();
        Ok(())
    }

    /// Apply the R move (clockwise right face turn).
    pub fn do_r_move(&mut self) -> PyResult<()> {
        self.do_r_move_corners();
        self.rotate_edges(5, 45, 25, 50, 0);
        Ok(())
    }

    /// Apply the R' move (counterclockwise right face turn).
    pub fn do_r_prime_move(&mut self) -> PyResult<()> {
        self.do_r_prime_move_corners();
        self.rotate_edges_prime(5, 45, 25, 50, 0);
        Ok(())
    }

    /// Apply the L move (clockwise left face turn).
    pub fn do_l_move(&mut self) -> PyResult<()> {
        self.do_l_move_corners();
        self.rotate_edges(15, 55, 35, 40, 0);
        Ok(())
    }

    /// Apply the L' move (counterclockwise left face turn).
    pub fn do_l_prime_move(&mut self) -> PyResult<()> {
        self.do_l_prime_move_corners();
        self.rotate_edges_prime(15, 55, 35, 40, 0);
        Ok(())
    }

    /// Apply the F move (clockwise front face turn).
    pub fn do_f_move(&mut self) -> PyResult<()> {
        self.do_f_move_corners();
        self.rotate_edges(10, 50, 30, 55, 1);
        Ok(())
    }

    /// Apply the F' move (counterclockwise front face turn).
    pub fn do_f_prime_move(&mut self) -> PyResult<()> {
        self.do_f_prime_move_corners();
        self.rotate_edges_prime(10, 50, 30, 55, 1);
        Ok(())
    }

    /// Apply the B move (clockwise back face turn).
    pub fn do_b_move(&mut self) -> PyResult<()> {
        self.do_b_move_corners();
        self.rotate_edges(0, 40, 20, 45, 1);
        Ok(())
    }

    /// Apply the B' move (counterclockwise back face turn).
    pub fn do_b_prime_move(&mut self) -> PyResult<()> {
        self.do_b_prime_move_corners();
        self.rotate_edges_prime(0, 40, 20, 45, 1);
        Ok(())
    }

    /// Apply a whitespace-separated sequence of cube moves.
    pub fn do_moves(&mut self, moves: String) -> PyResult<()> {
        for mv in moves.split_whitespace() {
            match mv {
                "U" => self.do_u_move()?,
                "U'" | "U!" => self.do_u_prime_move()?,
                "U2" => {
                    self.do_u_move()?;
                    self.do_u_move()?;
                }
                "D" => self.do_d_move()?,
                "D'" | "D!" => self.do_d_prime_move()?,
                "D2" => {
                    self.do_d_move()?;
                    self.do_d_move()?;
                }
                "R" => self.do_r_move()?,
                "R'" | "R!" => self.do_r_prime_move()?,
                "R2" => {
                    self.do_r_move()?;
                    self.do_r_move()?;
                }
                "L" => self.do_l_move()?,
                "L'" | "L!" => self.do_l_prime_move()?,
                "L2" => {
                    self.do_l_move()?;
                    self.do_l_move()?;
                }
                "F" => self.do_f_move()?,
                "F'" | "F!" => self.do_f_prime_move()?,
                "F2" => {
                    self.do_f_move()?;
                    self.do_f_move()?;
                }
                "B" => self.do_b_move()?,
                "B'" | "B!" => self.do_b_prime_move()?,
                "B2" => {
                    self.do_b_move()?;
                    self.do_b_move()?;
                }
                _ => continue,
            }
        }
        Ok(())
    }
}
