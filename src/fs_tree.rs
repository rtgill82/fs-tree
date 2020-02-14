use std::cell::{RefCell,RefMut};
use std::path::PathBuf;
use std::result;

pub mod fs_tree_builder;
pub mod into_iter;
pub mod iter;

use crate::error::Error;
use crate::read_dir::ReadDir;
use self::into_iter::IntoIter;
use self::iter::Iter;

pub type Result = result::Result<PathBuf, Error>;

///
/// An iterator that traverses a file system directory tree.
///
#[derive(Default)]
pub struct FsTree
{
    top: PathBuf,
    stack: RefCell<Vec<ReadDir>>,
    ignore_files: Option<Vec<PathBuf>>,
    ignore_paths: Option<Vec<PathBuf>>,
    max_depth: Option<usize>,
    min_depth: usize
}

impl FsTree {
    pub fn iter(&self) -> Iter {
        Iter(self)
    }

    pub(crate) fn top(&self) -> &PathBuf {
        &self.top
    }

    pub(crate) fn stack(&self) -> RefMut<Vec<ReadDir>> {
        self.stack.borrow_mut()
    }

    pub(crate) fn push_dir(&self, path: &PathBuf) -> result::Result<(), Error> {
        if let Some(max_depth) = self.max_depth() {
            if self.depth() >= max_depth {
                return Ok(());
            }
        }

        let read_dir = ReadDir::new(path)?;
        self.stack().push(read_dir);

        Ok(())
    }

    pub(crate) fn depth(&self) -> usize {
        self.stack().len()
    }

    pub(crate) fn ignore_file(&self, path: &PathBuf) -> bool {
        if let Some(ignore_files) = &self.ignore_files {
            ignore_files.contains(path)
        } else {
            false
        }
    }

    pub(crate) fn ignore_path(&self, path: &PathBuf) -> bool {
        if let Some(ignore_paths) = &self.ignore_paths {
            ignore_paths.contains(path)
        } else {
            false
        }
    }

    pub(crate) fn max_depth(&self) -> Option<usize> {
        self.max_depth
    }

    pub(crate) fn min_depth(&self) -> usize {
        self.min_depth
    }
}

impl IntoIterator for FsTree {
    type Item = Result;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

impl<'a> IntoIterator for &'a FsTree {
    type Item = Result;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Iter(self)
    }
}
