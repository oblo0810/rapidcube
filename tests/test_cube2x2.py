import unittest

import fastcube

Cube2x2 = fastcube.Cube2x2


MOVE_PAIRS = [
    ("do_u_move", "do_u_prime_move"),
    ("do_d_move", "do_d_prime_move"),
    ("do_r_move", "do_r_prime_move"),
    ("do_l_move", "do_l_prime_move"),
    ("do_f_move", "do_f_prime_move"),
    ("do_b_move", "do_b_prime_move"),
]


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

        inverse_of = {
            "do_u_move": "do_u_prime_move",
            "do_u_prime_move": "do_u_move",
            "do_d_move": "do_d_prime_move",
            "do_d_prime_move": "do_d_move",
            "do_r_move": "do_r_prime_move",
            "do_r_prime_move": "do_r_move",
            "do_l_move": "do_l_prime_move",
            "do_l_prime_move": "do_l_move",
            "do_f_move": "do_f_prime_move",
            "do_f_prime_move": "do_f_move",
            "do_b_move": "do_b_prime_move",
            "do_b_prime_move": "do_b_move",
        }

        for step in sequence:
            getattr(cube, step)()

        for step in reversed(sequence):
            getattr(cube, inverse_of[step])()

        self.assertEqual(cube.state, before)

    def test_str_returns_multiline_colored_net(self):
        cube = Cube2x2()
        text = str(cube)

        self.assertIsInstance(text, str)
        self.assertIn("\x1b[", text)
        self.assertEqual(len(text.splitlines()), 6)


if __name__ == "__main__":
    unittest.main()
