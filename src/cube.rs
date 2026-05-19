use ndarray::Array2;

pub(crate) trait Cube: Sized {
    const STICKER_DIM: usize;

    fn corners_state_internal(&self) -> Vec<(u8, u8)>;
    fn is_solved_internal(&self) -> bool;
    fn render_ansi_internal(&self) -> String;
    fn state_copy_internal(&self) -> Self;
    fn sticker_array_internal(&self) -> Vec<i64>;

    fn apply_move_index_internal(&mut self, move_index: usize);
    fn do_u_move_internal(&mut self);
    fn do_u_prime_move_internal(&mut self);
    fn do_d_move_internal(&mut self);
    fn do_d_prime_move_internal(&mut self);
    fn do_r_move_internal(&mut self);
    fn do_r_prime_move_internal(&mut self);
    fn do_l_move_internal(&mut self);
    fn do_l_prime_move_internal(&mut self);
    fn do_f_move_internal(&mut self);
    fn do_f_prime_move_internal(&mut self);
    fn do_b_move_internal(&mut self);
    fn do_b_prime_move_internal(&mut self);

    fn do_moves_internal(&mut self, moves: &str) {
        for mv in moves.split_whitespace() {
            match mv {
                "U" => self.do_u_move_internal(),
                "U'" | "U!" => self.do_u_prime_move_internal(),
                "U2" => {
                    self.do_u_move_internal();
                    self.do_u_move_internal();
                }
                "D" => self.do_d_move_internal(),
                "D'" | "D!" => self.do_d_prime_move_internal(),
                "D2" => {
                    self.do_d_move_internal();
                    self.do_d_move_internal();
                }
                "R" => self.do_r_move_internal(),
                "R'" | "R!" => self.do_r_prime_move_internal(),
                "R2" => {
                    self.do_r_move_internal();
                    self.do_r_move_internal();
                }
                "L" => self.do_l_move_internal(),
                "L'" | "L!" => self.do_l_prime_move_internal(),
                "L2" => {
                    self.do_l_move_internal();
                    self.do_l_move_internal();
                }
                "F" => self.do_f_move_internal(),
                "F'" | "F!" => self.do_f_prime_move_internal(),
                "F2" => {
                    self.do_f_move_internal();
                    self.do_f_move_internal();
                }
                "B" => self.do_b_move_internal(),
                "B'" | "B!" => self.do_b_prime_move_internal(),
                "B2" => {
                    self.do_b_move_internal();
                    self.do_b_move_internal();
                }
                _ => continue,
            }
        }
    }
}

pub(crate) fn next_states_internal<C: Cube>(cubes: &[C]) -> Array2<i64> {
    const NUM_MOVES: usize = 12;

    let mut array = Array2::<i64>::zeros((cubes.len() * NUM_MOVES, C::STICKER_DIM));

    for (cube_index, cube) in cubes.iter().enumerate() {
        for move_index in 0..NUM_MOVES {
            let mut next_cube = cube.state_copy_internal();
            next_cube.apply_move_index_internal(move_index);

            let sticker_array = next_cube.sticker_array_internal();
            let row_index = cube_index * NUM_MOVES + move_index;

            for (sticker_index, sticker_value) in sticker_array.iter().enumerate() {
                array[[row_index, sticker_index]] = *sticker_value;
            }
        }
    }

    array
}
