# fs-tree

_An iterator that traverses an entire directory tree._

An iterater that traverses and entire directory tree. It can be
configured to exclude specific files or entire directory trees.
A minimum or maximum directory depth may also be set.

# Example Usage

```rust
extern crate fs_tree;
use fs_tree::FsTreeBuilder;

fn main() {
    let fs_tree = FsTreeBuilder::new("/var")
        .max_depth(3)
        .ignore_files(&["/var/log/lastlog"])
        .build();

    for file in fs_tree {
        println!("{:?}", file);
    }
}
