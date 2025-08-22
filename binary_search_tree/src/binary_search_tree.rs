#[derive(Debug, Clone)]
pub struct Node {
    pub key: i64,
    pub value: String,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    pub fn print(&self) {
        println!("Key: {} | Value: {}", self.key, self.value)
    }
}

pub struct BinarySearchTree {
    pub length: usize,
    pub root: Option<Box<Node>>,
}

impl BinarySearchTree {
    pub fn new() -> BinarySearchTree {
        return BinarySearchTree {
            length: 0,
            root: None,
        };
    }

    pub fn insert(&mut self, node: Box<Node>) -> bool {
        if let Some(ref mut root) = self.root {
            return insert_node(root, node);
        }

        self.root = Some(node);
        return false;
    }

    pub fn delete(&mut self, key: i64) -> bool {
        return delete(&mut self.root, key);
    }

    pub fn search(&self, key: i64) -> Option<Box<Node>> {
        if let Some(ref root) = self.root {
            return search(root, key);
        }

        return None;
    }

    pub fn print_tree(&self) {
        println!("");
        println!("{}", "-".repeat(50));
        println!("");
        println!("Tree structure:");
        if let Some(ref root) = self.root {
            print_tree_structure(root, "", true);
        }
        println!("");
        println!("{}", "-".repeat(50));
        println!("");
    }
}

fn delete(n: &mut Option<Box<Node>>, key: i64) -> bool {
    if let Some(node) = n {
        if node.key == key {
            if node.left.is_none() && node.right.is_none() {
                *n = None;
            } else if node.left.is_none() {
                *n = node.right.take();
            } else if node.right.is_none() {
                *n = node.left.take();
            } else {
                // Find inorder successor (leftmost node in right subtree)
                if let Some(ref mut right_subtree) = node.right {
                    let mut pivot = right_subtree.as_mut();
                    while pivot.left.is_some() {
                        pivot = pivot.left.as_mut().unwrap();
                    }

                    let successor_key = pivot.key;
                    let successor_value = pivot.value.clone();

                    node.key = successor_key;
                    node.value = successor_value;

                    delete(&mut node.right, successor_key);
                }
            }
            return true;
        }

        if key > node.key {
            return delete(&mut node.right, key);
        }

        return delete(&mut node.left, key);
    }
    return false;
}

// Returns only a copy
fn search(node: &Box<Node>, key: i64) -> Option<Box<Node>> {
    if node.key == key {
        return Some(node.clone());
    }

    if let Some(ref right_node) = node.right {
        let right = search(right_node, key);
        if !right.is_none() {
            return right;
        }
    }

    if let Some(ref left_node) = node.left {
        let left = search(left_node, key);
        if !left.is_none() {
            return left;
        }
    }

    return None;
}

fn insert_node(root: &mut Box<Node>, node: Box<Node>) -> bool {
    if root.key == node.key {
        return true;
    }

    if node.key > root.key {
        if let Some(ref mut right) = root.right {
            return insert_node(right, node);
        }
        root.right = Some(node);
        return false;
    }

    if let Some(ref mut left) = root.left {
        return insert_node(left, node);
    }
    root.left = Some(node);
    return false;
}

fn print_tree_structure(node: &Box<Node>, prefix: &str, is_last: bool) {
    println!(
        "{}{}{}:{}",
        prefix,
        if is_last { "└── " } else { "├── " },
        node.key,
        node.value
    );

    let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });

    match (&node.left, &node.right) {
        (Some(left), Some(right)) => {
            print_tree_structure(left, &new_prefix, false);
            print_tree_structure(right, &new_prefix, true);
        }
        (Some(left), None) => {
            print_tree_structure(left, &new_prefix, true);
        }
        (None, Some(right)) => {
            print_tree_structure(right, &new_prefix, true);
        }
        (None, None) => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_tree() -> BinarySearchTree {
        let mut bst = BinarySearchTree::new();
        let values = [10, 6, 15, 3, 8, 12, 20];

        for value in values {
            bst.insert(Box::new(Node {
                key: value,
                value: format!("Node {}", value),
                left: None,
                right: None,
            }));
        }
        bst
    }

    #[test]
    fn test_insert_single_node() {
        let mut bst = BinarySearchTree::new();
        bst.insert(Box::new(Node {
            key: 10,
            value: "Root".to_string(),
            left: None,
            right: None,
        }));

        assert!(bst.root.is_some());
        if let Some(ref root) = bst.root {
            assert_eq!(root.key, 10);
            assert_eq!(root.value, "Root");
        }
    }

    #[test]
    fn test_search_existing_node() {
        let bst = create_test_tree();

        let result = bst.search(15);
        assert!(result.is_some());
        if let Some(node) = result {
            assert_eq!(node.key, 15);
            assert_eq!(node.value, "Node 15");
        }
    }

    #[test]
    fn test_search_nonexistent_node() {
        let bst = create_test_tree();
        let result = bst.search(100);
        assert!(result.is_none());
    }

    #[test]
    fn test_delete_leaf_node() {
        let mut bst = create_test_tree();
        let result = bst.delete(3);
        assert!(result);

        let search_result = bst.search(3);
        assert!(search_result.is_none());
    }

    #[test]
    fn test_delete_node_with_two_children() {
        let mut bst = create_test_tree();
        let result = bst.delete(6);
        assert!(result);

        assert!(bst.search(6).is_none());
        assert!(bst.search(3).is_some());
        assert!(bst.search(8).is_some());
    }
}
