pub mod binary_search_tree;

const NODES: [(i64, &str); 6] = [
    (10, "Root Node"),
    (12, "Node 1"),
    (19, "Node 2"),
    (6, "Node 3"),
    (2, "Node 4"),
    (11, "Node 5"),
];

fn main() {
    let mut bst = binary_search_tree::BinarySearchTree::new();

    for (key, value) in NODES.iter() {
        bst.insert(Box::new(binary_search_tree::Node {
            key: *key,
            left: None,
            right: None,
            value: value.to_string(),
        }));
    }

    bst.print_tree();

    const NODES_TO_SEARCH: [i64; 8] = [10, 0, 12, 19, 6, 28, 2, 11];

    for key in NODES_TO_SEARCH.iter() {
        let node = bst.search(*key);
        if let Some(ref node) = node {
            node.print();
        } else {
            println!("Node with key {} not found", key);
        }
    }

    bst.delete(19);

    bst.print_tree();
}
