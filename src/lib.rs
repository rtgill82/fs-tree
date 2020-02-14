//!
//! An iterater that traverses and entire directory tree. It can be
//! configured to exclude specific files or entire directory trees.
//! A minimum or maximum directory depth may also be set.
//!
//! # Example Usage
//!
//! ```
//! extern crate fs_tree;
//! use fs_tree::FsTreeBuilder;
//!
//! fn main() {
//!     let fs_tree = FsTreeBuilder::new("/var")
//!         .max_depth(3)
//!         .ignore_files(&["/var/log/lastlog"])
//!         .build();
//!
//!     for file in fs_tree {
//!         println!("{:?}", file);
//!     }
//! }
//! ```
//!
#[doc(hidden)]
pub mod error;
#[doc(inline)]
pub use error::Error;

#[doc(hidden)]
pub mod fs_tree;
#[doc(inline)]
pub use crate::fs_tree::FsTree;
#[doc(inline)]
pub use crate::fs_tree::fs_tree_builder::FsTreeBuilder;
#[doc(hidden)]
pub use crate::fs_tree::into_iter::IntoIter;
#[doc(hidden)]
pub use crate::fs_tree::iter::Iter;
#[doc(inline)]
pub use crate::fs_tree::Result;

#[doc(hidden)]
mod fs_tree_iter;
#[doc(hidden)]
mod read_dir;
