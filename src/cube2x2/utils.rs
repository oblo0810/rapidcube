use super::Cube2x2;

impl Cube2x2 {
    /// Return the corner and edge states as a tuple of 64-bit binary strings.
    pub(crate) fn binary_state(&self) -> String {
        format!("{:064b}", self.state)
    }

    /// Return the corner state as an array
    pub(crate) fn corners_state(&self) -> Vec<(u8, u8)> {
        let mut corners = Vec::new();
        for i in 0..8 {
            corners.push(self.get_corner_internal(i));
        }
        corners
    }

    /// Return True if the cube is solved
    pub(crate) fn is_solved_internal(&self) -> bool {
        let mut solved_state: u64 = 0;
        for i in 0..8 {
            solved_state |= (i as u64) << (i * 5);
        }

        self.state == solved_state
    }
}
