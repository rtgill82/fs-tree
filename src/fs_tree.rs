use std::cell::{Cell,RefCell,RefMut};
use std::path::PathBuf;
use std::result::Result as StdResult;

pub mod fs_tree_builder;
pub mod into_iter;
pub mod iter;

use crate::error::Error;
use crate::read_dir::ReadDir;
use self::into_iter::IntoIter;
use self::iter::Iter;

pub type Result = StdResult<PathBuf, Error>;

pub struct FsTree
{
    top: Cell<Option<ReadDir>>,
    stack: RefCell<Vec<ReadDir>>,
    ignore_files: Option<Vec<PathBuf>>,
    ignore_paths: Option<Vec<PathBuf>>,
    max_depth: Option<usize>
}

impl FsTree {
    pub fn iter(&self) -> Iter {
        Iter(self)
    }

    pub(crate) fn top(&self) -> Option<ReadDir> {
        self.top.replace(None)
    }

    pub(crate) fn stack(&self) -> RefMut<Vec<ReadDir>> {
        self.stack.borrow_mut()
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

    pub(crate) fn depth(&self) -> usize {
        self.stack.borrow().len()
    }

    pub(crate) fn max_depth(&self) -> Option<usize> {
        self.max_depth
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
