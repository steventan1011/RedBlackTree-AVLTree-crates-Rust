//! An automatically-implemented extension trait for nodes and trees
//!
//! Provides common functions for trees and nodes.

use std::cell::RefCell;
use std::cmp::max;
use std::fmt::{Debug, Display};
use std::rc::Rc;

/// Provide common functions for trees
// Common trait for Tree
pub trait CommonTreeTrait<T: Ord + Copy + Debug + Display, TreeNode: CommonTreeNodeTrait<T>> {
    fn get_root(&self) -> Option<Rc<RefCell<TreeNode>>>;

    /// Counts leaves(None nodes) of the Tree
    ///
    /// # Example
    ///
    /// ```compile_fail
    /// use tree_collections::commonTrait::CommonTreeTrait;
    /// let mut tree = CommonTreeTrait::new();
    /// tree.insert(1);
    /// println!("{}", tree.count_leaves());  // 2
    /// tree.insert(2);
    /// println!("{}", tree.count_leaves());  // 3
    /// tree.insert(3);
    /// println!("{}", tree.count_leaves());  // 4
    ///
    /// let mut leaf_number = tree.count_leaves();
    /// assert_eq!(4, leaf_number);
    /// ```
    // count the leaves (None nodes)
    fn count_leaves(&self) -> u32 {
        match self.get_root() {
            None => 0,
            Some(node) => node.borrow().count_leaves(),
        }
    }

    /// Gets height of the Tree (from root to leaves)
    ///
    /// # Example
    ///
    /// ```compile_fail
    /// use tree_collections::commonTrait::CommonTreeTrait;
    /// let mut tree = CommonTreeTrait::new();
    /// assert_eq!(0, tree.height());
    /// tree.insert(1);
    /// assert_eq!(2, tree.height());
    ///tree.insert(2);
    /// assert_eq!(3, tree.height());
    /// ```
    // from root to leaves
    fn height(&self) -> u32 {
        match self.get_root() {
            None => 0,
            Some(node) => node.borrow().get_height(),
        }
    }

    /// Prints Tree inorder
    ///
    /// # Example
    ///
    /// ```compile_fail
    /// use tree_collections::commonTrait::CommonTreeTrait;
    /// let mut tree = CommonTreeTrait::new();
    /// tree.insert(1);
    /// tree.insert(3);
    /// tree.insert(5);
    /// tree.insert(2);
    /// tree.insert(4);
    /// tree.in_order_traversal(); // Inorder traversal: 1 2 3 4 5
    ///
    fn in_order_traversal(&self) {
        match self.get_root() {
            None => println!("There is no node in the tree!"),
            Some(node) => {
                node.borrow().in_order_traversal();
                println!();
            }
        }
    }

    fn in_order_traversal_for_test(&self, container: &mut Vec<T>) {
        match self.get_root() {
            None => println!("There is no node in the tree!"),
            Some(node) => {
                node.borrow().in_order_traversal_for_test(container);
            }
        }
    }

    /// Prints red-black tree preorder
    ///
    /// # Example
    ///
    /// ```compile_fail
    /// use tree_collections::commonTrait::CommonTreeTrait;
    /// let mut tree = CommonTreeTrait::new();
    /// tree.insert(1);
    /// tree.insert(3);
    /// tree.insert(5);
    /// tree.insert(2);
    /// tree.insert(4);
    /// tree.preorder_traversal(); // Preorder traversal: 3 2 1 5 4
    ///
    fn pre_order_traversal(&self) {
        match self.get_root() {
            None => println!("There is no node in the tree!"),
            Some(node) => {
                node.borrow().pre_order_traversal();
                println!();
            }
        }
    }

    fn pre_order_traversal_for_test(&self, container: &mut Vec<T>) {
        match self.get_root() {
            None => println!("There is no node in the tree!"),
            Some(node) => {
                node.borrow().pre_order_traversal_for_test(container);
            }
        }
    }

    /// Determine whether the tree contains given value
    ///
    /// # Example
    ///
    /// ```compile_fail
    /// use tree_collections::commonTrait::CommonTreeTrait;
    /// let mut tree = CommonTreeTrait::new();
    /// tree.insert(1);
    /// assert_eq!(true, tree.contains(1));
    /// assert_eq!(false, tree.contains(0));
    ///
    fn contains(&self, value: T) -> bool {
        match self.get_root() {
            None => false,
            Some(node) => node.borrow().contains(value),
        }
    }

    // judge if the tree is empty
    fn is_tree_empty(&self) -> bool {
        self.get_root().map(|_| false).unwrap_or(true)
    }

    fn min(&self) -> Option<T> {
        match self.get_root() {
            None => None,
            Some(node) => Some(node.borrow().get_min_value_in_children()),
        }
    }

    fn max(&self) -> Option<T> {
        match self.get_root() {
            None => None,
            Some(node) => Some(node.borrow().get_max_value_in_children()),
        }
    }

    fn print(&self) {
        match self.get_root() {
            None => println!("This is an empty tree."),
            Some(node) => node.borrow().print(),
        }
    }
}

/// Provide common functions for nodes
// Common trait for TreeNode
pub trait CommonTreeNodeTrait<T: Ord + Copy + Debug + Display> {
    /// Get left child node
    fn get_left(&self) -> Option<Rc<RefCell<Self>>>;

    /// Get right child node
    fn get_right(&self) -> Option<Rc<RefCell<Self>>>;

    /// Get value from current node
    fn get_value(&self) -> T;

    /// Get value string from current node
    fn get_value_to_print(&self) -> String;

    /// Return the leaves number of current node, which will be called by
    /// [CommonTreeTrait.count_leaves](trait.CommonTreeTrait.html#method.count_leaves)
    fn count_leaves(&self) -> u32 {
        let left = self.get_left();
        let right = self.get_right();
        if left.is_none() && right.is_none() {
            return 1;
        } else if left.is_none() && right.is_some() {
            return right.unwrap().borrow().count_leaves();
        } else if left.is_some() && right.is_none() {
            return left.unwrap().borrow().count_leaves();
        } else {
            return right.unwrap().borrow().count_leaves() + left.unwrap().borrow().count_leaves();
        }
    }

    /// Return the height of current node, which will be called by
    /// [CommonTreeTrait.height](trait.CommonTreeTrait.html#method.height)
    fn get_height(&self) -> u32 {
        let left = self.get_left();
        let right = self.get_right();
        let left_height = match left {
            None => 0,
            Some(l) => l.borrow().get_height(),
        };
        let right_height = match right {
            None => 0,
            Some(r) => r.borrow().get_height(),
        };
        return max(left_height, right_height) + 1;
    }

    /// Print nodes inorder, which will be called by
    /// [CommonTreeTrait.in_order_traversal](trait.CommonTreeTrait.html#method.in_order_traversal)
    fn in_order_traversal(&self) {
        let left = self.get_left();
        if left.is_some() {
            left.unwrap().borrow().in_order_traversal();
        }
        print!("{} ", self.get_value_to_print());
        let right = self.get_right();
        if right.is_some() {
            right.unwrap().borrow().in_order_traversal();
        }
    }

    fn in_order_traversal_for_test(&self, container: &mut Vec<T>) {
        let left = self.get_left();
        if left.is_some() {
            left.unwrap()
                .borrow()
                .in_order_traversal_for_test(container);
        }
        container.push(self.get_value());
        let right = self.get_right();
        if right.is_some() {
            right
                .unwrap()
                .borrow()
                .in_order_traversal_for_test(container);
        }
    }

    /// Print nodes preorder, which will be called by
    /// [CommonTreeTrait.pre_order_traversal](trait.CommonTreeTrait.html#method.pre_order_traversal)
    fn pre_order_traversal(&self) {
        print!("{} ", self.get_value_to_print());
        let left = self.get_left();
        if left.is_some() {
            left.unwrap().borrow().pre_order_traversal();
        }
        let right = self.get_right();
        if right.is_some() {
            right.unwrap().borrow().pre_order_traversal();
        }
    }

    fn pre_order_traversal_for_test(&self, container: &mut Vec<T>) {
        container.push(self.get_value());
        let left = self.get_left();
        if left.is_some() {
            left.unwrap()
                .borrow()
                .pre_order_traversal_for_test(container);
        }
        let right = self.get_right();
        if right.is_some() {
            right
                .unwrap()
                .borrow()
                .pre_order_traversal_for_test(container);
        }
    }

    /// Determine whether the node and its successors contains given value,
    /// which will be called by
    /// [CommonTreeTrait.contains](trait.CommonTreeTrait.html#method.contains)
    fn contains(&self, value: T) -> bool {
        let current_value = self.get_value();
        return if current_value == value {
            true
        } else if current_value > value {
            match self.get_left() {
                None => false,
                Some(node) => node.borrow().contains(value),
            }
        } else {
            match self.get_right() {
                None => false,
                Some(node) => node.borrow().contains(value),
            }
        };
    }

    // find the min value in its children
    fn get_min_value_in_children(&self) -> T {
        match self.get_left() {
            Some(left) => left.borrow().get_min_value_in_children(),
            None => self.get_value(),
        }
    }

    // find the max value in its children
    fn get_max_value_in_children(&self) -> T {
        match self.get_right() {
            Some(right) => right.borrow().get_max_value_in_children(),
            None => self.get_value(),
        }
    }

    // print the tree with structure
    fn print(&self) {
        // get height
        let height = self.get_height() as usize;

        // the last row's width is 2^(n-1) * 3 + 1
        // to be the array's width
        let array_height = height * 2 - 1;
        let array_width = (2 << (height - 2)) * 3 + 1 as usize;
        // use an array to store all the elements
        let mut container_raw = vec![String::from(" "); array_width * array_height];
        let mut container_base: Vec<_> = container_raw
            .as_mut_slice()
            .chunks_mut(array_width)
            .collect();
        let container: &mut [&mut [String]] = container_base.as_mut_slice();

        // use print_helper to recursive traverse the tree
        self.print_helper(0, array_width / 2, container, height);

        // concatenate and print the structure of the tree
        for i in 0..container.len() {
            let mut line = String::new();
            for mut j in 0..container[i].len() {
                line += &container[i][j];
                if container[i][j].len() > 1 && j < container[i].len() {
                    if container[i][j].len() > 4 {
                        j += 2;
                    } else {
                        j += container[i][j].len() - 1;
                    }
                }
            }
            println!("{}", line);
        }
    }

    fn print_helper(
        &self,
        row_index: usize,
        column_index: usize,
        container: &mut [&mut [String]],
        height: usize,
    ) {
        // save current node into the container
        container[row_index][column_index] = self.get_value_to_print();
        // get current height
        let curr_height = (row_index + 1) / 2;

        // if it is the last level, then return
        if curr_height == height {
            return;
        }
        // compute the gap between curr height and the next level
        let gap = height - curr_height - 1;

        // if it has left child, then record "/" and the left node's value
        match self.get_left() {
            None => (),
            Some(node) => {
                container[row_index + 1][column_index - gap] = String::from("/");
                let left_child = node.borrow();
                left_child.print_helper(row_index + 2, column_index - gap * 2, container, height);
            }
        }

        // if it has right child, then record "\" and the right node's value
        match self.get_right() {
            None => (),
            Some(node) => {
                container[row_index + 1][column_index + gap] = String::from("\\");
                let right_child = node.borrow();
                right_child.print_helper(row_index + 2, column_index + gap * 2, container, height);
            }
        }
    }
}
