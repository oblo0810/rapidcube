import unittest

import rapidcube

Cube2x2 = rapidcube.Cube2x2
CubeBatch = rapidcube.cubeBatch


MOVE_PAIRS = [
    ("do_u_move", "do_u_prime_move"),
    ("do_d_move", "do_d_prime_move"),
    ("do_r_move", "do_r_prime_move"),
    ("do_l_move", "do_l_prime_move"),
    ("do_f_move", "do_f_prime_move"),
    ("do_b_move", "do_b_prime_move"),
]

INVERSE_OF = {move: inv for move, inv in MOVE_PAIRS}
INVERSE_OF.update({inv: move for move, inv in MOVE_PAIRS})


def _identity_state() -> int:
    state = 0
    for i in range(8):
        state |= i << (i * 5)
    return state


class TestCube2x2(unittest.TestCase):
    def test_new_cube_has_identity_state(self):
        cube = Cube2x2()
        self.assertEqual(cube.state, _identity_state())

    def test_to_binary_format(self):
        cube = Cube2x2()
        bits = cube.to_binary()
        self.assertEqual(len(bits), 64)
        self.assertTrue(set(bits).issubset({"0", "1"}))
        self.assertEqual(int(bits, 2), cube.state)

    def test_get_corners_returns_two_dimensional_integer_array(self):
        cube = Cube2x2()

        corners = cube.get_corners()

        self.assertIsInstance(corners, list)
        self.assertEqual(len(corners), 8)
        self.assertEqual(corners, [(i, 0) for i in range(8)])

        for row in corners:
            self.assertIsInstance(row, tuple)
            self.assertEqual(len(row), 2)
            self.assertTrue(all(isinstance(value, int) for value in row))

    def test_is_solved_tracks_state_changes(self):
        cube = Cube2x2()

        self.assertTrue(cube.is_solved())

        cube.do_f_move()
        self.assertFalse(cube.is_solved())

        cube.do_f_prime_move()
        self.assertTrue(cube.is_solved())

    def test_move_followed_by_inverse_restores_state(self):
        for move, inv in MOVE_PAIRS:
            with self.subTest(move=move, inv=inv):
                cube = Cube2x2()
                before = cube.state

                getattr(cube, move)()
                getattr(cube, inv)()

                self.assertEqual(cube.state, before)

    def test_four_quarter_turns_restore_state(self):
        for move, _ in MOVE_PAIRS:
            with self.subTest(move=move):
                cube = Cube2x2()
                before = cube.state

                for _ in range(4):
                    getattr(cube, move)()

                self.assertEqual(cube.state, before)

    def test_scramble_and_exact_inverse_restore_state(self):
        cube = Cube2x2()
        before = cube.state

        sequence = [
            "do_r_move",
            "do_u_move",
            "do_f_move",
            "do_l_prime_move",
            "do_d_move",
            "do_b_prime_move",
        ]

        for step in sequence:
            getattr(cube, step)()

        for step in reversed(sequence):
            getattr(cube, INVERSE_OF[step])()

        self.assertEqual(cube.state, before)

    def test_do_moves_matches_explicit_execution_and_ignores_unknown_tokens(self):
        parsed = Cube2x2()
        explicit = Cube2x2()

        parsed.do_moves("U R2 F! INVALID_TOKEN B' D2")

        explicit.do_u_move()
        explicit.do_r_move()
        explicit.do_r_move()
        explicit.do_f_prime_move()
        explicit.do_b_prime_move()
        explicit.do_d_move()
        explicit.do_d_move()

        self.assertEqual(parsed.state, explicit.state)

    def test_get_next_states_returns_qtm_successors(self):
        cubes = [Cube2x2(), Cube2x2()]

        states = rapidcube.get_next_states(cubes)

        self.assertEqual(states.shape, (len(cubes) * 12, 24))
        self.assertTrue(hasattr(states, "dtype"))

    def test_cube_batch_stores_mixed_cube_objects(self):
        batch = CubeBatch([Cube2x2(), rapidcube.Cube3x3()])

        self.assertEqual(batch.len(), 2)
        self.assertEqual(len(batch), 2)
        self.assertFalse(batch.is_empty())
        self.assertEqual(len(batch.cubes), 2)
        self.assertIsInstance(batch.cubes[0], Cube2x2)
        self.assertIsInstance(batch.cubes[1], rapidcube.Cube3x3)

    def test_cube_batch_supports_indexing(self):
        first = Cube2x2()
        second = rapidcube.Cube3x3()
        batch = CubeBatch([first, second])

        self.assertIs(batch[0], first)
        self.assertIs(batch[1], second)
        self.assertIs(batch[-1], second)

        with self.assertRaises(IndexError):
            _ = batch[2]

    def test_cube_batch_from_count_constructor(self):
        b2 = CubeBatch.from_count(3, "2x2")
        self.assertEqual(len(b2), 3)
        for i in range(3):
            self.assertIsInstance(b2[i], Cube2x2)

        b3 = CubeBatch.from_count(2, "3x3")
        self.assertEqual(len(b3), 2)
        for i in range(2):
            self.assertIsInstance(b3[i], rapidcube.Cube3x3)

    def test_str_returns_multiline_colored_net(self):
        cube = Cube2x2()
        text = str(cube)

        self.assertIsInstance(text, str)
        self.assertIn("\x1b[", text)
        self.assertEqual(len(text.splitlines()), 6)


if __name__ == "__main__":
    unittest.main()
