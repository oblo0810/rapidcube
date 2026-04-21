import unittest

import rapidcube

Cube3x3 = rapidcube.Cube3x3


class TestCube3x3StateArrays(unittest.TestCase):
    def test_get_corners_returns_two_dimensional_integer_array(self):
        cube = Cube3x3()

        corners = cube.get_corners()

        self.assertIsInstance(corners, list)
        self.assertEqual(len(corners), 8)
        self.assertEqual(corners, [(i, 0) for i in range(8)])

        for row in corners:
            self.assertIsInstance(row, tuple)
            self.assertEqual(len(row), 2)
            self.assertTrue(all(isinstance(value, int) for value in row))

    def test_get_edges_returns_two_dimensional_integer_array(self):
        cube = Cube3x3()

        edges = cube.get_edges()

        self.assertIsInstance(edges, list)
        self.assertEqual(len(edges), 12)
        self.assertEqual(edges, [(i, 0) for i in range(12)])

        for row in edges:
            self.assertIsInstance(row, tuple)
            self.assertEqual(len(row), 2)
            self.assertTrue(all(isinstance(value, int) for value in row))


if __name__ == "__main__":
    unittest.main()
