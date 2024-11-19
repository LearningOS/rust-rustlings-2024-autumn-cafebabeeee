/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/
use std::cmp::{self, Ordering};
use std::fmt::Debug;
use std::mem;
use std::ops::Not;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Clone, Copy)]
enum Side {
    Left,
    Right,
}

impl Not for Side {
    type Output = Side;

    fn not(self) -> Self::Output {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left
        }
    }
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
     
    fn child(&self, side: Side) -> &Option<Box<Self>> {
        match side {
            Side::Left => &self.left,
            Side::Right => &self.right
        }
    }

    fn child_mut(&mut self, side: Side) -> &mut Option<Box<Self>> {
        match side {
            Side::Left => &mut self.left,
            Side::Right => &mut self.right
        }
    }

    fn balance_factor(&self) -> isize {
        (self.height(Side::Right) - self.height(Side::Left)) as isize
    }
     
    fn rotate(&mut self, side: Side) {  
        let mut sub_tree = self.child_mut(!side)
            .take()
            .unwrap();
        *self.child_mut(!side) = sub_tree.child_mut(side).take();
        mem::swap(self, sub_tree.as_mut());
        *self.child_mut(side) = Some(sub_tree)
    }

    fn height(&self, side: Side) -> usize {
        self.child(side)
            .as_ref()
            .map_or_else(|| 0, |e| e.height(side))
    }

    fn update_height(&self) -> usize{
        cmp::max(
        self.height(Side::Left), 
        self.height(Side::Right)
        ) + 1 
    }

    fn rebalance(&mut self) {
        let side = match self.balance_factor() {
            2 => Side::Right,
            -2 => Side::Left,
            _ => return
        };

        let sub_tree = self.child_mut(side).as_mut().unwrap();    

        if let (Side::Left, 1) | (Side::Right, -1) = (side, sub_tree.balance_factor()) {
            sub_tree.rotate(side);
        }    

        self.rotate(!side);

    }

    fn search(&self, value: T) -> bool {
        match value.cmp(&self.value) {
            Ordering::Equal => true,
            Ordering::Less => self.left.as_ref().is_some_and(|f| f.search(value)),
            Ordering::Greater => self.right.as_ref().is_some_and(|f| f.search(value))
        }
    }

}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        Self { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        match self.root {
            None => self.root = Some(Box::new(TreeNode::new(value))),
            Some(ref mut node) => node.insert(value)
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        match self.root {
            None => false,
            Some(ref node) => node.search(value)
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        match value.cmp(&self.value) {
            Ordering::Equal => return,
            Ordering::Greater => {
                match self.right {
                    None => self.right = Some(Box::new(TreeNode::new(value))),
                    Some(ref mut right) => right.insert(value)
                }
            },
            Ordering::Less => {
                match self.left {
                    None => self.left = Some(Box::new(TreeNode::new(value))),
                    Some(ref mut left) => left.insert(value)
                }
            }
        };
        self.rebalance();
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

        println!("### {:?}", bst);
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_height() {
        let mut bst = BinarySearchTree::new();
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);
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
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    
