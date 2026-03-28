use pyo3::prelude::*;
use pyo3::exceptions::{PyIndexError, PyValueError};

#[pyclass]
pub struct Cube2x2 {
    #[pyo3(get)]
    pub state: u64,
}

impl Cube2x2 {
    // Interne Hilfsfunktion, die keine Python-Errors wirft (für schnellen Zugriff)
    fn get_corner_internal(&self, index: usize) -> (u8, u8) {
        let shift = index * 5;
        let data = (self.state >> shift) & 0b11111;
        ((data & 0b111) as u8, ((data >> 3) & 0b11) as u8)
    }

    fn add_ori(corner: u64, amount: u64) -> u64 {
            let pos = corner & 0b111;
            let ori = corner >> 3;
            let new_ori = (ori + amount) % 3;
            pos | (new_ori << 3)
    }

    // Wandelt einen Buchstaben in einen ANSI-gefärbten String um
    fn ansi_color(c: char) -> String {
        match c {
            'W' => format!("\x1b[37;1m{}\x1b[0m", c), // Weiß (Up)
            'Y' => format!("\x1b[33;1m{}\x1b[0m", c), // Gelb (Down)
            'G' => format!("\x1b[32;1m{}\x1b[0m", c), // Grün (Front)
            'B' => format!("\x1b[34;1m{}\x1b[0m", c), // Blau (Back)
            'O' => format!("\x1b[38;5;208m{}\x1b[0m", c), // Orange (Left)
            'R' => format!("\x1b[31;1m{}\x1b[0m", c), // Rot (Right)
            _ => c.to_string(),
        }
    }

    // Liest die richtige Farbe für einen bestimmten Aufkleber aus
    fn get_sticker(&self, pos: usize, face_index: usize) -> String {
        // Basis-Farben der 8 Puzzleteile in der Reihenfolge: [Up/Down, Im Uhrzeigersinn, Gegen den Uhrzeigersinn]
        const PIECE_COLORS: [[char; 3]; 8] = [
            ['W', 'B', 'O'], // 0: UBL (Up-Back-Left)
            ['W', 'R', 'B'], // 1: UBR
            ['W', 'G', 'R'], // 2: UFR
            ['W', 'O', 'G'], // 3: UFL
            ['Y', 'O', 'B'], // 4: DBL
            ['Y', 'B', 'R'], // 5: DBR
            ['Y', 'R', 'G'], // 6: DFR
            ['Y', 'G', 'O'], // 7: DFL
        ];

        let (piece, orientation) = self.get_corner_internal(pos);
        
        // Die Orientierung (0, 1, 2) verschiebt, welche Farbe auf welcher Achse landet
        let color_index = (face_index + orientation as usize) % 3;
        let char_color = PIECE_COLORS[piece as usize][color_index];
        
        Self::ansi_color(char_color)
    }
}

#[pymethods]
impl Cube2x2 {
    #[new]
    pub fn new() -> Self {
        let mut state: u64 = 0; 
        for i in 0..8 {
            state |= (i as u64) << (i * 5);
        }
        Cube2x2 { state }
    }

    pub fn do_u_move(&mut self) -> PyResult<()> {
        let u_mask = 0xFFFFF;
        let u_face = self.state & u_mask;
        let rest = self.state & !u_mask;
        let rotated_u = ((u_face << 5) | (u_face >> 15)) & u_mask;
        self.state = rest | rotated_u;
        Ok(())
    }

    pub fn do_u_prime_move(&mut self) -> PyResult<()> {
        let u_mask = 0xFFFFF;
        let u_face = self.state & u_mask;
        let rest = self.state & !u_mask;
        let rotated_u = ((u_face >> 5) | (u_face << 15)) & u_mask;
        self.state = rest | rotated_u;
        Ok(())
    }

    pub fn do_d_move(&mut self) -> PyResult<()> {
        let d_mask = 0xFFFFF00000;
        let d_face = self.state & d_mask;
        let rest = self.state & !d_mask;
        let rotated_d = ((d_face << 5) | (d_face >> 15)) & d_mask;
        self.state = rest | rotated_d;
        Ok(())
    }

    pub fn do_d_prime_move(&mut self) -> PyResult<()> {
        let d_mask = 0xFFFFF00000;
        let d_face = self.state & d_mask;
        let rest = self.state & !d_mask;
        let rotated_d = ((d_face >> 5) | (d_face << 15)) & d_mask;
        self.state = rest | rotated_d;
        Ok(())
    }

    pub fn do_r_move(&mut self) -> PyResult<()> {
        // The R face affects corners: 1 (UBR), 2 (UFR), 5 (DBR), 6 (DFR)
        
        // Extract the current 5-bit block for each corner
        let c1 = (self.state >> 5) & 0b11111;
        let c2 = (self.state >> 10) & 0b11111;
        let c5 = (self.state >> 25) & 0b11111;
        let c6 = (self.state >> 30) & 0b11111;

        // Cycle the pieces and apply the correct orientation twists
        let new_c1 = Self::add_ori(c2, 1); // UFR -> UBR (+1)
        let new_c5 = Self::add_ori(c1, 2); // UBR -> DBR (+2)
        let new_c6 = Self::add_ori(c5, 1); // DBR -> DFR (+1)
        let new_c2 = Self::add_ori(c6, 2); // DFR -> UFR (+2)

        // Clear out the old bits for these 4 corners
        let clear_mask = !((0b11111 << 5) | (0b11111 << 10) | (0b11111 << 25) | (0b11111 << 30));
        self.state &= clear_mask;

        // Mask the newly calculated corners back into the state
        self.state |= new_c1 << 5;
        self.state |= new_c2 << 10;
        self.state |= new_c5 << 25;
        self.state |= new_c6 << 30;

        Ok(())
    }

    pub fn do_r_prime_move(&mut self) -> PyResult<()> {
        // The R face affects corners: 1 (UBR), 2 (UFR), 5 (DBR), 6 (DFR)
        
        // Extract the current 5-bit block for each corner
        let c1 = (self.state >> 5) & 0b11111;
        let c2 = (self.state >> 10) & 0b11111;
        let c5 = (self.state >> 25) & 0b11111;
        let c6 = (self.state >> 30) & 0b11111;

        // Cycle the pieces for R' (anti-clockwise) and apply the inverted twists
        let new_c2 = Self::add_ori(c1, 2); // UBR -> UFR (+2)
        let new_c1 = Self::add_ori(c5, 1); // DBR -> UBR (+1)
        let new_c5 = Self::add_ori(c6, 2); // DFR -> DBR (+2)
        let new_c6 = Self::add_ori(c2, 1); // UFR -> DFR (+1)

        // Clear out the old bits for these 4 corners
        let clear_mask = !((0b11111 << 5) | (0b11111 << 10) | (0b11111 << 25) | (0b11111 << 30));
        self.state &= clear_mask;

        // Mask the newly calculated corners back into the state
        self.state |= new_c1 << 5;
        self.state |= new_c2 << 10;
        self.state |= new_c5 << 25;
        self.state |= new_c6 << 30;

        Ok(())
    }

    pub fn do_l_move(&mut self) -> PyResult<()> {
        // The L face affects corners: 0 (UBL), 3 (UFL), 4 (DBL), 7 (DFL)
        
        // Extract the current 5-bit block for each corner
        let c0 = self.state & 0b11111;
        let c3 = (self.state >> 15) & 0b11111;
        let c4 = (self.state >> 20) & 0b11111;
        let c7 = (self.state >> 35) & 0b11111;

        // Cycle the pieces for L (clockwise) and apply the orientation twists
        let new_c3 = Self::add_ori(c0, 1); // UBL -> UFL (+1)
        let new_c7 = Self::add_ori(c3, 2); // UFL -> DFL (+2)
        let new_c4 = Self::add_ori(c7, 1); // DFL -> DBL (+1)
        let new_c0 = Self::add_ori(c4, 2); // DBL -> UBL (+2)

        // Clear out the old bits for these 4 corners
        let clear_mask = !((0b11111) | (0b11111 << 15) | (0b11111 << 20) | (0b11111 << 35));
        self.state &= clear_mask;

        // Mask the newly calculated corners back into the state
        self.state |= new_c0;        // shift 0
        self.state |= new_c3 << 15;
        self.state |= new_c4 << 20;
        self.state |= new_c7 << 35;

        Ok(())
    }

    pub fn do_l_prime_move(&mut self) -> PyResult<()> {
        // The L face affects corners: 0 (UBL), 3 (UFL), 4 (DBL), 7 (DFL)
        
        // Extract the current 5-bit block for each corner
        let c0 = self.state & 0b11111;
        let c3 = (self.state >> 15) & 0b11111;
        let c4 = (self.state >> 20) & 0b11111;
        let c7 = (self.state >> 35) & 0b11111;

        // Cycle the pieces for L' (anti-clockwise) and apply the inverted twists
        let new_c0 = Self::add_ori(c3, 2); // UFL -> UBL (+2)
        let new_c4 = Self::add_ori(c0, 1); // UBL -> DBL (+1)
        let new_c7 = Self::add_ori(c4, 2); // DBL -> DFL (+2)
        let new_c3 = Self::add_ori(c7, 1); // DFL -> UFL (+1)

        // Clear out the old bits for these 4 corners
        let clear_mask = !((0b11111) | (0b11111 << 15) | (0b11111 << 20) | (0b11111 << 35));
        self.state &= clear_mask;

        // Mask the newly calculated corners back into the state
        self.state |= new_c0;        
        self.state |= new_c3 << 15;
        self.state |= new_c4 << 20;
        self.state |= new_c7 << 35;

        Ok(())
    }

    pub fn do_f_move(&mut self) -> PyResult<()> {
        // The F face affects corners: 2 (UFR), 3 (UFL), 6 (DFR), 7 (DFL)
        
        let c2 = (self.state >> 10) & 0b11111;
        let c3 = (self.state >> 15) & 0b11111;
        let c6 = (self.state >> 30) & 0b11111;
        let c7 = (self.state >> 35) & 0b11111;

        let add_ori = |corner: u64, amount: u64| -> u64 {
            let pos = corner & 0b111;
            let ori = corner >> 3;
            let new_ori = (ori + amount) % 3;
            pos | (new_ori << 3)
        };

        // Cycle the pieces for F (clockwise) and apply the orientation twists
        let new_c2 = add_ori(c3, 1); // UFL(3) -> UFR(2) (+1)
        let new_c6 = add_ori(c2, 2); // UFR(2) -> DFR(6) (+2)
        let new_c7 = add_ori(c6, 1); // DFR(6) -> DFL(7) (+1)
        let new_c3 = add_ori(c7, 2); // DFL(7) -> UFL(3) (+2)

        // Clear out the old bits for these 4 corners
        let clear_mask = !((0b11111 << 10) | (0b11111 << 15) | (0b11111 << 30) | (0b11111 << 35));
        self.state &= clear_mask;

        // Mask the newly calculated corners back into the state
        self.state |= new_c2 << 10;
        self.state |= new_c3 << 15;
        self.state |= new_c6 << 30;
        self.state |= new_c7 << 35;

        Ok(())
    }

    pub fn do_f_prime_move(&mut self) -> PyResult<()> {
        // The F face affects corners: 2 (UFR), 3 (UFL), 6 (DFR), 7 (DFL)
        
        let c2 = (self.state >> 10) & 0b11111;
        let c3 = (self.state >> 15) & 0b11111;
        let c6 = (self.state >> 30) & 0b11111;
        let c7 = (self.state >> 35) & 0b11111;

        let add_ori = |corner: u64, amount: u64| -> u64 {
            let pos = corner & 0b111;
            let ori = corner >> 3;
            let new_ori = (ori + amount) % 3;
            pos | (new_ori << 3)
        };

        // Cycle the pieces for F' (anti-clockwise) and apply the inverted twists
        let new_c3 = add_ori(c2, 2); // UFR(2) -> UFL(3) (+2)
        let new_c7 = add_ori(c3, 1); // UFL(3) -> DFL(7) (+1)
        let new_c6 = add_ori(c7, 2); // DFL(7) -> DFR(6) (+2)
        let new_c2 = add_ori(c6, 1); // DFR(6) -> UFR(2) (+1)

        // Clear out the old bits for these 4 corners
        let clear_mask = !((0b11111 << 10) | (0b11111 << 15) | (0b11111 << 30) | (0b11111 << 35));
        self.state &= clear_mask;

        // Mask the newly calculated corners back into the state
        self.state |= new_c2 << 10;
        self.state |= new_c3 << 15;
        self.state |= new_c6 << 30;
        self.state |= new_c7 << 35;

        Ok(())
    }

    pub fn do_b_move(&mut self) -> PyResult<()> {
        // The B face affects corners: 0 (UBL), 1 (UBR), 4 (DBL), 5 (DBR)
        
        let c0 = self.state & 0b11111;
        let c1 = (self.state >> 5) & 0b11111;
        let c4 = (self.state >> 20) & 0b11111;
        let c5 = (self.state >> 25) & 0b11111;

        let add_ori = |corner: u64, amount: u64| -> u64 {
            let pos = corner & 0b111;
            let ori = corner >> 3;
            let new_ori = (ori + amount) % 3;
            pos | (new_ori << 3)
        };

        // Cycle the pieces for B (clockwise) and apply the orientation twists
        let new_c0 = add_ori(c1, 1); // UBR(1) -> UBL(0) (+1)
        let new_c4 = add_ori(c0, 2); // UBL(0) -> DBL(4) (+2)
        let new_c5 = add_ori(c4, 1); // DBL(4) -> DBR(5) (+1)
        let new_c1 = add_ori(c5, 2); // DBR(5) -> UBR(1) (+2)

        // Clear out the old bits for these 4 corners
        let clear_mask = !((0b11111) | (0b11111 << 5) | (0b11111 << 20) | (0b11111 << 25));
        self.state &= clear_mask;

        // Mask the newly calculated corners back into the state
        self.state |= new_c0;
        self.state |= new_c1 << 5;
        self.state |= new_c4 << 20;
        self.state |= new_c5 << 25;

        Ok(())
    }

    pub fn do_b_prime_move(&mut self) -> PyResult<()> {
        // The B face affects corners: 0 (UBL), 1 (UBR), 4 (DBL), 5 (DBR)
        
        let c0 = self.state & 0b11111;
        let c1 = (self.state >> 5) & 0b11111;
        let c4 = (self.state >> 20) & 0b11111;
        let c5 = (self.state >> 25) & 0b11111;

        let add_ori = |corner: u64, amount: u64| -> u64 {
            let pos = corner & 0b111;
            let ori = corner >> 3;
            let new_ori = (ori + amount) % 3;
            pos | (new_ori << 3)
        };

        // Cycle the pieces for B' (anti-clockwise) and apply the inverted twists
        let new_c1 = add_ori(c0, 2); // UBL(0) -> UBR(1) (+2)
        let new_c5 = add_ori(c1, 1); // UBR(1) -> DBR(5) (+1)
        let new_c4 = add_ori(c5, 2); // DBR(5) -> DBL(4) (+2)
        let new_c0 = add_ori(c4, 1); // DBL(4) -> UBL(0) (+1)

        // Clear out the old bits for these 4 corners
        let clear_mask = !((0b11111) | (0b11111 << 5) | (0b11111 << 20) | (0b11111 << 25));
        self.state &= clear_mask;

        // Mask the newly calculated corners back into the state
        self.state |= new_c0;
        self.state |= new_c1 << 5;
        self.state |= new_c4 << 20;
        self.state |= new_c5 << 25;

        Ok(())
    }

    /// Implementiert die Python `__str__` Methode, damit wir print(cube) nutzen können.
    fn __str__(&self) -> String {
        // U face (0=UBL, 1=UBR, 2=UFL, 3=UFR), face_index für U/D ist 0
        let u0 = self.get_sticker(0, 0); let u1 = self.get_sticker(1, 0);
        let u2 = self.get_sticker(3, 0); let u3 = self.get_sticker(2, 0);

        // L face (L ist bei UBL die 2. Achse, bei UFL die 1. Achse etc.)
        let l0 = self.get_sticker(0, 2); let l1 = self.get_sticker(3, 1);
        let l2 = self.get_sticker(4, 1); let l3 = self.get_sticker(7, 2);

        // F face
        let f0 = self.get_sticker(3, 2); let f1 = self.get_sticker(2, 1);
        let f2 = self.get_sticker(7, 1); let f3 = self.get_sticker(6, 2);

        // R face
        let r0 = self.get_sticker(2, 2); let r1 = self.get_sticker(1, 1);
        let r2 = self.get_sticker(6, 1); let r3 = self.get_sticker(5, 2);

        // B face
        let b0 = self.get_sticker(1, 2); let b1 = self.get_sticker(0, 1);
        let b2 = self.get_sticker(5, 1); let b3 = self.get_sticker(4, 2);

        // D face (4=DBL, 5=DBR, 6=DFL, 7=DFR), face_index für U/D ist 0
        let d0 = self.get_sticker(7, 0); let d1 = self.get_sticker(6, 0);
        let d2 = self.get_sticker(4, 0); let d3 = self.get_sticker(5, 0);

        format!(
            "      {} {}\n      {} {}\n{} {}   {} {}   {} {}   {} {}\n{} {}   {} {}   {} {}   {} {}\n      {} {}\n      {} {}\n",
            u0, u1, 
            u2, u3,
            l0, l1, f0, f1, r0, r1, b0, b1,
            l2, l3, f2, f3, r2, r3, b2, b3,
            d0, d1,
            d2, d3
        )
    }

    pub fn to_binary(&self) -> String {
        // Formats the u64 into a String with exactly 64 characters, 
        // padded with leading zeros.
        format!("{:064b}", self.state)
    }
}

#[pymodule]
fn fastcube(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Cube2x2>()?;
    Ok(())
}
