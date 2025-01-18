use std::mem;

type BTreeNodeRef = Box<BTreeNode>;

struct BTreeNode {
    is_leaf: bool,
    keys: Vec<i32>,
    children: Vec<BTreeNodeRef>,
}

struct BTree {
    root: Option<BTreeNodeRef>,
}

const MAX_DEGREE: usize = 5;

impl BTreeNode {
    pub fn new(is_leaf: bool) -> BTreeNodeRef {
        Box::new(Self {
            is_leaf,
            keys: Vec::new(),
            children: Vec::new(),
        })
    }

    pub fn new_with_val(key: i32, is_leaf: bool) -> BTreeNodeRef {
        Box::new(Self {
            is_leaf,
            keys: vec![key],
            children: Vec::new(),
        })
    }
}

impl BTree {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, key: i32) {
        if self.root.is_none() {
            let new_root = BTreeNode::new_with_val(key, true);
            self.root = Some(new_root);
            return;
        }

        let root = self.root.as_mut().unwrap();

        if root.keys.len() == MAX_DEGREE - 1 {
            //println!("{:?}", root.keys);
            let mut new_root = BTreeNode::new(false);
            new_root
                .children
                .push(mem::replace(&mut self.root, None).unwrap());

            Self::split_node(&mut new_root, 0);
            self.root = Some(new_root);
        }

        Self::insert_not_full(self.root.as_mut().unwrap(), key);
    }

    pub fn display(&self) {
        self.traverse(self.root.as_ref());
    }

    pub fn search(&self, key: i32) -> bool {
        Self::search_helper(self.root.as_ref(), key)
    }

    fn split_node(parent: &mut BTreeNodeRef, index: usize) {
        let child = &mut parent.children[index];
        let mut new_node = BTreeNode::new(child.is_leaf);

        let mid = MAX_DEGREE / 2;

        while child.keys.len() > mid + 1 {
            let key = child.keys.pop().unwrap();
            new_node.keys.insert(0, key);
        }

        if !child.is_leaf {
            while child.children.len() > mid + 1 {
                let c = child.children.pop().unwrap();
                new_node.children.insert(0, c);
            }
        }

        let mid_key = child.keys.pop().unwrap();
        parent.keys.insert(index, mid_key);
        parent.children.insert(index + 1, new_node);
    }

    fn insert_not_full(node: &mut BTreeNodeRef, key: i32) {
        let mut start = 0;
        let mut end = node.keys.len();

        while start < end {
            let mid = (start + end) / 2;
            if key < node.keys[mid] {
                end = mid;
            } else if key > node.keys[mid] {
                start = mid + 1;
            } else {
                println!("{} => {:?}", key, node.keys);
                return;
            }
        }

        if node.is_leaf {
            node.keys.insert(start, key);
            return;
        }

        // otherwise start is now the index of the correct child

        if node.children[start].keys.len() == MAX_DEGREE - 1 {
            Self::split_node(node, start);

            if node.keys[start] < key {
                start += 1;
            }
        }
        Self::insert_not_full(&mut node.children[start], key);
    }

    fn traverse(&self, node: Option<&BTreeNodeRef>) {
        if let Some(node) = node {
            println!("{:?}", node.keys);
            for child in &node.children {
                self.traverse(Some(child));
            }
        }
    }

    fn search_helper(node: Option<&BTreeNodeRef>, key: i32) -> bool {
        if node.is_none() {
            return false;
        }

        let curr_node = node.unwrap().as_ref();
        let mut start = 0;
        let mut end = curr_node.keys.len();

        while start < end {
            let mid = (start + end) / 2;

            if curr_node.keys[mid] == key {
                return true;
            }
            if key < curr_node.keys[mid] {
                end = mid;
            } else if key > curr_node.keys[mid] {
                start = mid + 1;
            }
        }

        if curr_node.is_leaf {
            return false;
        }

        Self::search_helper(Some(&curr_node.children[start]), key)
    }
}

fn main() {
    let mut btree = BTree::new();
    btree.insert(10);
    btree.insert(20);
    btree.insert(5);
    btree.insert(6);
    btree.insert(110);
    btree.insert(12);
    btree.insert(30);
    btree.insert(60);
    btree.insert(80);
    btree.insert(90);
    btree.insert(100);
    btree.insert(111);
    btree.insert(445);
    btree.insert(3);
    btree.insert(89);
    btree.insert(29);
    btree.insert(61);
    btree.insert(68);

    btree.display();
    println!();

    println!("search for 10 : {}", btree.search(10));
    println!("search for 80 : {}", btree.search(80));
    println!("search for 110 : {}", btree.search(110));
    println!("search for 68 : {}", btree.search(68));
    println!("search for 10001 : {}", btree.search(10001));
}
