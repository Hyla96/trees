import unittest
from btree import BTree


class TestBTree(unittest.TestCase):
    def setUp(self):
        self.btree = BTree(degree=3)

    def test_initialization(self):
        self.assertIsNotNone(self.btree.root)
        self.assertTrue(self.btree.degree == 3)
        self.assertEqual(self.btree.degree, 3)

    def test_search(self):
        pass

    def test_insert(self):
        pass

    def test_delete(self):
        pass


if __name__ == "__main__":
    unittest.main()

