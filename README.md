# ECE-522-2021-FALL-PROJECT

The purpose of this library is to provide APIs that allows users to create memory efficient red-black tree and avl tree. Besides, by using this library, users can investigate the performance difference between red-black tree and avl tree, which helps them deeply understand the algorithms.

## Quick Start

```rust
use tree_collections::prelude::*;

let mut rb_tree = RBTree::new();
rb_tree.insert(1);
rb_tree.delete(1);

let mut avl_tree = AVLTree::new();
avl_tree.insert(1);
avl_tree.delete(1);

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

