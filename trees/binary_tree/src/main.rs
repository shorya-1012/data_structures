use std::{cell::RefCell, rc::Rc};

type TreeNodeRef = Option<Rc<RefCell<TreeNode>>>;

struct TreeNode {
    val: i32,
    left: TreeNodeRef,
    right: TreeNodeRef,
}

struct BinaryTree {
    root: TreeNodeRef,
}

impl TreeNode {
    fn new(val: i32) -> TreeNodeRef {
        Some(Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: None,
        })))
    }
}

impl BinaryTree {
    fn new() -> Self {
        Self { root: None }
    }

    fn insert(&mut self, val: i32) {
        self.root = BinaryTree::insert_helper(&mut self.root, val);
    }

    fn insert_helper(root: &TreeNodeRef, val: i32) -> TreeNodeRef {
        match root {
            Some(node) => {
                let mut node_borrow = node.borrow_mut();
                if val < node_borrow.val {
                    node_borrow.left = BinaryTree::insert_helper(&mut node_borrow.left, val);
                } else if val > node_borrow.val {
                    node_borrow.right = BinaryTree::insert_helper(&mut node_borrow.right, val);
                }
                root.clone()
            }
            None => TreeNode::new(val),
        }
    }

    fn preorder_traversal(&self) {
        Self::preorder_traversal_helper(&self.root);
    }

    fn inorder_traversal(&self) {
        Self::inorder_traversal_helper(&self.root);
    }

    fn postorder_traversal(&self) {
        Self::postorder_traversal_helper(&self.root);
    }

    fn preorder_traversal_helper(root: &TreeNodeRef) {
        if let Some(node) = root {
            let borrow_node = node.borrow();
            println!("{}", borrow_node.val);
            Self::preorder_traversal_helper(&borrow_node.left);
            Self::preorder_traversal_helper(&borrow_node.right);
        }
    }

    fn inorder_traversal_helper(root: &TreeNodeRef) {
        if let Some(node) = root {
            let borrow_node = node.borrow();
            Self::inorder_traversal_helper(&borrow_node.left);
            println!("{}", borrow_node.val);
            Self::inorder_traversal_helper(&borrow_node.right);
        }
    }

    fn postorder_traversal_helper(root: &TreeNodeRef) {
        if let Some(node) = root {
            let borrow_node = node.borrow();
            Self::postorder_traversal_helper(&borrow_node.left);
            Self::postorder_traversal_helper(&borrow_node.right);
            println!("{}", borrow_node.val);
        }
    }
}

fn main() {
    let mut tree = BinaryTree::new();
    tree.insert(11);
    tree.insert(9);
    tree.insert(8);
    tree.insert(10);
    tree.insert(13);

    tree.preorder_traversal();
    println!("-----------------");
    tree.inorder_traversal();
    println!("-----------------");
    tree.postorder_traversal();
}
