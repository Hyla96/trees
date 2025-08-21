pub mod binary_search_tree;

fn main() {
    let text = "Binary Search Tree Example";
    println!("{}", text);
    let left = binary_search_tree::Node {
        key: 12,
        value: "Left Node".to_string(),
        left: None,
        right: None,
    };

    let right = binary_search_tree::Node {
        key: 19,
        value: "Right Node".to_string(),
        left: None,
        right: None,
    };

    let root = binary_search_tree::Node {
        key: 10,
        value: "Root Node".to_string(),
        left: Some(Box::new(left)),
        right: Some(Box::new(right)),
    };

    let mut bst = binary_search_tree::BinarySearchTree::new();

    bst.insert(Box::new(root));

    bst.print();
}
