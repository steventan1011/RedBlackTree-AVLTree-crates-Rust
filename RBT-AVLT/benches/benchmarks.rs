use tree_collections::prelude::*;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::seq::{IteratorRandom, SliceRandom};
use rand::{rngs::StdRng, SeedableRng};

const N: usize = 64;
pub struct MyRngSeed(pub [u8; N]);
pub struct MyRng(MyRngSeed);

const TREE_SIZE: [u32; 5] = [10_000, 40_000, 70_000, 100_000, 130_000];

fn create_shuffled_data(tree_size: u32) -> Vec<u32> {
    let seed = [0u8; 32];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let mut data: Vec<u32> = (0..tree_size).collect();
    data.shuffle(&mut rng);
    return data;
}

fn benchmark_avl_ordered_insert(tree_size: u32) {
    let mut avl = AVLTree::new();
    for v in 0..tree_size {
        avl.insert(v);
    }
}

fn benchmark_fast_rbt_ordered_insert(tree_size: u32) {
    let mut rbt = FastRBTree::new();
    for v in 0..tree_size {
        rbt.insert(v);
    }
}

fn benchmark_rbt_ordered_insert(tree_size: u32) {
    let mut rbt = RBTree::new();
    for v in 0..tree_size {
        rbt.insert(v);
    }
}

fn benchmark_avl_ordered_delete(mut avl: AVLTree<u32>, tree_size: u32) {
    for v in 0..tree_size / 10 {
        avl.delete(v);
    }
}

fn benchmark_rbt_ordered_delete(mut rbt: RBTree<u32>, tree_size: u32) {
    for v in 0..tree_size / 10 {
        rbt.delete(v);
    }
}

// fn benchmark_fast_rbt_ordered_delete(mut fast_rbt: FastRBTree<u32>, tree_size: u32) {
//     for v in 0..tree_size / 10 {
//         fast_rbt.delete(v);
//     }
// }

fn benchmark_avl_ordered_search(avl: AVLTree<u32>, tree_size: u32) {
    for v in 0..tree_size / 10 {
        avl.contains(v);
    }
}

fn benchmark_rbt_ordered_search(rbt: RBTree<u32>, tree_size: u32) {
    for v in 0..tree_size / 10 {
        rbt.contains(v);
    }
}

fn benchmark_fast_rbt_ordered_search(fast_rbt: FastRBTree<u32>, tree_size: u32) {
    for v in 0..tree_size / 10 {
        fast_rbt.contains(v);
    }
}

fn benchmark_avl_random_insert(tree_size: u32) {
    let data: Vec<u32> = create_shuffled_data(tree_size);
    let mut avl = AVLTree::new();
    for v in &data {
        avl.insert(*v);
    }
}

fn benchmark_rbt_random_insert(tree_size: u32) {
    let data: Vec<u32> = create_shuffled_data(tree_size);
    let mut rbt = RBTree::new();
    for v in &data {
        rbt.insert(*v);
    }
}

fn benchmark_fast_rbt_random_insert(tree_size: u32) {
    let data: Vec<u32> = create_shuffled_data(tree_size);
    let mut fast_rbt = FastRBTree::new();
    for v in &data {
        fast_rbt.insert(*v);
    }
}

// fn benchmark_bst_random_insert(tree_size: u32) {
//     let data: Vec<u32> = create_shuffled_data(tree_size);
//     let mut avl = BSTree::new();
//     for v in &data {
//         avl.insert(v);
//     }
// }

// fn benchmark_bst_random_delete(mut bst: BSTree<u32>, tree_size: u32) {
//     let data: Vec<u32> = create_shuffled_data(tree_size);
//     let seed = [0u8; 32];
//     let mut rng: StdRng = SeedableRng::from_seed(seed);
//     let sample = data
//         .iter()
//         .choose_multiple(&mut rng, (tree_size / 100) as usize);

//     for v in sample.iter() {
//         bst.delete(**v);
//     }
// }

fn benchmark_avl_random_delete(mut avl: AVLTree<u32>, tree_size: u32) {
    let data: Vec<u32> = create_shuffled_data(tree_size);
    let seed = [0u8; 32];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let sample = data
        .iter()
        .choose_multiple(&mut rng, (tree_size / 100) as usize);

    for v in sample.iter() {
        avl.delete(**v);
    }
}

fn benchmark_rbt_random_delete(mut rbt: RBTree<u32>, tree_size: u32) {
    let data: Vec<u32> = create_shuffled_data(tree_size);
    let seed = [0u8; 32];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let sample = data
        .iter()
        .choose_multiple(&mut rng, (tree_size / 100) as usize);

    for v in sample.iter() {
        rbt.delete(**v);
    }
}

// fn benchmark_fast_rbt_random_delete(fast_rbt: FastRBTree<u32>, tree_size: u32) {
//     let data: Vec<u32> = create_shuffled_data(tree_size);
//     let seed = [0u8; 32];
//     let mut rng: StdRng = SeedableRng::from_seed(seed);
//     let sample = data.iter().choose_multiple(&mut rng, (tree_size / 100) as usize);

//     for v in sample.iter() {
//         fast_rbt.delete(**v);
//     }
// }

fn benchmark_avl_random_search(avl: AVLTree<u32>, tree_size: u32) {
    let data: Vec<u32> = create_shuffled_data(tree_size);
    let seed = [0u8; 32];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let sample = data
        .iter()
        .choose_multiple(&mut rng, (tree_size / 100) as usize);

    for v in sample.iter() {
        avl.contains(**v);
    }
}

fn benchmark_rbt_random_search(rbt: RBTree<u32>, tree_size: u32) {
    let data: Vec<u32> = create_shuffled_data(tree_size);
    let seed = [0u8; 32];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let sample = data
        .iter()
        .choose_multiple(&mut rng, (tree_size / 100) as usize);

    for v in sample.iter() {
        rbt.contains(**v);
    }
}

fn benchmark_fast_rbt_random_search(fast_rbt: FastRBTree<u32>, tree_size: u32) {
    let data: Vec<u32> = create_shuffled_data(tree_size);
    let seed = [0u8; 32];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let sample = data
        .iter()
        .choose_multiple(&mut rng, (tree_size / 100) as usize);

    for v in sample.iter() {
        fast_rbt.contains(**v);
    }
}

// fn benchmark_bst_random_search(bst: BSTree<u32>, tree_size: u32) {
//     let data: Vec<u32> = create_shuffled_data(tree_size);
//     let seed = [0u8; 32];
//     let mut rng: StdRng = SeedableRng::from_seed(seed);
//     let sample = data
//         .iter()
//         .choose_multiple(&mut rng, (tree_size / 100) as usize);

//     for v in sample.iter() {
//         bst.contains(**v);
//     }
// }


fn bench_compare_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("Insert");
    for (i, size) in TREE_SIZE.iter().enumerate() {
        group.bench_with_input(BenchmarkId::new("Ordered AVL", i), size, |b, n| {
            b.iter(|| benchmark_avl_ordered_insert(*n))
        });
        group.bench_with_input(BenchmarkId::new("Ordered RBT", i), size, |b, n| {
            b.iter(|| benchmark_rbt_ordered_insert(*n));
        });
        group.bench_with_input(BenchmarkId::new("Ordered Fast RBT", i), size, |b, n| {
            b.iter(|| benchmark_fast_rbt_ordered_insert(*n));
        });

        group.bench_with_input(BenchmarkId::new("Ramdom AVL", i), size, |b, n| {
            b.iter(|| benchmark_avl_random_insert(*n))
        });
        group.bench_with_input(BenchmarkId::new("Ramdom RBT", i), size, |b, n| {
            b.iter(|| benchmark_rbt_random_insert(*n));
        });
        group.bench_with_input(BenchmarkId::new("Ramdom Fast RBT", i), size, |b, n| {
            b.iter(|| benchmark_fast_rbt_random_insert(*n));
        });
        // group.bench_with_input(BenchmarkId::new("Ramdom BST", i), size, |b, n| {
        //     b.iter(|| benchmark_bst_random_insert(*n));
        // });
    }
    group.finish();
}

fn bench_compare_search(c: &mut Criterion) {
    let mut group = c.benchmark_group("Search");
    for (i, size) in TREE_SIZE.iter().enumerate() {
        group.bench_function(BenchmarkId::new("Ordered AVL", i), |b| {
            let mut avl = AVLTree::new();
            for v in 0..*size {
                avl.insert(v);
            }

            b.iter(|| {
                let avl_cp = avl.clone();
                benchmark_avl_ordered_search(avl_cp, *size);
            })
        });
        group.bench_function(BenchmarkId::new("Ordered RBT", i), |b| {
            let mut rbt = RBTree::new();
            for v in 0..*size {
                rbt.insert(v);
            }

            b.iter(|| {
                let rbt_cp = rbt.clone();
                benchmark_rbt_ordered_search(rbt_cp, *size);
            })
        });
        group.bench_function(BenchmarkId::new("Ordered Fast RBT", i), |b| {
            let mut fast_rbt = FastRBTree::new();
            for v in 0..*size {
                fast_rbt.insert(v);
            }

            b.iter(|| {
                let fast_rbt_cp = fast_rbt.clone();
                benchmark_fast_rbt_ordered_search(fast_rbt_cp, *size);
            })
        });
        group.bench_function(BenchmarkId::new("Random AVL", i), |b| {
            let data: Vec<u32> = create_shuffled_data(*size);
            let mut avl = AVLTree::new();
            for v in &data {
                avl.insert(*v);
            }

            b.iter(|| {
                let avl_cp = avl.clone();
                benchmark_avl_random_search(avl_cp, *size);
            })
        });
        group.bench_function(BenchmarkId::new("Random RBT", i), |b| {
            let data: Vec<u32> = create_shuffled_data(*size);
            let mut rbt = RBTree::new();
            for v in &data {
                rbt.insert(*v);
            }

            b.iter(|| {
                let rbt_cp = rbt.clone();
                benchmark_rbt_random_search(rbt_cp, *size);
            })
        });
        group.bench_function(BenchmarkId::new("Random Fast RBT", i), |b| {
            let mut fast_rbt = FastRBTree::new();
            for v in 0..*size {
                fast_rbt.insert(v);
            }

            b.iter(|| {
                let fast_rbt_cp = fast_rbt.clone();
                benchmark_fast_rbt_random_search(fast_rbt_cp, *size);
            })
        });
        // group.bench_function(BenchmarkId::new("Random BST", i), |b| {
        //     let mut bst = BSTree::new();
        //     for v in 0..*size {
        //         bst.insert(v);
        //     }

        //     b.iter(|| {
        //         let bst_cp = bst.clone();
        //         benchmark_bst_random_search(bst_cp, *size);
        //     })
        // });
    }
    group.finish();
}

fn bench_compare_delete(c: &mut Criterion) {
    let mut group = c.benchmark_group("Delete");
    for (i, size) in TREE_SIZE.iter().enumerate() {
        group.bench_function(BenchmarkId::new("Ordered AVL", i), |b| {
            let mut avl = AVLTree::new();
            for v in 0..*size {
                avl.insert(v);
            }

            b.iter(|| {
                let avl_cp = avl.clone();
                benchmark_avl_ordered_delete(avl_cp, *size);
            })
        });
        group.bench_function(BenchmarkId::new("Ordered RBT", i), |b| {
            let mut rbt = RBTree::new();
            for v in 0..*size {
                rbt.insert(v);
            }

            b.iter(|| {
                let rbt_cp = rbt.clone();
                benchmark_rbt_ordered_delete(rbt_cp, *size);
            })
        });
        group.bench_function(BenchmarkId::new("Random AVL", i), |b| {
            let data: Vec<u32> = create_shuffled_data(*size);
            let mut avl = AVLTree::new();
            for v in &data {
                avl.insert(*v);
            }

            b.iter(|| {
                let avl_cp = avl.clone();
                benchmark_avl_random_delete(avl_cp, *size);
            })
        });
        group.bench_function(BenchmarkId::new("Random RBT", i), |b| {
            let data: Vec<u32> = create_shuffled_data(*size);
            let mut rbt = RBTree::new();
            for v in &data {
                rbt.insert(*v);
            }

            b.iter(|| {
                let rbt_cp = rbt.clone();
                benchmark_rbt_random_delete(rbt_cp, *size);
            })
        });
        // group.bench_function(BenchmarkId::new("Random BST", i), |b| {
        //     let data: Vec<u32> = create_shuffled_data(*size);
        //     let mut bst = BSTree::new();
        //     for v in &data {
        //         bst.insert(*v);
        //     }

        //     b.iter(|| {
        //         let bst_cp = bst.clone();
        //         benchmark_bst_random_delete(bst_cp, *size);
        //     })
        // });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_compare_insert,
    bench_compare_search,
    bench_compare_delete
);
criterion_main!(benches);
