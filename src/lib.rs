pub mod error;
pub use error::Error;

pub mod fs_tree;
pub use crate::fs_tree::FsTree;
pub use crate::fs_tree::fs_tree_builder::FsTreeBuilder;
pub use crate::fs_tree::into_iter::IntoIter;
pub use crate::fs_tree::iter::Iter;
pub use crate::fs_tree::Result;

mod fs_tree_iter;
mod read_dir;
