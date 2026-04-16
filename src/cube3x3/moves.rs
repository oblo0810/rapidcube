use pyo3::prelude::*;

use super::Cube3x3;

impl Cube3x3 {
    fn new_solved() -> Self {
        let mut corners: u64 = 0;
        for i in 0..8 {
            corners |= (i as u64) << (i * 5);
        }

        let mut edges: u64 = 0;
        for i in 0..12 {
            edges |= (i as u64) << (i * 5);
        }

        Cube3x3 { corners, edges }
    }

    fn rotate_edges(&mut self, i0: u64, i1: u64, i2: u64, i3: u64, ori: u64) {
        let e0 = (self.edges >> i0) & 0b11111;
        let e1 = (self.edges >> i1) & 0b11111;
        let e2 = (self.edges >> i2) & 0b11111;
        let e3 = (self.edges >> i3) & 0b11111;

        let new_e0 = Self::add_ori_edge(e3, ori);
        let new_e1 = Self::add_ori_edge(e0, ori);
        let new_e2 = Self::add_ori_edge(e1, ori);
        let new_e3 = Self::add_ori_edge(e2, ori);

        let clear_mask = !((0b11111 << i0) | (0b11111 << i1) | (0b11111 << i2) | (0b11111 << i3));
        self.edges &= clear_mask;

        self.edges |= new_e0 << i0;
        self.edges |= new_e1 << i1;
        self.edges |= new_e2 << i2;
        self.edges |= new_e3 << i3;
    }

    fn rotate_edges_prime(&mut self, i0: u64, i1: u64, i2: u64, i3: u64, ori: u64) {
        let e0 = (self.edges >> i0) & 0b11111;
        let e1 = (self.edges >> i1) & 0b11111;
        let e2 = (self.edges >> i2) & 0b11111;
        let e3 = (self.edges >> i3) & 0b11111;

        let new_e0 = Self::add_ori_edge(e1, ori);
        let new_e1 = Self::add_ori_edge(e2, ori);
        let new_e2 = Self::add_ori_edge(e3, ori);
        let new_e3 = Self::add_ori_edge(e0, ori);

        let clear_mask = !((0b11111 << i0) | (0b11111 << i1) | (0b11111 << i2) | (0b11111 << i3));
        self.edges &= clear_mask;

        self.edges |= new_e0 << i0;
        self.edges |= new_e1 << i1;
        self.edges |= new_e2 << i2;
        self.edges |= new_e3 << i3;
    }

    fn do_u_move_corners(&mut self) {
        let u_mask = 0xFFFFF;
        let u_face = self.corners & u_mask;
        let rest = self.corners & !u_mask;
        let rotated_u = ((u_face << 5) | (u_face >> 15)) & u_mask;
        self.corners = rest | rotated_u;
    }

    fn do_u_move_edges(&mut self) {
        let u_mask = 0xFFFFF;
        let u_face = self.edges & u_mask;
        let rest = self.edges & !u_mask;
        let rotated_u = ((u_face << 5) | (u_face >> 15)) & u_mask;
        self.edges = rest | rotated_u;
    }

    fn do_u_prime_move_corners(&mut self) {
        let u_mask = 0xFFFFF;
        let u_face = self.corners & u_mask;
        let rest = self.corners & !u_mask;
        let rotated_u = ((u_face >> 5) | (u_face << 15)) & u_mask;
        self.corners = rest | rotated_u;
    }

    fn do_u_prime_move_edges(&mut self) {
        let u_mask = 0xFFFFF;
        let u_face = self.edges & u_mask;
        let rest = self.edges & !u_mask;
        let rotated_u = ((u_face >> 5) | (u_face << 15)) & u_mask;
        self.edges = rest | rotated_u;
    }

    fn do_d_move_corners(&mut self) {
        let d_mask = 0xFFFFF00000;
        let d_face = self.corners & d_mask;
        let rest = self.corners & !d_mask;
        let rotated_d = ((d_face >> 5) | (d_face << 15)) & d_mask;
        self.corners = rest | rotated_d;
    }

    fn do_d_move_edges(&mut self) {
        let d_mask = 0xFFFFF00000;
        let d_face = self.edges & d_mask;
        let rest = self.edges & !d_mask;
        let rotated_d = ((d_face >> 5) | (d_face << 15)) & d_mask;
        self.edges = rest | rotated_d;
    }

    fn do_d_prime_move_corners(&mut self) {
        let d_mask = 0xFFFFF00000;
        let d_face = self.corners & d_mask;
        let rest = self.corners & !d_mask;
        let rotated_d = ((d_face << 5) | (d_face >> 15)) & d_mask;
        self.corners = rest | rotated_d;
    }

    fn do_d_prime_move_edges(&mut self) {
        let d_mask = 0xFFFFF00000;
        let d_face = self.edges & d_mask;
        let rest = self.edges & !d_mask;
        let rotated_d = ((d_face << 5) | (d_face >> 15)) & d_mask;
        self.edges = rest | rotated_d;
    }

    fn do_r_move_corners(&mut self) {
        let c1 = (self.corners >> 5) & 0b11111;
        let c2 = (self.corners >> 10) & 0b11111;
        let c5 = (self.corners >> 25) & 0b11111;
        let c6 = (self.corners >> 30) & 0b11111;

        let new_c1 = Self::add_ori_corner(c2, 1);
        let new_c5 = Self::add_ori_corner(c1, 2);
        let new_c6 = Self::add_ori_corner(c5, 1);
        let new_c2 = Self::add_ori_corner(c6, 2);

        let clear_mask = !((0b11111 << 5) | (0b11111 << 10) | (0b11111 << 25) | (0b11111 << 30));
        self.corners &= clear_mask;

        self.corners |= new_c1 << 5;
        self.corners |= new_c2 << 10;
        self.corners |= new_c5 << 25;
        self.corners |= new_c6 << 30;
    }

    fn do_r_prime_move_corners(&mut self) {
        let c1 = (self.corners >> 5) & 0b11111;
        let c2 = (self.corners >> 10) & 0b11111;
        let c5 = (self.corners >> 25) & 0b11111;
        let c6 = (self.corners >> 30) & 0b11111;

        let new_c2 = Self::add_ori_corner(c1, 2);
        let new_c1 = Self::add_ori_corner(c5, 1);
        let new_c5 = Self::add_ori_corner(c6, 2);
        let new_c6 = Self::add_ori_corner(c2, 1);

        let clear_mask = !((0b11111 << 5) | (0b11111 << 10) | (0b11111 << 25) | (0b11111 << 30));
        self.corners &= clear_mask;

        self.corners |= new_c1 << 5;
        self.corners |= new_c2 << 10;
        self.corners |= new_c5 << 25;
        self.corners |= new_c6 << 30;
    }

    fn do_l_move_corners(&mut self) {
        let c0 = self.corners & 0b11111;
        let c3 = (self.corners >> 15) & 0b11111;
        let c4 = (self.corners >> 20) & 0b11111;
        let c7 = (self.corners >> 35) & 0b11111;

        let new_c3 = Self::add_ori_corner(c0, 1);
        let new_c7 = Self::add_ori_corner(c3, 2);
        let new_c4 = Self::add_ori_corner(c7, 1);
        let new_c0 = Self::add_ori_corner(c4, 2);

        let clear_mask = !((0b11111) | (0b11111 << 15) | (0b11111 << 20) | (0b11111 << 35));
        self.corners &= clear_mask;

        self.corners |= new_c0;
        self.corners |= new_c3 << 15;
        self.corners |= new_c4 << 20;
        self.corners |= new_c7 << 35;
    }

    fn do_l_prime_move_corners(&mut self) {
        let c0 = self.corners & 0b11111;
        let c3 = (self.corners >> 15) & 0b11111;
        let c4 = (self.corners >> 20) & 0b11111;
        let c7 = (self.corners >> 35) & 0b11111;

        let new_c0 = Self::add_ori_corner(c3, 2);
        let new_c4 = Self::add_ori_corner(c0, 1);
        let new_c7 = Self::add_ori_corner(c4, 2);
        let new_c3 = Self::add_ori_corner(c7, 1);

        let clear_mask = !((0b11111) | (0b11111 << 15) | (0b11111 << 20) | (0b11111 << 35));
        self.corners &= clear_mask;

        self.corners |= new_c0;
        self.corners |= new_c3 << 15;
        self.corners |= new_c4 << 20;
        self.corners |= new_c7 << 35;
    }

    fn do_f_move_corners(&mut self) {
        let c2 = (self.corners >> 10) & 0b11111;
        let c3 = (self.corners >> 15) & 0b11111;
        let c6 = (self.corners >> 30) & 0b11111;
        let c7 = (self.corners >> 35) & 0b11111;

        let new_c2 = Self::add_ori_corner(c3, 1);
        let new_c6 = Self::add_ori_corner(c2, 2);
        let new_c7 = Self::add_ori_corner(c6, 1);
        let new_c3 = Self::add_ori_corner(c7, 2);

        let clear_mask = !((0b11111 << 10) | (0b11111 << 15) | (0b11111 << 30) | (0b11111 << 35));
        self.corners &= clear_mask;

        self.corners |= new_c2 << 10;
        self.corners |= new_c3 << 15;
        self.corners |= new_c6 << 30;
        self.corners |= new_c7 << 35;
    }

    fn do_f_prime_move_corners(&mut self) {
        let c2 = (self.corners >> 10) & 0b11111;
        let c3 = (self.corners >> 15) & 0b11111;
        let c6 = (self.corners >> 30) & 0b11111;
        let c7 = (self.corners >> 35) & 0b11111;

        let new_c3 = Self::add_ori_corner(c2, 2);
        let new_c7 = Self::add_ori_corner(c3, 1);
        let new_c6 = Self::add_ori_corner(c7, 2);
        let new_c2 = Self::add_ori_corner(c6, 1);

        let clear_mask = !((0b11111 << 10) | (0b11111 << 15) | (0b11111 << 30) | (0b11111 << 35));
        self.corners &= clear_mask;

        self.corners |= new_c2 << 10;
        self.corners |= new_c3 << 15;
        self.corners |= new_c6 << 30;
        self.corners |= new_c7 << 35;
    }

    fn do_b_move_corners(&mut self) {
        let c0 = self.corners & 0b11111;
        let c1 = (self.corners >> 5) & 0b11111;
        let c4 = (self.corners >> 20) & 0b11111;
        let c5 = (self.corners >> 25) & 0b11111;

        let new_c0 = Self::add_ori_corner(c1, 1);
        let new_c4 = Self::add_ori_corner(c0, 2);
        let new_c5 = Self::add_ori_corner(c4, 1);
        let new_c1 = Self::add_ori_corner(c5, 2);

        let clear_mask = !((0b11111) | (0b11111 << 5) | (0b11111 << 20) | (0b11111 << 25));
        self.corners &= clear_mask;

        self.corners |= new_c0;
        self.corners |= new_c1 << 5;
        self.corners |= new_c4 << 20;
        self.corners |= new_c5 << 25;
    }

    fn do_b_prime_move_corners(&mut self) {
        let c0 = self.corners & 0b11111;
        let c1 = (self.corners >> 5) & 0b11111;
        let c4 = (self.corners >> 20) & 0b11111;
        let c5 = (self.corners >> 25) & 0b11111;

        let new_c1 = Self::add_ori_corner(c0, 2);
        let new_c5 = Self::add_ori_corner(c1, 1);
        let new_c4 = Self::add_ori_corner(c5, 2);
        let new_c0 = Self::add_ori_corner(c4, 1);

        let clear_mask = !((0b11111) | (0b11111 << 5) | (0b11111 << 20) | (0b11111 << 25));
        self.corners &= clear_mask;

        self.corners |= new_c0;
        self.corners |= new_c1 << 5;
        self.corners |= new_c4 << 20;
        self.corners |= new_c5 << 25;
    }
}

#[pymethods]
impl Cube3x3 {
    #[new]
    /// Create a solved 3x3 cube state.
    pub fn new() -> PyResult<Self> {
        Ok(Self::new_solved())
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

    /// Return the corner state as a 64-bit binary string.
    pub fn to_binary(&self) -> PyResult<String> {
        Ok(format!("{:064b}", self.corners))
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
