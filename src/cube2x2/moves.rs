use pyo3::prelude::*;

use super::Cube2x2;

#[pymethods]
impl Cube2x2 {
    #[new]
    /// Create a solved 2x2 cube state.
    pub fn new() -> Self {
        let mut state: u64 = 0;
        for i in 0..8 {
            state |= (i as u64) << (i * 5);
        }
        Cube2x2 { state }
    }

    /// Apply the U move (clockwise top face turn).
    pub fn do_u_move(&mut self) -> PyResult<()> {
        let u_mask = 0xFFFFF;
        let u_face = self.state & u_mask;
        let rest = self.state & !u_mask;
        let rotated_u = ((u_face << 5) | (u_face >> 15)) & u_mask;
        self.state = rest | rotated_u;
        Ok(())
    }

    /// Apply the U' move (counterclockwise top face turn).
    pub fn do_u_prime_move(&mut self) -> PyResult<()> {
        let u_mask = 0xFFFFF;
        let u_face = self.state & u_mask;
        let rest = self.state & !u_mask;
        let rotated_u = ((u_face >> 5) | (u_face << 15)) & u_mask;
        self.state = rest | rotated_u;
        Ok(())
    }

    /// Apply the D move (clockwise bottom face turn).
    pub fn do_d_move(&mut self) -> PyResult<()> {
        let d_mask = 0xFFFFF00000;
        let d_face = self.state & d_mask;
        let rest = self.state & !d_mask;
        let rotated_d = ((d_face >> 5) | (d_face << 15)) & d_mask;
        self.state = rest | rotated_d;
        Ok(())
    }

    /// Apply the D' move (counterclockwise bottom face turn).
    pub fn do_d_prime_move(&mut self) -> PyResult<()> {
        let d_mask = 0xFFFFF00000;
        let d_face = self.state & d_mask;
        let rest = self.state & !d_mask;
        let rotated_d = ((d_face << 5) | (d_face >> 15)) & d_mask;
        self.state = rest | rotated_d;
        Ok(())
    }

    /// Apply the R move (clockwise right face turn).
    pub fn do_r_move(&mut self) -> PyResult<()> {
        // R affects corners: 1 (UBR), 2 (UFR), 5 (DBR), 6 (DFR)
        let c1 = (self.state >> 5) & 0b11111;
        let c2 = (self.state >> 10) & 0b11111;
        let c5 = (self.state >> 25) & 0b11111;
        let c6 = (self.state >> 30) & 0b11111;

        let new_c1 = Self::add_ori(c2, 1); // UFR -> UBR (+1)
        let new_c5 = Self::add_ori(c1, 2); // UBR -> DBR (+2)
        let new_c6 = Self::add_ori(c5, 1); // DBR -> DFR (+1)
        let new_c2 = Self::add_ori(c6, 2); // DFR -> UFR (+2)

        let clear_mask = !((0b11111 << 5) | (0b11111 << 10) | (0b11111 << 25) | (0b11111 << 30));
        self.state &= clear_mask;

        self.state |= new_c1 << 5;
        self.state |= new_c2 << 10;
        self.state |= new_c5 << 25;
        self.state |= new_c6 << 30;

        Ok(())
    }

    /// Apply the R' move (counterclockwise right face turn).
    pub fn do_r_prime_move(&mut self) -> PyResult<()> {
        // R' affects corners: 1 (UBR), 2 (UFR), 5 (DBR), 6 (DFR)
        let c1 = (self.state >> 5) & 0b11111;
        let c2 = (self.state >> 10) & 0b11111;
        let c5 = (self.state >> 25) & 0b11111;
        let c6 = (self.state >> 30) & 0b11111;

        let new_c2 = Self::add_ori(c1, 2); // UBR -> UFR (+2)
        let new_c1 = Self::add_ori(c5, 1); // DBR -> UBR (+1)
        let new_c5 = Self::add_ori(c6, 2); // DFR -> DBR (+2)
        let new_c6 = Self::add_ori(c2, 1); // UFR -> DFR (+1)

        let clear_mask = !((0b11111 << 5) | (0b11111 << 10) | (0b11111 << 25) | (0b11111 << 30));
        self.state &= clear_mask;

        self.state |= new_c1 << 5;
        self.state |= new_c2 << 10;
        self.state |= new_c5 << 25;
        self.state |= new_c6 << 30;

        Ok(())
    }

    /// Apply the L move (clockwise left face turn).
    pub fn do_l_move(&mut self) -> PyResult<()> {
        // L affects corners: 0 (UBL), 3 (UFL), 4 (DBL), 7 (DFL)
        let c0 = self.state & 0b11111;
        let c3 = (self.state >> 15) & 0b11111;
        let c4 = (self.state >> 20) & 0b11111;
        let c7 = (self.state >> 35) & 0b11111;

        let new_c3 = Self::add_ori(c0, 1); // UBL -> UFL (+1)
        let new_c7 = Self::add_ori(c3, 2); // UFL -> DFL (+2)
        let new_c4 = Self::add_ori(c7, 1); // DFL -> DBL (+1)
        let new_c0 = Self::add_ori(c4, 2); // DBL -> UBL (+2)

        let clear_mask = !((0b11111) | (0b11111 << 15) | (0b11111 << 20) | (0b11111 << 35));
        self.state &= clear_mask;

        self.state |= new_c0;
        self.state |= new_c3 << 15;
        self.state |= new_c4 << 20;
        self.state |= new_c7 << 35;

        Ok(())
    }

    /// Apply the L' move (counterclockwise left face turn).
    pub fn do_l_prime_move(&mut self) -> PyResult<()> {
        // L' affects corners: 0 (UBL), 3 (UFL), 4 (DBL), 7 (DFL)
        let c0 = self.state & 0b11111;
        let c3 = (self.state >> 15) & 0b11111;
        let c4 = (self.state >> 20) & 0b11111;
        let c7 = (self.state >> 35) & 0b11111;

        let new_c0 = Self::add_ori(c3, 2); // UFL -> UBL (+2)
        let new_c4 = Self::add_ori(c0, 1); // UBL -> DBL (+1)
        let new_c7 = Self::add_ori(c4, 2); // DBL -> DFL (+2)
        let new_c3 = Self::add_ori(c7, 1); // DFL -> UFL (+1)

        let clear_mask = !((0b11111) | (0b11111 << 15) | (0b11111 << 20) | (0b11111 << 35));
        self.state &= clear_mask;

        self.state |= new_c0;
        self.state |= new_c3 << 15;
        self.state |= new_c4 << 20;
        self.state |= new_c7 << 35;

        Ok(())
    }

    /// Apply the F move (clockwise front face turn).
    pub fn do_f_move(&mut self) -> PyResult<()> {
        // F affects corners: 2 (UFR), 3 (UFL), 6 (DFR), 7 (DFL)
        let c2 = (self.state >> 10) & 0b11111;
        let c3 = (self.state >> 15) & 0b11111;
        let c6 = (self.state >> 30) & 0b11111;
        let c7 = (self.state >> 35) & 0b11111;

        let new_c2 = Self::add_ori(c3, 1); // UFL -> UFR (+1)
        let new_c6 = Self::add_ori(c2, 2); // UFR -> DFR (+2)
        let new_c7 = Self::add_ori(c6, 1); // DFR -> DFL (+1)
        let new_c3 = Self::add_ori(c7, 2); // DFL -> UFL (+2)

        let clear_mask = !((0b11111 << 10) | (0b11111 << 15) | (0b11111 << 30) | (0b11111 << 35));
        self.state &= clear_mask;

        self.state |= new_c2 << 10;
        self.state |= new_c3 << 15;
        self.state |= new_c6 << 30;
        self.state |= new_c7 << 35;

        Ok(())
    }

    /// Apply the F' move (counterclockwise front face turn).
    pub fn do_f_prime_move(&mut self) -> PyResult<()> {
        // F' affects corners: 2 (UFR), 3 (UFL), 6 (DFR), 7 (DFL)
        let c2 = (self.state >> 10) & 0b11111;
        let c3 = (self.state >> 15) & 0b11111;
        let c6 = (self.state >> 30) & 0b11111;
        let c7 = (self.state >> 35) & 0b11111;

        let new_c3 = Self::add_ori(c2, 2); // UFR -> UFL (+2)
        let new_c7 = Self::add_ori(c3, 1); // UFL -> DFL (+1)
        let new_c6 = Self::add_ori(c7, 2); // DFL -> DFR (+2)
        let new_c2 = Self::add_ori(c6, 1); // DFR -> UFR (+1)

        let clear_mask = !((0b11111 << 10) | (0b11111 << 15) | (0b11111 << 30) | (0b11111 << 35));
        self.state &= clear_mask;

        self.state |= new_c2 << 10;
        self.state |= new_c3 << 15;
        self.state |= new_c6 << 30;
        self.state |= new_c7 << 35;

        Ok(())
    }

    /// Apply the B move (clockwise back face turn).
    pub fn do_b_move(&mut self) -> PyResult<()> {
        // B affects corners: 0 (UBL), 1 (UBR), 4 (DBL), 5 (DBR)
        let c0 = self.state & 0b11111;
        let c1 = (self.state >> 5) & 0b11111;
        let c4 = (self.state >> 20) & 0b11111;
        let c5 = (self.state >> 25) & 0b11111;

        let new_c0 = Self::add_ori(c1, 1); // UBR -> UBL (+1)
        let new_c4 = Self::add_ori(c0, 2); // UBL -> DBL (+2)
        let new_c5 = Self::add_ori(c4, 1); // DBL -> DBR (+1)
        let new_c1 = Self::add_ori(c5, 2); // DBR -> UBR (+2)

        let clear_mask = !((0b11111) | (0b11111 << 5) | (0b11111 << 20) | (0b11111 << 25));
        self.state &= clear_mask;

        self.state |= new_c0;
        self.state |= new_c1 << 5;
        self.state |= new_c4 << 20;
        self.state |= new_c5 << 25;

        Ok(())
    }

    /// Apply the B' move (counterclockwise back face turn).
    pub fn do_b_prime_move(&mut self) -> PyResult<()> {
        // B' affects corners: 0 (UBL), 1 (UBR), 4 (DBL), 5 (DBR)
        let c0 = self.state & 0b11111;
        let c1 = (self.state >> 5) & 0b11111;
        let c4 = (self.state >> 20) & 0b11111;
        let c5 = (self.state >> 25) & 0b11111;

        let new_c1 = Self::add_ori(c0, 2); // UBL -> UBR (+2)
        let new_c5 = Self::add_ori(c1, 1); // UBR -> DBR (+1)
        let new_c4 = Self::add_ori(c5, 2); // DBR -> DBL (+2)
        let new_c0 = Self::add_ori(c4, 1); // DBL -> UBL (+1)

        let clear_mask = !((0b11111) | (0b11111 << 5) | (0b11111 << 20) | (0b11111 << 25));
        self.state &= clear_mask;

        self.state |= new_c0;
        self.state |= new_c1 << 5;
        self.state |= new_c4 << 20;
        self.state |= new_c5 << 25;

        Ok(())
    }

    /// Return an ANSI-colored string rendering of the cube.
    fn __str__(&self) -> String {
        let u0 = self.get_sticker(0, 0);
        let u1 = self.get_sticker(1, 0);
        let u2 = self.get_sticker(3, 0);
        let u3 = self.get_sticker(2, 0);

        let l0 = self.get_sticker(0, 2);
        let l1 = self.get_sticker(3, 1);
        let l2 = self.get_sticker(4, 1);
        let l3 = self.get_sticker(7, 2);

        let f0 = self.get_sticker(3, 2);
        let f1 = self.get_sticker(2, 1);
        let f2 = self.get_sticker(7, 1);
        let f3 = self.get_sticker(6, 2);

        let r0 = self.get_sticker(2, 2);
        let r1 = self.get_sticker(1, 1);
        let r2 = self.get_sticker(6, 1);
        let r3 = self.get_sticker(5, 2);

        let b0 = self.get_sticker(1, 2);
        let b1 = self.get_sticker(0, 1);
        let b2 = self.get_sticker(5, 1);
        let b3 = self.get_sticker(4, 2);

        let d0 = self.get_sticker(7, 0);
        let d1 = self.get_sticker(6, 0);
        let d2 = self.get_sticker(4, 0);
        let d3 = self.get_sticker(5, 0);

        format!(
            "      {} {}\n      {} {}\n{} {}   {} {}   {} {}   {} {}\n{} {}   {} {}   {} {}   {} {}\n      {} {}\n      {} {}\n",
            u0, u1, u2, u3, l0, l1, f0, f1, r0, r1, b0, b1, l2, l3, f2, f3, r2, r3, b2, b3,
            d0, d1, d2, d3
        )
    }

    /// Return the 64-bit cube state as a binary string.
    pub fn to_binary(&self) -> String {
        format!("{:064b}", self.state)
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
