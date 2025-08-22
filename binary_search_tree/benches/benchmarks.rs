use binary_search_tree::{BinarySearchTree, Node};
use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn create_large_tree(size: usize) -> BinarySearchTree {
    let mut bst = BinarySearchTree::new();

    // Insert random-ish values to create a somewhat balanced tree
    for i in 0..size {
        let key = (i * 17 + 31) % (size * 2); // Pseudo-random distribution
        bst.insert(Box::new(Node {
            key: key as i64,
            value: format!("Node {}", key),
            left: None,
            right: None,
        }));
    }
    bst
}

fn benchmark_insert(c: &mut Criterion) {
    c.bench_function("insert_1000", |b| {
        b.iter(|| {
            let mut bst = BinarySearchTree::new();
            for i in 0..1000 {
                bst.insert(Box::new(Node {
                    key: black_box(i),
                    value: format!("Node {}", i),
                    left: None,
                    right: None,
                }));
            }
        })
    });

    c.bench_function("insert_10000", |b| {
        b.iter(|| {
            let mut bst = BinarySearchTree::new();
            for i in 0..10000 {
                bst.insert(Box::new(Node {
                    key: black_box(i),
                    value: format!("Node {}", i),
                    left: None,
                    right: None,
                }));
            }
        })
    });
}

fn benchmark_search(c: &mut Criterion) {
    let small_tree = create_large_tree(1000);
    let large_tree = create_large_tree(10000);

    c.bench_function("search_1000_tree", |b| {
        b.iter(|| {
            for i in 0..100 {
                let key = black_box((i * 17 + 31) % 2000);
                small_tree.search(key);
            }
        })
    });

    c.bench_function("search_10000_tree", |b| {
        b.iter(|| {
            for i in 0..100 {
                let key = black_box((i * 17 + 31) % 20000);
                large_tree.search(key);
            }
        })
    });

    c.bench_function("search_worst_case", |b| {
        // Linear tree (worst case)
        let mut linear_tree = BinarySearchTree::new();
        for i in 0..1000 {
            linear_tree.insert(Box::new(Node {
                key: i,
                value: format!("Node {}", i),
                left: None,
                right: None,
            }));
        }

        b.iter(|| {
            linear_tree.search(black_box(999)); // Search for last element
        })
    });
}

fn benchmark_delete(c: &mut Criterion) {
    c.bench_function("delete_leaf_nodes", |b| {
        b.iter_batched(
            || create_large_tree(1000),
            |mut tree| {
                // Delete some leaf nodes
                for i in (0..100).step_by(10) {
                    let key = (i * 17 + 31) % 2000;
                    tree.delete(black_box(key));
                }
            },
            criterion::BatchSize::SmallInput,
        )
    });

    c.bench_function("delete_internal_nodes", |b| {
        b.iter_batched(
            || create_large_tree(1000),
            |mut tree| {
                // Delete nodes that likely have children
                for i in (0..50).step_by(5) {
                    let key = (i * 17 + 31) % 1000;
                    tree.delete(black_box(key));
                }
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn benchmark_mixed_operations(c: &mut Criterion) {
    c.bench_function("mixed_operations", |b| {
        b.iter(|| {
            let mut bst = BinarySearchTree::new();

            // Insert 500 nodes
            for i in 0..500 {
                bst.insert(Box::new(Node {
                    key: black_box(i),
                    value: format!("Node {}", i),
                    left: None,
                    right: None,
                }));
            }

            // Search 100 times
            for i in 0..100 {
                bst.search(black_box(i * 5));
            }

            // Delete 50 nodes
            for i in (0..50).step_by(1) {
                bst.delete(black_box(i * 10));
            }

            // Insert 50 more
            for i in 500..550 {
                bst.insert(Box::new(Node {
                    key: black_box(i),
                    value: format!("Node {}", i),
                    left: None,
                    right: None,
                }));
            }
        })
    });
}

criterion_group!(
    benches,
    benchmark_insert,
    benchmark_search,
    benchmark_delete,
    benchmark_mixed_operations
);
criterion_main!(benches);

