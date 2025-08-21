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

    pub fn insert(&mut self, node: Box<Node>) {
        if self.root.is_none() {
            self.root = Some(node);
            return;
        }
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

fn print_node(node: &Box<Node>) {
    println!("{node:?}!");
}
