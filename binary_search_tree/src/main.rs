pub mod binary_search_tree;

fn main() {
    let root = binary_search_tree::Node {
        key: 10,
        value: "Root Node".to_string(),
        left: None,
        right: None,
    };

    let mut bst = binary_search_tree::BinarySearchTree::new();

    bst.insert(Box::new(root));

    let node_1 = binary_search_tree::Node {
        key: 12,
        value: "Node 1".to_string(),
        left: None,
        right: None,
    };

    bst.insert(Box::new(node_1));

    let node_2 = binary_search_tree::Node {
        key: 19,
        value: "Node 2".to_string(),
        left: None,
        right: None,
    };

    bst.insert(Box::new(node_2));

    let node_3 = binary_search_tree::Node {
        key: 6,
        value: "Node 3".to_string(),
        left: None,
        right: None,
    };

    bst.insert(Box::new(node_3));

    let node_4 = binary_search_tree::Node {
        key: 2,
        value: "Node 4".to_string(),
        left: None,
        right: None,
    };

    bst.insert(Box::new(node_4));

    let node_5 = binary_search_tree::Node {
        key: 11,
        value: "Node 5".to_string(),
        left: None,
        right: None,
    };

    bst.insert(Box::new(node_5));

    bst.print();
}
