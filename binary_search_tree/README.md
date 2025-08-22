# Binary Search Tree

A basic implementation of a binary search tree (unbalanced) in Rust.

## What this provides

- **BinarySearchTree**: Basic BST with insert, search, and delete operations
- **Node**: Tree node structure with key-value pairs
- **Performance**: O(log n) best case, O(n) worst case (unbalanced trees)

## Usage

```rust
use binary_search_tree::{BinarySearchTree, Node};

let mut bst = BinarySearchTree::new();
bst.insert(Box::new(Node {
    key: 10,
    value: "Root".to_string(),
    left: None,
    right: None,
}));

let result = bst.search(10);
bst.delete(10);
```

## Commands

- `cargo run` - Run example with predefined nodes
- `cargo test` - Run unit tests
- `cargo bench` - Run performance benchmarks

## Note

This is a textbook BST implementation without self-balancing. Tree performance depends on insertion order.

