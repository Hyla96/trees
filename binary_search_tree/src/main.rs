pub mod binary_search_tree;

const NODES: [(i64, &str); 11] = [
    (10, "Root Node"),
    (12, "Node 1"),
    (19, "Node 2"),
    (6, "Node 3"),
    (2, "Node 4"),
    (11, "Node 5"),
    (7, "Node 6"),
    (17, "Node 7"),
    (20, "Node 8"),
    (25, "Node 9"),
    (22, "Node 10"),
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

    println!("Deleting key 6");
    bst.delete(6);
    bst.print_tree();

    println!("Deleting key 19");
    bst.delete(19);
    bst.print_tree();

    println!("Deleting key 20");
    bst.delete(20);
    bst.print_tree();

    println!("Deleting key 10");
    bst.delete(10);
    bst.print_tree();
}
