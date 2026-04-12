use super::Cube3x3;

impl Cube3x3 {
    // Convert sticker char to ANSI-colored output.
    pub(super) fn ansi_color(c: char) -> String {
        match c {
            'W' => format!("\x1b[37;1m{}\x1b[0m", c),
            'Y' => format!("\x1b[33;1m{}\x1b[0m", c),
            'G' => format!("\x1b[32;1m{}\x1b[0m", c),
            'B' => format!("\x1b[34;1m{}\x1b[0m", c),
            'O' => format!("\x1b[38;5;208m{}\x1b[0m", c),
            'R' => format!("\x1b[31;1m{}\x1b[0m", c),
            _ => c.to_string(),
        }
    }

    // Read the visible sticker color for a piece position and local face index.
    pub(super) fn get_corner_sticker(&self, pos: usize, face_index: usize) -> String {
        const CORNER_COLORS: [[char; 3]; 8] = [
            ['W', 'B', 'O'], // 0: UBL
            ['W', 'R', 'B'], // 1: UBR
            ['W', 'G', 'R'], // 2: UFR
            ['W', 'O', 'G'], // 3: UFL
            ['Y', 'O', 'B'], // 4: DBL
            ['Y', 'B', 'R'], // 5: DBR
            ['Y', 'R', 'G'], // 6: DFR
            ['Y', 'G', 'O'], // 7: DFL
        ];

        let (piece, orientation) = self.get_corner_internal(pos);
        let color_index = (face_index + orientation as usize) % 3;
        let char_color = CORNER_COLORS[piece as usize][color_index];

        Self::ansi_color(char_color)
    }

    pub(super) fn get_edge_sticker(&self, pos: usize, face_index: usize) -> String {
        const EDGE_COLORS: [[char; 2]; 12] = [
            ['W', 'B'], // 0:  UB
            ['W', 'R'], // 1:  UR
            ['W', 'G'], // 2:  UF
            ['W', 'O'], // 3:  UL
            ['Y', 'B'], // 4:  DB
            ['Y', 'R'], // 5:  DR
            ['Y', 'G'], // 6:  DF
            ['Y', 'O'], // 7:  DL
            ['B', 'O'], // 8:  BL
            ['B', 'R'], // 9:  BR
            ['G', 'R'], // 10: FR
            ['O', 'G'], // 11: FL
        ];

        let (piece, orientation) = self.get_edge_internal(pos);
        let color_index = (face_index + orientation as usize) % 2;
        let char_color = EDGE_COLORS[piece as usize][color_index];

        Self::ansi_color(char_color)
    }
}

