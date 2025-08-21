#[derive(Debug)]
pub struct Node {
    pub key: i64,
    pub value: String,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
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

    pub fn delete(&mut self, key: i64) {}

    pub fn search(&self, key: i64) -> Option<Box<Node>> {
        let node = Node {
            key: key,
            value: "".to_string(),
            left: None,
            right: None,
        };
        return Some(Box::new(node));
    }

    pub fn print(&self) {
        if let Some(ref root) = self.root {
            print_node(root)
        }
    }
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

fn print_node(node: &Box<Node>) {
    println!("{node:?}!");
}
