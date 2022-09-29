# Tree_collections

The purpose of this library is to provide APIs that allows users to create memory efficient binary search tree, red-black tree and AVL tree. Besides, by using this library, users can investigate the performance difference between red-black tree and avl tree, which helps them deeply understand the algorithms.

## Quick Start

```rust
use tree_collections::prelude::*;

let mut rb_tree = RBTree::new();
rb_tree.insert(1);
rb_tree.insert(2);
rb_tree.insert(3);
rb_tree.insert(4);
rb_tree.insert(5);
rb_tree.delete(3);
println!("Red-Black Tree:");
rb_tree.print();
println!("Number of leaves: {:?}", rb_tree.count_leaves());
println!("Height of tree: {:?}", rb_tree.height());

let mut avl_tree = AVLTree::new();
avl_tree.insert(1);
avl_tree.insert(2);
avl_tree.insert(3);
avl_tree.insert(4);
avl_tree.insert(5);
avl_tree.delete(3);
println!("AVL Tree:");
avl_tree.print();
println!("Number of leaves: {:?}", rb_tree.count_leaves());
println!("Height of tree: {:?}", rb_tree.height());

```
## Documentation

Building the documentation using

```
$ cargo doc
```
Find the API doc at 

```
./target/doc/tree_collections/index.html
```
## User Promote

Run the user promote

```
$ cargo run
```

List of operations
```
$ insert
$ delete
$ count
$ height
$ inorder print
$ preorder print
$ empty
$ search
$ print tree
$ exit
```

## Testing

Run the tests using

```
$ cargo test
```

## Benchmark

Run the Benchmark using

```
$ cargo bench
```

## Requirements

Cargo version: 1.56^

Ref: https://blog.rust-lang.org/2021/10/21/Rust-1.56.0.html
