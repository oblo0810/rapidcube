use super::Cube3x3;

impl Cube3x3 {
    /// Return the corner state as a 64-bit binary string.
    pub(crate) fn binary_corners_string(&self) -> String {
        format!("{:064b}", self.corners)
    }

    /// Return the edge state as a 64-bit binary string.
    pub(crate) fn binary_edges_string(&self) -> String {
        format!("{:064b}", self.edges)
    }

    /// Return the corner and edge states as a tuple of 64-bit binary strings.
    pub(crate) fn binary_state(&self) -> (String, String) {
        (
            format!("{:064b}", self.corners),
            format!("{:064b}", self.edges),
        )
    }

    /// Return the corner state as an array
    pub(crate) fn corners_state(&self) -> Vec<(u8, u8)> {
        let mut corners = Vec::new();
        for i in 0..8 {
            corners.push(self.get_corner_internal(i));
        }
        corners
    }

    /// Return the edge state as an array
    pub(crate) fn edges_state(&self) -> Vec<(u8, u8)> {
        let mut edges = Vec::new();
        for i in 0..12 {
            edges.push(self.get_edge_internal(i));
        }
        edges
    }

    /// Return True if the cube is solved
    pub(crate) fn is_solved_internal(&self) -> bool {
        let mut solved_corners: u64 = 0;
        for i in 0..8 {
            solved_corners |= (i as u64) << (i * 5);
        }

        let mut solved_edges: u64 = 0;
        for i in 0..12 {
            solved_edges |= (i as u64) << (i * 5);
        }

        self.corners == solved_corners && self.edges == solved_edges
    }
}
