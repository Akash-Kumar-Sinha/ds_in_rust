use std::{cell::RefCell, collections::VecDeque, rc::Rc};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

pub struct Tree<T> {
    root: Link<T>,
}

#[derive(PartialEq, PartialOrd)]
pub struct Node<T> {
    elem: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T: PartialOrd> Node<T> {
    pub fn new(elem: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            elem,
            left: None,
            right: None,
        }))
    }
}

impl<T: PartialOrd + Clone> Tree<T> {
    pub fn new() -> Self {
        Tree { root: None }
    }

    pub fn insertion(&mut self, elem: T) {
        Self::insert(&mut self.root, elem);
    }

    fn insert(link: &mut Link<T>, elem: T) {
        match link {
            Some(node) => {
                if elem < node.borrow().elem {
                    Self::insert(&mut node.borrow_mut().left, elem);
                } else if elem > node.borrow().elem {
                    Self::insert(&mut node.borrow_mut().right, elem);
                }
            }
            None => {
                *link = Some(Node::new(elem));
            }
        }
    }
    //  Pre Order Traversal
    pub fn pre_order_traversal(&mut self) -> Vec<T> {
        let mut result = Vec::new();
        Self::pre_order(&self.root, &mut result);
        result
    }

    fn pre_order(node: &Link<T>, result: &mut Vec<T>) {
        if let Some(node) = node {
            let val = node.borrow();
            result.push(val.elem.clone());
            Self::pre_order(&val.left, result);
            Self::pre_order(&val.right, result);
        }
    }

    // Post order Traversal
    pub fn post_order_traversal(&mut self) -> Vec<T> {
        let mut result = Vec::new();
        Self::post_order(&self.root, &mut result);
        result
    }

    fn post_order(node: &Link<T>, result: &mut Vec<T>) {
        if let Some(node) = node {
            let val = node.borrow();
            Self::post_order(&val.left, result);
            Self::post_order(&val.right, result);
            result.push(val.elem.clone());
        }
    }

    // In order Traversal
    pub fn in_order_traversal(&mut self) -> Vec<T> {
        let mut result = Vec::new();
        Self::in_order(&self.root, &mut result);
        result
    }

    fn in_order(node: &Link<T>, result: &mut Vec<T>) {
        if let Some(node) = node {
            let val = node.borrow();
            Self::in_order(&val.left, result);
            result.push(val.elem.clone());
            Self::in_order(&val.right, result);
        }
    }

    // Level Order Traversal
    pub fn level_order_traversal(&self) -> Vec<T> {
        let mut result = Vec::new();
        let mut queue = VecDeque::new();

        if let Some(root) = &self.root {
            queue.push_back(root.clone());
        }

        while let Some(node) = queue.pop_front() {
            let node = node.borrow();
            result.push(node.elem.clone());

            if let Some(left) = &node.left {
                queue.push_back(left.clone());
            }
            if let Some(right) = &node.right {
                queue.push_back(right.clone());
            }
        }

        result
    }

    pub fn height(&self) -> i32 {
        Self::tree_height(&self.root)
    }

    fn tree_height(node: &Link<T>) -> i32 {
        match node {
            Some(n) => {
                let val = n.borrow();
                let left_height = Self::tree_height(&val.left);
                let right_height = Self::tree_height(&val.right);
                1 + left_height.max(right_height)
            }
            None => -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_elem(node: &Link<i32>) -> Option<i32> {
        node.as_ref().map(|rc| rc.borrow().elem)
    }

    #[test]
    fn test_insertion_structure() {
        let mut tree = Tree::new();

        let values = vec![10, 5, 15, 20, 0, 2, 20, 8, 3, 7, 12, 18];
        for v in values {
            tree.insertion(v);
        }

        // Root checks
        let root = tree.root.clone().unwrap();
        assert_eq!(root.borrow().elem, 10);

        let root_ref = root.borrow();

        // Left Subtree
        let left = root_ref.left.clone().unwrap(); // 5
        let left_ref = left.borrow();

        assert_eq!(left_ref.elem, 5);
        assert_eq!(get_elem(&left_ref.left), Some(0));
        assert_eq!(get_elem(&left_ref.right), Some(8));

        // Check node 8 subtree
        let node_8 = left_ref.right.clone().unwrap(); // 8
        let node_8_ref = node_8.borrow();
        assert_eq!(get_elem(&node_8_ref.left), Some(7));

        // Check node 7 subtree
        let node_7 = node_8_ref.left.clone().unwrap(); // 7
        let node_7_ref = node_7.borrow();
        assert_eq!(get_elem(&node_7_ref.left), None);
        assert_eq!(get_elem(&node_7_ref.right), None);

        // Right Subtree
        let right = root_ref.right.clone().unwrap(); // 15
        let right_ref = right.borrow();
        assert_eq!(right_ref.elem, 15);
        assert_eq!(get_elem(&right_ref.left), Some(12));
        assert_eq!(get_elem(&right_ref.right), Some(20));

        // Check node 20 subtree
        let node_20 = right_ref.right.clone().unwrap(); // 20
        let node_20_ref = node_20.borrow();
        assert_eq!(get_elem(&node_20_ref.left), Some(18));
        assert_eq!(get_elem(&node_20_ref.right), None);

        // Check left subtree of 0
        let node_0 = left_ref.left.clone().unwrap(); // 0
        let node_0_ref = node_0.borrow();
        assert_eq!(get_elem(&node_0_ref.left), None);
        assert_eq!(get_elem(&node_0_ref.right), Some(2));

        // Node 2 right child
        let node_2 = node_0_ref.right.clone().unwrap(); // 2
        assert_eq!(get_elem(&node_2.borrow().right), Some(3));
    }

    #[test]
    fn post_order_traversals() {
        let mut tree = Tree::new();
        let values = vec![10, 5, 15, 20, 0, 2, 8, 3, 7, 12, 18];
        for v in values {
            tree.insertion(v);
        }
    }

    #[test]
    fn pre_order_traversals() {
        let mut tree = Tree::new();
        let values = vec![10, 5, 15, 20, 0, 2, 8, 3, 7, 12, 18];
        for v in values {
            tree.insertion(v);
        }
        let preorder_result = tree.pre_order_traversal();
        assert_eq!(preorder_result, vec![10, 5, 0, 2, 3, 8, 7, 15, 12, 20, 18]);
    }

    #[test]
    fn in_order_traversals() {
        let mut tree = Tree::new();
        let values = vec![10, 5, 15, 20, 0, 2, 8, 3, 7, 12, 18];
        for v in values {
            tree.insertion(v);
        }

        // In-order traversal: (Left, Root, Right) => should be sorted
        let inorder_result = tree.in_order_traversal();
        assert_eq!(inorder_result, vec![0, 2, 3, 5, 7, 8, 10, 12, 15, 18, 20]);
    }

    #[test]
    fn level_order_traversals() {
        let mut tree = Tree::new();
        let values = vec![10, 5, 15, 20, 0, 2, 8, 3, 7, 12, 18];
        for v in values {
            tree.insertion(v);
        }
        let level_order_result = tree.level_order_traversal();
        assert_eq!(
            level_order_result,
            vec![10, 5, 15, 0, 8, 12, 20, 2, 7, 18, 3]
        );
    }

    #[test]
    fn tree_height() {
        let mut tree = Tree::new();
        let values = vec![10, 5, 15, 20, 0, 2, 8, 3, 7, 12, 18];
        for v in values {
            tree.insertion(v);
        }

        let height = tree.height();
        assert_eq!(height, 4);
    }
}
