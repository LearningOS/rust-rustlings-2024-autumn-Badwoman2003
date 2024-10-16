/*
    binary_search tree
    This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;
use std::ptr::NonNull;

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord + Copy,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        match self.root.as_mut() {
            None => {
                self.root = Some(Box::new(TreeNode::new(value)));
            }
            Some(root_ptr) => unsafe {
                root_ptr.insert(value);
            },
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        match self.root.as_ref() {
            None => return false,
            Some(root_ptr) => unsafe {
                return root_ptr.search(value);
            },
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord + Copy,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        if self.value == value {
            return;
        }
        match (self.left.as_mut(), self.right.as_mut()) {
            (None, None) => {
                let node = Some(Box::new(TreeNode::new(value)));
                if value <= self.value {
                    self.left = node;
                } else {
                    self.right = node;
                }
            }
            (Some(l_node), None) => {
                if value <= self.value {
                    l_node.insert(value);
                } else {
                    let node = Some(Box::new(TreeNode::new(value)));
                    self.right = node;
                }
            }
            (None, Some(r_node)) => {
                if value <= self.value {
                    let node = Some(Box::new(TreeNode::new(value)));
                    self.left = node;
                } else {
                    r_node.insert(value);
                }
            }
            (Some(l_node), Some(r_node)) => {
                if value <= self.value {
                    l_node.insert(value);
                } else {
                    r_node.insert(value);
                }
            }
        }
    }
    fn search(&self, value: T) -> bool {
        if self.value == value {
            return true;
        }

        match (&self.left, &self.right) {
            (None, None) => false,
            (Some(l_node), None) => {
                if value <= self.value {
                    l_node.search(value)
                } else {
                    false
                }
            }
            (None, Some(r_node)) => {
                if value <= self.value {
                    false
                } else {
                    r_node.search(value)
                }
            }
            (Some(l_node), Some(r_node)) => {
                if value <= self.value {
                    l_node.search(value)
                } else {
                    r_node.search(value)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        assert_eq!(bst.search(1), false);

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        bst.insert(1);
        bst.insert(1);

        assert_eq!(bst.search(1), true);

        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            }
            None => panic!("Root should not be None after insertion"),
        }
    }
}
