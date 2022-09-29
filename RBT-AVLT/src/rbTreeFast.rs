//! Red-black tree
//!
//! You can generate a red-black tree, and insert or delete nodes.

use std::cell::RefCell;
use std::cmp::max;
use std::fmt::{Debug, Display};
use std::rc::Rc;

pub use crate::commonTrait::{CommonTreeNodeTrait, CommonTreeTrait};

/// Color representation for the [TreeNode](struct.TreeNode.html)
/// of [FastRBTree](struct.FastRBTree.html) struct
#[derive(Clone, Debug, PartialEq)]
pub enum NodeColor {
    /// Red color
    Red,
    /// Black color, the root of [FastRBTree](struct.FastRBTree.html) will be set to Black
    Black,
}

/// Structure of FastRBTree
#[derive(Clone, Debug, PartialEq)]
pub struct FastRBTree<T: Ord + Copy + Debug + Display> {
    root: OptionFastRBTreeNode<T>,
}

/// Node struct for [FastRBTree](struct.FastRBTree.html) struct
#[derive(Clone, Debug, PartialEq)]
pub struct TreeNode<T: Ord + Copy + Debug + Display> {
    /// The color of the node
    color: NodeColor,
    /// Data stored in the node
    value: T,
    left: OptionFastRBTreeNode<T>,
    right: OptionFastRBTreeNode<T>,
}

type FastRBTreeNode<T: Ord + Copy + Debug + Display> = Rc<RefCell<TreeNode<T>>>;
type OptionFastRBTreeNode<T: Ord + Copy + Debug + Display> = Option<FastRBTreeNode<T>>;

/// Implementations of NodeColor
impl NodeColor {
    fn to_string(&self) -> &str {
        if self == &NodeColor::Red {
            "r"
        } else {
            "b"
        }
    }
}

// extend from common tree trait
impl<T: Ord + Copy + Debug + Display> CommonTreeTrait<T, TreeNode<T>> for FastRBTree<T> {
    fn get_root(&self) -> OptionFastRBTreeNode<T> {
        return self.root.clone();
    }
    // @Override
    fn height(&self) -> u32 {
        match self.get_root() {
            None => 0,
            Some(node) => node.borrow().get_height(),
        }
    }
}

// extend from common tree node trait
impl<T: Ord + Copy + Debug + Display> CommonTreeNodeTrait<T> for TreeNode<T> {
    fn get_left(&self) -> OptionFastRBTreeNode<T> {
        return self.left.clone();
    }

    fn get_right(&self) -> OptionFastRBTreeNode<T> {
        return self.right.clone();
    }

    fn get_value(&self) -> T {
        return self.value;
    }

    fn get_value_to_print(&self) -> String {
        let value = self.value.to_string();
        let color = self.color.to_string();
        return value + &color;
    }

    // @Override
    fn get_height(&self) -> u32 {
        let left = self.get_left();
        let right = self.get_right();
        let left_height = match left {
            None => 1,
            Some(l) => l.borrow().get_height(),
        };
        let right_height = match right {
            None => 1,
            Some(r) => r.borrow().get_height(),
        };
        return max(left_height, right_height) + 1;
    }

    // @override
    fn count_leaves(&self) -> u32 {
        let left = self.get_left();
        let right = self.get_right();
        if left.is_none() && right.is_none() {
            return 2;
        } else if left.is_none() && right.is_some() {
            return 1 + right.unwrap().borrow().count_leaves();
        } else if left.is_some() && right.is_none() {
            return 1 + left.unwrap().borrow().count_leaves();
        } else {
            return right.unwrap().borrow().count_leaves() + left.unwrap().borrow().count_leaves();
        }
    }
}

/// Implementations of FastRBTree
// FastRBTree
impl<T: Ord + Copy + Debug + Display> FastRBTree<T> {
    /// Create a new red-black Tree
    ///
    /// # Example
    ///
    /// ```
    /// use tree_collections::rbTreeFast::FastRBTree;
    /// let mut tree = FastRBTree::<u32>::new();
    /// ```
    pub fn new() -> Self {
        FastRBTree { root: None }
    }

    /// Insert a new value to the red-black Tree
    ///
    /// # Example
    ///
    /// ```
    /// use tree_collections::rbTreeFast::FastRBTree;
    /// let mut tree = FastRBTree::<u32>::new();
    /// tree.insert(1);
    /// ```
    pub fn insert(&mut self, insert_value: T) {
        self.root = TreeNode::node_insert(self.root.clone(), insert_value);
        self.root.clone().unwrap().borrow_mut().color = NodeColor::Black;
    }

    /// Delete a value from the red-black Tree
    ///
    /// # Example
    ///
    /// ```
    /// use tree_collections::rbTreeFast::FastRBTree;
    /// let mut tree = FastRBTree::new();
    /// tree.delete(1);
    /// ```
    pub fn delete(&mut self, delete_value: T) {
        match self.root.clone() {
            None => (),
            Some(root) => {
                if TreeNode::is_black(root.borrow().get_left())
                    && TreeNode::is_black(root.borrow().get_right())
                {
                    root.borrow_mut().color = NodeColor::Red;
                }
                self.root = TreeNode::node_delete(Some(root), delete_value);
                if self.root.is_some() {
                    self.root.clone().unwrap().borrow_mut().color = NodeColor::Black;
                }
            }
        }
    }

    pub fn pre_order_traverse(&self, node: FastRBTreeNode<T>, container: &mut Vec<T>) {
        container.push(node.borrow().value);
        let left = node.borrow().left.clone();
        if left.is_some() {
            self.pre_order_traverse(left.unwrap(), container);
        }
        let right = node.borrow().right.clone();
        if right.is_some() {
            self.pre_order_traverse(right.unwrap(), container);
        }
    }

    /// Return true if the tree match propertity 5
    /// Propertity 5. Every path from any node to all of its descendent Nil nodes
    /// has the same number of black nodes.
    pub fn is_valid_red_black_tree(root: OptionFastRBTreeNode<T>) -> bool {
        let result = TreeNode::calculate_black_height(root);
        match result {
            Some(_) => true,
            None => false,
        }
    }
}

/// Implementations of TreeNode
// TreeNode
impl<T: Ord + Copy + Debug + Display> TreeNode<T> {
    /// Create a new node
    /// ,which will be called by [FastRBTree](struct.FastRBTree.html)
    fn new(value: T) -> Self {
        TreeNode {
            color: NodeColor::Red,
            value: value,
            left: None,
            right: None,
        }
    }

    /// Insert data into the subtree, performs any rotations
    /// necessary to maintain banlance, and then returns the new root to this subtree
    /// , which will be called by
    /// [FastRBTree.insert](struct.FastRBTree.html#method.insert)
    fn node_insert(node: OptionFastRBTreeNode<T>, insert_value: T) -> OptionFastRBTreeNode<T> {
        // if h is none, then return the first node
        match node {
            None => return Some(Rc::new(RefCell::new(TreeNode::new(insert_value)))),
            Some(n) => {
                // compare with root
                let node_value = n.borrow().value;
                if insert_value < node_value {
                    // insert to left
                    let left = n.borrow().left.clone();
                    n.borrow_mut().left = Self::node_insert(left, insert_value);
                } else if insert_value > node_value {
                    // insert to right
                    let right = n.borrow().right.clone();
                    n.borrow_mut().right = Self::node_insert(right, insert_value);
                } else {
                    // insert here
                    n.borrow_mut().value = insert_value; // equal, update value
                }
                // maintain the tree by the RB tree rule
                return Self::maintain(n.clone());
            }
        }
    }

    /// Delete data from the subtree, performs any rotations
    /// necessary to maintain banlance, and then returns the new root to this subtree
    /// , which will be called by
    /// [FastRBTree.delete](struct.FastRBTree.html#method.delete)
    fn node_delete(node: OptionFastRBTreeNode<T>, delete_value: T) -> OptionFastRBTreeNode<T> {
        match node {
            None => return None,
            Some(mut n) => {
                let node_value = n.borrow().value;
                if delete_value < node_value {
                    if Self::is_black(n.borrow().get_left())
                        && Self::is_black(
                            n.borrow().get_left().unwrap().clone().borrow().get_left(),
                        )
                    {
                        n = Self::move_red_left(Some(n.clone()));
                    }
                    let left = n.borrow().left.clone().take();
                    n.borrow_mut().left = Self::node_delete(left, delete_value);
                } else {
                    if Self::is_red(n.borrow().get_left()) {
                        n = Self::right_rotate(n);
                    }
                    if delete_value == node_value && n.borrow().get_right() == None {
                        return None;
                    }
                    if Self::is_black(n.borrow().get_right())
                        && Self::is_black(
                            n.borrow().get_right().unwrap().clone().borrow().get_left(),
                        )
                    {
                        n = Self::move_red_right(Some(n.clone()));
                    }
                    if delete_value == node_value {
                        let right = n.borrow().right.clone().unwrap();
                        let min_value = right.clone().borrow().get_min_value_in_children(); // Find the value of node A which is the minimum value of the right subtree
                        n.borrow_mut().value = min_value; // Change the value of node n to the value of node A.
                        let right = n.borrow().right.clone().take();
                        n.borrow_mut().right = Self::node_delete(right, min_value);
                    // Delete the node A in the right subtree.
                    } else {
                        let right = n.borrow().right.clone();
                        n.borrow_mut().right = Self::node_delete(right, delete_value);
                    }
                }
                return Self::maintain(n.clone());
            }
        };
    }

    /// Repair the coloring from inserting or deleting into a tree.
    fn maintain(node: FastRBTreeNode<T>) -> OptionFastRBTreeNode<T> {
        // if right is red and left is black, then left rotate
        if Self::is_red(node.borrow().get_right()) && Self::is_black(node.borrow().get_left()) {
            let temp1 = Self::left_rotate(node.clone());
            // if left and left's left are both red, then right rotate
            if Self::is_red(temp1.borrow().get_left())
                && Self::is_red(temp1.borrow().get_left().unwrap().borrow().get_left())
            {
                let temp2 = Self::right_rotate(temp1.clone());
                // if left and right are both red, then change color
                if Self::is_red(temp2.borrow().get_left())
                    && Self::is_red(temp2.borrow().get_right())
                {
                    Self::flip_color(temp2.clone());
                }
                return Some(temp2);
            }
            return Some(temp1);
        }

        // if left and left's left are both red, then right rotate
        if Self::is_red(node.borrow().get_left())
            && Self::is_red(node.borrow().get_left().unwrap().borrow().get_left())
        {
            let temp1 = Self::right_rotate(node.clone());
            // if left and right are both red, then change color
            if Self::is_red(temp1.borrow().get_left()) && Self::is_red(temp1.borrow().get_right()) {
                Self::flip_color(temp1.clone());
            }
            return Some(temp1);
        }

        // if left and right are both red, then change color
        if Self::is_red(node.borrow().get_left()) && Self::is_red(node.borrow().get_right()) {
            Self::flip_color(node.clone());
        }

        return Some(node);
    }

    fn move_red_left(node: OptionFastRBTreeNode<T>) -> FastRBTreeNode<T> {
        let mut n = node.unwrap().clone();
        Self::flip_color(n.clone());
        if Self::is_red(
            n.clone()
                .borrow()
                .get_right()
                .unwrap()
                .clone()
                .borrow()
                .get_left(),
        ) {
            let right = n.borrow().right.clone();
            n.borrow_mut().right = Some(Self::right_rotate(right.unwrap()));
            n = Self::left_rotate(n.clone());
            Self::flip_color(n.clone());
        }

        return n.clone();
    }

    fn move_red_right(node: OptionFastRBTreeNode<T>) -> FastRBTreeNode<T> {
        let mut n = node.unwrap().clone();
        Self::flip_color(n.clone());
        if Self::is_red(
            n.clone()
                .borrow()
                .get_left()
                .unwrap()
                .clone()
                .borrow()
                .get_left(),
        ) {
            n = Self::right_rotate(n.clone());
            Self::flip_color(n.clone());
        }

        return n.clone();
    }

    // left and right rotate
    // node is the root of the subtree

    /// Rotate the subtree rooted at this node to the left and
    /// return the new root to this subtree.
    fn left_rotate(node: FastRBTreeNode<T>) -> FastRBTreeNode<T> {
        let node_right = node.borrow().right.clone().unwrap();
        let temp = node_right.borrow().left.clone().take();
        // left rotate
        node_right.borrow_mut().left = Some(node.clone());
        node.borrow_mut().right = temp;
        node_right.borrow_mut().color = node.borrow().color.clone();
        node.borrow_mut().color = NodeColor::Red;
        return node_right;
    }

    /// Rotate the subtree rooted at this node to the right and
    /// returns the new root to this subtree.
    fn right_rotate(node: FastRBTreeNode<T>) -> FastRBTreeNode<T> {
        let node_left = node.borrow().left.clone().unwrap();
        let temp = node_left.borrow().right.clone().take();
        // right rotate
        node_left.borrow_mut().right = Some(node.clone());
        node.borrow_mut().left = temp;
        node_left.borrow_mut().color = node.borrow().color.clone();
        node.borrow_mut().color = NodeColor::Red;
        return node_left;
    }

    fn is_red(node: OptionFastRBTreeNode<T>) -> bool {
        return Self::get_color(node) == NodeColor::Red;
    }

    fn is_black(node: OptionFastRBTreeNode<T>) -> bool {
        return Self::get_color(node) == NodeColor::Black;
    }

    // Helper function for maintaining
    fn flip_color(node: FastRBTreeNode<T>) {
        node.borrow_mut().left.clone().unwrap().borrow_mut().color = NodeColor::Black;
        node.borrow_mut().right.clone().unwrap().borrow_mut().color = NodeColor::Black;
        node.borrow_mut().color = NodeColor::Red;
    }

    // Helper function for maintaining
    // make None to be real leaves with black color
    fn get_color(node: OptionFastRBTreeNode<T>) -> NodeColor {
        match node {
            None => NodeColor::Black,
            Some(node) => node.borrow().color.clone(),
        }
    }

    // Helper function for FastRBTree::is_valid_red_black_tree
    fn calculate_black_height(node: OptionFastRBTreeNode<T>) -> Option<usize> {
        match node {
            None => Some(1),
            Some(node) => {
                let left_height = Self::calculate_black_height(node.borrow().left.clone());
                let right_height = Self::calculate_black_height(node.borrow().right.clone());
                match (left_height, right_height) {
                    (Some(left_height), Some(right_height)) => {
                        if left_height != right_height {
                            //The 2 children have unequal depths
                            None
                        } else {
                            let node_color = &node.borrow().color;
                            //Return the black depth of children,plus 1 if the node is black
                            match node_color {
                                NodeColor::Red => Some(left_height),
                                NodeColor::Black => Some(left_height + 1),
                            }
                        }
                    }
                    _ => None,
                }
            }
        }
    }

    fn clear(&mut self) {
        match self.left.take() {
            None => {}
            Some(node) => {
                node.borrow_mut().clear();
            }
        }
        self.left = None;
        match self.right.take() {
            None => {}
            Some(node) => {
                node.borrow_mut().clear();
            }
        }
        self.right = None;
    }
}

impl<T: Ord + Copy + Debug + Display> Drop for FastRBTree<T> {
    fn drop(&mut self) {
        match self.root.take() {
            Some(node) => node.borrow_mut().clear(),
            None => return,
        }
    }
}

impl<T: Ord + Copy + Debug + Display> Drop for TreeNode<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tree_traversal() {
        // Test the three different tree traversal functions.
        let mut tree = FastRBTree::new();
        tree.insert(0);
        vec![16, 16, 8, 24, 20, 22].iter().for_each(|v| {
            tree.insert(*v);
        });
        let mut container = vec![];
        tree.pre_order_traversal_for_test(&mut container);
        assert_eq!(container, vec![20, 8, 0, 16, 24, 22]);
    }

    #[test]
    fn test_insert() {
        let mut rb_tree = FastRBTree::new();
        rb_tree.insert(12);
        rb_tree.insert(1);
        rb_tree.insert(9);
        rb_tree.insert(2);
        rb_tree.insert(0);
        rb_tree.insert(11);
        rb_tree.insert(7);
        rb_tree.insert(19);
        rb_tree.insert(4);
        rb_tree.insert(15);
        rb_tree.insert(18);
        rb_tree.insert(5);
        rb_tree.insert(14);
        rb_tree.insert(13);
        rb_tree.insert(10);
        rb_tree.insert(16);
        rb_tree.insert(6);
        rb_tree.insert(3);
        rb_tree.insert(8);
        rb_tree.insert(17);

        let result = FastRBTree::is_valid_red_black_tree(rb_tree.root.clone());
        assert_eq!(result, true);
    }
}
