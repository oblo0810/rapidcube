import unittest

import rapidcube

CubeBatch = rapidcube.CubeBatch
Cube2x2 = rapidcube.Cube2x2
Cube3x3 = rapidcube.Cube3x3


class TestCubeBatch(unittest.TestCase):
    def test_create_batch_2x2(self):
        batch = CubeBatch(4, 2)
        self.assertEqual(len(batch), 4)
        for i in range(len(batch)):
            self.assertIsInstance(batch[i], Cube2x2)

    def test_create_batch_3x3(self):
        batch = CubeBatch(4, 3)
        self.assertEqual(len(batch), 4)
        for i in range(len(batch)):
            self.assertIsInstance(batch[i], Cube3x3)

    def test_empty_batch(self):
        batch = CubeBatch(0, 2)
        self.assertEqual(len(batch), 0)
        self.assertEqual(list(batch), [])

    def test_invalid_cube_type(self):
        with self.assertRaises(TypeError):
            CubeBatch(5, 4)

    def test_cube_batch_supports_indexing(self):
        batch = CubeBatch(2, 2)

        self.assertIsInstance(batch[0], Cube2x2)
        self.assertIsInstance(batch[1], Cube2x2)

        with self.assertRaises(IndexError):
            _ = batch[2]

        with self.assertRaises(IndexError):
            _ = batch[-3]

        b3 = CubeBatch(2, 3)
        self.assertEqual(len(b3), 2)
        for i in range(2):
            self.assertIsInstance(b3[i], Cube3x3)

    def test_cube_batch_iteration(self):
        batch = CubeBatch(3, 2)
        items = list(batch)
        self.assertEqual(len(items), 3)
        for item in items:
            self.assertIsInstance(item, Cube2x2)
